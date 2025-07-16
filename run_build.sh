#!/bin/bash
# Script to build precompiled binaries for super_native_extensions with GitHub token
# This script will automatically set the required environment variables and run the build

set -e

# Set environment variables
export GITHUB_TOKEN=
export PRIVATE_KEY=64b2b5b8e1bf93dd9b09a8457177d1aea6eaf7873c08439976f17283e2f00b2c8847e8fbd560c1356d06027ccbab7564bac0c1761867ab1f8bc50680639c5029
export CARGOKIT_TOOL_TEMP_DIR=/tmp/cargokit_temp

echo "Starting precompiled binaries build process..."
echo "Repository: marcosgcd/super_native_extensions"
echo "Manifest directory: /Users/marcos/repositories/sandbox/super_native_extensions/super_native_extensions/rust"
echo ""

# Setup the build environment
echo "Setting up build environment..."
mkdir -p $CARGOKIT_TOOL_TEMP_DIR
cd $CARGOKIT_TOOL_TEMP_DIR

# Create pubspec.yaml with absolute path
cat > pubspec.yaml << 'EOF'
name: build_tool_runner
version: 1.0.0
publish_to: none

environment:
  sdk: '>=3.0.0 <4.0.0'

dependencies:
  build_tool:
    path: "/Users/marcos/repositories/sandbox/super_native_extensions/super_native_extensions/cargokit/build_tool"
EOF

# Create bin directory and runner
mkdir -p bin
cat > bin/build_tool_runner.dart << 'EOF'
import 'package:build_tool/build_tool.dart' as build_tool;
void main(List<String> args) {
  build_tool.runMain(args);
}
EOF

# Build the tool if not already built
if [ ! -f "bin/build_tool_runner.dill" ]; then
    echo "Compiling build tool..."
    dart pub get --no-precompile
    dart compile kernel bin/build_tool_runner.dart
fi

echo "Building precompiled binaries..."
echo "This may take several minutes..."
echo ""

# Build precompiled binaries for macOS (current platform)
dart bin/build_tool_runner.dill precompile-binaries \
  --repository marcosgcd/super_native_extensions \
  --manifest-dir /Users/marcos/repositories/sandbox/super_native_extensions/super_native_extensions/rust \
  --verbose

if [ $? -eq 0 ]; then
    echo ""
    echo "✅ SUCCESS: Precompiled binaries have been generated and uploaded to your repository!"
    echo ""
    echo "📋 Summary:"
    echo "- Repository: https://github.com/marcosgcd/super_native_extensions"
    echo "- Public key: 8847e8fbd560c1356d06027ccbab7564bac0c1761867ab1f8bc50680639c5029"
    echo "- cargokit.yaml has been updated with new repository URL and public key"
    echo ""
    echo "🔄 Next steps:"
    echo "1. Commit and push the changes to your repository"
    echo "2. Update your external Flutter project to use your repository"
    echo "3. The renamed drop_md.rs file should now be included in the precompiled binaries"
    echo ""
    echo "⚠️  IMPORTANT: Keep your private key secure and do not share it!"
else
    echo ""
    echo "❌ ERROR: Failed to build precompiled binaries"
    echo "Please check the error messages above and try again"
    exit 1
fi
