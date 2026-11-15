//! Real Win32 controls: keyboard-operable settings and a visitor preview gallery.
#![allow(clippy::too_many_arguments)] // Mirrors Win32 control rectangles and styles.
use super::*;
use std::result::Result::Ok;
// Win32 SDK constants omitted from the windows-sys generated control module.
const TBM_GETPOS: u32 = WM_USER;
const SS_CENTER: u32 = 1;
pub const SETTINGS_CLASS: &str = "LittleTimber.Settings";
pub const GALLERY_CLASS: &str = "LittleTimber.Gallery";
const MODE: i32 = 201;
const VOICE: i32 = 202;
const BELL: i32 = 203;
const VOLUME: i32 = 204;
const HOUR: i32 = 205;
const HALF: i32 = 206;
const COUNT: i32 = 207;
const QUIET: i32 = 208;
const START: i32 = 209;
const END: i32 = 210;
const QUIET_MOTION: i32 = 211;
const MOTION: i32 = 212;
const SECONDS: i32 = 213;
const DIGITAL: i32 = 214;
const CLOCK24: i32 = 215;
const LOCK: i32 = 216;
const SIZE: i32 = 217;
const KEY: i32 = 218;
const SHIFT: i32 = 219;
const STARTUP: i32 = 220;
const APPLY: i32 = 221;
const DONE: i32 = 222;
const PREVIEW: i32 = 223;
const GALLERY: i32 = 224;
const CANCEL: i32 = 225;
const LIST: i32 = 301;
const PLAY: i32 = 302;
const NEXT: i32 = 303;
const STOP: i32 = 304;
const GALLERY_CAPTION: i32 = 305;

impl App {
    unsafe fn control(
        &self,
        parent: HWND,
        class: &str,
        label: &str,
        id: i32,
        style: u32,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
    ) -> HWND {
        let s = self.ui_scale;
        let control = CreateWindowExW(
            if class == "EDIT" || class == "LISTBOX" {
                WS_EX_CLIENTEDGE
            } else {
                0
            },
            wide(class).as_ptr(),
            wide(label).as_ptr(),
            WS_CHILD | WS_VISIBLE | style,
            (x as f32 * s) as i32,
            (y as f32 * s) as i32,
            (w as f32 * s) as i32,
            (h as f32 * s) as i32,
            parent,
            id as usize as HMENU,
            GetModuleHandleW(null()),
            null(),
        );
        SendMessageW(control, WM_SETFONT, self.font as usize, 1);
        control
    }
    unsafe fn label(&self, parent: HWND, label: &str, x: i32, y: i32, w: i32) {
        self.control(parent, "STATIC", label, 0, 0, x, y, w, 22);
    }
    unsafe fn check(
        &self,
        parent: HWND,
        label: &str,
        id: i32,
        checked: bool,
        x: i32,
        y: i32,
        w: i32,
    ) {
        let h = self.control(
            parent,
            "BUTTON",
            label,
            id,
            BS_AUTOCHECKBOX as u32 | WS_TABSTOP,
            x,
            y,
            w,
            23,
        );
        SendMessageW(
            h,
            BM_SETCHECK,
            if checked {
                BST_CHECKED as usize
            } else {
                BST_UNCHECKED as usize
            },
            0,
        );
    }
    unsafe fn combo(
        &self,
        parent: HWND,
        id: i32,
        items: &[&str],
        value: usize,
        x: i32,
        y: i32,
        w: i32,
    ) {
        let h = self.control(
            parent,
            "COMBOBOX",
            "",
            id,
            CBS_DROPDOWNLIST as u32 | WS_TABSTOP | WS_VSCROLL,
            x,
            y,
            w,
            220,
        );
        for item in items {
            SendMessageW(h, CB_ADDSTRING, 0, wide(item).as_ptr() as isize);
        }
        SendMessageW(h, CB_SETCURSEL, value, 0);
    }
    unsafe fn button(&self, parent: HWND, label: &str, id: i32, x: i32, y: i32, w: i32) {
        self.control(
            parent,
            "BUTTON",
            label,
            id,
            WS_TABSTOP | BS_PUSHBUTTON as u32,
            x,
            y,
            w,
            30,
        );
    }
    unsafe fn panel(&self, class: &str, title: &str, width: i32, height: i32) -> HWND {
        let s = self.ui_scale;
        let mut rect = RECT {
            left: 0,
            top: 0,
            right: (width as f32 * s) as i32,
            bottom: (height as f32 * s) as i32,
        };
        let style = WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_MINIMIZEBOX;
        AdjustWindowRectEx(&mut rect, style, 0, WS_EX_CONTROLPARENT);
        CreateWindowExW(
            WS_EX_CONTROLPARENT,
            wide(class).as_ptr(),
            wide(title).as_ptr(),
            style,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            rect.right - rect.left,
            rect.bottom - rect.top,
            null_mut(),
            null_mut(),
            GetModuleHandleW(null()),
            null(),
        )
    }
    pub(super) unsafe fn open_settings(&mut self) {
        if !self.settings_hwnd.is_null() {
            ShowWindow(self.settings_hwnd, SW_RESTORE);
            SetForegroundWindow(self.settings_hwnd);
            return;
        }
        let h = self.panel(SETTINGS_CLASS, "Little Timber — Settings", 650, 650);
        if h.is_null() {
            self.error("Could not open Settings.");
            return;
        }
        self.settings_hwnd = h;
        let s = &self.saved.settings;
        let heading = self.control(h, "STATIC", "Make yourself at home", 0, 0, 24, 18, 600, 33);
        SendMessageW(heading, WM_SETFONT, self.heading_font as usize, 1);
        self.label(
            h,
            "A little clock, a little company. Changes take effect with Apply.",
            24,
            56,
            610,
        );
        self.label(h, "SOUND && VISITS", 24, 94, 270);
        self.label(h, "Mode", 24, 125, 90);
        self.combo(
            h,
            MODE,
            &[
                "Cuckoo & character voices",
                "Zen · meditation bells",
                "Silent · visual visits only",
            ],
            match s.mode {
                Mode::Cuckoo => 0,
                Mode::Zen => 1,
                Mode::Silent => 2,
            },
            126,
            121,
            194,
        );
        self.label(h, "Cuckoo voice", 24, 161, 100);
        self.combo(
            h,
            VOICE,
            &[
                "Each character's own voice",
                "Classic cuckoo",
                "Soft wooden cuckoo",
                "Forest whistle",
            ],
            s.voice,
            126,
            157,
            194,
        );
        self.label(h, "Bell sound", 24, 197, 100);
        self.combo(
            h,
            BELL,
            &[
                "Warm singing bowl",
                "Bright singing bowl",
                "Temple bell",
                "Variety · all three",
            ],
            s.bell,
            126,
            193,
            194,
        );
        self.label(h, "Volume", 24, 235, 100);
        let v = self.control(
            h,
            "msctls_trackbar32",
            "Volume",
            VOLUME,
            TBS_AUTOTICKS | WS_TABSTOP,
            120,
            226,
            204,
            35,
        );
        SendMessageW(v, TBM_SETRANGE, 1, (100 << 16) as isize);
        SendMessageW(v, TBM_SETPOS, 1, s.volume as isize);
        self.check(
            h,
            "On the hour · 12 bird visitors",
            HOUR,
            s.hourly,
            24,
            272,
            296,
        );
        self.check(
            h,
            "On the half-hour · 12 woodland visitors",
            HALF,
            s.half_hourly,
            24,
            300,
            306,
        );
        self.check(
            h,
            "Chime the hour count after the visit",
            COUNT,
            s.count_hour,
            24,
            328,
            296,
        );
        self.button(h, "Preview sequence", PREVIEW, 24, 366, 140);
        self.button(h, "Meet all 24...", GALLERY, 174, 366, 146);
        self.label(h, "Preview bypasses quiet hours and snooze.", 24, 405, 305);
        self.label(h, "QUIET TIME", 24, 446, 285);
        self.check(h, "Enable quiet hours", QUIET, s.quiet, 24, 475, 290);
        self.label(h, "From", 24, 509, 42);
        self.control(
            h,
            "EDIT",
            &format_time(s.quiet_start),
            START,
            ES_AUTOHSCROLL as u32 | WS_TABSTOP,
            70,
            505,
            73,
            26,
        );
        self.label(h, "to", 151, 509, 25);
        self.control(
            h,
            "EDIT",
            &format_time(s.quiet_end),
            END,
            ES_AUTOHSCROLL as u32 | WS_TABSTOP,
            178,
            505,
            73,
            26,
        );
        self.label(h, "24h", 261, 509, 52);
        self.check(
            h,
            "Also pause visitors during quiet hours",
            QUIET_MOTION,
            s.quiet_motion,
            24,
            542,
            305,
        );
        self.label(h, "Same start/end means quiet all day.", 24, 572, 300);
        self.label(h, "DESKTOP && APPEARANCE", 350, 94, 280);
        self.label(h, "Clock size", 350, 125, 100);
        self.combo(
            h,
            SIZE,
            &["Small", "Medium", "Large", "Extra large"],
            s.size,
            459,
            121,
            166,
        );
        self.check(
            h,
            "Gentle animation && pendulum",
            MOTION,
            s.motion,
            350,
            161,
            280,
        );
        self.check(h, "Show second hand", SECONDS, s.seconds, 350, 192, 280);
        self.check(
            h,
            "Show digital time on dial",
            DIGITAL,
            s.digital,
            350,
            223,
            280,
        );
        self.check(
            h,
            "Use 24-hour digital time",
            CLOCK24,
            s.clock_24h,
            350,
            254,
            280,
        );
        self.check(h, "Lock clock position", LOCK, s.locked, 350, 285, 280);
        self.label(h, "BRING FORWARD", 350, 335, 270);
        self.label(h, "Ctrl + Alt +", 350, 369, 100);
        self.control(
            h,
            "EDIT",
            &(s.hotkey_letter as char).to_string(),
            KEY,
            ES_UPPERCASE as u32 | WS_TABSTOP,
            457,
            365,
            45,
            26,
        );
        SendMessageW(GetDlgItem(h, KEY), EM_SETLIMITTEXT, 1, 0);
        self.check(h, "+ Shift", SHIFT, s.hotkey_shift, 514, 366, 110);
        self.label(h, "Escape sends the clock behind windows.", 350, 405, 280);
        self.check(
            h,
            "Launch when I sign in to Windows",
            STARTUP,
            s.startup,
            350,
            451,
            280,
        );
        self.label(
            h,
            "Drag the house to move it. Right-click for",
            350,
            494,
            280,
        );
        self.label(h, "snooze, visitor previews, and Quit.", 350, 516, 280);
        self.label(h, "Space: preview visit    F2: settings", 350, 552, 280);
        self.button(h, "Cancel", CANCEL, 350, 608, 85);
        self.button(h, "Apply", APPLY, 446, 608, 85);
        self.button(h, "Save", DONE, 542, 608, 84);
        ShowWindow(h, SW_SHOW);
        SetWindowPos(
            h,
            HWND_TOP,
            0,
            0,
            0,
            0,
            SWP_NOMOVE | SWP_NOSIZE | SWP_SHOWWINDOW,
        );
        SetForegroundWindow(h);
        SetFocus(GetDlgItem(h, MODE));
    }
    unsafe fn read_settings(&self) -> Result<Settings, String> {
        let h = self.settings_hwnd;
        let mut s = self.saved.settings.clone();
        let combo = |id| SendMessageW(GetDlgItem(h, id), CB_GETCURSEL, 0, 0).max(0) as usize;
        let check = |id| SendMessageW(GetDlgItem(h, id), BM_GETCHECK, 0, 0) == BST_CHECKED as isize;
        let value = |id| {
            let mut buf = [0u16; 128];
            let n = GetWindowTextW(GetDlgItem(h, id), buf.as_mut_ptr(), 128);
            String::from_utf16_lossy(&buf[..n.max(0) as usize])
        };
        s.mode = match combo(MODE) {
            1 => Mode::Zen,
            2 => Mode::Silent,
            _ => Mode::Cuckoo,
        };
        s.voice = combo(VOICE);
        s.bell = combo(BELL);
        s.size = combo(SIZE);
        s.volume = SendMessageW(GetDlgItem(h, VOLUME), TBM_GETPOS, 0, 0) as u8;
        s.hourly = check(HOUR);
        s.half_hourly = check(HALF);
        s.count_hour = check(COUNT);
        s.quiet = check(QUIET);
        s.quiet_motion = check(QUIET_MOTION);
        s.motion = check(MOTION);
        s.seconds = check(SECONDS);
        s.digital = check(DIGITAL);
        s.clock_24h = check(CLOCK24);
        s.locked = check(LOCK);
        s.startup = check(STARTUP);
        s.hotkey_shift = check(SHIFT);
        s.quiet_start =
            parse_time(&value(START)).ok_or("Quiet time 'From' must be a time such as 22:00.")?;
        s.quiet_end =
            parse_time(&value(END)).ok_or("Quiet time 'to' must be a time such as 08:00.")?;
        let key = value(KEY).to_ascii_uppercase();
        if key.len() != 1 || !key.as_bytes()[0].is_ascii_uppercase() {
            return Err("Choose one letter A–Z for the hotkey.".into());
        }
        s.hotkey_letter = key.as_bytes()[0];
        Ok(s)
    }
    unsafe fn apply_settings(&mut self) -> bool {
        let proposed = match self.read_settings() {
            Ok(s) => s,
            Err(e) => {
                self.error(&e);
                return false;
            }
        };
        let old = self.saved.settings.clone();
        let changed_key = proposed.hotkey_letter != old.hotkey_letter
            || proposed.hotkey_shift != old.hotkey_shift
            || !self.hotkey_ok;
        self.saved.settings = proposed;
        if changed_key && !self.register_hotkey() {
            self.saved.settings = old;
            self.register_hotkey();
            self.error("That shortcut is already used by another application. Choose another letter or add Shift. No settings were changed.");
            return false;
        }
        if self.saved.settings.startup != old.startup {
            if let Err(e) = self.startup(self.saved.settings.startup) {
                self.saved.settings = old;
                if changed_key {
                    self.register_hotkey();
                }
                self.error(&e);
                return false;
            }
        }
        if let Err(e) = self.save() {
            self.error(&format!("Settings could not be saved: {e}"));
            return false;
        }
        self.stop_sound();
        self.remove_tray();
        self.add_tray();
        self.resize();
        self.log("Settings applied");
        true
    }
    unsafe fn close_settings(&mut self) {
        let h = self.settings_hwnd;
        self.settings_hwnd = null_mut();
        DestroyWindow(h);
        self.behind();
    }
    pub(super) unsafe fn open_gallery(&mut self) {
        if !self.gallery_hwnd.is_null() {
            ShowWindow(self.gallery_hwnd, SW_RESTORE);
            SetForegroundWindow(self.gallery_hwnd);
            return;
        }
        let h = self.panel(GALLERY_CLASS, "Little Timber — Meet the visitors", 650, 530);
        if h.is_null() {
            self.error("Could not open the visitor gallery.");
            return;
        }
        self.gallery_hwnd = h;
        let heading = self.control(
            h,
            "STATIC",
            "Small friends. Big personalities.",
            0,
            0,
            24,
            20,
            600,
            36,
        );
        SendMessageW(heading, WM_SETFONT, self.heading_font as usize, 1);
        self.label(
            h,
            "Two shuffled casts. Everyone gets a turn before anyone repeats.",
            24,
            61,
            610,
        );
        let list = self.control(
            h,
            "LISTBOX",
            "Visitors",
            LIST,
            LBS_NOTIFY as u32 | WS_VSCROLL | WS_TABSTOP,
            24,
            103,
            205,
            366,
        );
        for (id, c) in cast::CAST.iter().enumerate() {
            SendMessageW(
                list,
                LB_ADDSTRING,
                0,
                wide(&format!(
                    "{}  {} · {}",
                    if id < 12 { "H" } else { "½" },
                    c.name,
                    c.kind
                ))
                .as_ptr() as isize,
            );
        }
        SendMessageW(list, LB_SETCURSEL, self.gallery_id, 0);
        self.control(
            h,
            "STATIC",
            "",
            GALLERY_CAPTION,
            SS_CENTER,
            251,
            372,
            375,
            70,
        );
        self.update_gallery_caption();
        self.label(h, "H = on the hour     ½ = on the half-hour", 24, 484, 320);
        self.button(h, "Hear && visit", PLAY, 272, 456, 111);
        self.button(h, "Next friend", NEXT, 391, 456, 111);
        self.button(h, "Stop", STOP, 510, 456, 90);
        self.label(
            h,
            "Preview uses the saved sound mode and volume.",
            267,
            497,
            360,
        );
        ShowWindow(h, SW_SHOW);
        SetWindowPos(
            h,
            HWND_TOP,
            0,
            0,
            0,
            0,
            SWP_NOMOVE | SWP_NOSIZE | SWP_SHOWWINDOW,
        );
        SetForegroundWindow(h);
        SetFocus(list);
    }
    unsafe fn update_gallery_caption(&self) {
        let c = cast::CAST[self.gallery_id];
        let text = format!(
            "{} the {}\r\n{} · {}",
            c.name,
            c.kind,
            if self.gallery_id < 12 {
                "On the hour"
            } else {
                "On the half-hour"
            },
            c.action
        );
        SetWindowTextW(
            GetDlgItem(self.gallery_hwnd, GALLERY_CAPTION),
            wide(&text).as_ptr(),
        );
        self.invalidate_gallery_preview();
    }
    pub(super) unsafe fn invalidate_gallery_preview(&self) {
        // The native list, caption and buttons are not animation surfaces.
        // Invalidating their parent over them repeatedly erases list rows.
        let s = self.ui_scale;
        let rect = RECT {
            left: (251. * s) as i32,
            top: (103. * s) as i32,
            right: (626. * s) as i32,
            bottom: (363. * s) as i32,
        };
        InvalidateRect(self.gallery_hwnd, &rect, 0);
    }
    unsafe fn paint_gallery(&self, hwnd: HWND) {
        let mut ps: PAINTSTRUCT = std::mem::zeroed();
        let dc = BeginPaint(hwnd, &mut ps);
        let s = self.ui_scale;
        // Only paint the illustration panel; child controls own their own surfaces.
        if let Ok(d) = draw::Surface::new((375. * s) as i32, (260. * s) as i32) {
            d.clear(0xfff5eddf);
            d.transform(s, 0., 0., 0.);
            d.ellipse(62., 207., 250., 22., 0xe6d8c1);
            d.image_region(
                &self.artwork.house,
                (62., 190., 250., 23.),
                (116., 880., 568., 46.),
            );
            let t = self.start.elapsed().as_secs_f32();
            d.transform(1.25 * s, 187. * s, 190. * s, 0.);
            let frame = if self.saved.settings.motion {
                performance_frame(t)
            } else {
                0
            };
            draw::character(&d, self.gallery_id, frame, &self.artwork);
            GdipFlush(d.g, FlushIntentionSync);
            BitBlt(
                dc,
                (251. * s) as i32,
                (103. * s) as i32,
                d.width,
                d.height,
                d.dc,
                0,
                0,
                SRCCOPY,
            );
        }
        EndPaint(hwnd, &ps);
    }
}

pub unsafe extern "system" fn settings_proc(hwnd: HWND, msg: u32, w: WPARAM, l: LPARAM) -> LRESULT {
    if msg == WM_CTLCOLORSTATIC || msg == WM_CTLCOLORBTN {
        SetBkMode(w as HDC, TRANSPARENT as i32);
        SetTextColor(w as HDC, 0x004c4b34);
        return GetSysColorBrush(COLOR_WINDOW) as isize;
    }
    APP.with(|state| {
        if let Ok(mut guard) = state.try_borrow_mut() {
            if let Some(a) = guard.as_mut() {
                match msg {
                    WM_COMMAND => {
                        let id = (w & 0xffff) as i32;
                        match id {
                            APPLY => {
                                a.apply_settings();
                            }
                            DONE => {
                                if a.apply_settings() {
                                    a.close_settings();
                                }
                            }
                            CANCEL | 2 => a.close_settings(),
                            PREVIEW => match a.read_settings() {
                                Ok(s) => a.visit(a.gallery_id, true, None, Some(s)),
                                Err(e) => a.error(&e),
                            },
                            GALLERY => a.open_gallery(),
                            _ => {}
                        }
                        return 0;
                    }
                    WM_CLOSE => {
                        a.close_settings();
                        return 0;
                    }
                    _ => {}
                }
            }
        }
        DefWindowProcW(hwnd, msg, w, l)
    })
}
pub unsafe extern "system" fn gallery_proc(hwnd: HWND, msg: u32, w: WPARAM, l: LPARAM) -> LRESULT {
    if msg == WM_CTLCOLORSTATIC || msg == WM_CTLCOLORBTN {
        SetBkMode(w as HDC, TRANSPARENT as i32);
        SetTextColor(w as HDC, 0x004c4b34);
        return GetSysColorBrush(COLOR_WINDOW) as isize;
    }
    APP.with(|state| {
        if let Ok(mut guard) = state.try_borrow_mut() {
            if let Some(a) = guard.as_mut() {
                match msg {
                    WM_PAINT => {
                        a.paint_gallery(hwnd);
                        return 0;
                    }
                    WM_COMMAND => {
                        let id = (w & 0xffff) as i32;
                        match id {
                            LIST if (w >> 16) as u32 == LBN_SELCHANGE => {
                                let sel = SendMessageW(GetDlgItem(hwnd, LIST), LB_GETCURSEL, 0, 0);
                                if (0..24).contains(&sel) {
                                    a.gallery_id = sel as usize;
                                    a.update_gallery_caption();
                                }
                            }
                            PLAY => {
                                a.visit(a.gallery_id, true, None, None);
                            }
                            NEXT => {
                                a.gallery_id = (a.gallery_id + 1) % 24;
                                SendMessageW(GetDlgItem(hwnd, LIST), LB_SETCURSEL, a.gallery_id, 0);
                                a.update_gallery_caption();
                            }
                            STOP => {
                                a.end_visit();
                                a.render();
                            }
                            2 => {
                                a.gallery_hwnd = null_mut();
                                DestroyWindow(hwnd);
                                a.behind();
                            }
                            _ => {}
                        }
                        return 0;
                    }
                    WM_CLOSE => {
                        a.gallery_hwnd = null_mut();
                        DestroyWindow(hwnd);
                        a.behind();
                        return 0;
                    }
                    _ => {}
                }
            }
        }
        DefWindowProcW(hwnd, msg, w, l)
    })
}
