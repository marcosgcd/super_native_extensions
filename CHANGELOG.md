# Change Log

All notable changes to this project will be documented in this file.
See [Conventional Commits](https://conventionalcommits.org) for commit guidelines.

## 2025-07-17

### Changes

---

Packages with breaking changes:

 - [`super_clipboard` - `v1025.1.1`](#super_clipboard---v102511)
 - [`super_drag_and_drop` - `v1025.1.1`](#super_drag_and_drop---v102511)
 - [`super_context_menu` - `v1025.1.1`](#super_context_menu---v102511)
 - [`super_native_extensions` - `v1025.1.1`](#super_native_extensions---v102511)
 - [`super_hot_key` - `v1025.1.1`](#super_hot_key---v102511)
 - [`super_keyboard_layout` - `v1025.1.1`](#super_keyboard_layout---v102511)

Packages with other changes:

 - There are no other changes in this release.

---

#### `super_clipboard` - `v1025.1.1`

 - **FIX**: correct GIF uniform type identifier (#276).
 - **FIX**: readFile fails when item is a String (#202).
 - **FIX**: do not use path dependencies for published packages in pubspec.yaml (#183).
 - **FIX**: update engine_context dependency.
 - **FIX**: correct imports and add missing exports (#155).
 - **FIX**: [android] build failing with proguard enabled (#114).
 - **FIX**: properly unitialize com on windows.
 - **FIX**: minor clean-ups.
 - **FEAT**: add flac and ogg format definitions (#331).
 - **FEAT**: add m4a format definition (#306).
 - **FEAT**: improve performance with large number of items (#283).
 - **FEAT**: implement copy and cut events (#253).
 - **FEAT**: preventDefault for paste event (#249).
 - **FEAT**: implement paste event on web (#246).
 - **FEAT**: add support for epub and md (#205).
 - **FEAT**: improve compatibility with current Flutter main (#163).
 - **FEAT**: add htmlFile format (#107).
 - **FEAT**: make format in DataReader.getFile optional (#90).
 - **FEAT**: add Formats.plainTextFile.
 - **FEAT**: declare more well-known formats (#58).
 - **FEAT**: cleanup receiving of files (#54).
 - **FEAT**: initialize ole on windows (#51).
 - **FEAT**: migrate to irondash (#27).
 - **FEAT**: add_super_keyboard_layout (#20).
 - **DOCS**: update comments.
 - **DOCS**: [android] mention minSdkVersion in readme (#150).
 - **DOCS**: update NDK installation information (#149).
 - **BREAKING** **FIX**: correct typos and spelling in code (#156).
 - **BREAKING** **FEAT**: upgrade to Dart 3 and jni 0.21.1 (#138).
 - **BREAKING** **FEAT**: implement unified content receiving (#47).
 - **BREAKING** **FEAT**: refactor format (#46).
 - **BREAKING** **CHORE**: remove Pair and replace it with dart 3 record (#157).

#### `super_drag_and_drop` - `v1025.1.1`

 - **FIX**: super_drag_and_drop should reexport Format (#83).
 - **FIX**: detect drag cancelled on desktop while waiting for data (#377).
 - **FIX**: various exceptions when getting snapshots (#328).
 - **FIX**: various exceptions when getting snapshots (#327).
 - **FIX**: [ios] respect isLocationDraggable check (#109).
 - **FIX**: multi-touch issues on Android (#196).
 - **FIX**: do not use path dependencies for published packages in pubspec.yaml (#183).
 - **FIX**: update engine_context dependency.
 - **FIX**: ensure drop regions are attached when invoking events (#147).
 - **FIX**: cancel mouse hover during dragging (#34).
 - **FIX**: cache active items for snapshotter (#146).
 - **FIX**: minor clean-ups.
 - **FIX**: [android] build failing with proguard enabled (#114).
 - **FIX**: Improve Drag&Drop on Web (#19).
 - **FIX**: properly unitialize com on windows.
 - **FEAT**: increase hit slop required for dragging on desktop (#463).
 - **FEAT**: improve performance with large number of items (#283).
 - **FEAT**: use super_sliver_list in example (#281).
 - **FEAT**: improve touch device detection (#227).
 - **FEAT**: add_super_keyboard_layout (#20).
 - **FEAT**: improve compatibility with current Flutter main (#163).
 - **FEAT**: migrate to irondash (#27).
 - **FEAT**: expose isLocationDraggable  for DraggableWidget (#31).
 - **FEAT**: super_drag_and_drop: reexport formats from super_clipboard (#32).
 - **FEAT**: allow merging of snapshot prepare requests (#110).
 - **FEAT**: improve performance with large number of items (#274).
 - **FEAT**: simplify lift snapshot logic on iOS (#108).
 - **FEAT**: initialize ole on windows (#51).
 - **FEAT**: cleanup receiving of files (#54).
 - **FEAT**: improve snapshot API (#101).
 - **FEAT**: use widget to customize snapshot setting (#100).
 - **FEAT**: implement drag shadow on all platforms (#87).
 - **FEAT**: [drop] add support for slivers (#35).
 - **DOCS**: fix typo.
 - **DOCS**: minor fix.
 - **DOCS**: improve super_drag_and_drop documentation (#106).
 - **DOCS**: fix typo.
 - **DOCS**: update NDK installation information (#149).
 - **DOCS**: fix example.
 - **DOCS**: fix typo (#473).
 - **DOCS**: [android] mention minSdkVersion in readme (#150).
 - **DOCS**: update comments.
 - **DOCS**: fix example.
 - **BREAKING** **FIX**: correct typos and spelling in code (#156).
 - **BREAKING** **FEAT**: implement unified content receiving (#47).
 - **BREAKING** **FEAT**: refactor format (#46).
 - **BREAKING** **FEAT**: upgrade to Dart 3 and jni 0.21.1 (#138).

#### `super_context_menu` - `v1025.1.1`

 - **FIX**: regression when context menu sometimes does not show (#220).
 - **FIX**: properly handle overlapping menu widgets (#217).
 - **FIX**: multi-touch issues on Android (#196).
 - **FIX**: do not use path dependencies for published packages in pubspec.yaml (#183).
 - **FIX**: update engine_context dependency.
 - **FIX**: [iOS] gesture recognizer workaround (#176).
 - **FIX**: use destructive icon theme when serializing menu images (#162).
 - **FIX**: context menu in list view not working on iOS (#144).
 - **FEAT**: improve focus interaction with non-native context menu (#442).
 - **FEAT**(macOS): preliminary support for writing tools (#441).
 - **FEAT**: export default builder to make it enable for use single brightness (#416).
 - **FEAT**: improve touch device detection (#227).
 - **FEAT**: restore focus before invoking menu callback (#191).
 - **FEAT**: improve compatibility with current Flutter main (#163).
 - **FEAT**: implement safe triangle for desktop menu (#153).
 - **DOCS**: update NDK installation information (#149).
 - **DOCS**: fixup unnecessary capitalization.
 - **BREAKING** **FIX**: correct typos and spelling in code (#156).
 - **BREAKING** **FEAT**: upgrade to Dart 3 and jni 0.21.1 (#138).

#### `super_native_extensions` - `v1025.1.1`

 - **FIX**: no security scope NSURL access on macos (#271).
 - **FIX**: update public key for verifying downloaded prebuilt binaries in cargokit.yaml.
 - **FIX**: update public key for verifying downloaded prebuilt binaries in cargokit.yaml.
 - **FIX**: update public key for verifying prebuilt binaries in cargokit.yaml.
 - **FIX**: Synthetize mouse up event during drag on linux.
 - **FIX**: Workaround for Xcode warning.
 - **FIX**: Broken buid on iOS with Rust 1.65.
 - **FIX**: crash on android with merged platform and UI threads (#483).
 - **FIX**: widget snapshots not working properly with WASM (#469).
 - **FIX**: clipboard read error on wasm (#464).
 - **FIX**: workaround for exception when running in test environment (#458).
 - **FIX**: drag crashing on Android 15 (#453).
 - **FIX**: workaround for deadlock on iOS 18 (#449).
 - **FIX**: FFI errors in flutter tester.
 - **FIX**: don't panic with thread local AccessError when shutting down (#426).
 - **FIX**: Avoid adding duplicate listeners for drag-n-drop on the web (#422).
 - **FIX**: compilation error on web with latest Flutter main (#425).
 - **FIX**: do not build release binary with nightly (#412).
 - **FIX**: panic in ANSI branch of extract_drop_files (#404).
 - **FIX**: synthesize_button_up might cause crash on Linux (#394).
 - **FIX**: dragging stuck on web when cancelled too quickly (#398).
 - **FIX**: paste caused crash when clipboard is empty on linux  (#393).
 - **FIX**: hide menu drag preview immediately when pan gesture detected (#385).
 - **FIX**: invalid javascript object cast (#380).
 - **FIX**: context menu on iPad with universal control (#378).
 - **FIX**: detect drag cancelled on desktop while waiting for data (#377).
 - **FIX**: use startDragAndDrop instead of startDrag on Android sdk24 and above (#372).
 - **FIX**: remove obsolete code (#364).
 - **FIX**: ignore scroll event in web drag driver.
 - **FIX**: ignore unknown pointer device kind (#344).
 - **FIX**: delay menu fade-out on iOS (#333).
 - **FIX**: regression with custom snapshot (#330).
 - **FIX**: various exceptions when getting snapshots (#327).
 - **FIX**: fit menu position to bounds after inflating (#322).
 - **FIX**: assertion when taking snapshot of material widget (#320).
 - **FIX**: remove leftover logging (#284).
 - **FIX**: create phony file in BUILD_PRODUCTS_DIR.
 - **FIX**: [android] possible deadlock when reading from clipboard (#282).
 - **FIX**: minor clean-ups.
 - **FIX**: [ios] store user interaction properly (#272).
 - **FIX**: update public key for verifying downloaded prebuilt binaries in cargokit.yaml.
 - **FIX**: make clippy happy (#36).
 - **FIX**: access NSURL within security scope (#264).
 - **FIX**: rare crash while dragging on iOS (#40).
 - **FIX**: window dragging on macos with full size content view (#43).
 - **FIX**: serialize invocation of drop events (#49).
 - **FIX**: increase buffer size.
 - **FIX**: let zone handle menu callback errors (#228).
 - **FIX**: lift image being ignored on iOS (#59).
 - **FIX**: remove trailing null terminator from NSString (#207).
 - **FIX**: [iOS] crash when deferred image is set too quickly (#206).
 - **FIX**: [macOS] missing image on NSMenuItem with children (#197).
 - **FIX**: multi-touch issues on Android (#196).
 - **FIX**: improve closing of menu overlay on touch devices (#193).
 - **FIX**: update engine_context dependency.
 - **FIX**: escape script invocation in podspec.
 - **FIX**: declare proper output in podspec script phase.
 - **FIX**: update rust dependencies (#170).
 - **FIX**: [windows] handle null terminator in CF_UNICODETEXT (#169).
 - **FIX**: do not recreate drag and drop contexts on hot reload (#61).
 - **FIX**: use destructive icon theme when serializing menu images (#162).
 - **FIX**: [windows] keep IDataObjectAsyncCapability reference during drop (#161).
 - **FIX**: [windows] properly handle data objects that don't support async capability (#160).
 - **FIX**: [macos] error instead of panic when no mouse event is found (#60).
 - **FIX**: regression when dropping plain text on web (#66).
 - **FIX**: formatting.
 - **FIX**: [macos] assertion when loading deferred menu (#152).
 - **FIX**: [macos] control key stuck after context menu closed (#151).
 - **FIX**: web drag avatar shown in non-root overlay (#139).
 - **FIX**: [windows] use cached length when reading virtual stream (#69).
 - **FIX**: pasting text with semicolon on macOS (#133).
 - **FIX**: [win] rewind OLE streams before reading (#117).
 - **FIX**: [android] local data only dragging not working (#115).
 - **FIX**: [android] build failing with proguard enabled (#114).
 - **FIX**: clipboard copy on web in release mode (#72).
 - **FIX**: custom snapshot should propagate exception from renderbox (#104).
 - **FIX**: [ios] revert memory leak fix removal (#103).
 - **FIX**: [android] throw exception if wrong mime filter is requested.
 - **FIX**: use unpremultiplied alpha for encoding image data (#85).
 - **FIX**: [windows] pasting files from explorer (#88).
 - **FIX**: Improve Drag&Drop on Web (#19).
 - **FIX**: [ios] use shadow path from correct image (#97).
 - **FIX**: [web] dragging ocasionally getting stuck (#89).
 - **FIX**: [ios] force separate drag image to account for shadow difference (#92).
 - **FIX**: [web] dropping over platform views not working (#99).
 - **FEAT**: try revoking drop target first on windows (#63).
 - **FEAT**: use widget to customize snapshot setting (#100).
 - **FEAT**: implement drag shadow on all platforms (#87).
 - **FEAT**: improve snapshot API (#101).
 - **FEAT**: [macos] receiving virtual files from outlook attachments (#81).
 - **FEAT**: add super_hot_key (#77).
 - **FEAT**: snapshot optimization (#102).
 - **FEAT**: allow merging of snapshot prepare requests (#110).
 - **FEAT**: add_super_keyboard_layout (#20).
 - **FEAT**: [windows] use thread pool for virtual file background thread (#68).
 - **FEAT**: [windows] delay virtual file request until IStream is read (#67).
 - **FEAT**: update public key for precompiled binaries.
 - **FEAT**: [ios] use real shadow path instead of layer shadow (#95).
 - **FEAT**: configure own precompiled binaries release.
 - **FEAT**: improve compatibility with current Flutter main (#163).
 - **FEAT**: improve touch device detection (#227).
 - **FEAT**: declare more well-known formats (#58).
 - **FEAT**: migrate to objc2 (#239).
 - **FEAT**: add support for inplace file reading on ios (#55).
 - **FEAT**: cleanup receiving of files (#54).
 - **FEAT**: add support for the maximum page size of 16 KB for Android (#521).
 - **FEAT**: initialize ole on windows (#51).
 - **FEAT**: implement paste event on web (#246).
 - **FEAT**(macOS): preliminary support for writing tools (#441).
 - **FEAT**: [ios] remove drag item provider memory leak workaround (#93).
 - **FEAT**: implement copy and cut events (#253).
 - **FEAT**: prevent finalizer invoked too early in release mode (#38).
 - **FEAT**: [windows] cache file descriptors in reader (#266).
 - **FEAT**: improve performance with large number of items (#274).
 - **FEAT**: improve performance with large number of items (#283).
 - **FEAT**: migrate to irondash (#27).
 - **FEAT**: preventDefault for paste event (#249).
 - **BREAKING** **FIX**: correct typos and spelling in code (#156).
 - **BREAKING** **FEAT**: implement unified content receiving (#47).
 - **BREAKING** **FEAT**: upgrade to Dart 3 and jni 0.21.1 (#138).
 - **BREAKING** **FEAT**: refactor format (#46).
 - **BREAKING** **CHORE**: remove Pair and replace it with dart 3 record (#157).

#### `super_hot_key` - `v1025.1.1`

 - **FIX**: hot key event handlers should not be required (#248).
 - **FEAT**: add super_hot_key (#77).
 - **BREAKING** **FEAT**: upgrade to Dart 3 and jni 0.21.1 (#138).

#### `super_keyboard_layout` - `v1025.1.1`

 - **FIX**: do not use path dependencies for published packages in pubspec.yaml (#183).
 - **FIX**: update engine_context dependency.
 - **FIX**: [android] build failing with proguard enabled (#114).
 - **FIX**: minor clean-ups.
 - **FEAT**: improve compatibility with current Flutter main (#163).
 - **FEAT**: add super_hot_key (#77).
 - **FEAT**: migrate to irondash (#27).
 - **FEAT**: add_super_keyboard_layout (#20).
 - **DOCS**: update NDK installation information (#149).
 - **DOCS**: Improve documentation.
 - **BREAKING** **FEAT**: upgrade to Dart 3 and jni 0.21.1 (#138).
 - **BREAKING** **FEAT**: implement unified content receiving (#47).


## 2025-06-11

### Changes

---

Packages with breaking changes:

 - There are no breaking changes in this release.

Packages with other changes:

 - [`super_clipboard` - `v0.9.1`](#super_clipboard---v091)
 - [`super_drag_and_drop` - `v0.9.1`](#super_drag_and_drop---v091)
 - [`super_context_menu` - `v0.9.1`](#super_context_menu---v091)
 - [`super_native_extensions` - `v0.9.1`](#super_native_extensions---v091)
 - [`super_hot_key` - `v0.9.1`](#super_hot_key---v091)
 - [`super_keyboard_layout` - `v0.9.1`](#super_keyboard_layout---v091)

---

#### `super_clipboard` - `v0.9.1`

#### `super_drag_and_drop` - `v0.9.1`

#### `super_context_menu` - `v0.9.1`

#### `super_native_extensions` - `v0.9.1`

#### `super_hot_key` - `v0.9.1`

 - Bump "super_hot_key" to `0.9.1`.

#### `super_keyboard_layout` - `v0.9.1`


## 2025-06-08

### Changes

---

Packages with breaking changes:

 - There are no breaking changes in this release.

Packages with other changes:

 - [`super_clipboard` - `v0.9.0`](#super_clipboard---v090)
 - [`super_drag_and_drop` - `v0.9.0`](#super_drag_and_drop---v090)
 - [`super_context_menu` - `v0.9.0`](#super_context_menu---v090)
 - [`super_native_extensions` - `v0.9.0`](#super_native_extensions---v090)
 - [`super_hot_key` - `v0.9.0`](#super_hot_key---v090)
 - [`super_keyboard_layout` - `v0.9.0`](#super_keyboard_layout---v090)

---

#### `super_clipboard` - `v0.9.0`

#### `super_drag_and_drop` - `v0.9.0`

#### `super_context_menu` - `v0.9.0`

#### `super_native_extensions` - `v0.9.0`

 - **FEAT**: add support for the maximum page size of 16 KB for Android (#521).

#### `super_hot_key` - `v0.9.0`

 - Bump "super_hot_key" to `0.9.0`.

#### `super_keyboard_layout` - `v0.9.0`


## 2024-12-29

### Changes

---

Packages with breaking changes:

 - There are no breaking changes in this release.

Packages with other changes:

 - [`super_clipboard` - `v0.9.0-dev.6`](#super_clipboard---v090-dev6)
 - [`super_drag_and_drop` - `v0.9.0-dev.6`](#super_drag_and_drop---v090-dev6)
 - [`super_context_menu` - `v0.9.0-dev.6`](#super_context_menu---v090-dev6)
 - [`super_native_extensions` - `v0.9.0-dev.6`](#super_native_extensions---v090-dev6)
 - [`super_hot_key` - `v0.9.0-dev.6`](#super_hot_key---v090-dev6)
 - [`super_keyboard_layout` - `v0.9.0-dev.6`](#super_keyboard_layout---v090-dev6)

---

#### `super_clipboard` - `v0.9.0-dev.6`

#### `super_drag_and_drop` - `v0.9.0-dev.6`

 - **DOCS**: fix typo (#473).

#### `super_context_menu` - `v0.9.0-dev.6`

#### `super_native_extensions` - `v0.9.0-dev.6`

 - **FIX**: crash on android with merged platform and UI threads (#483).

#### `super_hot_key` - `v0.9.0-dev.6`

 - Bump "super_hot_key" to `0.9.0-dev.6`.

#### `super_keyboard_layout` - `v0.9.0-dev.6`


## 2024-11-17

### Changes

---

Packages with breaking changes:

 - There are no breaking changes in this release.

Packages with other changes:

 - [`super_clipboard` - `v0.9.0-dev.5`](#super_clipboard---v090-dev5)
 - [`super_drag_and_drop` - `v0.9.0-dev.5`](#super_drag_and_drop---v090-dev5)
 - [`super_context_menu` - `v0.9.0-dev.5`](#super_context_menu---v090-dev5)
 - [`super_native_extensions` - `v0.9.0-dev.5`](#super_native_extensions---v090-dev5)
 - [`super_hot_key` - `v0.9.0-dev.5`](#super_hot_key---v090-dev5)
 - [`super_keyboard_layout` - `v0.9.0-dev.5`](#super_keyboard_layout---v090-dev5)

---

#### `super_clipboard` - `v0.9.0-dev.5`

#### `super_drag_and_drop` - `v0.9.0-dev.5`

#### `super_context_menu` - `v0.9.0-dev.5`

#### `super_native_extensions` - `v0.9.0-dev.5`

 - Bump "super_native_extensions" to `0.9.0-dev.5`.

#### `super_hot_key` - `v0.9.0-dev.5`

 - Bump "super_hot_key" to `0.9.0-dev.5`.

#### `super_keyboard_layout` - `v0.9.0-dev.5`


## 2024-11-13

### Changes

---

Packages with breaking changes:

 - There are no breaking changes in this release.

Packages with other changes:

 - [`super_clipboard` - `v0.9.0-dev.4`](#super_clipboard---v090-dev4)
 - [`super_drag_and_drop` - `v0.9.0-dev.4`](#super_drag_and_drop---v090-dev4)
 - [`super_context_menu` - `v0.9.0-dev.4`](#super_context_menu---v090-dev4)
 - [`super_native_extensions` - `v0.9.0-dev.4`](#super_native_extensions---v090-dev4)
 - [`super_hot_key` - `v0.9.0-dev.4`](#super_hot_key---v090-dev4)
 - [`super_keyboard_layout` - `v0.9.0-dev.4`](#super_keyboard_layout---v090-dev4)

---

#### `super_clipboard` - `v0.9.0-dev.4`

#### `super_drag_and_drop` - `v0.9.0-dev.4`

#### `super_context_menu` - `v0.9.0-dev.4`

#### `super_native_extensions` - `v0.9.0-dev.4`

 - **FIX**: widget snapshots not working properly with WASM (#469).

#### `super_hot_key` - `v0.9.0-dev.4`

 - Bump "super_hot_key" to `0.9.0-dev.4`.

#### `super_keyboard_layout` - `v0.9.0-dev.4`


## 2024-11-04

### Changes

---

Packages with breaking changes:

 - There are no breaking changes in this release.

Packages with other changes:

 - [`super_clipboard` - `v0.9.0-dev.3`](#super_clipboard---v090-dev3)
 - [`super_drag_and_drop` - `v0.9.0-dev.3`](#super_drag_and_drop---v090-dev3)
 - [`super_context_menu` - `v0.9.0-dev.3`](#super_context_menu---v090-dev3)
 - [`super_native_extensions` - `v0.9.0-dev.3`](#super_native_extensions---v090-dev3)
 - [`super_hot_key` - `v0.9.0-dev.3`](#super_hot_key---v090-dev3)
 - [`super_keyboard_layout` - `v0.9.0-dev.3`](#super_keyboard_layout---v090-dev3)

---

#### `super_clipboard` - `v0.9.0-dev.3`

#### `super_drag_and_drop` - `v0.9.0-dev.3`

 - **FEAT**: increase hit slop required for dragging on desktop (#463).

#### `super_context_menu` - `v0.9.0-dev.3`

#### `super_native_extensions` - `v0.9.0-dev.3`

 - **FIX**: clipboard read error on wasm (#464).

#### `super_hot_key` - `v0.9.0-dev.3`

 - Bump "super_hot_key" to `0.9.0-dev.3`.

#### `super_keyboard_layout` - `v0.9.0-dev.3`


## 2024-10-21

### Changes

---

Packages with breaking changes:

 - There are no breaking changes in this release.

Packages with other changes:

 - [`super_clipboard` - `v0.9.0-dev.2`](#super_clipboard---v090-dev2)
 - [`super_drag_and_drop` - `v0.9.0-dev.2`](#super_drag_and_drop---v090-dev2)
 - [`super_context_menu` - `v0.9.0-dev.2`](#super_context_menu---v090-dev2)
 - [`super_native_extensions` - `v0.9.0-dev.2`](#super_native_extensions---v090-dev2)
 - [`super_hot_key` - `v0.9.0-dev.2`](#super_hot_key---v090-dev2)
 - [`super_keyboard_layout` - `v0.9.0-dev.2`](#super_keyboard_layout---v090-dev2)

---

#### `super_clipboard` - `v0.9.0-dev.2`

#### `super_drag_and_drop` - `v0.9.0-dev.2`

#### `super_context_menu` - `v0.9.0-dev.2`

#### `super_native_extensions` - `v0.9.0-dev.2`

 - **FIX**: workaround for exception when running in test environment (#458).

#### `super_hot_key` - `v0.9.0-dev.2`

 - Bump "super_hot_key" to `0.9.0-dev.2`.

#### `super_keyboard_layout` - `v0.9.0-dev.2`


## 2024-10-18

### Changes

---

Packages with breaking changes:

 - There are no breaking changes in this release.

Packages with other changes:

 - [`super_clipboard` - `v0.9.0-dev.1`](#super_clipboard---v090-dev1)
 - [`super_drag_and_drop` - `v0.9.0-dev.1`](#super_drag_and_drop---v090-dev1)
 - [`super_context_menu` - `v0.9.0-dev.1`](#super_context_menu---v090-dev1)
 - [`super_native_extensions` - `v0.9.0-dev.1`](#super_native_extensions---v090-dev1)
 - [`super_hot_key` - `v0.9.0-dev.1`](#super_hot_key---v090-dev1)
 - [`super_keyboard_layout` - `v0.9.0-dev.1`](#super_keyboard_layout---v090-dev1)

---

#### `super_clipboard` - `v0.9.0-dev.1`

 - Bump "super_clipboard" to `0.9.0-dev.1`.

#### `super_drag_and_drop` - `v0.9.0-dev.1`

 - Bump "super_drag_and_drop" to `0.9.0-dev.1`.

#### `super_context_menu` - `v0.9.0-dev.1`

 - Bump "super_context_menu" to `0.9.0-dev.1`.

#### `super_native_extensions` - `v0.9.0-dev.1`

 - Bump "super_native_extensions" to `0.9.0-dev.1`.

#### `super_hot_key` - `v0.9.0-dev.1`

 - Bump "super_hot_key" to `0.9.0-dev.1`.

#### `super_keyboard_layout` - `v0.9.0-dev.1`

 - Bump "super_keyboard_layout" to `0.9.0-dev.1`.


## 2024-10-13

### Changes

---

Packages with breaking changes:

 - There are no breaking changes in this release.

Packages with other changes:

 - [`super_clipboard` - `v0.8.24`](#super_clipboard---v0824)
 - [`super_drag_and_drop` - `v0.8.24`](#super_drag_and_drop---v0824)
 - [`super_context_menu` - `v0.8.24`](#super_context_menu---v0824)
 - [`super_native_extensions` - `v0.8.24`](#super_native_extensions---v0824)
 - [`super_hot_key` - `v0.8.24`](#super_hot_key---v0824)
 - [`super_keyboard_layout` - `v0.8.24`](#super_keyboard_layout---v0824)

---

#### `super_clipboard` - `v0.8.24`

#### `super_drag_and_drop` - `v0.8.24`

#### `super_context_menu` - `v0.8.24`

#### `super_native_extensions` - `v0.8.24`

 - **FIX**: drag crashing on Android 15 (#453).

#### `super_hot_key` - `v0.8.24`

 - Bump "super_hot_key" to `0.8.24`.

#### `super_keyboard_layout` - `v0.8.24`


## 2024-10-06

### Changes

---

Packages with breaking changes:

 - There are no breaking changes in this release.

Packages with other changes:

 - [`super_clipboard` - `v0.8.23`](#super_clipboard---v0823)
 - [`super_drag_and_drop` - `v0.8.23`](#super_drag_and_drop---v0823)
 - [`super_context_menu` - `v0.8.23`](#super_context_menu---v0823)
 - [`super_native_extensions` - `v0.8.23`](#super_native_extensions---v0823)
 - [`super_hot_key` - `v0.8.23`](#super_hot_key---v0823)
 - [`super_keyboard_layout` - `v0.8.23`](#super_keyboard_layout---v0823)

---

#### `super_clipboard` - `v0.8.23`

 - Bump "super_clipboard" to `0.8.23`.

#### `super_drag_and_drop` - `v0.8.23`

 - Bump "super_drag_and_drop" to `0.8.23`.

#### `super_context_menu` - `v0.8.23`

 - Bump "super_context_menu" to `0.8.23`.

#### `super_native_extensions` - `v0.8.23`

 - **FIX**: workaround for deadlock on iOS 18 (#449).

#### `super_hot_key` - `v0.8.23`

 - Bump "super_hot_key" to `0.8.23`.

#### `super_keyboard_layout` - `v0.8.23`

 - Bump "super_keyboard_layout" to `0.8.23`.


## 2024-09-30

### Changes

---

Packages with breaking changes:

 - There are no breaking changes in this release.

Packages with other changes:

 - [`super_clipboard` - `v0.8.23-dev.1`](#super_clipboard---v0823-dev1)
 - [`super_drag_and_drop` - `v0.8.23-dev.1`](#super_drag_and_drop---v0823-dev1)
 - [`super_context_menu` - `v0.8.23-dev.1`](#super_context_menu---v0823-dev1)
 - [`super_native_extensions` - `v0.8.23-dev.1`](#super_native_extensions---v0823-dev1)
 - [`super_hot_key` - `v0.8.23-dev.1`](#super_hot_key---v0823-dev1)
 - [`super_keyboard_layout` - `v0.8.23-dev.1`](#super_keyboard_layout---v0823-dev1)

---

#### `super_clipboard` - `v0.8.23-dev.1`

#### `super_drag_and_drop` - `v0.8.23-dev.1`

#### `super_context_menu` - `v0.8.23-dev.1`

 - **FEAT**: improve focus interaction with non-native context menu (#442).
 - **FEAT**(macOS): preliminary support for writing tools (#441).

#### `super_native_extensions` - `v0.8.23-dev.1`

 - **FEAT**(macOS): preliminary support for writing tools (#441).

#### `super_hot_key` - `v0.8.23-dev.1`

 - Bump "super_hot_key" to `0.8.23-dev.1`.

#### `super_keyboard_layout` - `v0.8.23-dev.1`


## 2024-09-10

### Changes

---

Packages with breaking changes:

 - There are no breaking changes in this release.

Packages with other changes:

 - [`super_clipboard` - `v0.8.22`](#super_clipboard---v0822)
 - [`super_drag_and_drop` - `v0.8.22`](#super_drag_and_drop---v0822)
 - [`super_context_menu` - `v0.8.22`](#super_context_menu---v0822)
 - [`super_native_extensions` - `v0.8.22`](#super_native_extensions---v0822)
 - [`super_hot_key` - `v0.8.22`](#super_hot_key---v0822)
 - [`super_keyboard_layout` - `v0.8.22`](#super_keyboard_layout---v0822)

---

#### `super_clipboard` - `v0.8.22`

#### `super_drag_and_drop` - `v0.8.22`

#### `super_context_menu` - `v0.8.22`

 - **FEAT**: export default builder to make it enable for use single brightness (#416).

#### `super_native_extensions` - `v0.8.22`

 - **FIX**: don't panic with thread local AccessError when shutting down (#426).

#### `super_hot_key` - `v0.8.22`

 - Bump "super_hot_key" to `0.8.22`.

#### `super_keyboard_layout` - `v0.8.22`


## 2024-09-09

### Changes

---

Packages with breaking changes:

 - There are no breaking changes in this release.

Packages with other changes:

 - [`super_clipboard` - `v0.8.21`](#super_clipboard---v0821)
 - [`super_drag_and_drop` - `v0.8.21`](#super_drag_and_drop---v0821)
 - [`super_context_menu` - `v0.8.21`](#super_context_menu---v0821)
 - [`super_native_extensions` - `v0.8.21`](#super_native_extensions---v0821)
 - [`super_hot_key` - `v0.8.21`](#super_hot_key---v0821)
 - [`super_keyboard_layout` - `v0.8.21`](#super_keyboard_layout---v0821)

---

#### `super_clipboard` - `v0.8.21`

#### `super_drag_and_drop` - `v0.8.21`

#### `super_context_menu` - `v0.8.21`

#### `super_native_extensions` - `v0.8.21`

 - **FIX**: Avoid adding duplicate listeners for drag-n-drop on the web (#422).
 - **FIX**: compilation error on web with latest Flutter main (#425).

#### `super_hot_key` - `v0.8.21`

 - Bump "super_hot_key" to `0.8.21`.

#### `super_keyboard_layout` - `v0.8.21`


## 2024-08-26

### Changes

---

Packages with breaking changes:

 - There are no breaking changes in this release.

Packages with other changes:

 - [`super_clipboard` - `v0.8.20`](#super_clipboard---v0820)
 - [`super_drag_and_drop` - `v0.8.20`](#super_drag_and_drop---v0820)
 - [`super_context_menu` - `v0.8.20`](#super_context_menu---v0820)
 - [`super_native_extensions` - `v0.8.20`](#super_native_extensions---v0820)
 - [`super_hot_key` - `v0.8.20`](#super_hot_key---v0820)
 - [`super_keyboard_layout` - `v0.8.20`](#super_keyboard_layout---v0820)

---

#### `super_clipboard` - `v0.8.20`

#### `super_drag_and_drop` - `v0.8.20`

#### `super_context_menu` - `v0.8.20`

#### `super_native_extensions` - `v0.8.20`

 - **FIX**: do not build release binary with nightly (#412).
 - **FIX**: do not build release binary with nightly (#412).

#### `super_hot_key` - `v0.8.20`

#### `super_keyboard_layout` - `v0.8.20`


## 2024-08-08

### Changes

---

Packages with breaking changes:

 - There are no breaking changes in this release.

Packages with other changes:

 - [`super_clipboard` - `v0.8.19`](#super_clipboard---v0819)
 - [`super_drag_and_drop` - `v0.8.19`](#super_drag_and_drop---v0819)
 - [`super_context_menu` - `v0.8.19`](#super_context_menu---v0819)
 - [`super_native_extensions` - `v0.8.19`](#super_native_extensions---v0819)
 - [`super_hot_key` - `v0.8.19`](#super_hot_key---v0819)
 - [`super_keyboard_layout` - `v0.8.19`](#super_keyboard_layout---v0819)

---

#### `super_clipboard` - `v0.8.19`

#### `super_drag_and_drop` - `v0.8.19`

#### `super_context_menu` - `v0.8.19`

#### `super_native_extensions` - `v0.8.19`

 - **FIX**: do not build release binary with nightly (#412).
 - **FIX**: panic in ANSI branch of extract_drop_files (#404).
 - **FIX**: synthesize_button_up might cause crash on Linux (#394).

#### `super_hot_key` - `v0.8.19`

 - Bump "super_hot_key" to `0.8.19`.

#### `super_keyboard_layout` - `v0.8.19`


## 2024-07-24

### Changes

---

Packages with breaking changes:

 - There are no breaking changes in this release.

Packages with other changes:

 - [`super_clipboard` - `v0.8.18`](#super_clipboard---v0818)
 - [`super_drag_and_drop` - `v0.8.18`](#super_drag_and_drop---v0818)
 - [`super_context_menu` - `v0.8.18`](#super_context_menu---v0818)
 - [`super_native_extensions` - `v0.8.18`](#super_native_extensions---v0818)
 - [`super_hot_key` - `v0.8.18`](#super_hot_key---v0818)
 - [`super_keyboard_layout` - `v0.8.18`](#super_keyboard_layout---v0818)

---

#### `super_clipboard` - `v0.8.18`

#### `super_drag_and_drop` - `v0.8.18`

#### `super_context_menu` - `v0.8.18`

#### `super_native_extensions` - `v0.8.18`

 - **FIX**: dragging stuck on web when cancelled too quickly (#398).
 - **FIX**: paste caused crash when clipboard is empty on linux  (#393).

#### `super_hot_key` - `v0.8.18`

 - Bump "super_hot_key" to `0.8.18`.

#### `super_keyboard_layout` - `v0.8.18`


## 2024-06-13

### Changes

---

Packages with breaking changes:

 - There are no breaking changes in this release.

Packages with other changes:

 - [`super_clipboard` - `v0.8.17`](#super_clipboard---v0817)
 - [`super_drag_and_drop` - `v0.8.17`](#super_drag_and_drop---v0817)
 - [`super_context_menu` - `v0.8.17`](#super_context_menu---v0817)
 - [`super_native_extensions` - `v0.8.17`](#super_native_extensions---v0817)
 - [`super_hot_key` - `v0.8.17`](#super_hot_key---v0817)
 - [`super_keyboard_layout` - `v0.8.17`](#super_keyboard_layout---v0817)

---

#### `super_clipboard` - `v0.8.17`

#### `super_drag_and_drop` - `v0.8.17`

#### `super_context_menu` - `v0.8.17`

#### `super_native_extensions` - `v0.8.17`

 - **FIX**: hide menu drag preview immediately when pan gesture detected (#385).
 - **FIX**: invalid javascript object cast (#380).
 - **FIX**: context menu on iPad with universal control (#378).

#### `super_hot_key` - `v0.8.17`

 - Bump "super_hot_key" to `0.8.17`.

#### `super_keyboard_layout` - `v0.8.17`


## 2024-05-27

### Changes

---

Packages with breaking changes:

 - There are no breaking changes in this release.

Packages with other changes:

 - [`super_clipboard` - `v0.8.16`](#super_clipboard---v0816)
 - [`super_drag_and_drop` - `v0.8.16`](#super_drag_and_drop---v0816)
 - [`super_context_menu` - `v0.8.16`](#super_context_menu---v0816)
 - [`super_native_extensions` - `v0.8.16`](#super_native_extensions---v0816)
 - [`super_hot_key` - `v0.8.16`](#super_hot_key---v0816)
 - [`super_keyboard_layout` - `v0.8.16`](#super_keyboard_layout---v0816)

---

#### `super_clipboard` - `v0.8.16`

#### `super_drag_and_drop` - `v0.8.16`

 - **FIX**: detect drag cancelled on desktop while waiting for data (#377).

#### `super_context_menu` - `v0.8.16`

#### `super_native_extensions` - `v0.8.16`

 - **FIX**: detect drag cancelled on desktop while waiting for data (#377).
 - **FIX**: use startDragAndDrop instead of startDrag on Android sdk24 and above (#372).

#### `super_hot_key` - `v0.8.16`

 - Bump "super_hot_key" to `0.8.16`.

#### `super_keyboard_layout` - `v0.8.16`


## 2024-05-20

### Changes

---

Packages with breaking changes:

 - There are no breaking changes in this release.

Packages with other changes:

 - [`super_clipboard` - `v0.8.15`](#super_clipboard---v0815)
 - [`super_drag_and_drop` - `v0.8.15`](#super_drag_and_drop---v0815)
 - [`super_context_menu` - `v0.8.15`](#super_context_menu---v0815)
 - [`super_native_extensions` - `v0.8.15`](#super_native_extensions---v0815)
 - [`super_hot_key` - `v0.8.15`](#super_hot_key---v0815)
 - [`super_keyboard_layout` - `v0.8.15`](#super_keyboard_layout---v0815)

---

#### `super_clipboard` - `v0.8.15`

#### `super_drag_and_drop` - `v0.8.15`

#### `super_context_menu` - `v0.8.15`

#### `super_native_extensions` - `v0.8.15`

 - **FIX**: remove obsolete code (#364).

#### `super_hot_key` - `v0.8.15`

 - Bump "super_hot_key" to `0.8.15`.

#### `super_keyboard_layout` - `v0.8.15`


## 2024-05-20

### Changes

---

Packages with breaking changes:

 - There are no breaking changes in this release.

Packages with other changes:

 - [`super_clipboard` - `v0.8.14`](#super_clipboard---v0814)
 - [`super_drag_and_drop` - `v0.8.14`](#super_drag_and_drop---v0814)
 - [`super_context_menu` - `v0.8.14`](#super_context_menu---v0814)
 - [`super_native_extensions` - `v0.8.14`](#super_native_extensions---v0814)
 - [`super_hot_key` - `v0.8.14`](#super_hot_key---v0814)
 - [`super_keyboard_layout` - `v0.8.14`](#super_keyboard_layout---v0814)

---

#### `super_clipboard` - `v0.8.14`

 - Bump "super_clipboard" to `0.8.14`.

#### `super_drag_and_drop` - `v0.8.14`

 - Bump "super_drag_and_drop" to `0.8.14`.

#### `super_context_menu` - `v0.8.14`

 - Bump "super_context_menu" to `0.8.14`.

#### `super_native_extensions` - `v0.8.14`

 - Bump "super_native_extensions" to `0.8.14`.

#### `super_hot_key` - `v0.8.14`

 - Bump "super_hot_key" to `0.8.14`.

#### `super_keyboard_layout` - `v0.8.14`

 - Bump "super_keyboard_layout" to `0.8.14`.


## 2024-05-15

### Changes

---

Packages with breaking changes:

 - There are no breaking changes in this release.

Packages with other changes:

 - [`super_clipboard` - `v0.8.13`](#super_clipboard---v0813)
 - [`super_drag_and_drop` - `v0.8.13`](#super_drag_and_drop---v0813)
 - [`super_context_menu` - `v0.8.13`](#super_context_menu---v0813)
 - [`super_native_extensions` - `v0.8.13`](#super_native_extensions---v0813)
 - [`super_hot_key` - `v0.8.13`](#super_hot_key---v0813)
 - [`super_keyboard_layout` - `v0.8.13`](#super_keyboard_layout---v0813)

---

#### `super_clipboard` - `v0.8.13`

#### `super_drag_and_drop` - `v0.8.13`

#### `super_context_menu` - `v0.8.13`

#### `super_native_extensions` - `v0.8.13`

#### `super_hot_key` - `v0.8.13`

 - Bump "super_hot_key" to `0.8.13`.

#### `super_keyboard_layout` - `v0.8.13`


## 2024-04-18

### Changes

---

Packages with breaking changes:

 - There are no breaking changes in this release.

Packages with other changes:

 - [`super_clipboard` - `v0.8.12`](#super_clipboard---v0812)
 - [`super_drag_and_drop` - `v0.8.12`](#super_drag_and_drop---v0812)
 - [`super_context_menu` - `v0.8.12`](#super_context_menu---v0812)
 - [`super_native_extensions` - `v0.8.12`](#super_native_extensions---v0812)
 - [`super_hot_key` - `v0.8.12`](#super_hot_key---v0812)
 - [`super_keyboard_layout` - `v0.8.12`](#super_keyboard_layout---v0812)

---

#### `super_clipboard` - `v0.8.12`

#### `super_drag_and_drop` - `v0.8.12`

#### `super_context_menu` - `v0.8.12`

#### `super_native_extensions` - `v0.8.12`

 - Bump "super_native_extensions" to `0.8.12`.

#### `super_hot_key` - `v0.8.12`

 - Bump "super_hot_key" to `0.8.12`.

#### `super_keyboard_layout` - `v0.8.12`


## 2024-04-09

### Changes

---

Packages with breaking changes:

 - There are no breaking changes in this release.

Packages with other changes:

 - [`super_clipboard` - `v0.8.11`](#super_clipboard---v0811)
 - [`super_drag_and_drop` - `v0.8.11`](#super_drag_and_drop---v0811)
 - [`super_context_menu` - `v0.8.11`](#super_context_menu---v0811)
 - [`super_native_extensions` - `v0.8.11`](#super_native_extensions---v0811)
 - [`super_hot_key` - `v0.8.11`](#super_hot_key---v0811)
 - [`super_keyboard_layout` - `v0.8.11`](#super_keyboard_layout---v0811)

---

#### `super_clipboard` - `v0.8.11`

#### `super_drag_and_drop` - `v0.8.11`

#### `super_context_menu` - `v0.8.11`

#### `super_native_extensions` - `v0.8.11`

 - **FIX**: ignore scroll event in web drag driver.
 - **FIX**: ignore unknown pointer device kind (#344).

#### `super_hot_key` - `v0.8.11`

 - Bump "super_hot_key" to `0.8.11`.

#### `super_keyboard_layout` - `v0.8.11`


## 2024-03-26

### Changes

---

Packages with breaking changes:

 - There are no breaking changes in this release.

Packages with other changes:

 - [`super_clipboard` - `v0.8.10`](#super_clipboard---v0810)
 - [`super_drag_and_drop` - `v0.8.10`](#super_drag_and_drop---v0810)
 - [`super_context_menu` - `v0.8.10`](#super_context_menu---v0810)
 - [`super_native_extensions` - `v0.8.10`](#super_native_extensions---v0810)
 - [`super_hot_key` - `v0.8.10`](#super_hot_key---v0810)
 - [`super_keyboard_layout` - `v0.8.10`](#super_keyboard_layout---v0810)

---

#### `super_clipboard` - `v0.8.10`

#### `super_drag_and_drop` - `v0.8.10`

#### `super_context_menu` - `v0.8.10`

#### `super_native_extensions` - `v0.8.10`

#### `super_hot_key` - `v0.8.10`

 - Bump "super_hot_key" to `0.8.10`.

#### `super_keyboard_layout` - `v0.8.10`


## 2024-03-13

### Changes

---

Packages with breaking changes:

 - There are no breaking changes in this release.

Packages with other changes:

 - [`super_clipboard` - `v0.8.9`](#super_clipboard---v089)
 - [`super_drag_and_drop` - `v0.8.9`](#super_drag_and_drop---v089)
 - [`super_context_menu` - `v0.8.9`](#super_context_menu---v089)
 - [`super_native_extensions` - `v0.8.9`](#super_native_extensions---v089)
 - [`super_hot_key` - `v0.8.9`](#super_hot_key---v089)
 - [`super_keyboard_layout` - `v0.8.9`](#super_keyboard_layout---v089)

---

#### `super_clipboard` - `v0.8.9`

#### `super_drag_and_drop` - `v0.8.9`

#### `super_context_menu` - `v0.8.9`

#### `super_native_extensions` - `v0.8.9`

 - **FIX**: delay menu fade-out on iOS (#333).

#### `super_hot_key` - `v0.8.9`

 - Bump "super_hot_key" to `0.8.9`.

#### `super_keyboard_layout` - `v0.8.9`


## 2024-03-11

### Changes

---

Packages with breaking changes:

 - There are no breaking changes in this release.

Packages with other changes:

 - [`super_clipboard` - `v0.8.8`](#super_clipboard---v088)
 - [`super_drag_and_drop` - `v0.8.8`](#super_drag_and_drop---v088)
 - [`super_context_menu` - `v0.8.8`](#super_context_menu---v088)
 - [`super_native_extensions` - `v0.8.8`](#super_native_extensions---v088)
 - [`super_hot_key` - `v0.8.8`](#super_hot_key---v088)
 - [`super_keyboard_layout` - `v0.8.8`](#super_keyboard_layout---v088)

---

#### `super_clipboard` - `v0.8.8`

 - **FEAT**: add flac and ogg format definitions (#331).

#### `super_drag_and_drop` - `v0.8.8`

#### `super_context_menu` - `v0.8.8`

#### `super_native_extensions` - `v0.8.8`

 - **FIX**: regression with custom snapshot (#330).

#### `super_hot_key` - `v0.8.8`

 - Bump "super_hot_key" to `0.8.8`.

#### `super_keyboard_layout` - `v0.8.8`


## 2024-03-11

### Changes

---

Packages with breaking changes:

 - There are no breaking changes in this release.

Packages with other changes:

 - [`super_clipboard` - `v0.8.7`](#super_clipboard---v087)
 - [`super_drag_and_drop` - `v0.8.7`](#super_drag_and_drop---v087)
 - [`super_context_menu` - `v0.8.7`](#super_context_menu---v087)
 - [`super_native_extensions` - `v0.8.7`](#super_native_extensions---v087)
 - [`super_hot_key` - `v0.8.7`](#super_hot_key---v087)
 - [`super_keyboard_layout` - `v0.8.7`](#super_keyboard_layout---v087)

---

#### `super_clipboard` - `v0.8.7`

#### `super_drag_and_drop` - `v0.8.7`

 - **FIX**: various exceptions when getting snapshots (#328).

#### `super_context_menu` - `v0.8.7`

#### `super_native_extensions` - `v0.8.7`

#### `super_hot_key` - `v0.8.7`

#### `super_keyboard_layout` - `v0.8.7`


## 2024-03-11

### Changes

---

Packages with breaking changes:

 - There are no breaking changes in this release.

Packages with other changes:

 - [`super_clipboard` - `v0.8.6`](#super_clipboard---v086)
 - [`super_drag_and_drop` - `v0.8.6`](#super_drag_and_drop---v086)
 - [`super_context_menu` - `v0.8.6`](#super_context_menu---v086)
 - [`super_native_extensions` - `v0.8.6`](#super_native_extensions---v086)
 - [`super_hot_key` - `v0.8.6`](#super_hot_key---v086)
 - [`super_keyboard_layout` - `v0.8.6`](#super_keyboard_layout---v086)

---

#### `super_clipboard` - `v0.8.6`

#### `super_drag_and_drop` - `v0.8.6`

 - **FIX**: various exceptions when getting snapshots (#327).

#### `super_context_menu` - `v0.8.6`

#### `super_native_extensions` - `v0.8.6`

 - **FIX**: various exceptions when getting snapshots (#327).
 - **FIX**: fit menu position to bounds after inflating (#322).
 - **FIX**: assertion when taking snapshot of material widget (#320).

#### `super_hot_key` - `v0.8.6`

 - Bump "super_hot_key" to `0.8.6`.

#### `super_keyboard_layout` - `v0.8.6`


## 2024-02-08

### Changes

---

Packages with breaking changes:

 - There are no breaking changes in this release.

Packages with other changes:

 - [`super_clipboard` - `v0.8.5`](#super_clipboard---v085)
 - [`super_drag_and_drop` - `v0.8.5`](#super_drag_and_drop---v085)
 - [`super_context_menu` - `v0.8.5`](#super_context_menu---v085)
 - [`super_native_extensions` - `v0.8.5`](#super_native_extensions---v085)
 - [`super_hot_key` - `v0.8.5`](#super_hot_key---v085)
 - [`super_keyboard_layout` - `v0.8.5`](#super_keyboard_layout---v085)

---

#### `super_clipboard` - `v0.8.5`

 - **FEAT**: add m4a format definition (#306).

#### `super_drag_and_drop` - `v0.8.5`

#### `super_context_menu` - `v0.8.5`

#### `super_native_extensions` - `v0.8.5`

#### `super_hot_key` - `v0.8.5`

 - Bump "super_hot_key" to `0.8.5`.

#### `super_keyboard_layout` - `v0.8.5`


## 2024-01-16

### Changes

---

Packages with breaking changes:

 - There are no breaking changes in this release.

Packages with other changes:

 - [`super_clipboard` - `v0.8.4`](#super_clipboard---v084)
 - [`super_drag_and_drop` - `v0.8.4`](#super_drag_and_drop---v084)
 - [`super_context_menu` - `v0.8.4`](#super_context_menu---v084)
 - [`super_native_extensions` - `v0.8.4`](#super_native_extensions---v084)
 - [`super_hot_key` - `v0.8.4`](#super_hot_key---v084)
 - [`super_keyboard_layout` - `v0.8.4`](#super_keyboard_layout---v084)

---

#### `super_clipboard` - `v0.8.4`

#### `super_drag_and_drop` - `v0.8.4`

#### `super_context_menu` - `v0.8.4`

#### `super_native_extensions` - `v0.8.4`

#### `super_hot_key` - `v0.8.4`

 - Bump "super_hot_key" to `0.8.4`.

#### `super_keyboard_layout` - `v0.8.4`


## 2024-01-11

### Changes

---

Packages with breaking changes:

 - There are no breaking changes in this release.

Packages with other changes:

 - [`super_clipboard` - `v0.8.3`](#super_clipboard---v083)
 - [`super_drag_and_drop` - `v0.8.3`](#super_drag_and_drop---v083)
 - [`super_context_menu` - `v0.8.3`](#super_context_menu---v083)
 - [`super_native_extensions` - `v0.8.3`](#super_native_extensions---v083)
 - [`super_hot_key` - `v0.8.3`](#super_hot_key---v083)
 - [`super_keyboard_layout` - `v0.8.3`](#super_keyboard_layout---v083)

---

#### `super_clipboard` - `v0.8.3`

 - updated cargokit

#### `super_drag_and_drop` - `v0.8.3`

 - updated cargokit

#### `super_context_menu` - `v0.8.3`

 - updated cargokit

#### `super_native_extensions` - `v0.8.3`

#### `super_hot_key` - `v0.8.3`

 - updated cargokit

#### `super_keyboard_layout` - `v0.8.3`

 - updated cargokit


## 2024-01-02

### Changes

---

Packages with breaking changes:

 - There are no breaking changes in this release.

Packages with other changes:

 - [`super_clipboard` - `v0.8.2+1`](#super_clipboard---v0821)
 - [`super_drag_and_drop` - `v0.8.2+1`](#super_drag_and_drop---v0821)
 - [`super_context_menu` - `v0.8.2+1`](#super_context_menu---v0821)
 - [`super_native_extensions` - `v0.8.2+1`](#super_native_extensions---v0821)
 - [`super_hot_key` - `v0.8.2+1`](#super_hot_key---v0821)
 - [`super_keyboard_layout` - `v0.8.2+1`](#super_keyboard_layout---v0821)

---

#### `super_clipboard` - `v0.8.2+1`

#### `super_drag_and_drop` - `v0.8.2+1`

#### `super_context_menu` - `v0.8.2+1`

#### `super_native_extensions` - `v0.8.2+1`

 - **FIX**: remove leftover logging (#284).

#### `super_hot_key` - `v0.8.2+1`

 - Bump "super_hot_key" to `0.8.2+1`.

#### `super_keyboard_layout` - `v0.8.2+1`


## 2024-01-02

### Changes

---

Packages with breaking changes:

 - There are no breaking changes in this release.

Packages with other changes:

 - [`super_clipboard` - `v0.8.2`](#super_clipboard---v082)
 - [`super_drag_and_drop` - `v0.8.2`](#super_drag_and_drop---v082)
 - [`super_context_menu` - `v0.8.2`](#super_context_menu---v082)
 - [`super_native_extensions` - `v0.8.2`](#super_native_extensions---v082)
 - [`super_hot_key` - `v0.8.2`](#super_hot_key---v082)
 - [`super_keyboard_layout` - `v0.8.2`](#super_keyboard_layout---v082)

---

#### `super_clipboard` - `v0.8.2`

 - **FIX**: correct GIF uniform type identifier (#276).
 - **FEAT**: improve performance with large number of items (#283).

#### `super_drag_and_drop` - `v0.8.2`

 - **FEAT**: improve performance with large number of items (#283).
 - **FEAT**: use super_sliver_list in example (#281).
 - **FEAT**: improve performance with large number of items (#274).

#### `super_context_menu` - `v0.8.2`

#### `super_native_extensions` - `v0.8.2`

 - **FIX**: [android] possible deadlock when reading from clipboard (#282).
 - **FEAT**: improve performance with large number of items (#283).
 - **FEAT**: improve performance with large number of items (#274).

#### `super_hot_key` - `v0.8.2`

 - Bump "super_hot_key" to `0.8.2`.

#### `super_keyboard_layout` - `v0.8.2`


## 2023-12-26

### Changes

---

Packages with breaking changes:

 - There are no breaking changes in this release.

Packages with other changes:

 - [`super_clipboard` - `v0.8.1`](#super_clipboard---v081)
 - [`super_drag_and_drop` - `v0.8.1`](#super_drag_and_drop---v081)
 - [`super_context_menu` - `v0.8.1`](#super_context_menu---v081)
 - [`super_native_extensions` - `v0.8.1`](#super_native_extensions---v081)
 - [`super_hot_key` - `v0.8.1`](#super_hot_key---v081)
 - [`super_keyboard_layout` - `v0.8.1`](#super_keyboard_layout---v081)

---

#### `super_clipboard` - `v0.8.1`

#### `super_drag_and_drop` - `v0.8.1`

#### `super_context_menu` - `v0.8.1`

#### `super_native_extensions` - `v0.8.1`

 - **FIX**: [ios] store user interaction properly (#272).
 - **FIX**: no security scope NSURL access on macos (#271).
 - **FEAT**: [windows] cache file descriptors in reader (#266).

#### `super_hot_key` - `v0.8.1`

 - Bump "super_hot_key" to `0.8.1`.

#### `super_keyboard_layout` - `v0.8.1`


## 2023-12-22

### Changes

---

Packages with breaking changes:

 - There are no breaking changes in this release.

Packages with other changes:

 - [`super_clipboard` - `v0.8.0`](#super_clipboard---v080)
 - [`super_drag_and_drop` - `v0.8.0`](#super_drag_and_drop---v080)
 - [`super_context_menu` - `v0.8.0`](#super_context_menu---v080)
 - [`super_native_extensions` - `v0.8.0`](#super_native_extensions---v080)
 - [`super_hot_key` - `v0.8.0`](#super_hot_key---v080)
 - [`super_keyboard_layout` - `v0.8.0`](#super_keyboard_layout---v080)

---

#### `super_clipboard` - `v0.8.0`

#### `super_drag_and_drop` - `v0.8.0`

#### `super_context_menu` - `v0.8.0`

#### `super_native_extensions` - `v0.8.0`

 - **FIX**: access NSURL within security scope (#264).

#### `super_hot_key` - `v0.8.0`

 - Bump "super_hot_key" to `0.8.0`.

#### `super_keyboard_layout` - `v0.8.0`


## 2023-12-11

### Changes

---

Packages with breaking changes:

 - There are no breaking changes in this release.

Packages with other changes:

 - [`super_clipboard` - `v0.8.0-dev.3`](#super_clipboard---v080-dev3)
 - [`super_drag_and_drop` - `v0.8.0-dev.3`](#super_drag_and_drop---v080-dev3)
 - [`super_context_menu` - `v0.8.0-dev.3`](#super_context_menu---v080-dev3)
 - [`super_native_extensions` - `v0.8.0-dev.3`](#super_native_extensions---v080-dev3)
 - [`super_hot_key` - `v0.8.0-dev.3`](#super_hot_key---v080-dev3)
 - [`super_keyboard_layout` - `v0.8.0-dev.3`](#super_keyboard_layout---v080-dev3)

---

#### `super_clipboard` - `v0.8.0-dev.3`

 - **FEAT**: implement copy and cut events (#253).

#### `super_drag_and_drop` - `v0.8.0-dev.3`

 - Bump "super_drag_and_drop" to `0.8.0-dev.3`.

#### `super_context_menu` - `v0.8.0-dev.3`

 - Bump "super_context_menu" to `0.8.0-dev.3`.

#### `super_native_extensions` - `v0.8.0-dev.3`

 - **FEAT**: implement copy and cut events (#253).

#### `super_hot_key` - `v0.8.0-dev.3`

 - Bump "super_hot_key" to `0.8.0-dev.3`.

#### `super_keyboard_layout` - `v0.8.0-dev.3`

 - Bump "super_keyboard_layout" to `0.8.0-dev.3`.


## 2023-12-07

### Changes

---

Packages with breaking changes:

 - There are no breaking changes in this release.

Packages with other changes:

 - [`super_clipboard` - `v0.8.0-dev.2`](#super_clipboard---v080-dev2)
 - [`super_drag_and_drop` - `v0.8.0-dev.2`](#super_drag_and_drop---v080-dev2)
 - [`super_context_menu` - `v0.8.0-dev.2`](#super_context_menu---v080-dev2)
 - [`super_native_extensions` - `v0.8.0-dev.2`](#super_native_extensions---v080-dev2)
 - [`super_hot_key` - `v0.8.0-dev.2`](#super_hot_key---v080-dev2)
 - [`super_keyboard_layout` - `v0.8.0-dev.2`](#super_keyboard_layout---v080-dev2)

---

#### `super_clipboard` - `v0.8.0-dev.2`

 - Bump "super_clipboard" to `0.8.0-dev.2`.

#### `super_drag_and_drop` - `v0.8.0-dev.2`

 - Bump "super_drag_and_drop" to `0.8.0-dev.2`.

#### `super_context_menu` - `v0.8.0-dev.2`

 - Bump "super_context_menu" to `0.8.0-dev.2`.

#### `super_native_extensions` - `v0.8.0-dev.2`

 - Bump "super_native_extensions" to `0.8.0-dev.2`.

#### `super_hot_key` - `v0.8.0-dev.2`

 - Bump "super_hot_key" to `0.8.0-dev.2`.

#### `super_keyboard_layout` - `v0.8.0-dev.2`

 - Bump "super_keyboard_layout" to `0.8.0-dev.2`.


## 2023-12-07

### Changes

---

Packages with breaking changes:

 - There are no breaking changes in this release.

Packages with other changes:

 - [`super_clipboard` - `v0.8.0-dev.1`](#super_clipboard---v080-dev1)
 - [`super_drag_and_drop` - `v0.8.0-dev.1`](#super_drag_and_drop---v080-dev1)
 - [`super_context_menu` - `v0.8.0-dev.1`](#super_context_menu---v080-dev1)
 - [`super_native_extensions` - `v0.8.0-dev.1`](#super_native_extensions---v080-dev1)
 - [`super_hot_key` - `v0.8.0-dev.1`](#super_hot_key---v080-dev1)
 - [`super_keyboard_layout` - `v0.8.0-dev.1`](#super_keyboard_layout---v080-dev1)

---

#### `super_clipboard` - `v0.8.0-dev.1`

 - **FEAT**: preventDefault for paste event (#249).
 - **FEAT**: implement paste event on web (#246).

#### `super_drag_and_drop` - `v0.8.0-dev.1`

#### `super_context_menu` - `v0.8.0-dev.1`

#### `super_native_extensions` - `v0.8.0-dev.1`

 - **FEAT**: preventDefault for paste event (#249).
 - **FEAT**: implement paste event on web (#246).
 - **FEAT**: migrate to objc2 (#239).

#### `super_hot_key` - `v0.8.0-dev.1`

 - **FIX**: hot key event handlers should not be required (#248).

#### `super_keyboard_layout` - `v0.8.0-dev.1`


## 2023-11-15

### Changes

---

Packages with breaking changes:

 - There are no breaking changes in this release.

Packages with other changes:

 - [`super_clipboard` - `v0.7.3`](#super_clipboard---v073)
 - [`super_drag_and_drop` - `v0.7.3`](#super_drag_and_drop---v073)
 - [`super_context_menu` - `v0.7.3`](#super_context_menu---v073)
 - [`super_native_extensions` - `v0.7.3`](#super_native_extensions---v073)
 - [`super_hot_key` - `v0.7.3`](#super_hot_key---v073)
 - [`super_keyboard_layout` - `v0.7.3`](#super_keyboard_layout---v073)

---

#### `super_clipboard` - `v0.7.3`

#### `super_drag_and_drop` - `v0.7.3`

 - **FEAT**: improve touch device detection (#227).

#### `super_context_menu` - `v0.7.3`

 - **FEAT**: improve touch device detection (#227).

#### `super_native_extensions` - `v0.7.3`

 - **FIX**: let zone handle menu callback errors (#228).
 - **FEAT**: improve touch device detection (#227).

#### `super_hot_key` - `v0.7.3`

 - Bump "super_hot_key" to `0.7.3`.

#### `super_keyboard_layout` - `v0.7.3`


## 2023-10-25

### Changes

---

Packages with breaking changes:

 - There are no breaking changes in this release.

Packages with other changes:

 - [`super_clipboard` - `v0.7.2`](#super_clipboard---v072)
 - [`super_drag_and_drop` - `v0.7.2`](#super_drag_and_drop---v072)
 - [`super_context_menu` - `v0.7.2`](#super_context_menu---v072)
 - [`super_native_extensions` - `v0.7.2`](#super_native_extensions---v072)
 - [`super_hot_key` - `v0.7.2`](#super_hot_key---v072)
 - [`super_keyboard_layout` - `v0.7.2`](#super_keyboard_layout---v072)

---

#### `super_clipboard` - `v0.7.2`

#### `super_drag_and_drop` - `v0.7.2`

#### `super_context_menu` - `v0.7.2`

 - **FIX**: regression when context menu sometimes does not show (#220).

#### `super_native_extensions` - `v0.7.2`

 - Bump "super_native_extensions" to `0.7.2`.

#### `super_hot_key` - `v0.7.2`

 - Bump "super_hot_key" to `0.7.2`.

#### `super_keyboard_layout` - `v0.7.2`


## 2023-10-23

### Changes

---

Packages with breaking changes:

 - There are no breaking changes in this release.

Packages with other changes:

 - [`super_clipboard` - `v0.7.1`](#super_clipboard---v071)
 - [`super_drag_and_drop` - `v0.7.1`](#super_drag_and_drop---v071)
 - [`super_context_menu` - `v0.7.1`](#super_context_menu---v071)
 - [`super_native_extensions` - `v0.7.1`](#super_native_extensions---v071)
 - [`super_hot_key` - `v0.7.1`](#super_hot_key---v071)
 - [`super_keyboard_layout` - `v0.7.1`](#super_keyboard_layout---v071)

---

#### `super_clipboard` - `v0.7.1`

 - Bump "super_clipboard" to `0.7.1`.

#### `super_drag_and_drop` - `v0.7.1`

 - Bump "super_drag_and_drop" to `0.7.1`.

#### `super_context_menu` - `v0.7.1`

 - **FIX**: properly handle overlapping menu widgets (#217).

#### `super_native_extensions` - `v0.7.1`

#### `super_hot_key` - `v0.7.1`

 - Bump "super_hot_key" to `0.7.1`.

#### `super_keyboard_layout` - `v0.7.1`

 - Bump "super_keyboard_layout" to `0.7.1`.


## 2023-10-22

### Changes

---

Packages with breaking changes:

 - There are no breaking changes in this release.

Packages with other changes:

 - [`super_clipboard` - `v0.7.0`](#super_clipboard---v070)
 - [`super_drag_and_drop` - `v0.7.0`](#super_drag_and_drop---v070)
 - [`super_context_menu` - `v0.7.0`](#super_context_menu---v070)
 - [`super_native_extensions` - `v0.7.0`](#super_native_extensions---v070)
 - [`super_hot_key` - `v0.7.0`](#super_hot_key---v070)
 - [`super_keyboard_layout` - `v0.7.0`](#super_keyboard_layout---v070)

---

#### `super_clipboard` - `v0.7.0`

#### `super_drag_and_drop` - `v0.7.0`

#### `super_context_menu` - `v0.7.0`

#### `super_native_extensions` - `v0.7.0`

#### `super_hot_key` - `v0.7.0`

 - Bump "super_hot_key" to `0.7.0`.

#### `super_keyboard_layout` - `v0.7.0`


## 2023-10-11

### Changes

---

Packages with breaking changes:

 - There are no breaking changes in this release.

Packages with other changes:

 - [`super_clipboard` - `v0.7.0-dev.7`](#super_clipboard---v070-dev7)
 - [`super_drag_and_drop` - `v0.7.0-dev.7`](#super_drag_and_drop---v070-dev7)
 - [`super_context_menu` - `v0.7.0-dev.7`](#super_context_menu---v070-dev7)
 - [`super_native_extensions` - `v0.7.0-dev.7`](#super_native_extensions---v070-dev7)
 - [`super_hot_key` - `v0.7.0-dev.7`](#super_hot_key---v070-dev7)
 - [`super_keyboard_layout` - `v0.7.0-dev.7`](#super_keyboard_layout---v070-dev7)

---

#### `super_clipboard` - `v0.7.0-dev.7`

 - Bump "super_clipboard" to `0.7.0-dev.7`.

#### `super_drag_and_drop` - `v0.7.0-dev.7`

 - Bump "super_drag_and_drop" to `0.7.0-dev.7`.

#### `super_context_menu` - `v0.7.0-dev.7`

 - Bump "super_context_menu" to `0.7.0-dev.7`.

#### `super_native_extensions` - `v0.7.0-dev.7`

#### `super_hot_key` - `v0.7.0-dev.7`

 - Bump "super_hot_key" to `0.7.0-dev.7`.

#### `super_keyboard_layout` - `v0.7.0-dev.7`

 - Bump "super_keyboard_layout" to `0.7.0-dev.7`.


## 2023-10-07

### Changes

---

Packages with breaking changes:

 - There are no breaking changes in this release.

Packages with other changes:

 - [`super_clipboard` - `v0.7.0-dev.6`](#super_clipboard---v070-dev6)
 - [`super_drag_and_drop` - `v0.7.0-dev.6`](#super_drag_and_drop---v070-dev6)
 - [`super_context_menu` - `v0.7.0-dev.6`](#super_context_menu---v070-dev6)
 - [`super_native_extensions` - `v0.7.0-dev.6`](#super_native_extensions---v070-dev6)
 - [`super_hot_key` - `v0.7.0-dev.6`](#super_hot_key---v070-dev6)
 - [`super_keyboard_layout` - `v0.7.0-dev.6`](#super_keyboard_layout---v070-dev6)

---

#### `super_clipboard` - `v0.7.0-dev.6`

 - **FEAT**: add support for epub and md (#205).

#### `super_drag_and_drop` - `v0.7.0-dev.6`

#### `super_context_menu` - `v0.7.0-dev.6`

#### `super_native_extensions` - `v0.7.0-dev.6`

 - **FIX**: remove trailing null terminator from NSString (#207).
 - **FIX**: [iOS] crash when deferred image is set too quickly (#206).

#### `super_hot_key` - `v0.7.0-dev.6`

 - Bump "super_hot_key" to `0.7.0-dev.6`.

#### `super_keyboard_layout` - `v0.7.0-dev.6`


## 2023-09-26

### Changes

---

Packages with breaking changes:

 - There are no breaking changes in this release.

Packages with other changes:

 - [`super_clipboard` - `v0.7.0-dev.5`](#super_clipboard---v070-dev5)
 - [`super_drag_and_drop` - `v0.7.0-dev.5`](#super_drag_and_drop---v070-dev5)
 - [`super_context_menu` - `v0.7.0-dev.5`](#super_context_menu---v070-dev5)
 - [`super_native_extensions` - `v0.7.0-dev.5`](#super_native_extensions---v070-dev5)
 - [`super_hot_key` - `v0.7.0-dev.5`](#super_hot_key---v070-dev5)
 - [`super_keyboard_layout` - `v0.7.0-dev.5`](#super_keyboard_layout---v070-dev5)

---

#### `super_clipboard` - `v0.7.0-dev.5`

 - **FIX**: readFile fails when item is a String (#202).

#### `super_drag_and_drop` - `v0.7.0-dev.5`

#### `super_context_menu` - `v0.7.0-dev.5`

#### `super_native_extensions` - `v0.7.0-dev.5`

 - Bump "super_native_extensions" to `0.7.0-dev.5`.

#### `super_hot_key` - `v0.7.0-dev.5`

 - Bump "super_hot_key" to `0.7.0-dev.5`.

#### `super_keyboard_layout` - `v0.7.0-dev.5`


## 2023-09-22

### Changes

---

Packages with breaking changes:

 - There are no breaking changes in this release.

Packages with other changes:

 - [`super_clipboard` - `v0.7.0-dev.4`](#super_clipboard---v070-dev4)
 - [`super_drag_and_drop` - `v0.7.0-dev.4`](#super_drag_and_drop---v070-dev4)
 - [`super_context_menu` - `v0.7.0-dev.4`](#super_context_menu---v070-dev4)
 - [`super_native_extensions` - `v0.7.0-dev.4`](#super_native_extensions---v070-dev4)
 - [`super_hot_key` - `v0.7.0-dev.4`](#super_hot_key---v070-dev4)
 - [`super_keyboard_layout` - `v0.7.0-dev.4`](#super_keyboard_layout---v070-dev4)

---

#### `super_clipboard` - `v0.7.0-dev.4`

#### `super_drag_and_drop` - `v0.7.0-dev.4`

#### `super_context_menu` - `v0.7.0-dev.4`

#### `super_native_extensions` - `v0.7.0-dev.4`

#### `super_hot_key` - `v0.7.0-dev.4`

 - Bump "super_hot_key" to `0.7.0-dev.4`.

#### `super_keyboard_layout` - `v0.7.0-dev.4`


## 2023-09-12

### Changes

---

Packages with breaking changes:

 - There are no breaking changes in this release.

Packages with other changes:

 - [`super_clipboard` - `v0.7.0-dev.3`](#super_clipboard---v070-dev3)
 - [`super_drag_and_drop` - `v0.7.0-dev.3`](#super_drag_and_drop---v070-dev3)
 - [`super_context_menu` - `v0.7.0-dev.3`](#super_context_menu---v070-dev3)
 - [`super_native_extensions` - `v0.7.0-dev.3`](#super_native_extensions---v070-dev3)
 - [`super_hot_key` - `v0.7.0-dev.3`](#super_hot_key---v070-dev3)
 - [`super_keyboard_layout` - `v0.7.0-dev.3`](#super_keyboard_layout---v070-dev3)

---

#### `super_clipboard` - `v0.7.0-dev.3`

#### `super_drag_and_drop` - `v0.7.0-dev.3`

#### `super_context_menu` - `v0.7.0-dev.3`

#### `super_native_extensions` - `v0.7.0-dev.3`

 - **FIX**: [macOS] missing image on NSMenuItem with children (#197).

#### `super_hot_key` - `v0.7.0-dev.3`

 - Bump "super_hot_key" to `0.7.0-dev.3`.

#### `super_keyboard_layout` - `v0.7.0-dev.3`


## 2023-09-06

### Changes

---

Packages with breaking changes:

 - There are no breaking changes in this release.

Packages with other changes:

 - [`super_clipboard` - `v0.7.0-dev.2`](#super_clipboard---v070-dev2)
 - [`super_drag_and_drop` - `v0.7.0-dev.2`](#super_drag_and_drop---v070-dev2)
 - [`super_context_menu` - `v0.7.0-dev.2`](#super_context_menu---v070-dev2)
 - [`super_native_extensions` - `v0.7.0-dev.2`](#super_native_extensions---v070-dev2)
 - [`super_hot_key` - `v0.7.0-dev.2`](#super_hot_key---v070-dev2)
 - [`super_keyboard_layout` - `v0.7.0-dev.2`](#super_keyboard_layout---v070-dev2)

---

#### `super_clipboard` - `v0.7.0-dev.2`

#### `super_drag_and_drop` - `v0.7.0-dev.2`

 - **FIX**: multi-touch issues on Android (#196).

#### `super_context_menu` - `v0.7.0-dev.2`

 - **FIX**: multi-touch issues on Android (#196).
 - **FEAT**: restore focus before invoking menu callback (#191).

#### `super_native_extensions` - `v0.7.0-dev.2`

 - **FIX**: multi-touch issues on Android (#196).
 - **FIX**: improve closing of menu overlay on touch devices (#193).

#### `super_hot_key` - `v0.7.0-dev.2`

 - Bump "super_hot_key" to `0.7.0-dev.2`.

#### `super_keyboard_layout` - `v0.7.0-dev.2`


## 2023-08-30

### Changes

---

Packages with breaking changes:

 - There are no breaking changes in this release.

Packages with other changes:

 - [`super_clipboard` - `v0.7.0-dev.1`](#super_clipboard---v070-dev1)
 - [`super_drag_and_drop` - `v0.7.0-dev.1`](#super_drag_and_drop---v070-dev1)
 - [`super_context_menu` - `v0.7.0-dev.1`](#super_context_menu---v070-dev1)
 - [`super_native_extensions` - `v0.7.0-dev.1`](#super_native_extensions---v070-dev1)
 - [`super_hot_key` - `v0.7.0-dev.1`](#super_hot_key---v070-dev1)
 - [`super_keyboard_layout` - `v0.7.0-dev.1`](#super_keyboard_layout---v070-dev1)

---

#### `super_clipboard` - `v0.7.0-dev.1`

 - **FIX**: do not use path dependencies for published packages in pubspec.yaml (#183).

#### `super_drag_and_drop` - `v0.7.0-dev.1`

 - **FIX**: do not use path dependencies for published packages in pubspec.yaml (#183).

#### `super_context_menu` - `v0.7.0-dev.1`

 - **FIX**: do not use path dependencies for published packages in pubspec.yaml (#183).

#### `super_native_extensions` - `v0.7.0-dev.1`

#### `super_hot_key` - `v0.7.0-dev.1`

 - Bump "super_hot_key" to `0.7.0-dev.1`.

#### `super_keyboard_layout` - `v0.7.0-dev.1`

 - **FIX**: do not use path dependencies for published packages in pubspec.yaml (#183).


## 2023-08-23

### Changes

---

Packages with breaking changes:

 - There are no breaking changes in this release.

Packages with other changes:

 - [`super_clipboard` - `v0.6.4`](#super_clipboard---v064)
 - [`super_drag_and_drop` - `v0.6.4`](#super_drag_and_drop---v064)
 - [`super_context_menu` - `v0.6.4`](#super_context_menu---v064)
 - [`super_native_extensions` - `v0.6.4`](#super_native_extensions---v064)
 - [`super_hot_key` - `v0.6.4`](#super_hot_key---v064)
 - [`super_keyboard_layout` - `v0.6.4`](#super_keyboard_layout---v064)

---

#### `super_clipboard` - `v0.6.4`

 - **FIX**: update engine_context dependency.

#### `super_drag_and_drop` - `v0.6.4`

 - **FIX**: update engine_context dependency.

#### `super_context_menu` - `v0.6.4`

 - **FIX**: update engine_context dependency.

#### `super_native_extensions` - `v0.6.4`

 - **FIX**: update engine_context dependency.
 - **FIX**: escape script invocation in podspec.

#### `super_hot_key` - `v0.6.4`

 - Bump "super_hot_key" to `0.6.4`.

#### `super_keyboard_layout` - `v0.6.4`

 - **FIX**: update engine_context dependency.


## 2023-08-22

### Changes

---

Packages with breaking changes:

 - There are no breaking changes in this release.

Packages with other changes:

 - [`super_clipboard` - `v0.6.3`](#super_clipboard---v063)
 - [`super_drag_and_drop` - `v0.6.3`](#super_drag_and_drop---v063)
 - [`super_context_menu` - `v0.6.3`](#super_context_menu---v063)
 - [`super_native_extensions` - `v0.6.3`](#super_native_extensions---v063)
 - [`super_hot_key` - `v0.6.3`](#super_hot_key---v063)
 - [`super_keyboard_layout` - `v0.6.3`](#super_keyboard_layout---v063)

---

#### `super_clipboard` - `v0.6.3`

 - Bump "super_clipboard" to `0.6.3`.

#### `super_drag_and_drop` - `v0.6.3`

 - Bump "super_drag_and_drop" to `0.6.3`.

#### `super_context_menu` - `v0.6.3`

 - Bump "super_context_menu" to `0.6.3`.

#### `super_native_extensions` - `v0.6.3`

#### `super_hot_key` - `v0.6.3`

 - Bump "super_hot_key" to `0.6.3`.

#### `super_keyboard_layout` - `v0.6.3`

 - Bump "super_keyboard_layout" to `0.6.3`.


## 2023-08-22

### Changes

---

Packages with breaking changes:

 - There are no breaking changes in this release.

Packages with other changes:

 - [`super_clipboard` - `v0.6.2`](#super_clipboard---v062)
 - [`super_drag_and_drop` - `v0.6.2`](#super_drag_and_drop---v062)
 - [`super_context_menu` - `v0.6.2`](#super_context_menu---v062)
 - [`super_native_extensions` - `v0.6.2`](#super_native_extensions---v062)
 - [`super_hot_key` - `v0.6.2`](#super_hot_key---v062)
 - [`super_keyboard_layout` - `v0.6.2`](#super_keyboard_layout---v062)

---

#### `super_clipboard` - `v0.6.2`

 - Bump "super_clipboard" to `0.6.2`.

#### `super_drag_and_drop` - `v0.6.2`

 - Bump "super_drag_and_drop" to `0.6.2`.

#### `super_context_menu` - `v0.6.2`

 - Bump "super_context_menu" to `0.6.2`.

#### `super_native_extensions` - `v0.6.2`

#### `super_hot_key` - `v0.6.2`

 - Bump "super_hot_key" to `0.6.2`.

#### `super_keyboard_layout` - `v0.6.2`

 - Bump "super_keyboard_layout" to `0.6.2`.


## 2023-08-21

### Changes

---

Packages with breaking changes:

 - There are no breaking changes in this release.

Packages with other changes:

 - [`super_clipboard` - `v0.6.1`](#super_clipboard---v061)
 - [`super_drag_and_drop` - `v0.6.1`](#super_drag_and_drop---v061)
 - [`super_context_menu` - `v0.6.1`](#super_context_menu---v061)

---

#### `super_clipboard` - `v0.6.1`

 - **DOCS**: update comments.

#### `super_drag_and_drop` - `v0.6.1`

 - **DOCS**: update comments.

#### `super_context_menu` - `v0.6.1`

 - **FIX**: [iOS] gesture recognizer workaround (#176).


## 2023-08-07

### Changes

---

Packages with breaking changes:

 - [`super_clipboard` - `v0.6.0`](#super_clipboard---v060)
 - [`super_drag_and_drop` - `v0.6.0`](#super_drag_and_drop---v060)
 - [`super_native_extensions` - `v0.6.0`](#super_native_extensions---v060)
 - [`super_context_menu` - `v0.6.0`](#super_context_menu---v060)

Packages with other changes:

 - [`super_keyboard_layout` - `v0.6.0`](#super_keyboard_layout---v060)
 - [`super_hot_key` - `v0.6.0`](#super_hot_key---v060)

---

#### `super_clipboard` - `v0.6.0`

 - **FIX**: correct imports and add missing exports (#155).
 - **FEAT**: improve compatibility with current Flutter main (#163).
 - **BREAKING** **FIX**: correct typos and spelling in code (#156).
 - **BREAKING** **CHORE**: remove Pair and replace it with dart 3 record (#157).

#### `super_drag_and_drop` - `v0.6.0`

 - **FEAT**: improve compatibility with current Flutter main (#163).
 - **BREAKING** **FIX**: correct typos and spelling in code (#156).

#### `super_native_extensions` - `v0.6.0`

 - **FIX**: declare proper output in podspec script phase.
 - **FIX**: update rust dependencies (#170).
 - **FIX**: [windows] handle null terminator in CF_UNICODETEXT (#169).
 - **FIX**: use destructive icon theme when serializing menu images (#162).
 - **FIX**: [windows] keep IDataObjectAsyncCapability reference during drop (#161).
 - **FIX**: [windows] properly handle data objects that don't support async capability (#160).
 - **FIX**: formatting.
 - **FEAT**: improve compatibility with current Flutter main (#163).
 - **BREAKING** **FIX**: correct typos and spelling in code (#156).
 - **BREAKING** **CHORE**: remove Pair and replace it with dart 3 record (#157).

#### `super_context_menu` - `v0.6.0`

 - **FIX**: use destructive icon theme when serializing menu images (#162).
 - **FEAT**: improve compatibility with current Flutter main (#163).
 - **BREAKING** **FIX**: correct typos and spelling in code (#156).

#### `super_keyboard_layout` - `v0.6.0`

 - **FEAT**: improve compatibility with current Flutter main (#163).

#### `super_hot_key` - `v0.6.0`

 - Bump "super_hot_key" to `0.6.0`.


## 2023-07-22

### Changes

---

Packages with breaking changes:

 - [`super_clipboard` - `v0.5.0`](#super_clipboard---v050)
 - [`super_context_menu` - `v0.5.0`](#super_context_menu---v050)
 - [`super_drag_and_drop` - `v0.5.0`](#super_drag_and_drop---v050)
 - [`super_hot_key` - `v0.5.0`](#super_hot_key---v050)
 - [`super_keyboard_layout` - `v0.5.0`](#super_keyboard_layout---v050)
 - [`super_native_extensions` - `v0.5.0`](#super_native_extensions---v050)

Packages with other changes:

 - There are no other changes in this release.

---

#### `super_clipboard` - `v0.5.0`

 - **DOCS**: [android] mention minSdkVersion in readme (#150).
 - **DOCS**: update NDK installation information (#149).
 - **BREAKING** **FEAT**: upgrade to Dart 3 and jni 0.21.1 (#138).

#### `super_context_menu` - `v0.5.0`

 - **FIX**: context menu in list view not working on iOS (#144).
 - **FEAT**: implement safe triangle for desktop menu (#153).
 - **DOCS**: update NDK installation information (#149).
 - **DOCS**: fixup unnecessary capitalization.
 - **BREAKING** **FEAT**: upgrade to Dart 3 and jni 0.21.1 (#138).

#### `super_drag_and_drop` - `v0.5.0`

 - **FIX**: ensure drop regions are attached when invoking events (#147).
 - **FIX**: cache active items for snapshotter (#146).
 - **DOCS**: [android] mention minSdkVersion in readme (#150).
 - **DOCS**: update NDK installation information (#149).
 - **DOCS**: fix example.
 - **BREAKING** **FEAT**: upgrade to Dart 3 and jni 0.21.1 (#138).

#### `super_hot_key` - `v0.5.0`

 - **BREAKING** **FEAT**: upgrade to Dart 3 and jni 0.21.1 (#138).

#### `super_keyboard_layout` - `v0.5.0`

 - **DOCS**: update NDK installation information (#149).
 - **BREAKING** **FEAT**: upgrade to Dart 3 and jni 0.21.1 (#138).

#### `super_native_extensions` - `v0.5.0`

 - **FIX**: [macos] assertion when loading deferred menu (#152).
 - **FIX**: [macos] control key stuck after context menu closed (#151).
 - **FIX**: web drag avatar shown in non-root overlay (#139).
 - **FIX**: pasting text with semicolon on macOS (#133).
 - **BREAKING** **FEAT**: upgrade to Dart 3 and jni 0.21.1 (#138).


## 2023-05-22

### Changes

---

Packages with breaking changes:

 - There are no breaking changes in this release.

Packages with other changes:

 - [`super_clipboard` - `v0.4.0`](#super_clipboard---v040)
 - [`super_drag_and_drop` - `v0.4.0`](#super_drag_and_drop---v040)
 - [`super_native_extensions` - `v0.4.0`](#super_native_extensions---v040)
 - [`super_keyboard_layout` - `v0.4.0`](#super_keyboard_layout---v040)
 - [`super_hot_key` - `v0.4.0`](#super_hot_key---v040)
 - [`super_context_menu` - `v0.1.0`](#super_context_menu---v010)

---

#### `super_clipboard` - `v0.4.0`

 - Bump "super_clipboard" to `0.4.0`.

#### `super_drag_and_drop` - `v0.4.0`

 - Bump "super_drag_and_drop" to `0.4.0`.

#### `super_native_extensions` - `v0.4.0`

 - Bump "super_native_extensions" to `0.4.0`.

#### `super_keyboard_layout` - `v0.4.0`

 - Bump "super_keyboard_layout" to `0.4.0`.

#### `super_hot_key` - `v0.4.0`

 - Bump "super_hot_key" to `0.4.0`.

#### `super_context_menu` - `v0.1.0`

 - Bump "super_context_menu" to `0.1.0`.


## 2023-04-03

### Changes

---

Packages with breaking changes:

 - There are no breaking changes in this release.

Packages with other changes:

 - [`super_native_extensions` - `v0.3.0+2`](#super_native_extensions---v0302)
 - [`super_clipboard` - `v0.3.0+2`](#super_clipboard---v0302)
 - [`super_drag_and_drop` - `v0.3.0+2`](#super_drag_and_drop---v0302)
 - [`super_hot_key` - `v0.3.0+2`](#super_hot_key---v0302)
 - [`super_keyboard_layout` - `v0.3.0+2`](#super_keyboard_layout---v0302)

Packages with dependency updates only:

> Packages listed below depend on other packages in this workspace that have had changes. Their versions have been incremented to bump the minimum dependency versions of the packages they depend upon in this project.

 - `super_clipboard` - `v0.3.0+2`
 - `super_drag_and_drop` - `v0.3.0+2`
 - `super_hot_key` - `v0.3.0+2`
 - `super_keyboard_layout` - `v0.3.0+2`

---

#### `super_native_extensions` - `v0.3.0+2`

 - **FIX**: [win] rewind OLE streams before reading (#117).


## 2023-03-30

### Changes

---

Packages with breaking changes:

 - There are no breaking changes in this release.

Packages with other changes:

 - [`super_native_extensions` - `v0.3.0+1`](#super_native_extensions---v0301)
 - [`super_clipboard` - `v0.3.0+1`](#super_clipboard---v0301)
 - [`super_drag_and_drop` - `v0.3.0+1`](#super_drag_and_drop---v0301)
 - [`super_keyboard_layout` - `v0.3.0+1`](#super_keyboard_layout---v0301)
 - [`super_hot_key` - `v0.3.0+1`](#super_hot_key---v0301)

Packages with dependency updates only:

> Packages listed below depend on other packages in this workspace that have had changes. Their versions have been incremented to bump the minimum dependency versions of the packages they depend upon in this project.

 - `super_clipboard` - `v0.3.0+1`
 - `super_drag_and_drop` - `v0.3.0+1`
 - `super_keyboard_layout` - `v0.3.0+1`
 - `super_hot_key` - `v0.3.0+1`

---

#### `super_native_extensions` - `v0.3.0+1`

 - **FIX**: [android] local data only dragging not working (#115).


## 2023-03-29

### Changes

---

Packages with breaking changes:

 - There are no breaking changes in this release.

Packages with other changes:

 - [`super_clipboard` - `v0.3.0`](#super_clipboard---v030)
 - [`super_drag_and_drop` - `v0.3.0`](#super_drag_and_drop---v030)
 - [`super_native_extensions` - `v0.3.0`](#super_native_extensions---v030)
 - [`super_keyboard_layout` - `v0.3.0`](#super_keyboard_layout---v030)
 - [`super_hot_key` - `v0.3.0`](#super_hot_key---v030)

---

#### `super_clipboard` - `v0.3.0`

 - **FIX**: [android] build failing with proguard enabled (#114).
 - **FEAT**: add htmlFile format (#107).
 - **FEAT**: make format in DataReader.getFile optional (#90).

#### `super_drag_and_drop` - `v0.3.0`

 - **FIX**: [android] build failing with proguard enabled (#114).
 - **FIX**: [ios] respect isLocationDraggable check (#109).
 - **FIX**: super_drag_and_drop should reexport Format (#83).
 - **FEAT**: allow merging of snapshot prepare requests (#110).
 - **FEAT**: simplify lift snapshot logic on iOS (#108).
 - **FEAT**: improve snapshot API (#101).
 - **FEAT**: use widget to customize snapshot setting (#100).
 - **FEAT**: implement drag shadow on all platforms (#87).
 - **DOCS**: fix typo.
 - **DOCS**: improve super_drag_and_drop documentation (#106).

#### `super_native_extensions` - `v0.3.0`

 - **FIX**: [android] build failing with proguard enabled (#114).
 - **FIX**: custom snapshot should propagate exception from renderbox (#104).
 - **FIX**: [ios] revert memory leak fix removal (#103).
 - **FIX**: [web] dropping over platform views not working (#99).
 - **FIX**: [ios] use shadow path from correct image (#97).
 - **FIX**: [ios] force separate drag image to account for shadow difference (#92).
 - **FIX**: [web] dragging ocasionally getting stuck (#89).
 - **FIX**: [windows] pasting files from explorer (#88).
 - **FIX**: use unpremultiplied alpha for encoding image data (#85).
 - **FEAT**: allow merging of snapshot prepare requests (#110).
 - **FEAT**: snapshot optimization (#102).
 - **FEAT**: improve snapshot API (#101).
 - **FEAT**: use widget to customize snapshot setting (#100).
 - **FEAT**: [ios] use real shadow path instead of layer shadow (#95).
 - **FEAT**: [ios] remove drag item provider memory leak workaround (#93).
 - **FEAT**: implement drag shadow on all platforms (#87).

#### `super_keyboard_layout` - `v0.3.0`

 - **FIX**: [android] build failing with proguard enabled (#114).

#### `super_hot_key` - `v0.3.0`

 - n


## 2023-03-14

### Changes

---

Packages with breaking changes:

 - There are no breaking changes in this release.

Packages with other changes:

 - [`super_native_extensions` - `v0.2.4`](#super_native_extensions---v024)
 - [`super_clipboard` - `v0.2.3+1`](#super_clipboard---v0231)
 - [`super_hot_key` - `v0.1.1+1`](#super_hot_key---v0111)
 - [`super_drag_and_drop` - `v0.2.3+1`](#super_drag_and_drop---v0231)
 - [`super_keyboard_layout` - `v0.2.1+1`](#super_keyboard_layout---v0211)

Packages with dependency updates only:

> Packages listed below depend on other packages in this workspace that have had changes. Their versions have been incremented to bump the minimum dependency versions of the packages they depend upon in this project.

 - `super_clipboard` - `v0.2.3+1`
 - `super_hot_key` - `v0.1.1+1`
 - `super_drag_and_drop` - `v0.2.3+1`
 - `super_keyboard_layout` - `v0.2.1+1`

---

#### `super_native_extensions` - `v0.2.4`

 - **FEAT**: [macos] receiving virtual files from outlook attachments (#81).

