# Little Timber: Windows user and developer guide

A freestanding Windows clock written in Rust, with native Win32 controls and GDI+ drawing. Its original 3D-rendered artwork depicts rounded maple, colorful matte paint, a routed bird alcove, a turned dial, and 24 wooden toy visitors. The turquoise roof and base, sunflower shutters, coral dial surround, and small floral inlays take their cues from painted wooden toys. No browser runtime, network connection, account, or downloaded sound packs. The artwork is embedded in the executable; Blender is not needed to run it.

## Run

Open `dist\LittleTimber\LittleTimber.exe`. The application is portable; keep the executable wherever you want. It normally rests behind other application windows. The notification-area icon remains available (Windows may put it under the tray's overflow arrow).

- **Ctrl+Alt+C** brings the clock forward. Change the letter or add Shift in Settings. Shortcut conflicts are reported rather than silently ignored.
- **Escape** sends it behind other windows. When brought forward, it temporarily appears in the taskbar and Alt+Tab; it disappears from those when sent behind.
- **Drag the house** to move it. Lock its position in Settings.
- **Right-click the clock** for **Sound**, **Visit**, and **Settings**. There is no permanent control bar.
- **F2** opens Settings; **Space** previews a shuffled visit when the clock has focus.
- The tray icon also provides the menu. Use **Recover clock position** if a display layout changes.
- Choose **Quit Little Timber** to stop the application. Starting the executable again brings the existing clock forward, rather than creating a second clock.

## Visits and sounds

The hour has 12 bird characters; the half-hour has 12 woodland characters. Each cast uses its own shuffled bag: all 12 appear before a repeat, with no immediate repeat between bags. Bags and position are saved between runs. Shuffled previews consume a turn in the bag; choosing a specific friend in the gallery does not.

Each character has its own wooden construction, markings, and articulated animation. Examples include the low duck with a flat bill and wagging tail, block-shaped owl with a swiveling head, tall cockatoo with a lifting crest, long-eared hopping rabbit, barrel-shaped bear waving a paw, tiny mouse scratching with its front paws, and four-legged deer stepping a hoof. The gallery describes and animates each character's action. Twelve rendered articulation poses and thirteen real side/back turn views are embedded per character; there is no 3D runtime. Fixed studio lighting and discrete rendered frames remain visible limitations.

Default sounds include duck quacks, owl hoots, dove coos, woodpecker taps, mouse claw scratches, rabbit foot taps, a mellow bear growl, a cat meow, frog croaks, and squirrel chatter. Duck, Mouse, and Bear use embedded naturalistic sound-effect samples generated for Little Timber, edited and level-matched for gentle playback. The duck has two short quacks; the bear has a breathy, irregular low rumble; the mouse alternates three dry claw strokes on its existing paw-contact phases. Other voices remain procedural toy impressions. None are field recordings.

Each visit starts with a soft 1.6-second bell at the hour/half-hour boundary, with the doors closed. The sunflower-yellow doors then open over 1.1 seconds; the solid visitor travels out from behind the left side onto the ledge over 1.6 seconds. It performs and makes one character call (or a meditation tone in Zen mode), turns through 180 degrees over 1.2 seconds, then returns inside to the right over 1.6 seconds. The doorway physically occludes it before the doors close. A quarter-second after the doors finish closing, the clock chimes the hour count (1–12) or one strike for the half-hour. Midnight and noon both count twelve. The opening bell is softer than the closing chimes; both use the selected Bell sound. The character does not repeat its call to count the hour.

There are no opacity fades or crossfades. The ordinary visual sequence including the opening bell lasts 12.7 seconds; Zen gives its visitor tone time to ring before the turn. Closing bell strikes are spaced 1.3 seconds apart, with overlapping six-second resonances. Each complete visit is mixed into one PCM soundtrack: the opening bell continues beneath the doors, and the final chimes retain their tails. Quiet release clicks, hinge friction, wooden stops, foot contacts, and a turn scuff follow the same timing as the animation. Mouse scratches, woodpecker pecks, rabbit landings, deer hoof contacts, and panda nibbling follow their repeating poses. Reduced motion omits travel/hinge sounds and retains only the discrete door contacts. Zen reduces mechanical detail. Previews use the complete sequence; a gallery bird previews the current hour count and a woodland animal previews one half-hour strike. Turning off gentle animation uses static front/back poses and discrete door changes. Stop, Silent, snooze, applying settings, or starting another preview cancels pending sound cues. Active visits are cancelled on Windows suspend notification or an animation timer stall exceeding 750 ms, so an old soundtrack is not resumed behind a new visual state.

- **Cuckoo:** each character's own voice, classic cuckoo, soft wooden cuckoo, or forest whistle.
- **Zen:** warm singing bowl, bright singing bowl, temple bell, or a variety of all three. These are original synthesized bell tones with different resonance profiles, paired frequencies for natural beating, mallet attacks, and individual decays; they are not acoustic recordings.
- **Silent:** visual visits without sound.
- Independent hour and half-hour switches. Hour-count chimes are enabled by default; unchecking “Chime the hour count after the visit” leaves one closing bell. Zen keeps meditation tones throughout the sequence.
- Volume, quiet hours, optional suppression of visitors during quiet hours, and 1-hour/4-hour snooze.
- Preview bypasses quiet hours and snooze, but respects the selected Silent mode and volume. Settings previews use the unsaved controls; gallery previews use saved settings.
- Scheduled hour and half-hour visits temporarily bring the clock above ordinary application windows, without taking keyboard focus. It stays visible through the closing chimes, then returns behind other windows. If you had deliberately brought it forward, it returns to that normal (non-topmost) placement instead. Stop, sleep, or a stalled timer releases the temporary topmost placement. The hotkey or Send behind windows overrides it immediately. Gallery/manual previews do not automatically pop forward. Quiet hours and snooze still govern sound; suppressing visitors during quiet hours also suppresses the popup. Windows secure desktops and exclusive fullscreen applications are outside this desktop-window behavior.

The scheduler follows the Windows local clock. It only announces live hour/half-hour boundaries and never replays a backlog after sleep, launch, or a large time adjustment. A backward clock adjustment re-enables previously announced boundaries that are now in the future, so setting 19:01 back to 18:58 permits the next 19:00 visit. Changes within an already-announced minute remain deduplicated. Clock/timer discontinuities are recorded in clock.log. It does not wake a sleeping computer. Sound plays through the Windows default output device; the clock's volume does not change system volume.

## Settings

Settings include four sizes, animation/pendulum toggle, optional second hand and digital time, 12/24-hour digital format, position locking, and opt-in launch at sign-in. Fresh defaults are medium size, 35% volume, both visit schedules enabled, quiet hours 22:00–08:00, an analog-only face, and startup disabled, with closing hour-count chimes enabled. There are no status plaques, brand labels, or permanent controls on the clock. The dial has Bahnschrift numerals printed into the rendered face, a quiet minute track, and live tapered hands.

**Apply** saves while keeping Settings open. **Save** saves and closes. **Cancel** discards edits made since the last save. Quiet hours use `HH:MM` in 24-hour notation; equal start and end means quiet all day.

Data is local:

```text
%LOCALAPPDATA%\LittleTimber\settings.json
%LOCALAPPDATA%\LittleTimber\clock.log
```

Settings are written through a temporary file and renamed into place. Unreadable settings are backed up before defaults are used. Launch at sign-in uses the current user's `Software\Microsoft\Windows\CurrentVersion\Run\LittleTimber` value. No administrator privileges are needed. Keep the executable in a stable location if you enable launch at sign-in; disable it before moving/removing the app.

Command-line entry points (also work when an instance is already running):

```powershell
.\LittleTimber.exe --settings
.\LittleTimber.exe --gallery
.\LittleTimber.exe --quit
.\LittleTimber.exe --behind
.\LittleTimber.exe --stop
```

For development checks, `--test-hour` and `--test-half` trigger the actual scheduled-visit path immediately without changing the Windows clock. They respect quiet/sound settings, consume a shuffled visitor, and are explicitly marked as diagnostics in `clock.log`. They do not change the next real boundary or the schedule switches.

## Build from source

Run build commands from the repository root.

Requires stable Rust for `x86_64-pc-windows-msvc`, Visual Studio C++ build tools, and the Windows SDK resource compiler. Runtime target is Windows 10/11 x64. `Cargo.lock` pins the dependency resolution.

```powershell
Set-Location KookieClock
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo build --release
.\target\release\little-timber.exe
powershell -ExecutionPolicy Bypass -File .\tools\package.ps1 -SkipBuild
```

The package script copies the release executable and README into `dist\LittleTimber`, and creates `dist\LittleTimber-0.1.0-windows-x64.zip`. The checked-in icon can optionally be regenerated with `python tools\make_icon.py`; Python is not required to build or run the app.

## Implementation and verification

- `src/lib.rs`: portable library exposing the clock model, cast, and sound generation.
- `src/main.rs`: window lifecycle, tray, hotkey, configuration, playback, scheduler integration.
- `src/draw.rs`: native composition, live hands, animation, and premultiplied-alpha layered-window canvas.
- `src/artwork.rs`: embedded PNG decoding and lifetime management for native GDI+ images.
- `src/ui.rs`: native settings controls and visitor gallery.
- `src/model.rs`: quiet hours, persisted settings, shuffle bags, boundary scheduler.
- `src/cast.rs`: the 24 character definitions.
- `src/audio.rs`: local PCM mixing of procedural sounds and embedded samples, played asynchronously through Windows.
- `assets/audio/`: processed 24 kHz mono PCM samples, original generated MP3 sources, and generation provenance.
- `tools/prepare_creature_audio.py`: reproducible filtering, edits, edge smoothing, and level matching; requires developer-only ffmpeg, NumPy, and SciPy. No runtime audio codec or network dependency.
- `assets/app.manifest`: Per-Monitor-V2 DPI awareness, common controls, standard-user execution.
- `assets/woodland/`: the house and pendulum, 24 visitor contact atlases with 25 poses each, 24 neutral visitor images, 17 door-opening frames, and the clock's Blender source scene.
- `tools/render_woodland.py`: reproducible geometry, materials, lighting, and dial typography for all components. Character palettes are read from `src/cast.rs`. Windows' Bahnschrift font is used at render time; no font file is redistributed.

To regenerate the artwork with the installed Blender 5.1 (not required for ordinary Rust builds):

```powershell
& 'C:\Program Files\Blender Foundation\Blender 5.1\blender.exe' --background --factory-startup --threads 4 --python .\tools\render_woodland.py -- all
```

The script runs in an isolated Blender process and does not open or alter a user's Blender scene. Individual targets are `house`, `pendulum`, `doors`, `characters`, or a character index `0`–`23`. Rebuild the executable after regenerating images.

Gallery animation invalidates only the illustration rectangle. The native visitor list, caption, and buttons retain their own repaint lifecycle. The idle row-disappearance bug was reproduced on November 10, 2026 and the fix was checked in the running packaged app, both at the top of the list and after selecting/scrolling to Olive the Otter.

Seventeen automated tests cover scheduling (including backward clock resets), quiet/mute/snooze behavior, shuffle bags, the 24 distinct voices, bell presets, audio headroom, travel/turn/door ordering, foot-contact timing, reduced-motion sound events, complete mixed-track overlap, cancellation/stalls, one/seven/twelve strike counts, and embedded sample levels, boundaries, and mouse contact windows. An additional ignored test exports audition WAVs and cue sheets for output-loopback comparison:

```powershell
cargo test --release
cargo test --release export_audition_tracks -- --ignored --nocapture
```

The synchronized-sound revision was checked in the running packaged app on November 10, 2026. Duck and Mouse previews showed solid entry, performance, return, and closure; Duck was observed back-facing on the right before disappearing inside. A Zen Mouse preview completed with the longer bowl timing. The actual 20:00 scheduled Mango visit started at 20:00:00.164 local time, closed its doors, and played eight closing strikes. WASAPI output captures of Duck, Mouse, Zen Mouse, and the scheduled Mango visit matched their reference mixes with correlations above 0.999999. These measurements validate the complete soundtrack, overlapping tails, and output continuity; they are not an independent subjective listening assessment. Stop was exercised during Bruno's final ringing tail: the capture stopped early and remained silent. The final build also passed an unsaved reduced-motion preview: the pendulum stayed still, Pip used static front/back poses, doors changed discretely, and no travel/hinge events were logged. Preview motion is captured with its soundtrack so unsaved controls cannot desynchronize the two. Saved animation remained enabled after cancelling Settings. Physical sleep/resume was not performed; cancellation is covered by tests and a Windows suspend handler.

QA files are under `artifacts/audio-qa/`, including `loopback-comparison.json`, reference WAVs/cue sheets, and live output recordings. `tools/record_loopback.py` captures only the default Windows output, never a microphone, using optional PyAudioWPatch. It is a developer tool, not an application dependency.

```powershell
python -m pip install --target artifacts/qa-python PyAudioWPatch
python tools/record_loopback.py artifacts/audio-qa/live.wav --seconds 45
```


On November 10, 2026, the built application was checked on this Windows desktop: rendering/transparency, native settings, saving/reloading selected size and mode, Ctrl+Alt+C, Escape, the gallery, menu access, and a Zen preview were exercised. A real 18:00 hourly event was logged with Otis in Silent mode. The subsequent wooden-artwork revision was checked in the live clock for transparency, dial/hand alignment, absence of labels, and a Duck visitor's scale and animation. The door revision was then observed at rest (closed), during opening, fully open with Pip, and closed again after the visit. The creature-audio revision was exercised through the Duck gallery preview with the default character voice. A real 18:30 half-hour event was logged for Moss the Frog with sound enabled. The final colorful build was inspected live with closed decorated shutters and Sunny performing on the ledge, followed by closed shutters again. Automated tests verify the travel/door ordering and that audio finishes before retreat. Windows accepted playback without an error; speaker sound quality was not independently assessed. Wake behavior is covered by logic tests, not a physical suspend/resume session. The later three-stage audio sequence was checked live with Pearl and at the actual 19:00 boundary with Knock: the critter turned and returned before the doors closed, and logs recorded the opening bell, visitor sound, and closing time-chime cue in order. The hourly event selected seven closing strikes.

This is a normal shaped desktop window using documented Windows APIs, not a Windows Widgets panel or an Explorer wallpaper extension. Windows **Show desktop** may hide it; use the hotkey or tray to recover it. Full multi-monitor DPI transitions, taskbar recreation, and sign-in startup have not been manually tested. The custom-drawn clock itself has no individual accessibility elements; settings, the context menu, and gallery use native keyboard-accessible controls. The portable executable is unsigned.

The reported missing repeat 19:00 visit was traced to a Windows clock reset from 19:01:17 to 18:58:06 after the initial event. The old deduplication history suppressed the repeated boundary. A regression test reproduced that suppression and passes with the corrected scheduler. This rollback was simulated in the scheduler tests; verification did not alter the Windows clock.

The November 11 foreground revision was checked in the running packaged app using `--test-hour` and `--test-half`: scheduled visits appeared above maximized Notepad without changing keyboard focus, then returned behind it after the bell tail. Stop and manual hotkey/send-behind overrides released topmost placement. These were explicit scheduled-path diagnostics, not a wait for a new wall-clock boundary. The updated Duck, Mouse, and Bear samples were captured through WASAPI and matched complete reference mixes above 0.999999 correlation, with no capture status errors. Settings and position were preserved. Evidence: `artifacts/foreground-qa/verification.md`, `runtime.log`, and `artifacts/natural-audio/live-comparison.json` in the original local workspace; these local captures and logs are not included in the public repository.
