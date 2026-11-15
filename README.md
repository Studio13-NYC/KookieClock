<p align="center">
  <img src="docs/images/kookieclock-hero.png" alt="KookieClock — A little company. Right on time. A painted wooden clock with duck, bear and mouse friends." width="1000">
</p>

<p align="center"><strong>A tiny woodland ritual for your desktop.</strong><br>Native Windows · Built with Rust · 24 visiting friends · Works offline</p>

<p align="center"><a href="https://github.com/Studio13-NYC/KookieClock/releases/latest"><strong>Download for Windows</strong></a> · <a href="#meet-the-cast">Meet the cast</a> · <a href="docs/windows-guide.md">User guide</a></p>

<p align="center"><sub>Promotional artwork created with imagegen from the real clock and character assets. Actual app screenshots below.</sub></p>

# KookieClock

[![Build and test](https://github.com/Studio13-NYC/KookieClock/actions/workflows/ci.yml/badge.svg)](https://github.com/Studio13-NYC/KookieClock/actions/workflows/ci.yml)

A little wooden clock with 24 visiting friends. Written in Rust, with a native Windows desktop interface, original rendered wooden-toy artwork, and offline sound.

The Windows application is currently named **Little Timber**. The repository is **KookieClock**; existing application settings and shortcuts keep working.

**A moment of delight, twice an hour.** A gentle bell announces the moment. Little wooden doors swing open. A friend comes out to greet you, makes its own little sound, turns around and heads home. Then the clock chimes the time.

**Made to live on your desktop.** No slide-out shelf. No permanent toolbar. Just a clock, with its controls a right-click away.

**A quieter kind of company.** Switch to Zen mode for singing bowls and temple bells, or set quiet hours and let the pendulum keep you company in silence.

## On your desktop

<table>
  <tr>
    <td align="center" width="42%"><img src="docs/images/clock-live.png" alt="Actual running Windows clock with turquoise roof, yellow doors, floral decoration and live hands" width="330"><br><strong>Just the clock.</strong><br>A little painted timber, a gently moving pendulum.</td>
    <td align="center" width="58%"><img src="docs/images/visitor-gallery.png" alt="Actual native visitor gallery showing Dot the Duck, the cast list, and Hear and visit controls" width="500"><br><strong>Meet your next visitor.</strong><br>Browse the cast and preview their greetings.</td>
  </tr>
</table>

These are unretouched captures of the running Windows application, shown at reduced size. The gallery uses native Windows controls; the clock itself has no visible menu bar.

## Windows download

Download the Windows x64 ZIP from [Releases](https://github.com/Studio13-NYC/KookieClock/releases), extract it, and open `LittleTimber.exe`. No installer, account, browser runtime, or network connection is required. Windows 10/11 x64; the executable is unsigned.

The clock rests behind other windows. At the hour and half-hour it temporarily comes forward without stealing keyboard focus, completes its visit and final chimes, then returns behind them.

- **Ctrl+Alt+C:** bring forward. **Escape:** send behind.
- **Right-click:** Sound, Visit, Settings, and Quit.
- **Drag:** move the clock; optionally lock its position in Settings.
- **F2:** settings. **Space:** preview a shuffled visitor.
- The notification-area icon also opens the controls.

## Meet the cast

### The hourly hello

<table>
  <tr>
    <td align="center"><img src="assets/woodland/visitor-00.png" alt="Pip the Cuckoo, original wooden character from the application" width="120"><br><strong>Pip</strong><br><sub>Cuckoo</sub></td>
    <td align="center"><img src="assets/woodland/visitor-01.png" alt="Rosie the Robin, original wooden character from the application" width="120"><br><strong>Rosie</strong><br><sub>Robin</sub></td>
    <td align="center"><img src="assets/woodland/visitor-02.png" alt="Otis the Owl, original wooden character from the application" width="120"><br><strong>Otis</strong><br><sub>Owl</sub></td>
    <td align="center"><img src="assets/woodland/visitor-03.png" alt="Blue the Bluebird, original wooden character from the application" width="120"><br><strong>Blue</strong><br><sub>Bluebird</sub></td>
    <td align="center"><img src="assets/woodland/visitor-04.png" alt="Sunny the Cockatoo, original wooden character from the application" width="120"><br><strong>Sunny</strong><br><sub>Cockatoo</sub></td>
    <td align="center"><img src="assets/woodland/visitor-05.png" alt="Pebble the Penguin, original wooden character from the application" width="120"><br><strong>Pebble</strong><br><sub>Penguin</sub></td>
  </tr>
  <tr>
    <td align="center"><img src="assets/woodland/visitor-06.png" alt="Mango the Toucan, original wooden character from the application" width="120"><br><strong>Mango</strong><br><sub>Toucan</sub></td>
    <td align="center"><img src="assets/woodland/visitor-07.png" alt="Dot the Duck, original wooden character from the application" width="120"><br><strong>Dot</strong><br><sub>Duck</sub></td>
    <td align="center"><img src="assets/woodland/visitor-08.png" alt="Fern the Finch, original wooden character from the application" width="120"><br><strong>Fern</strong><br><sub>Finch</sub></td>
    <td align="center"><img src="assets/woodland/visitor-09.png" alt="Knock the Woodpecker, original wooden character from the application" width="120"><br><strong>Knock</strong><br><sub>Woodpecker</sub></td>
    <td align="center"><img src="assets/woodland/visitor-10.png" alt="Pearl the Dove, original wooden character from the application" width="120"><br><strong>Pearl</strong><br><sub>Dove</sub></td>
    <td align="center"><img src="assets/woodland/visitor-11.png" alt="Indigo the Peacock, original wooden character from the application" width="120"><br><strong>Indigo</strong><br><sub>Peacock</sub></td>
  </tr>
</table>

### The half-hour company

<table>
  <tr>
    <td align="center"><img src="assets/woodland/visitor-12.png" alt="Clover the Fox, original wooden character from the application" width="120"><br><strong>Clover</strong><br><sub>Fox</sub></td>
    <td align="center"><img src="assets/woodland/visitor-13.png" alt="Coco the Rabbit, original wooden character from the application" width="120"><br><strong>Coco</strong><br><sub>Rabbit</sub></td>
    <td align="center"><img src="assets/woodland/visitor-14.png" alt="Bruno the Bear, original wooden character from the application" width="120"><br><strong>Bruno</strong><br><sub>Bear</sub></td>
    <td align="center"><img src="assets/woodland/visitor-15.png" alt="Clementine the Cat, original wooden character from the application" width="120"><br><strong>Clementine</strong><br><sub>Cat</sub></td>
    <td align="center"><img src="assets/woodland/visitor-16.png" alt="Hazel the Hedgehog, original wooden character from the application" width="120"><br><strong>Hazel</strong><br><sub>Hedgehog</sub></td>
    <td align="center"><img src="assets/woodland/visitor-17.png" alt="Miso the Mouse, original wooden character from the application" width="120"><br><strong>Miso</strong><br><sub>Mouse</sub></td>
  </tr>
  <tr>
    <td align="center"><img src="assets/woodland/visitor-18.png" alt="Moss the Frog, original wooden character from the application" width="120"><br><strong>Moss</strong><br><sub>Frog</sub></td>
    <td align="center"><img src="assets/woodland/visitor-19.png" alt="Fawn the Deer, original wooden character from the application" width="120"><br><strong>Fawn</strong><br><sub>Deer</sub></td>
    <td align="center"><img src="assets/woodland/visitor-20.png" alt="Acorn the Squirrel, original wooden character from the application" width="120"><br><strong>Acorn</strong><br><sub>Squirrel</sub></td>
    <td align="center"><img src="assets/woodland/visitor-21.png" alt="Bandit the Raccoon, original wooden character from the application" width="120"><br><strong>Bandit</strong><br><sub>Raccoon</sub></td>
    <td align="center"><img src="assets/woodland/visitor-22.png" alt="Bamboo the Panda, original wooden character from the application" width="120"><br><strong>Bamboo</strong><br><sub>Panda</sub></td>
    <td align="center"><img src="assets/woodland/visitor-23.png" alt="Olive the Otter, original wooden character from the application" width="120"><br><strong>Olive</strong><br><sub>Otter</sub></td>
  </tr>
</table>

Original in-app portraits. Each friend has a different silhouette and its own small performance, from Otis’s curious swivel to Miso’s scratching paws.


Twelve birds visit on the hour and twelve woodland friends on the half-hour. Each cast shuffles without repeating until everyone has had a turn. Characters stay solid: doors open, the visitor enters from the left, performs, turns around, returns inside to the right, and the doors close before the time chimes.

The opening bell, wooden door contacts, foot movements, character voice, and closing chimes share one soundtrack and animation timeline. Duck, mouse, and bear use embedded generated naturalistic samples; other voices and bells are synthesized. Zen mode offers singing bowls and temple bells.

Controls include volume, sound choices, quiet hours, snooze, separate hour/half-hour switches, hour-count chimes, reduced motion, clock size, optional second hand/digital time, position locking, startup at sign-in, and a configurable hotkey.

## Platform status

| Platform | Current status |
| --- | --- |
| Windows x64 | Working native Win32/GDI+ application and portable ZIP |
| macOS Apple Silicon | Portable Rust core checked in CI; desktop interface still to be implemented |
| macOS Intel | Portable Rust core checked in CI; desktop interface still to be implemented |
| Linux | Portable Rust core checked in CI; no desktop interface |

**There is no Mac application download yet.** The scheduler, settings model, shuffled cast, choreography, and audio mixer are a shared Rust library. A Mac port needs a native window, renderer, controls, playback, and OS integration. See [the Mac port assessment](docs/macos.md).

## Build and test

On Windows, install stable Rust for `x86_64-pc-windows-msvc` and Visual Studio C++ Build Tools with a Windows SDK. From this repository:

```powershell
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked --release
cargo build --locked --release
powershell -ExecutionPolicy Bypass -File .\tools\package.ps1 -SkipBuild
```

The package is written to `dist/LittleTimber-0.1.0-windows-x64.zip`. Artwork and audio are embedded; Blender and Python are only needed to regenerate assets or run developer QA tools.

The portable core can be tested on Windows, macOS, or Linux:

```sh
cargo test --locked --release --lib
```

There are 17 automated tests covering scheduling, quiet/mute/snooze, shuffling, solid travel and door ordering, audio synchronization, bell counts, sample boundaries, and headroom. One additional ignored test exports audition WAVs. Mac core tests do **not** establish Mac UI, audio-device playback, or desktop behavior.

All repository text uses **UTF-8**, without a byte-order mark. Editor defaults are in `.editorconfig`; CI checks the encoding of tracked text files.

## Documentation and assets

- [Windows user and developer guide](docs/windows-guide.md): settings, commands, build instructions, and prior desktop verification.
- [Mac port assessment](docs/macos.md): reusable code, remaining work, and validation limits.
- [Clock behavior and sound research](docs/cuckoo-behavior-and-sound-research.md).
- [Audio provenance](assets/audio/provenance.json): generated samples and their source prompts.
- `tools/render_woodland.py` and `assets/woodland/clock-source.blend`: original artwork source.
- `src/lib.rs`: shared Rust clock and audio library; `src/main.rs`, `src/ui.rs`, and `src/draw.rs`: Windows application.

Local settings, runtime logs, recordings, build output, and unrelated projects are excluded from version control. App data stays in `%LOCALAPPDATA%\LittleTimber` on Windows.

## License

A project license has not yet been selected.
