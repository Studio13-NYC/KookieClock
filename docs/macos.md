# KookieClock on macOS

The Mac app uses AppKit for the desktop clock, context menu, settings, and visitor gallery. It shares the artwork, visit schedule, two shuffled casts, character movement, and sound mix with the Windows app.

## Run a locally built copy

Install Rust, Python 3, and the Xcode Command Line Tools on a Mac. From the repository:

```sh
bash tools/package-macos.sh
open dist/macos/KookieClock.app
```

The script builds for the Mac it runs on and writes `dist/macos/KookieClock-macos-arm64.zip` or `dist/macos/KookieClock-macos-x86_64.zip`. The deployment target is macOS 13. GitHub Actions builds and runs the desktop check on macOS 15 using separate Apple Silicon and Intel runners, then combines the tested builds into one universal ZIP.

## Share a test build with a friend

Download `KookieClock-macos-universal.zip` from the [Mac test release](https://github.com/Studio13-NYC/KookieClock/releases/tag/macos-v0.1.0-test). Share the release link or the ZIP. Your friend can extract it and move `KookieClock.app` to Applications.

The test build can be shared without Apple Developer membership. If macOS blocks the first launch and your friend trusts the copy you sent, macOS may offer **Open Anyway** under **System Settings → Privacy & Security** after they try to open the app. A managed Mac may restrict this choice. [Apple's instructions for opening downloaded apps](https://support.apple.com/en-us/102445)

## Controls

- Drag the clock to move it. Right-click the clock or click the clock icon in the menu bar to open its controls.
- Control–Option–C brings it forward by default. Under **Settings → Bring Forward**, choose an A–Z key and optionally add Shift. The letters refer to US keyboard positions. If the system rejects a new combination, the previous shortcut stays active. **Bring Clock Forward** in the menu bar remains available. Choose **Send Behind Windows** to put it back.
- At the hour and half-hour, the clock uses a floating window for the visit and returns to its previous level when the visit ends. Settings include sound mode, voice, bell, volume, visit intervals, quiet hours, motion, and clock size.
- Choose **Visit → Meet the Visitors** to select and preview any of the 24 friends. **Sound** also offers snooze and resume.
- Move the app to Applications before turning on **Settings → Launch at Login**. If macOS requests approval, click **Login Items…** in Settings and turn on KookieClock there. To stop it launching, turn the setting off.
- macOS saves settings and clock position in the app's preferences.

## Build checks

The [Mac workflow](https://github.com/Studio13-NYC/KookieClock/actions/workflows/macos.yml) tests the shared Rust library, extracts each ZIP, and runs the packaged app. The desktop check changes the global shortcut, checks login-item registration and removal when supported by the runner, opens the gallery, and walks through a scheduled-path visit with Dot. The log records the login-item result and whether the runner accepted audio playback. App captures and test ZIPs are attached to successful runs.

Those checks use a diagnostic visit and call the shortcut handler directly after registration. A Mac user can verify actual launch after signing in, key presses while another app is active, and a live hour boundary. Spaces, fullscreen apps, sleep and wake, and physical audio output are outside the automated checks.

The GitHub prerelease uses the universal build from CI with an ad hoc signature. For a new test release, download `KookieClock-macos-universal` from the successful Actions run and attach the enclosed ZIP to a GitHub prerelease.
