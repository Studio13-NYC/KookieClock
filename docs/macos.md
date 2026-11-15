# Mac port assessment

## Current state

KookieClock ships a Windows application. It does not yet ship a macOS `.app`.

The portable Rust library exposes the existing `model`, `cast`, and `audio` modules. Both the Windows application and library tests use these same implementations. Windows dependencies and resource compilation are platform-gated. GitHub Actions runs the 17 core tests on Apple Silicon (`macos-15`), Intel (`macos-15-intel`), Linux, and Windows. The workflow run, rather than this document, is the authority on a particular commit's results.

On a Mac with Rust installed, `cargo test --locked --release --lib` tests the portable core. A plain `cargo run` still targets the Windows frontend and is not a way to launch a Mac app.

## Feasibility without owning a Mac

A Mac version is technically feasible. The 24 character atlases, door frames, house artwork, audio assets, scheduler, sound synthesis, and choreography can be reused. The main work is replacing the Win32 frontend, not re-creating the clock's behavior or artwork.

GitHub provides standard hosted Intel and Apple Silicon macOS runners; standard runners are free for public repositories. These can build a Mac app and run automated tests without the developer owning a Mac. They are not a substitute for hands-on testing of desktop placement, Spaces, fullscreen apps, sleep/resume, hotkeys, Retina displays, and perceived sound synchronization. [GitHub runner documentation](https://docs.github.com/en/actions/reference/runners/github-hosted-runners)

Engineering assessment: high confidence that a working Mac port can be built. Confidence in a polished public release remains lower until at least one Mac user tests it. No percentage would be a measured probability here.

## Proposed native implementation

Keep the Rust library as the one source for scheduling, shuffle state, pose calculations, and PCM soundtracks. Add an AppKit frontend with native drawing and controls, using the same rendered assets. Avoid a browser runtime or a separate copy of clock logic.

Remaining work:

1. Transparent, draggable desktop window; temporary floating placement during scheduled visits; reliable return behind normal windows.
2. Core Graphics rendering of the existing image layers, clipped character frames, and live hands.
3. Native settings, context menu, menu-bar access, and global shortcut.
4. Audio playback tied to the same monotonic visit timeline, with cancellation on stop/suspend/stall.
5. Mac settings storage, launch-at-login integration, and display/Spaces behavior.
6. Apple Silicon and Intel builds, `.app` packaging, and desktop smoke tests.
7. Mac-user testing of visuals, audio, and ordinary desktop use.

## Distribution

For a normal downloadable public Mac release, plan for Developer ID signing and Apple notarization. This requires the appropriate Apple developer credentials; it is separate from compiling and testing the code. Do not publish an unsigned prototype as a verified, polished Mac release. [Apple distribution guidance](https://developer.apple.com/macos/distribution) and [notarization documentation](https://developer.apple.com/documentation/security/notarizing-macos-software-before-distribution/)

No Mac hardware has been purchased, no Apple account has been enrolled, and no signing credentials are included in this repository.
