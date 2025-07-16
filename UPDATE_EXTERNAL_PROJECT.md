# Steps to update your external Flutter project to use the new precompiled binaries

## 1. Update pubspec.yaml in your external Flutter project

Change the dependency from:
```yaml
dependencies:
  super_drag_and_drop:
    git: https://github.com/superlistapp/super_native_extensions.git
    path: super_drag_and_drop
```

To:
```yaml
dependencies:
  super_drag_and_drop:
    git: https://github.com/marcosgcd/super_native_extensions.git
    path: super_drag_and_drop
```

## 2. Clear Flutter cache and rebuild

Run these commands in your external Flutter project:

```bash
# Clear Flutter cache
flutter clean

# Delete pub cache (optional but recommended)
rm -rf ~/.pub-cache/git/

# Get new dependencies
flutter pub get

# For iOS, clean the build folder
rm -rf ios/build/

# For Android, clean the build folder  
rm -rf android/build/

# Rebuild the project
flutter build <your-target>
```

## 3. Alternative: Use dependency override

If you want to keep the original dependency but use your fork, add this to pubspec.yaml:

```yaml
dependency_overrides:
  super_native_extensions:
    git:
      url: https://github.com/marcosgcd/super_native_extensions.git
      path: super_native_extensions
```

## 4. Disable precompiled binaries (if needed)

If you still have issues, you can force building from source by creating a `cargokit_options.yaml` file in the root of your external Flutter project:

```yaml
use_precompiled_binaries: false
```

This will force the build system to compile from source using your `drop_md.rs` file.

## 5. Verify the change

After rebuilding, check if the error messages now show `drop_md.rs` instead of `drop.rs`.
