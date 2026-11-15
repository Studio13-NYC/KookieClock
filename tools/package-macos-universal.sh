#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 2 || "$(uname -s)" != Darwin ]]; then
    echo "Usage on macOS: bash tools/package-macos-universal.sh ARM64_ZIP INTEL_ZIP" >&2
    exit 1
fi

scratch="$(mktemp -d)"
trap 'rm -rf "$scratch"' EXIT
ditto -x -k "$1" "$scratch/arm"
ditto -x -k "$2" "$scratch/intel"
arm="$scratch/arm/KookieClock.app"
intel="$scratch/intel/KookieClock.app"
binary="Contents/MacOS/KookieClock"

codesign --verify --strict "$arm"
codesign --verify --strict "$intel"
cmp "$arm/Contents/Info.plist" "$intel/Contents/Info.plist"
diff -qr "$arm/Contents/Resources" "$intel/Contents/Resources"
lipo "$arm/$binary" -verify_arch arm64
lipo "$intel/$binary" -verify_arch x86_64
lipo -create "$arm/$binary" "$intel/$binary" -output "$scratch/KookieClock"
mv "$scratch/KookieClock" "$arm/$binary"
chmod 755 "$arm/$binary"
lipo "$arm/$binary" -verify_arch arm64 x86_64
codesign --force --sign - "$arm"
codesign --verify --strict "$arm"

mkdir -p dist/macos
ditto -c -k --keepParent "$arm" dist/macos/KookieClock-macos-universal.zip
