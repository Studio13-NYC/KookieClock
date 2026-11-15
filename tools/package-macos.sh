#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."
export MACOSX_DEPLOYMENT_TARGET=13.0
arch="$(uname -m)"
if [[ "$(uname -s)" != Darwin ]]; then
    echo "Building the AppKit bundle requires macOS and Xcode Command Line Tools." >&2
    exit 1
fi

cargo rustc --locked --release --lib --crate-type staticlib
target_dir="$(cargo metadata --locked --no-deps --format-version 1 |
    python3 -c 'import json, sys; print(json.load(sys.stdin)["target_directory"])')"

app="dist/macos/KookieClock.app"
resources="$app/Contents/Resources"
mkdir -p "$app/Contents/MacOS" "$resources/woodland"
cp macos/Info.plist "$app/Contents/Info.plist"
cp assets/woodland/{house,pendulum,doors-*,visitor-*}.png "$resources/woodland/"

xcrun --sdk macosx clang -fobjc-arc -mmacosx-version-min="$MACOSX_DEPLOYMENT_TARGET" \
    -Wall -Wextra -Wno-unused-parameter -Wno-deprecated-declarations \
    macos/main.m macos/ClockView.m "$target_dir/release/liblittle_timber.a" \
    -framework AppKit -framework AVFoundation -framework Carbon -framework CoreGraphics -framework ImageIO \
    -framework CoreFoundation -framework Security -framework SystemConfiguration \
    -liconv -lresolv -o "$app/Contents/MacOS/KookieClock"

plutil -lint "$app/Contents/Info.plist"
codesign --force --sign - "$app"
codesign --verify --strict "$app"
ditto -c -k --keepParent "$app" "dist/macos/KookieClock-macos-$arch.zip"
