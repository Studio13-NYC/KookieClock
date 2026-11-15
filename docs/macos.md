# KookieClock on macOS

The Mac preview uses AppKit for the desktop clock, context menu, settings, and visitor gallery. It shares the artwork, visit schedule, two shuffled casts, character movement, and sound mix with the Windows app.

## Run a locally built copy

Install Rust, Python 3, and the Xcode Command Line Tools on a Mac. From the repository:

```sh
bash tools/package-macos.sh
open dist/macos/KookieClock.app
```

The script builds for the Mac it runs on and writes `dist/macos/KookieClock-macos-arm64.zip` or `dist/macos/KookieClock-macos-x86_64.zip`. The deployment target is macOS 13. GitHub Actions builds and runs the desktop check on macOS 15 using separate Apple Silicon and Intel runners.

## Controls

- Drag the clock to move it. Right-click the clock or click the clock icon in the menu bar to open its controls.
- Control–Option–C brings it forward. Choose **Send Behind Windows** to put it back. If another app already uses the shortcut, use the menu bar item.
- At the hour and half-hour, the clock uses a floating window for the visit and returns to its previous level when the visit ends. Settings include sound mode, voice, bell, volume, visit intervals, quiet hours, motion, and clock size.
- Choose **Visit → Meet the Visitors** to select and preview any of the 24 friends. **Sound** also offers snooze and resume.
- macOS saves settings and clock position in the app's preferences. Startup at login and changing the shortcut are not part of this preview.

## Build checks

The [Mac workflow](https://github.com/Studio13-NYC/KookieClock/actions/workflows/macos.yml) tests the shared Rust library, packages the app, extracts each ZIP, and runs the app from that copy. Its desktop check opens settings and the visitor gallery, verifies that the clock draws, and walks through a scheduled-path visit with Dot, including the open door, return, closure, floating window, and soundtrack decoding. The log records whether the runner accepts audio playback. App captures and preview ZIPs are attached to the run when those steps succeed.

Those checks use a diagnostic visit. A Mac user still needs to verify a live hour boundary, the shortcut and desktop behavior across Spaces and fullscreen apps, sleep and wake, and sound from real speakers.

## Distribution

The build script uses ad hoc signing for development. The preview ZIPs are available through GitHub Actions for testing. Public distribution with the standard Gatekeeper experience requires Developer ID signing and Apple notarization. [Apple's distribution guidance](https://developer.apple.com/macos/distribution/)
