#!/bin/bash
# Script to build precompiled binaries for super_native_extensions
# 
# Before running this script, you need to:
# 1. Set your GitHub token: export GITHUB_TOKEN=your_github_token_here
# 2. Set the private key: export PRIVATE_KEY=64b2b5b8e1bf93dd9b09a8457177d1aea6eaf7873c08439976f17283e2f00b2c8847e8fbd560c1356d06027ccbab7564bac0c1761867ab1f8bc50680639c5029
# 3. Run this script

cd /tmp/cargokit_temp

# Build precompiled binaries for macOS (current platform)
dart bin/build_tool_runner.dill precompile-binaries \
  --repository marcosgcd/super_native_extensions \
  --manifest-dir /Users/marcos/repositories/sandbox/super_native_extensions/super_native_extensions/rust \
  --verbose

echo "Precompiled binaries have been generated and uploaded to your repository!"
echo "The following files have been updated:"
echo "- cargokit.yaml (with new repository URL and public key)"
echo ""
echo "Your new public key is: 8847e8fbd560c1356d06027ccbab7564bac0c1761867ab1f8bc50680639c5029"
echo "Your private key is: 64b2b5b8e1bf93dd9b09a8457177d1aea6eaf7873c08439976f17283e2f00b2c8847e8fbd560c1356d06027ccbab7564bac0c1761867ab1f8bc50680639c5029"
echo ""
echo "IMPORTANT: Keep your private key secure and do not share it!"
