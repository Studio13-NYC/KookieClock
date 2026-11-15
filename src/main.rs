#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::result::Result::Ok;
mod artwork;
mod draw;
mod ui;

use chrono::{Local, Timelike};
use little_timber::{audio, cast, model};
use model::*;
use std::{
    cell::RefCell,
    fs,
    io::Write,
    path::PathBuf,
    ptr::{null, null_mut},
    time::Instant,
};
use windows_sys::Win32::{
    Foundation::*,
    Graphics::{Gdi::*, GdiPlus::*},
    Media::Audio::*,
    Security::Cryptography::*,
    System::{LibraryLoader::*, Registry::*, Threading::*},
    UI::{Controls::*, HiDpi::*, Input::KeyboardAndMouse::*, Shell::*, WindowsAndMessaging::*},
};
const CLASS: &str = "LittleTimber.Clock";
const TRAY: u32 = WM_APP + 1;
const REVEAL: u32 = WM_APP + 2;
const HOTKEY: i32 = 1;
const MENU_SHOW: usize = 101;
const MENU_SETTINGS: usize = 102;
const MENU_GALLERY: usize = 103;
const MENU_VISIT: usize = 104;
const MENU_CUCKOO: usize = 105;
const MENU_ZEN: usize = 113;
const MENU_SILENT: usize = 114;
const MENU_HOUR: usize = 115;
const MENU_HALF: usize = 116;
const MENU_SNOOZE: usize = 106;
const MENU_WAKE: usize = 107;
const MENU_BACK: usize = 108;
const MENU_QUIT: usize = 109;
const MENU_STOP: usize = 110;
const MENU_SNOOZE4: usize = 111;
const MENU_RESET: usize = 112;
thread_local! {static APP:RefCell<Option<App>>=const{RefCell::new(None)};}
pub fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(Some(0)).collect()
}
pub struct Visit {
    pub id: usize,
    pub start: Instant,
    pub motion: bool,
    pub audio: audio::VisitAudio,
}
pub struct App {
    artwork: artwork::Artwork,
    hwnd: HWND,
    settings_hwnd: HWND,
    gallery_hwnd: HWND,
    saved: Saved,
    dir: PathBuf,
    canvas: Option<draw::Surface>,
    scheduler: Scheduler,
    start: Instant,
    visit: Option<Visit>,
    sound: Vec<u8>,
    foreground: bool,
    // Previous manual placement while a scheduled visit temporarily stays on top.
    scheduled_front: Option<bool>,
    gallery_id: usize,
    preview_half: bool,
    hotkey_ok: bool,
    taskbar_message: u32,
    font: HFONT,
    heading_font: HFONT,
    ui_scale: f32,
    icon: HICON,
    last_status: String,
}
impl App {
    fn load(dir: PathBuf) -> (Saved, Option<String>) {
        let path = dir.join("settings.json");
        let (mut saved, warning) = match fs::read(&path) {
            Ok(data) => match serde_json::from_slice::<Saved>(&data) {
                Ok(s) => (s, None),
                Err(e) => {
                    let backup = dir.join(format!(
                        "settings-unreadable-{}.json",
                        Local::now().format("%Y%m%d-%H%M%S")
                    ));
                    let copied = fs::copy(&path, &backup);
                    (
                        Saved::default(),
                        Some(format!(
                            "Settings could not be read: {e}.\nOriginal backup: {} ({})",
                            backup.display(),
                            if copied.is_ok() {
                                "saved"
                            } else {
                                "backup failed; original remains"
                            }
                        )),
                    )
                }
            },
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => (Saved::default(), None),
            Err(e) => (
                Saved::default(),
                Some(format!("Could not read {}: {e}", path.display())),
            ),
        };
        saved.settings.normalize();
        saved.hour_bag.validate();
        saved.half_bag.validate();
        (saved, warning)
    }
    fn log(&self, s: &str) {
        if let Ok(mut f) = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(self.dir.join("clock.log"))
        {
            let _ = writeln!(f, "{} {s}", Local::now().to_rfc3339());
        }
    }
    fn save(&self) -> Result<(), String> {
        let data = serde_json::to_vec_pretty(&self.saved).map_err(|e| e.to_string())?;
        let tmp = self.dir.join("settings.tmp");
        let mut f = fs::File::create(&tmp).map_err(|e| e.to_string())?;
        f.write_all(&data).map_err(|e| e.to_string())?;
        f.sync_all().map_err(|e| e.to_string())?;
        drop(f);
        fs::rename(tmp, self.dir.join("settings.json")).map_err(|e| e.to_string())
    }
    unsafe fn save_or_warn(&self) {
        if let Err(e) = self.save() {
            self.error(&format!(
                "Your changes work for this session, but could not be saved.\n{e}"
            ));
        }
    }
    unsafe fn error(&self, message: &str) {
        self.log(message);
        MessageBoxW(
            if self.settings_hwnd.is_null() {
                self.hwnd
            } else {
                self.settings_hwnd
            },
            wide(message).as_ptr(),
            wide("Little Timber").as_ptr(),
            MB_OK | MB_ICONWARNING,
        );
    }
    unsafe fn register_hotkey(&mut self) -> bool {
        UnregisterHotKey(self.hwnd, HOTKEY);
        self.hotkey_ok = RegisterHotKey(
            self.hwnd,
            HOTKEY,
            MOD_CONTROL
                | MOD_ALT
                | MOD_NOREPEAT
                | if self.saved.settings.hotkey_shift {
                    MOD_SHIFT
                } else {
                    0
                },
            self.saved.settings.hotkey_letter as u32,
        ) != 0;
        self.hotkey_ok
    }
    fn hotkey_label(&self) -> String {
        format!(
            "Ctrl+Alt+{}{}",
            if self.saved.settings.hotkey_shift {
                "Shift+"
            } else {
                ""
            },
            self.saved.settings.hotkey_letter as char
        )
    }
    unsafe fn add_tray(&self) {
        let mut n: NOTIFYICONDATAW = std::mem::zeroed();
        n.cbSize = std::mem::size_of_val(&n) as u32;
        n.hWnd = self.hwnd;
        n.uID = 1;
        n.uFlags = NIF_ICON | NIF_MESSAGE | NIF_TIP;
        n.uCallbackMessage = TRAY;
        n.hIcon = self.icon;
        let tip = wide(&format!(
            "Little Timber • {} • Right-click for controls",
            self.hotkey_label()
        ));
        let len = tip.len().min(n.szTip.len() - 1);
        n.szTip[..len].copy_from_slice(&tip[..len]);
        if Shell_NotifyIconW(NIM_ADD, &n) == 0 {
            self.log("Could not add notification-area icon");
        }
    }
    unsafe fn remove_tray(&self) {
        let mut n: NOTIFYICONDATAW = std::mem::zeroed();
        n.cbSize = std::mem::size_of_val(&n) as u32;
        n.hWnd = self.hwnd;
        n.uID = 1;
        Shell_NotifyIconW(NIM_DELETE, &n);
    }
    unsafe fn foreground(&mut self) {
        self.release_scheduled_front();
        self.foreground = true;
        // Expose a normal taskbar/Alt-Tab entry while deliberately brought forward.
        // The resting desktop ornament remains a tool window with only its tray icon.
        ShowWindow(self.hwnd, SW_HIDE);
        SetWindowLongPtrW(
            self.hwnd,
            GWL_EXSTYLE,
            (WS_EX_LAYERED | WS_EX_APPWINDOW) as isize,
        );
        ShowWindow(self.hwnd, SW_SHOWNORMAL);
        SetWindowPos(
            self.hwnd,
            HWND_TOP,
            0,
            0,
            0,
            0,
            SWP_NOMOVE | SWP_NOSIZE | SWP_SHOWWINDOW | SWP_FRAMECHANGED,
        );
        SetForegroundWindow(self.hwnd);
        self.render();
    }
    unsafe fn behind(&mut self) {
        self.release_scheduled_front();
        if self.foreground {
            ShowWindow(self.hwnd, SW_HIDE);
            SetWindowLongPtrW(
                self.hwnd,
                GWL_EXSTYLE,
                (WS_EX_LAYERED | WS_EX_TOOLWINDOW) as isize,
            );
            ShowWindow(self.hwnd, SW_SHOWNOACTIVATE);
        }
        self.foreground = false;
        SetWindowPos(
            self.hwnd,
            HWND_BOTTOM,
            0,
            0,
            0,
            0,
            SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE | SWP_FRAMECHANGED | SWP_SHOWWINDOW,
        );
    }
    unsafe fn raise_scheduled_visit(&mut self) {
        let focused = GetForegroundWindow();
        self.scheduled_front = Some(self.foreground);
        let ok = SetWindowPos(
            self.hwnd,
            HWND_TOPMOST,
            0,
            0,
            0,
            0,
            SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE | SWP_SHOWWINDOW,
        ) != 0;
        self.log(&format!(
            "Scheduled visibility: raised={ok} topmost={} keyboard_focus_preserved={}",
            GetWindowLongPtrW(self.hwnd, GWL_EXSTYLE) & WS_EX_TOPMOST as isize != 0,
            focused == GetForegroundWindow()
        ));
    }
    unsafe fn release_scheduled_front(&mut self) -> Option<bool> {
        let previous = self.scheduled_front.take()?;
        SetWindowPos(
            self.hwnd,
            HWND_NOTOPMOST,
            0,
            0,
            0,
            0,
            SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE,
        );
        self.log(&format!(
            "Scheduled visibility: released topmost={}",
            GetWindowLongPtrW(self.hwnd, GWL_EXSTYLE) & WS_EX_TOPMOST as isize != 0
        ));
        Some(previous)
    }
    unsafe fn end_visit(&mut self) {
        self.stop_sound();
        self.visit = None;
        if self.release_scheduled_front() == Some(false) {
            self.behind();
        }
    }
    unsafe fn resize(&mut self) {
        let dpi = GetDpiForWindow(self.hwnd).max(96) as f32 / 96.;
        let factor = [0.65, 0.85, 1.0, 1.2][self.saved.settings.size] * dpi;
        let w = (draw::WIDTH * factor).round() as i32;
        let h = (draw::HEIGHT * factor).round() as i32;
        match draw::Surface::new(w, h) {
            Ok(c) => self.canvas = Some(c),
            Err(e) => {
                self.error(&e);
                return;
            }
        }
        SetWindowPos(
            self.hwnd,
            null_mut(),
            0,
            0,
            w,
            h,
            SWP_NOZORDER | SWP_NOMOVE | SWP_NOACTIVATE,
        );
        self.ensure_visible(false);
        self.render();
    }
    unsafe fn ensure_visible(&mut self, reset: bool) {
        let mut rect: RECT = std::mem::zeroed();
        GetWindowRect(self.hwnd, &mut rect);
        let monitor = MonitorFromRect(&rect, MONITOR_DEFAULTTONEAREST);
        let mut info: MONITORINFO = std::mem::zeroed();
        info.cbSize = std::mem::size_of_val(&info) as u32;
        GetMonitorInfoW(monitor, &mut info);
        let a = info.rcWork;
        let w = rect.right - rect.left;
        let h = rect.bottom - rect.top;
        let x = if reset {
            a.right - w - 24
        } else {
            rect.left.clamp(a.left, (a.right - w).max(a.left))
        };
        let y = if reset {
            a.top + 24
        } else {
            rect.top.clamp(a.top, (a.bottom - h).max(a.top))
        };
        SetWindowPos(
            self.hwnd,
            null_mut(),
            x,
            y,
            0,
            0,
            SWP_NOSIZE | SWP_NOZORDER | SWP_NOACTIVATE,
        );
        self.saved.position = Some((x, y));
    }
    fn status(&self) -> String {
        let now = Local::now();
        let s = &self.saved.settings;
        if let Some(v) = &self.visit {
            return format!("{} the {}", cast::CAST[v.id].name, cast::CAST[v.id].kind);
        }
        if now.timestamp() < s.snooze_until {
            return format!(
                "Snoozing · {}",
                chrono::DateTime::from_timestamp(s.snooze_until, 0)
                    .unwrap_or_default()
                    .with_timezone(&Local)
                    .format("%H:%M")
            );
        }
        if s.is_quiet((now.hour() * 60 + now.minute()) as u16) {
            return "Quiet hours · a little peace".into();
        }
        if s.mode == Mode::Silent {
            return "Silent · still keeping time".into();
        }
        if !s.hourly && !s.half_hourly {
            return "Chimes off · visits on request".into();
        }
        for offset in 1..=60 {
            let total = now.hour() * 60 + now.minute() + offset;
            let m = total % 60;
            if (m == 0 && s.hourly) || (m == 30 && s.half_hourly) {
                return format!(
                    "Next visit {:02}:{:02} · {}",
                    (total / 60) % 24,
                    m,
                    if s.mode == Mode::Zen {
                        "zen"
                    } else {
                        "woodland"
                    }
                );
            }
        }
        String::new()
    }
    unsafe fn render(&mut self) {
        let status = self.status();
        if self.last_status != status {
            self.last_status = status.clone();
            SetWindowTextW(
                self.hwnd,
                wide(&format!("Little Timber — {status}")).as_ptr(),
            );
        }
        if let Some(d) = &self.canvas {
            let mut settings = self.saved.settings.clone();
            if let Some(v) = &self.visit {
                settings.motion = v.motion;
            }
            let visit = self
                .visit
                .as_ref()
                .map(|v| (v.start.elapsed().as_secs_f32(), v.audio.visual_duration));
            draw::clock(
                d,
                draw::ClockFrame {
                    settings: &settings,
                    now: Local::now(),
                    time: self.start.elapsed().as_secs_f32(),
                    visitor: self.visit.as_ref().map_or(0, |v| v.id),
                    visit,
                },
                &self.artwork,
            );
            if !d.present(self.hwnd) {
                self.log("UpdateLayeredWindow failed");
            }
        }
    }
    unsafe fn stop_sound(&mut self) {
        if let Some(v) = &mut self.visit {
            v.audio.cancel();
        }
        PlaySoundW(null(), null_mut(), 0);
        self.sound.clear();
    }
    unsafe fn play_wave(&mut self, wav: Vec<u8>) {
        // Replacing one cue must not cancel the remaining stages.
        PlaySoundW(null(), null_mut(), 0);
        self.sound = wav;
        if PlaySoundW(
            self.sound.as_ptr() as *const u16,
            null_mut(),
            SND_MEMORY | SND_ASYNC | SND_NODEFAULT,
        ) == 0
        {
            self.error("Windows could not play the sound. Check your default audio output device.");
        }
    }
    unsafe fn visit(
        &mut self,
        id: usize,
        preview: bool,
        slot: Option<Slot>,
        override_settings: Option<Settings>,
    ) {
        let s = override_settings.unwrap_or_else(|| self.saved.settings.clone());
        let now = Local::now();
        let quiet = s.is_quiet((now.hour() * 60 + now.minute()) as u16);
        let hourly = slot.map_or(id < 12, |slot| slot == Slot::Hour);
        let count = closing_chime_count(now.hour(), hourly, s.count_hour);
        let audible = if preview {
            s.mode != Mode::Silent && s.volume > 0
        } else {
            s.sound_allowed((now.hour() * 60 + now.minute()) as u16, now.timestamp())
        };
        self.end_visit();
        self.visit = if preview || !(s.quiet_motion && quiet) {
            // Prepare before starting the shared audio/animation clock.
            let audio = audio::VisitAudio::new(id, &s, count, audible);
            Some(Visit {
                id,
                start: Instant::now(),
                motion: s.motion,
                audio,
            })
        } else {
            None
        };
        self.log(&format!(
            "{} {:?}: {} ({}) mode={:?} audible={audible} count={count}",
            if preview { "Preview" } else { "Scheduled" },
            slot,
            cast::CAST[id].name,
            cast::CAST[id].kind,
            s.mode
        ));
        if !preview && self.visit.is_some() {
            self.raise_scheduled_visit();
        }
        self.play_visit_cue();
        self.render();
    }
    unsafe fn random_visit(&mut self, slot: Slot, preview: bool) {
        let id = match slot {
            Slot::Hour => self.saved.hour_bag.draw(random),
            Slot::Half => 12 + self.saved.half_bag.draw(random),
        };
        self.visit(id, preview, Some(slot), None);
        self.save_or_warn();
    }
    unsafe fn play_visit_cue(&mut self) {
        if self
            .visit
            .as_mut()
            .is_some_and(|v| v.audio.stalled(v.start.elapsed().as_secs_f32()))
        {
            self.log("Visit cancelled after a timer stall; no delayed soundtrack replay");
            self.end_visit();
            return;
        }
        let ready = self.visit.as_mut().and_then(|v| {
            let elapsed = v.start.elapsed().as_secs_f32();
            v.audio.take_due(elapsed).map(|cue| (v.id, elapsed, cue))
        });
        if let Some((id, elapsed, cue)) = ready {
            self.log(&format!(
                "Visit cue: {} / {} at {:.2}s",
                cast::CAST[id].name,
                cue.name,
                elapsed
            ));
            self.play_wave(cue.wav);
        }
        let mut markers = Vec::new();
        if let Some(v) = &mut self.visit {
            let elapsed = v.start.elapsed().as_secs_f32();
            while let Some((name, at)) = v.audio.take_marker(elapsed) {
                markers.push(format!(
                    "Visit timeline: {} / {} at {:.2}s (score {:.2}s)",
                    cast::CAST[v.id].name,
                    name,
                    elapsed,
                    at
                ));
            }
        }
        for marker in markers {
            self.log(&marker);
        }
    }
    unsafe fn tick(&mut self) {
        let now = Local::now();
        if let Some(slot) = self
            .scheduler
            .poll(now.timestamp(), now.minute(), &self.saved.settings)
        {
            self.random_visit(slot, false);
        }
        if let Some((previous, current)) = self.scheduler.take_time_change() {
            if current < previous {
                self.log(&format!(
                    "Clock moved backward by {}s; upcoming visit boundaries rearmed",
                    previous - current
                ));
            } else {
                self.log(&format!(
                    "Timer gap or forward clock jump of {}s; missed visits skipped",
                    current - previous
                ));
            }
        }
        self.play_visit_cue();
        if self
            .visit
            .as_ref()
            .is_some_and(|v| v.start.elapsed().as_secs_f32() > v.audio.finish)
        {
            self.end_visit();
        }
        // Avoid painting at animation speed when motion has been disabled.
        if self.saved.settings.motion || self.visit.is_some() || now.timestamp_subsec_millis() < 70
        {
            self.render();
        }
        if !self.gallery_hwnd.is_null() && self.saved.settings.motion {
            self.invalidate_gallery_preview();
        }
    }
    unsafe fn build_menu(&self) -> HMENU {
        let menu = CreatePopupMenu();
        let sound = CreatePopupMenu();
        for (id, label, mode) in [
            (MENU_CUCKOO, "Cuckoo & character voices", Mode::Cuckoo),
            (MENU_ZEN, "Zen · meditation bells", Mode::Zen),
            (MENU_SILENT, "Silent", Mode::Silent),
        ] {
            AppendMenuW(
                sound,
                MF_STRING
                    | if self.saved.settings.mode == mode {
                        MF_CHECKED
                    } else {
                        0
                    },
                id,
                wide(label).as_ptr(),
            );
        }
        AppendMenuW(sound, MF_SEPARATOR, 0, null());
        for (id, label) in [
            (MENU_SNOOZE, "Snooze for 1 hour"),
            (MENU_SNOOZE4, "Snooze for 4 hours"),
            (MENU_WAKE, "End snooze"),
        ] {
            AppendMenuW(sound, MF_STRING, id, wide(label).as_ptr());
        }
        let visits = CreatePopupMenu();
        for (id, label) in [
            (MENU_VISIT, "Surprise me"),
            (MENU_HOUR, "An hourly bird"),
            (MENU_HALF, "A half-hour friend"),
            (MENU_GALLERY, "Meet all 24..."),
            (MENU_STOP, "Stop current visit & sound"),
        ] {
            AppendMenuW(visits, MF_STRING, id, wide(label).as_ptr());
        }
        AppendMenuW(menu, MF_POPUP, sound as usize, wide("Sound").as_ptr());
        AppendMenuW(menu, MF_POPUP, visits as usize, wide("Visit").as_ptr());
        AppendMenuW(menu, MF_STRING, MENU_SETTINGS, wide("Settings...").as_ptr());
        AppendMenuW(menu, MF_SEPARATOR, 0, null());
        for (id, label) in [
            (
                MENU_SHOW,
                format!("Bring clock forward\t{}", self.hotkey_label()),
            ),
            (MENU_BACK, "Send behind windows\tEsc".into()),
            (0, String::new()),
            (MENU_RESET, "Recover clock position".into()),
            (0, String::new()),
            (MENU_QUIT, "Quit Little Timber".into()),
        ] {
            AppendMenuW(
                menu,
                if id == 0 { MF_SEPARATOR } else { MF_STRING },
                id,
                wide(&label).as_ptr(),
            );
        }
        menu
    }
    unsafe fn command(&mut self, id: usize) {
        match id {
            MENU_SHOW => self.foreground(),
            MENU_BACK => self.behind(),
            MENU_SETTINGS => self.open_settings(),
            MENU_GALLERY => self.open_gallery(),
            MENU_HOUR => self.random_visit(Slot::Hour, true),
            MENU_HALF => self.random_visit(Slot::Half, true),
            MENU_VISIT => {
                let slot = if self.preview_half {
                    Slot::Half
                } else {
                    Slot::Hour
                };
                self.preview_half = !self.preview_half;
                self.random_visit(slot, true);
            }
            MENU_STOP => {
                self.end_visit();
                self.render();
            }
            MENU_CUCKOO | MENU_ZEN | MENU_SILENT => {
                self.saved.settings.mode = match id {
                    MENU_ZEN => Mode::Zen,
                    MENU_SILENT => Mode::Silent,
                    _ => Mode::Cuckoo,
                };
                self.stop_sound();
                self.save_or_warn();
                self.render();
            }
            MENU_SNOOZE | MENU_SNOOZE4 | MENU_WAKE => {
                self.saved.settings.snooze_until = if id == MENU_WAKE {
                    0
                } else {
                    Local::now().timestamp() + if id == MENU_SNOOZE4 { 14400 } else { 3600 }
                };
                self.stop_sound();
                self.save_or_warn();
                self.render();
            }
            MENU_RESET => {
                self.ensure_visible(true);
                self.save_or_warn();
                self.foreground();
            }
            MENU_QUIT => {
                self.stop_sound();
                self.save_or_warn();
                self.remove_tray();
                UnregisterHotKey(self.hwnd, HOTKEY);
                KillTimer(self.hwnd, 1);
                PostQuitMessage(0);
            }
            _ => {}
        }
    }
    unsafe fn startup(&self, enabled: bool) -> Result<(), String> {
        let key = wide("Software\\Microsoft\\Windows\\CurrentVersion\\Run");
        let name = wide("LittleTimber");
        let result = if enabled {
            let exe = std::env::current_exe().map_err(|e| e.to_string())?;
            let value = wide(&format!("\"{}\"", exe.display()));
            RegSetKeyValueW(
                HKEY_CURRENT_USER,
                key.as_ptr(),
                name.as_ptr(),
                REG_SZ,
                value.as_ptr() as *const _,
                (value.len() * 2) as u32,
            )
        } else {
            RegDeleteKeyValueW(HKEY_CURRENT_USER, key.as_ptr(), name.as_ptr())
        };
        if result == ERROR_SUCCESS || (!enabled && result == ERROR_FILE_NOT_FOUND) {
            Ok(())
        } else {
            Err(format!(
                "Windows could not update launch at sign-in (error {result})."
            ))
        }
    }
}
fn random() -> u64 {
    unsafe {
        let mut n = 0u64;
        let status = BCryptGenRandom(
            null_mut(),
            &mut n as *mut u64 as *mut u8,
            8,
            BCRYPT_USE_SYSTEM_PREFERRED_RNG,
        );
        if status < 0 {
            std::collections::hash_map::RandomState::new().hash_one(Instant::now())
        } else {
            n
        }
    }
}
use std::hash::BuildHasher;

unsafe extern "system" fn clock_proc(hwnd: HWND, msg: u32, w: WPARAM, l: LPARAM) -> LRESULT {
    // TrackPopupMenu pumps messages. Release the application borrow before entering
    // that loop so the clock and scheduled chimes continue while the menu is open.
    if msg == WM_CONTEXTMENU
        || msg == WM_NCRBUTTONUP
        || (msg == TRAY && matches!(l as u32, WM_RBUTTONUP | WM_CONTEXTMENU))
    {
        let menu = APP.with(|state| {
            state
                .try_borrow()
                .ok()
                .and_then(|a| a.as_ref().map(|a| a.build_menu()))
        });
        if let Some(menu) = menu {
            let mut p = POINT { x: 0, y: 0 };
            GetCursorPos(&mut p);
            SetForegroundWindow(hwnd);
            let id = TrackPopupMenu(
                menu,
                TPM_RETURNCMD | TPM_RIGHTBUTTON,
                p.x,
                p.y,
                0,
                hwnd,
                null(),
            );
            DestroyMenu(menu);
            PostMessageW(hwnd, WM_NULL, 0, 0);
            if id != 0 {
                PostMessageW(hwnd, WM_COMMAND, id as usize, 0);
            }
        }
        return 0;
    }
    APP.with(|state| {
        if let Ok(mut guard) = state.try_borrow_mut() {
            if let Some(a) = guard.as_mut() {
                if msg == a.taskbar_message {
                    a.add_tray();
                    return 0;
                }
                match msg {
                    WM_TIMER => {
                        a.tick();
                        return 0;
                    }
                    WM_POWERBROADCAST => {
                        // PBT_APMSUSPEND: cancel before the device can resume an
                        // old pre-mixed soundtrack after sleep.
                        if w == 4 {
                            a.end_visit();
                            a.log("System suspending; active visit cancelled");
                        }
                        return 1;
                    }
                    WM_HOTKEY => {
                        a.foreground();
                        return 0;
                    }
                    REVEAL => {
                        a.log(&format!("Open request {w}"));
                        match w {
                            1 => a.open_settings(),
                            2 => a.open_gallery(),
                            3 => a.command(MENU_QUIT),
                            4 | 5 => {
                                let slot = if w == 4 { Slot::Hour } else { Slot::Half };
                                a.log(&format!("Scheduled diagnostic requested: {slot:?}"));
                                a.random_visit(slot, false);
                            }
                            6 => a.behind(),
                            7 => a.command(MENU_STOP),
                            _ => a.foreground(),
                        }
                        return 0;
                    }
                    WM_KEYDOWN => {
                        match w as u16 {
                            VK_ESCAPE => a.behind(),
                            VK_SPACE => a.command(MENU_VISIT),
                            VK_F2 => a.open_settings(),
                            _ => {}
                        }
                        return 0;
                    }
                    WM_COMMAND => {
                        a.command(w & 0xffff);
                        return 0;
                    }
                    TRAY => {
                        match l as u32 {
                            WM_LBUTTONUP => a.foreground(),
                            WM_LBUTTONDBLCLK => a.open_settings(),
                            _ => {}
                        }
                        return 0;
                    }
                    WM_NCHITTEST => {
                        if a.saved.settings.locked {
                            return HTCLIENT as isize;
                        }
                        return HTCAPTION as isize;
                    }
                    WM_NCLBUTTONDBLCLK => {
                        a.open_settings();
                        return 0;
                    }
                    WM_EXITSIZEMOVE => {
                        a.ensure_visible(false);
                        a.save_or_warn();
                        return 0;
                    }
                    WM_ACTIVATE => {
                        if w & 0xffff == WA_INACTIVE as usize
                            && a.scheduled_front.is_none()
                            && a.settings_hwnd.is_null()
                            && a.gallery_hwnd.is_null()
                        {
                            a.behind();
                        }
                        return 0;
                    }
                    WM_DPICHANGED => {
                        let r = &*(l as *const RECT);
                        SetWindowPos(
                            hwnd,
                            null_mut(),
                            r.left,
                            r.top,
                            0,
                            0,
                            SWP_NOSIZE | SWP_NOZORDER | SWP_NOACTIVATE,
                        );
                        a.resize();
                        return 0;
                    }
                    WM_DISPLAYCHANGE => {
                        a.ensure_visible(false);
                        a.render();
                        return 0;
                    }
                    WM_CLOSE => {
                        a.command(MENU_QUIT);
                        return 0;
                    }
                    WM_ERASEBKGND => return 1,
                    _ => {}
                }
            }
        }
        DefWindowProcW(hwnd, msg, w, l)
    })
}
unsafe fn register_class(
    name: &str,
    proc: WNDPROC,
    brush: HBRUSH,
    icon: HICON,
) -> Result<(), String> {
    let c = WNDCLASSW {
        style: CS_DBLCLKS,
        lpfnWndProc: proc,
        hInstance: GetModuleHandleW(null()),
        hCursor: LoadCursorW(null_mut(), IDC_ARROW),
        hbrBackground: brush,
        hIcon: icon,
        ..std::mem::zeroed()
    };
    // Keep the class name allocation alive through registration.
    let name = wide(name);
    let c = WNDCLASSW {
        lpszClassName: name.as_ptr(),
        ..c
    };
    if RegisterClassW(&c) == 0 {
        Err(format!(
            "Could not register window class ({}).",
            GetLastError()
        ))
    } else {
        Ok(())
    }
}
unsafe fn run() -> Result<(), String> {
    let args: Vec<_> = std::env::args().collect();
    let request = if args.iter().any(|a| a == "--settings") {
        1
    } else if args.iter().any(|a| a == "--gallery") {
        2
    } else if args.iter().any(|a| a == "--quit") {
        3
    } else if args.iter().any(|a| a == "--test-hour") {
        4
    } else if args.iter().any(|a| a == "--test-half") {
        5
    } else if args.iter().any(|a| a == "--behind") {
        6
    } else if args.iter().any(|a| a == "--stop") {
        7
    } else {
        0
    };
    SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2);
    let mutex = CreateMutexW(
        null(),
        0,
        wide("Local\\LittleTimber.Clock.Singleton").as_ptr(),
    );
    if mutex.is_null() {
        return Err("Could not initialize the application lock.".into());
    }
    if GetLastError() == ERROR_ALREADY_EXISTS {
        let existing = FindWindowW(wide(CLASS).as_ptr(), null());
        if !existing.is_null() {
            let mut pid = 0;
            GetWindowThreadProcessId(existing, &mut pid);
            AllowSetForegroundWindow(pid);
            PostMessageW(existing, REVEAL, request, 0);
        }
        CloseHandle(mutex);
        return Ok(());
    }
    if request == 3 {
        CloseHandle(mutex);
        return Ok(());
    }
    let mut token = 0;
    let input = GdiplusStartupInput {
        GdiplusVersion: 1,
        ..std::mem::zeroed()
    };
    if GdiplusStartup(&mut token, &input, null_mut()) != 0 {
        return Err("Could not initialize Windows drawing.".into());
    }
    let controls = INITCOMMONCONTROLSEX {
        dwSize: std::mem::size_of::<INITCOMMONCONTROLSEX>() as u32,
        dwICC: ICC_STANDARD_CLASSES | ICC_BAR_CLASSES,
    };
    InitCommonControlsEx(&controls);
    let icon_data = include_bytes!("../assets/clock.ico");
    let icon = CreateIconFromResourceEx(
        icon_data[22..].as_ptr(),
        (icon_data.len() - 22) as u32,
        1,
        0x00030000,
        32,
        32,
        LR_DEFAULTCOLOR,
    );
    let icon = if icon.is_null() {
        LoadIconW(null_mut(), IDI_APPLICATION)
    } else {
        icon
    };
    register_class(CLASS, Some(clock_proc), null_mut(), icon)?;
    register_class(
        ui::SETTINGS_CLASS,
        Some(ui::settings_proc),
        (COLOR_WINDOW + 1) as HBRUSH,
        icon,
    )?;
    register_class(
        ui::GALLERY_CLASS,
        Some(ui::gallery_proc),
        (COLOR_WINDOW + 1) as HBRUSH,
        icon,
    )?;
    let dir = std::env::var_os("LOCALAPPDATA")
        .map(PathBuf::from)
        .unwrap_or(std::env::current_dir().map_err(|e| e.to_string())?)
        .join("LittleTimber");
    fs::create_dir_all(&dir).map_err(|e| format!("Could not create {}: {e}", dir.display()))?;
    let (saved, warning) = App::load(dir.clone());
    let p = saved.position.unwrap_or((100, 100));
    let hwnd = CreateWindowExW(
        WS_EX_LAYERED | WS_EX_TOOLWINDOW,
        wide(CLASS).as_ptr(),
        wide("Little Timber").as_ptr(),
        WS_POPUP,
        p.0,
        p.1,
        340,
        519,
        null_mut(),
        null_mut(),
        GetModuleHandleW(null()),
        null(),
    );
    if hwnd.is_null() {
        return Err(format!("Could not create the clock ({}).", GetLastError()));
    }
    let scale = GetDpiForWindow(hwnd).max(96) as f32 / 96.;
    let font = CreateFontW(
        -(14. * scale) as i32,
        0,
        0,
        0,
        400,
        0,
        0,
        0,
        DEFAULT_CHARSET as u32,
        0,
        0,
        CLEARTYPE_QUALITY as u32,
        0,
        wide("Segoe UI").as_ptr(),
    );
    let heading_font = CreateFontW(
        -(22. * scale) as i32,
        0,
        0,
        0,
        600,
        0,
        0,
        0,
        DEFAULT_CHARSET as u32,
        0,
        0,
        CLEARTYPE_QUALITY as u32,
        0,
        wide("Segoe UI").as_ptr(),
    );
    let mut app = App {
        artwork: artwork::Artwork::load()?,
        hwnd,
        settings_hwnd: null_mut(),
        gallery_hwnd: null_mut(),
        saved,
        dir,
        canvas: None,
        scheduler: Scheduler::new(Local::now().timestamp()),
        start: Instant::now(),
        visit: None,
        sound: vec![],
        foreground: false,
        scheduled_front: None,
        gallery_id: 0,
        preview_half: false,
        hotkey_ok: false,
        taskbar_message: RegisterWindowMessageW(wide("TaskbarCreated").as_ptr()),
        font,
        heading_font,
        ui_scale: scale,
        icon,
        last_status: String::new(),
    };
    app.resize();
    if app.saved.position.is_none() || !app.dir.join("settings.json").exists() {
        app.ensure_visible(true);
    }
    app.add_tray();
    if !app.register_hotkey() {
        app.error(&format!("{} is already used by another app. Choose another letter in Settings. The tray icon still opens the clock.",app.hotkey_label()));
    }
    if let Some(w) = warning {
        app.error(&w);
    }
    app.log("Started Little Timber 0.1.0");
    app.render();
    ShowWindow(hwnd, SW_SHOWNOACTIVATE);
    app.behind();
    APP.with(|state| *state.borrow_mut() = Some(app));
    if SetTimer(hwnd, 1, 50, None) == 0 {
        return Err("Windows could not start the clock timer.".into());
    }
    if request != 0 {
        PostMessageW(hwnd, REVEAL, request, 0);
    }
    let mut msg: MSG = std::mem::zeroed();
    loop {
        let result = GetMessageW(&mut msg, null_mut(), 0, 0);
        if result <= 0 {
            break;
        }
        let (settings, gallery) = APP.with(|state| {
            state
                .borrow()
                .as_ref()
                .map_or((null_mut(), null_mut()), |a| {
                    (a.settings_hwnd, a.gallery_hwnd)
                })
        });
        if (!settings.is_null()
            && (msg.hwnd == settings || IsChild(settings, msg.hwnd) != 0)
            && IsDialogMessageW(settings, &msg) != 0)
            || (!gallery.is_null()
                && (msg.hwnd == gallery || IsChild(gallery, msg.hwnd) != 0)
                && IsDialogMessageW(gallery, &msg) != 0)
        {
            continue;
        }
        TranslateMessage(&msg);
        DispatchMessageW(&msg);
    }
    APP.with(|state| {
        if let Some(mut a) = state.borrow_mut().take() {
            a.stop_sound();
            a.remove_tray();
            DestroyWindow(a.settings_hwnd);
            DestroyWindow(a.gallery_hwnd);
            DestroyWindow(a.hwnd);
            DeleteObject(a.font);
            DeleteObject(a.heading_font);
        }
    });
    GdiplusShutdown(token);
    CloseHandle(mutex);
    Ok(())
}
fn main() {
    unsafe {
        if let Err(e) = run() {
            MessageBoxW(
                null_mut(),
                wide(&e).as_ptr(),
                wide("Little Timber could not start").as_ptr(),
                MB_OK | MB_ICONERROR,
            );
        }
    }
}
