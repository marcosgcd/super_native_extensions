use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    rc::{Rc, Weak},
    sync::Arc,
};

// hallo
// hallo 2

use irondash_engine_context::EngineContext;
use irondash_message_channel::{Late, Value};
use irondash_run_loop::{platform::PollSession, RunLoop};
use log::warn;
use windows::{
    core::{implement, ComInterface, PCWSTR},
    Win32::{
        Foundation::{E_OUTOFMEMORY, HWND, POINT, POINTL, S_OK},
        Graphics::Gdi::ScreenToClient,
        System::{
            Com::IDataObject,
            LibraryLoader::GetModuleHandleW,
            Ole::{
                IDropTarget, IDropTarget_Impl, RegisterDragDrop, RevokeDragDrop, DROPEFFECT,
                DROPEFFECT_NONE,
            },
            SystemServices::MODIFIERKEYS_FLAGS,
            Threading::{GetCurrentProcessId, GetCurrentThreadId},
        },
        UI::{
            Accessibility::{SetWinEventHook, UnhookWinEvent, HWINEVENTHOOK},
            Shell::{CLSID_DragDropHelper, IDataObjectAsyncCapability, IDropTargetHelper},
            WindowsAndMessaging::{EVENT_OBJECT_DESTROY, OBJID_WINDOW, WINEVENT_INCONTEXT},
        },
    },
};

use crate::{
    api_model::{DropOperation, Point},
    drop_manager::{
        BaseDropEvent, DropEvent, DropItem, DropSessionId, PlatformDropContextDelegate,
        PlatformDropContextId,
    },
    error::{NativeExtensionsError, NativeExtensionsResult},
    log::OkLog,
    reader_manager::RegisteredDataReader,
    util::{DropNotifier, NextId},
};

use super::{
    common::{create_instance, get_dpi_for_window},
    drag_common::DropOperationExt,
    PlatformDataReader,
};

pub struct PlatformDropContext {
    id: PlatformDropContextId,
    weak_self: Late<Weak<Self>>,
    view: HWND,
    delegate: Weak<dyn PlatformDropContextDelegate>,
    hook: Late<HWINEVENTHOOK>,
    next_session_id: Cell<i64>,
    current_session: RefCell<Option<Rc<Session>>>,
}

thread_local! {
    static HOOK_TO_HWND: RefCell<HashMap<isize, HWND>> = RefCell::new(HashMap::new());
}

struct Session {
    id: DropSessionId,
    is_inside: Cell<bool>,
    missing_drop_end: Cell<bool>,
    data_object: IDataObject,
    last_operation: Cell<DropOperation>,
    async_result: Rc<RefCell<Option<(IDataObjectAsyncCapability, DROPEFFECT)>>>,
    reader: Rc<PlatformDataReader>,
    registered_reader: RegisteredDataReader,
}

impl PlatformDropContext {
    pub fn new(
        id: PlatformDropContextId,
        engine_handle: i64,
        delegate: Weak<dyn PlatformDropContextDelegate>,
    ) -> NativeExtensionsResult<Self> {
        let view = EngineContext::get()?.get_flutter_view(engine_handle)?;
        Ok(Self {
            id,
            weak_self: Late::new(),
            view: HWND(view),
            delegate,
            hook: Late::new(),
            next_session_id: Cell::new(0),
            current_session: RefCell::new(None),
        })
    }

    pub fn register_drop_formats(&self, _formats: &[String]) -> NativeExtensionsResult<()> {
        Ok(())
    }

    unsafe extern "system" fn hook_procfn(
        hwineventhook: HWINEVENTHOOK,
        _event: u32,
        hwnd: HWND,
        idobject: i32,
        _idchild: i32,
        _ideventthread: u32,
        _dwmseventtime: u32,
    ) {
        if idobject != OBJID_WINDOW.0 {
            return;
        }
        let hook_hwnd = HOOK_TO_HWND.try_with(|a| a.borrow().get(&hwineventhook.0).cloned());
        match hook_hwnd {
            Ok(hook_hwnd) => {
                if let Some(hook_hwnd) = hook_hwnd {
                    if hook_hwnd == hwnd {
                        RevokeDragDrop(hook_hwnd).ok_log();
                    }
                }
            }
            Err(_) => {
                // ignore - shutting down
            }
        }
    }

    pub fn assign_weak_self(&self, weak_self: Weak<Self>) {
        self.weak_self.set(weak_self.clone());
        let target: IDropTarget = DropTarget::new(self.view, weak_self).into();
        unsafe {
            if RevokeDragDrop(self.view).is_ok() {
                warn!("Flutter HWND had already a drop target registered!");
            }
            if let Err(err) = RegisterDragDrop(self.view, &target) {
                if err.code() == E_OUTOFMEMORY {
                    eprintln!("**");
                    eprintln!("** RegisterDragDrop failed: ");
                    eprintln!(
                        "** Please use OleInitialize instead of CoInitializeEx to initialize COM."
                    );
                    eprintln!("**");
                }
                Result::<(), _>::Err(err).ok_log();
            }

            // Unregistering in drop is too late as the HWND is already destroyed.
            // Instead we setup hook for OBJECT_DESTROY and revoke drop target there.
            let hook = SetWinEventHook(
                EVENT_OBJECT_DESTROY,
                EVENT_OBJECT_DESTROY,
                GetModuleHandleW(PCWSTR(std::ptr::null_mut())).unwrap(),
                Some(Self::hook_procfn),
                GetCurrentProcessId(),
                GetCurrentThreadId(),
                WINEVENT_INCONTEXT,
            );
            self.hook.set(hook);
            HOOK_TO_HWND.with(|a| a.borrow_mut().insert(hook.0, self.view));
        }
    }

    fn delegate(&self) -> NativeExtensionsResult<Rc<dyn PlatformDropContextDelegate>> {
        self.delegate
            .upgrade()
            .ok_or_else(|| NativeExtensionsError::OtherError("missing context delegate".into()))
    }

    fn drop_exit(&self) -> NativeExtensionsResult<()> {
        if let Some(session) = self.current_session.borrow().as_ref().cloned() {
            log::debug!("drop_exit: sending drop leave for session {:?}", session.id);
            self.delegate()?.send_drop_leave(
                self.id,
                BaseDropEvent {
                    session_id: session.id,
                },
            );
        }
        Ok(())
    }

    fn drop_end(&self) -> NativeExtensionsResult<()> {
        if let Some(session) = self.current_session.borrow_mut().take() {
            log::debug!("drop_end: ending session {:?}", session.id);
            self.delegate()?.send_drop_ended(
                self.id,
                BaseDropEvent {
                    session_id: session.id,
                },
            );
        }
        Ok(())
    }

    pub fn local_dragging_did_end(&self) -> NativeExtensionsResult<()> {
        let missing_drop_end = self
            .current_session
            .borrow()
            .as_ref()
            .map(|s| s.missing_drop_end.get())
            .unwrap_or(false);
        if missing_drop_end {
            log::debug!("local_dragging_did_end: ending session that was missing drop_end");
            self.drop_end()?;
        }
        Ok(())
    }

    fn local_dragging(&self) -> NativeExtensionsResult<bool> {
        Ok(self
            .delegate()?
            .get_platform_drag_contexts()
            .iter()
            .any(|c| c.is_dragging_active()))
    }

    fn event_for_session(
        &self,
        session: &Rc<Session>,
        pt: &POINTL,
        _grfkeystate: MODIFIERKEYS_FLAGS,
        mask: DROPEFFECT,
        accepted_operation: Option<DropOperation>,
    ) -> NativeExtensionsResult<DropEvent> {
        let local_data = self
            .delegate()?
            .get_platform_drag_contexts()
            .iter()
            .map(|c| c.get_local_data())
            .find(|c| c.is_some())
            .flatten()
            .unwrap_or_default();

        let mut pt = POINT { x: pt.x, y: pt.y };
        unsafe {
            ScreenToClient(self.view, &mut pt as *mut _);
        }
        let scaling = get_dpi_for_window(self.view) as f64 / 96.0;

        let reader_items = match session.reader.get_items_sync() {
            Ok(items) => items,
            Err(e) => {
                log::debug!("Failed to get items: {}", e);
                // Return empty items for graceful handling
                Vec::new()
            }
        };

        let items: Vec<_> = (0..local_data.len().max(reader_items.len()))
            .map(|index| {
                Ok(DropItem {
                    item_id: (index as i64).into(),
                    formats: match reader_items.get(index) {
                        Some(item) => {
                            match session.reader.get_formats_for_item_sync(*item) {
                                Ok(formats) => formats,
                                Err(e) => {
                                    log::debug!("Failed to get formats for item {}: {}", item, e);
                                    Vec::new()
                                }
                            }
                        },
                        None => Vec::new(),
                    },
                    local_data: local_data.get(index).cloned().unwrap_or(Value::Null),
                })
            })
            .collect::<NativeExtensionsResult<_>>()?;

        Ok(DropEvent {
            session_id: session.id,
            location_in_view: Point {
                x: pt.x as f64 / scaling,
                y: pt.y as f64 / scaling,
            },
            allowed_operations: DropOperation::from_platform_mask(mask),
            accepted_operation,
            items,
            reader: Some(session.registered_reader.clone()),
        })
    }

    fn on_drag_enter(
        &self,
        pdataobj: Option<&IDataObject>,
        grfkeystate: MODIFIERKEYS_FLAGS,
        pt: &POINTL,
        pdweffect: *mut DROPEFFECT,
    ) -> NativeExtensionsResult<()> {
        log::debug!("on_drag_enter: position=({}, {}), effect={:?}", pt.x, pt.y, unsafe { *pdweffect });
        if self.current_session.borrow().is_some() && !self.local_dragging()? {
            // shouldn't happen
            if self
                .current_session
                .borrow()
                .as_ref()
                .unwrap()
                .is_inside
                .get()
            {
                log::warn!("on_drag_enter: unexpected drag enter while session is inside");
                self.drop_exit()?;
            }
            log::debug!("on_drag_enter: ending previous session");
            self.drop_end()?;
        }
        let effect = unsafe { &mut *pdweffect };
        if let Some(data_object) = pdataobj {
            log::debug!("on_drag_enter: creating new session for data object");
            let delegate = self.delegate()?;
            let session = self
                .current_session
                .borrow_mut()
                .get_or_insert_with(|| {
                    let async_result = Rc::new(RefCell::new(
                        Option::<(IDataObjectAsyncCapability, DROPEFFECT)>::None,
                    ));
                    let async_result_clone = async_result.clone();
                    // Drop notifier invoked when reader gets destroyed. If we started
                    // async operation on data object this will end it.
                    let drop_notifier = Arc::new(DropNotifier::new(move || {
                        if let Some((capability, effect)) = async_result_clone.borrow_mut().take() {
                            unsafe {
                                capability.EndOperation(S_OK, None, effect.0).ok_log();
                            }
                        }
                    }));
                    let reader = PlatformDataReader::new_with_data_object(
                        data_object.clone(),
                        Some(drop_notifier),
                    );
                    let registered_reader =
                        delegate.register_platform_reader(self.id, reader.clone());
                    let session_id = self.next_session_id.next_id().into();
                    log::debug!("on_drag_enter: created new session with ID {:?}", session_id);
                    Rc::new(Session {
                        id: session_id,
                        is_inside: Cell::new(true),
                        missing_drop_end: Cell::new(false),
                        data_object: data_object.clone(),
                        last_operation: Cell::new(DropOperation::None),
                        async_result,
                        reader,
                        registered_reader,
                    })
                })
                .clone();
            session.is_inside.set(true);
            session.missing_drop_end.set(false);
            let session_clone = session.clone();
            let event = self.event_for_session(&session, pt, grfkeystate, *effect, None)?;
            delegate.send_drop_update(
                self.id,
                event,
                Box::new(move |res| {
                    let res = res.ok_log().unwrap_or(DropOperation::None);
                    session_clone.last_operation.set(res);
                }),
            );
            *effect = session.last_operation.get().to_platform();
        } else {
            log::debug!("on_drag_enter: no data object provided, setting DROPEFFECT_NONE");
            *effect = DROPEFFECT_NONE;
        }

        log::debug!("on_drag_enter: final effect={:?}", *effect);
        Ok(())
    }

    fn on_drag_over(
        &self,
        grfkeystate: MODIFIERKEYS_FLAGS,
        pt: &POINTL,
        pdweffect: *mut DROPEFFECT,
    ) -> NativeExtensionsResult<()> {
        log::trace!("on_drag_over: position=({}, {}), effect={:?}", pt.x, pt.y, unsafe { *pdweffect });
        let effect = unsafe { &mut *pdweffect };
        if let Some(session) = self.current_session.borrow().as_ref().cloned() {
            session.missing_drop_end.set(false);
            let session_clone = session.clone();
            let event = self.event_for_session(&session, pt, grfkeystate, *effect, None)?;
            self.delegate()?.send_drop_update(
                self.id,
                event,
                Box::new(move |res| {
                    let res = res.ok_log().unwrap_or(DropOperation::None);
                    session_clone.last_operation.set(res);
                }),
            );
            *effect = session.last_operation.get().to_platform();
        } else {
            log::debug!("on_drag_over: no active session, setting DROPEFFECT_NONE");
            *effect = DROPEFFECT_NONE;
        }
        Ok(())
    }

    fn on_drag_leave(&self) -> NativeExtensionsResult<()> {
        log::debug!("on_drag_leave: leaving drop target");
        self.drop_exit()?;
        let local_dragging = self.local_dragging()?;
        log::debug!("on_drag_leave: local_dragging={}", local_dragging);
        if let Some(s) = self.current_session.borrow_mut().as_ref() {
            s.is_inside.set(false);

            // will invoke drop_end when local drag session ends
            s.missing_drop_end.set(local_dragging);
        }
        // Keep session alive for local dragging
        if !local_dragging {
            log::debug!("on_drag_leave: ending session (not local dragging)");
            self.drop_end()?;
        } else {
            log::debug!("on_drag_leave: keeping session alive for local dragging");
        }
        Ok(())
    }

    fn on_drop(
        &self,
        _pdataobj: Option<&IDataObject>,
        grfkeystate: MODIFIERKEYS_FLAGS,
        pt: &POINTL,
        pdweffect: *mut DROPEFFECT,
    ) -> NativeExtensionsResult<()> {
        log::debug!("on_drop: position=({}, {}), effect={:?}", pt.x, pt.y, unsafe { *pdweffect });
        let effect = unsafe { &mut *pdweffect };
        let session = self.current_session.borrow().as_ref().cloned();
        if let Some(session) = session {
            log::debug!("on_drop: processing drop for session {:?}", session.id);
            *effect = session.last_operation.get().to_platform();
            let event = match self.event_for_session(
                &session,
                pt,
                grfkeystate,
                *effect,
                Some(session.last_operation.get()),
            ) {
                Ok(event) => event,
                Err(e) => {
                    log::debug!("Failed to create event for session: {}", e);
                    crate::drop_manager::DropEvent {
                        session_id: session.id,
                        location_in_view: crate::api_model::Point { x: 0.0, y: 0.0 },
                        allowed_operations: crate::api_model::DropOperation::from_platform_mask(*effect),
                        accepted_operation: Some(session.last_operation.get()),
                        items: Vec::new(),
                        reader: Some(session.registered_reader.clone()),
                    }
                }
            };

            // Ensure safe_get_data is used for IDataObject calls
            if let Some(data_object) = _pdataobj {
                let format = FORMATETC { /* populate format */ };
                match safe_get_data(data_object, &format) {
                    Ok(Some(_)) => log::debug!("Data retrieved successfully"),
                    Ok(None) => log::debug!("Format not available"),
                    Err(e) => log::debug!("Error retrieving data: {}", e),
                }
            }

            let done = Rc::new(Cell::new(false));
            let done_clone = done.clone();
            self.delegate()?.send_perform_drop(
                self.id,
                event,
                Box::new(move |r| {
                    r.ok_log();
                    done_clone.set(true);
                }),
            );

            let data_object_async = session.data_object.cast::<IDataObjectAsyncCapability>();
            if let Ok(data_object_async) = data_object_async {
                if let Ok(res) = unsafe { data_object_async.GetAsyncMode() } {
                    if res.as_bool() {
                        log::debug!("on_drop: data object supports async mode");
                        // this will be read by drop notifier in DataReader and used for
                        // IDataObjectAsyncCapability::EndOperation result (when data reader gets dropped)
                        session
                            .async_result
                            .replace(Some((data_object_async.clone(), *effect)));
                        session.reader.set_supports_async();
                        unsafe {
                            data_object_async.StartOperation(None).ok_log();
                        }
                    } else {
                        log::debug!("on_drop: data object does not support async mode");
                    }
                } else {
                    log::debug!("on_drop: failed to get async mode for data object");
                }
            } else {
                log::debug!("on_drop: data object does not support async capability");
            }
            let mut poll_session = PollSession::new();
            log::debug!("on_drop: waiting for drop operation to complete");
            while !done.get() {
                RunLoop::current()
                    .platform_run_loop
                    .poll_once(&mut poll_session);
            }
            log::debug!("on_drop: drop operation completed, ending session");
            self.drop_end()?;
        } else {
            log::debug!("on_drop: no active session, setting DROPEFFECT_NONE");
            *effect = DROPEFFECT_NONE;
        }
        log::debug!("on_drop: final effect={:?}", *effect);
        Ok(())
    }
}

impl Drop for PlatformDropContext {
    fn drop(&mut self) {
        unsafe {
            let hook = *self.hook;
            UnhookWinEvent(hook);
            HOOK_TO_HWND
                .try_with(|a| a.borrow_mut().remove(&hook.0))
                .ok();
        }
    }
}

#[implement(IDropTarget)]
struct DropTarget {
    hwnd: HWND,
    platform_context: Weak<PlatformDropContext>,
    drop_target_helper: Option<IDropTargetHelper>,
}

impl DropTarget {
    fn new(hwnd: HWND, platform_context: Weak<PlatformDropContext>) -> Self {
        Self {
            hwnd,
            platform_context,
            drop_target_helper: create_instance(&CLSID_DragDropHelper).ok_log(),
        }
    }

    fn validate_formatetc(format: &FORMATETC) -> windows::core::Result<()> {
        if format.tymed == 0 || format.cfFormat == 0 {
            return Err(windows::core::Error::new(
                windows::core::HRESULT(0x80040064), // Invalid FORMATETC structure
                "Invalid FORMATETC structure",
            ));
        }
        Ok(())
    }
}

#[allow(non_snake_case)]
impl IDropTarget_Impl for DropTarget {
    fn DragEnter(
        &self,
        pdataobj: Option<&IDataObject>,
        grfkeystate: MODIFIERKEYS_FLAGS,
        pt: &POINTL,
        pdweffect: *mut DROPEFFECT,
    ) -> windows::core::Result<()> {
        if let Some(drop_target_helper) = &self.drop_target_helper {
            unsafe {
                drop_target_helper
                    .DragEnter(
                        self.hwnd,
                        pdataobj.unwrap(),
                        pt as *const POINTL as *const _,
                        *pdweffect,
                    )
                    .ok();
            }
        }

        if let Some(context) = self.platform_context.upgrade() {
            if let Some(data_object) = pdataobj {
                let format = FORMATETC { /* populate format */ };
                if let Err(e) = Self::validate_formatetc(&format) {
                    log::debug!("Invalid FORMATETC structure: {}", e);
                    return Err(e);
                }
            }

            match context.on_drag_enter(pdataobj, grfkeystate, pt, pdweffect) {
                Ok(_) => {}
                Err(e) => {
                    log::debug!("Error in on_drag_enter: {}", e);
                }
            }
        }
        Ok(())
    }

    fn DragOver(
        &self,
        grfkeystate: MODIFIERKEYS_FLAGS,
        pt: &POINTL,
        pdweffect: *mut DROPEFFECT,
    ) -> windows::core::Result<()> {
        if let Some(drop_target_helper) = &self.drop_target_helper {
            unsafe {
                drop_target_helper
                    .DragOver(pt as *const POINTL as *const _, *pdweffect)
                    .ok();
            }
        }
        if let Some(context) = self.platform_context.upgrade() {
            match context.on_drag_over(grfkeystate, pt, pdweffect) {
                Ok(_) => {}
                Err(e) => {
                    log::debug!("Error in on_drag_over: {}", e);
                }
            }
        }
        Ok(())
    }

    fn DragLeave(&self) -> windows::core::Result<()> {
        if let Some(drop_target_helper) = &self.drop_target_helper {
            unsafe {
                drop_target_helper.DragLeave().ok();
            }
        }
        if let Some(context) = self.platform_context.upgrade() {
            match context.on_drag_leave() {
                Ok(_) => {}
                Err(e) => {
                    log::debug!("Error in on_drag_leave: {}", e);
                }
            }
        }
        Ok(())
    }

    fn Drop(
        &self,
        pdataobj: Option<&IDataObject>,
        grfkeystate: MODIFIERKEYS_FLAGS,
        pt: &POINTL,
        pdweffect: *mut DROPEFFECT,
    ) -> windows::core::Result<()> {
        if let Some(drop_target_helper) = &self.drop_target_helper {
            unsafe {
                drop_target_helper
                    .Drop(
                        pdataobj.unwrap(),
                        pt as *const POINTL as *const _,
                        *pdweffect,
                    )
                    .ok();
            }
        }
        if let Some(context) = self.platform_context.upgrade() {
            match context.on_drop(pdataobj, grfkeystate, pt, pdweffect) {
                Ok(_) => {}
                Err(e) => {
                    log::debug!("Error in on_drop: {}", e);
                }
            }
        }
        Ok(())
    }
}
