use serde::{Deserialize, Serialize};
pub const OPENING_BELL_SECONDS: f32 = 1.6;
pub const DOOR_SECONDS: f32 = 1.1;
pub const ENTER_SECONDS: f32 = 1.6;
pub const TURN_SECONDS: f32 = 1.2;
pub const RETREAT_SECONDS: f32 = 1.6;
pub const PERFORMANCE_START: f32 = OPENING_BELL_SECONDS + DOOR_SECONDS + ENTER_SECONDS;
pub const STEP_RADIANS_PER_SECOND: f32 = 13.;
pub const PERFORMANCE_FRAMES: usize = 12;
pub const PERFORMANCE_FPS: f32 = 8.;

/// Shared by the renderer and sound score; all times are relative to playback.
pub struct VisitTiming {
    pub turn: f32,
    pub retreat: f32,
    pub close: f32,
    pub closed: f32,
}
impl VisitTiming {
    pub fn new(duration: f32) -> Self {
        let close = duration - DOOR_SECONDS;
        let retreat = close - RETREAT_SECONDS;
        Self {
            turn: retreat - TURN_SECONDS,
            retreat,
            close,
            closed: duration,
        }
    }
}

pub fn visit_duration(sound_seconds: f32) -> f32 {
    PERFORMANCE_START + sound_seconds.max(4.2) + 0.3 + TURN_SECONDS + RETREAT_SECONDS + DOOR_SECONDS
}

/// Solid travel through the opening: turn in place, then walk away facing
/// backwards. The case occludes the visitor at either end of the journey.
pub struct VisitPose {
    pub doors: f32,
    pub depth: f32,
    pub lateral: f32,
    pub visible: bool,
    pub frame: usize,
}

pub fn visit_pose(elapsed: f32, duration: f32, motion: bool) -> VisitPose {
    let VisitTiming {
        close,
        retreat,
        turn,
        ..
    } = VisitTiming::new(duration);
    let ease = |x: f32| {
        let x = x.clamp(0., 1.);
        x * x * (3. - 2. * x)
    };
    let doors = ease((elapsed - OPENING_BELL_SECONDS) / DOOR_SECONDS)
        .min(ease((duration - elapsed) / DOOR_SECONDS));
    let depth = ease((elapsed - OPENING_BELL_SECONDS - DOOR_SECONDS) / ENTER_SECONDS)
        .min(ease((close - elapsed) / RETREAT_SECONDS));
    let frame = if elapsed < PERFORMANCE_START {
        12 + ((1. - depth) * 6.).round() as usize
    } else if elapsed < turn {
        performance_frame(elapsed - PERFORMANCE_START)
    } else {
        12 + (ease((elapsed - turn) / TURN_SECONDS) * 12.).round() as usize
    };
    if !motion {
        return VisitPose {
            doors: if elapsed >= OPENING_BELL_SECONDS + DOOR_SECONDS && elapsed < close {
                1.
            } else {
                0.
            },
            depth: 1.,
            lateral: 0.,
            visible: elapsed >= PERFORMANCE_START && elapsed < retreat,
            frame: if elapsed >= turn { 24 } else { 0 },
        };
    }
    VisitPose {
        doors,
        depth,
        lateral: if elapsed < PERFORMANCE_START {
            depth - 1.
        } else {
            1. - depth
        },
        visible: elapsed >= OPENING_BELL_SECONDS + DOOR_SECONDS && elapsed < close,
        frame,
    }
}

pub fn closing_chime_count(hour: u32, hourly: bool, enabled: bool) -> usize {
    if hourly && enabled {
        ((hour + 11) % 12 + 1) as usize
    } else {
        1
    }
}

pub fn performance_frame(seconds: f32) -> usize {
    (seconds.max(0.) * PERFORMANCE_FPS) as usize % PERFORMANCE_FRAMES
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Mode {
    #[default]
    Cuckoo,
    Zen,
    Silent,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub mode: Mode,
    pub voice: usize,
    pub bell: usize,
    pub volume: u8,
    pub hourly: bool,
    pub half_hourly: bool,
    pub count_hour: bool,
    pub quiet: bool,
    pub quiet_start: u16,
    pub quiet_end: u16,
    pub quiet_motion: bool,
    pub motion: bool,
    pub seconds: bool,
    pub digital: bool,
    pub clock_24h: bool,
    pub locked: bool,
    pub size: usize,
    pub hotkey_letter: u8,
    pub hotkey_shift: bool,
    pub startup: bool,
    pub snooze_until: i64,
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            mode: Mode::Cuckoo,
            voice: 0,
            bell: 0,
            volume: 35,
            hourly: true,
            half_hourly: true,
            count_hour: true,
            quiet: true,
            quiet_start: 22 * 60,
            quiet_end: 8 * 60,
            quiet_motion: false,
            motion: true,
            seconds: false,
            digital: false,
            clock_24h: false,
            locked: false,
            size: 1,
            hotkey_letter: b'C',
            hotkey_shift: false,
            startup: false,
            snooze_until: 0,
        }
    }
}
impl Settings {
    pub fn normalize(&mut self) {
        self.voice = self.voice.min(3);
        self.bell = self.bell.min(3);
        self.volume = self.volume.min(100);
        self.size = self.size.min(3);
        self.quiet_start %= 1440;
        self.quiet_end %= 1440;
        if !self.hotkey_letter.is_ascii_uppercase() {
            self.hotkey_letter = b'C';
        }
    }
    pub fn is_quiet(&self, minute: u16) -> bool {
        self.quiet && in_time_range(minute, self.quiet_start, self.quiet_end)
    }
    pub fn sound_allowed(&self, minute: u16, now: i64) -> bool {
        self.mode != Mode::Silent
            && self.volume > 0
            && !self.is_quiet(minute)
            && now >= self.snooze_until
    }
}
pub fn in_time_range(minute: u16, start: u16, end: u16) -> bool {
    if start == end {
        true
    } else if start < end {
        minute >= start && minute < end
    } else {
        minute >= start || minute < end
    }
}
pub fn parse_time(s: &str) -> Option<u16> {
    let (h, m) = s.trim().split_once(':')?;
    let (h, m) = (h.parse::<u16>().ok()?, m.parse::<u16>().ok()?);
    (h < 24 && m < 60).then_some(h * 60 + m)
}
pub fn format_time(t: u16) -> String {
    format!("{:02}:{:02}", t / 60, t % 60)
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Bag {
    pub remaining: Vec<usize>,
    pub last: Option<usize>,
}
impl Bag {
    pub fn validate(&mut self) {
        let mut seen = [false; 12];
        if self
            .remaining
            .iter()
            .any(|&v| v >= 12 || std::mem::replace(&mut seen[v], true))
        {
            self.remaining.clear();
        }
        if self.last.is_some_and(|v| v >= 12) {
            self.last = None;
        }
    }
    pub fn draw(&mut self, mut random: impl FnMut() -> u64) -> usize {
        if self.remaining.is_empty() {
            self.remaining = (0..12).collect();
            for i in (1..12).rev() {
                self.remaining.swap(i, (random() % (i as u64 + 1)) as usize);
            }
            if self.last == self.remaining.last().copied() {
                self.remaining.swap(0, 11);
            }
        }
        let result = self.remaining.pop().unwrap();
        self.last = Some(result);
        result
    }
}

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Saved {
    pub settings: Settings,
    pub hour_bag: Bag,
    pub half_bag: Bag,
    pub position: Option<(i32, i32)>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slot {
    Hour,
    Half,
}
pub struct Scheduler {
    last: i64,
    fired: std::collections::VecDeque<i64>,
    time_change: Option<(i64, i64)>,
}
impl Scheduler {
    pub fn new(now: i64) -> Self {
        Self {
            last: now,
            fired: Default::default(),
            time_change: None,
        }
    }
    pub fn poll(&mut self, now: i64, local_minute: u32, s: &Settings) -> Option<Slot> {
        let previous = self.last;
        self.last = now;
        if now < previous {
            // A boundary that is in the future again must be allowed to fire
            // when the clock reaches it. Keep this minute deduplicated.
            self.fired.retain(|&minute| minute <= now.div_euclid(60));
            self.time_change = Some((previous, now));
            return None;
        }
        if now - previous > 5 {
            self.time_change = Some((previous, now));
            return None;
        }
        // Only a live boundary crossing. Launch, resume, and time jumps never catch up.
        if now == previous || now.div_euclid(60) == previous.div_euclid(60) {
            return None;
        }
        let key = now.div_euclid(60);
        if self.fired.contains(&key) {
            return None;
        }
        let slot = match local_minute {
            0 if s.hourly => Slot::Hour,
            30 if s.half_hourly => Slot::Half,
            _ => return None,
        };
        self.fired.push_back(key);
        if self.fired.len() > 64 {
            self.fired.pop_front();
        }
        Some(slot)
    }
    pub fn take_time_change(&mut self) -> Option<(i64, i64)> {
        self.time_change.take()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn time_chimes_count_twelve_at_midnight_and_noon_and_one_at_half_hour() {
        assert_eq!(closing_chime_count(0, true, true), 12);
        assert_eq!(closing_chime_count(12, true, true), 12);
        assert_eq!(closing_chime_count(19, true, true), 7);
        assert_eq!(closing_chime_count(19, false, true), 1);
        assert_eq!(closing_chime_count(19, true, false), 1);
    }
    #[test]
    fn visitors_travel_only_through_fully_open_doors() {
        for sound in [1.6, 6., 21.95] {
            let duration = visit_duration(sound);
            let close = duration - DOOR_SECONDS;
            let retreat = close - RETREAT_SECONDS;
            let turn = retreat - TURN_SECONDS;
            assert!(PERFORMANCE_START + sound < turn);
            for step in 0..=(duration * 100.) as usize {
                let t = step as f32 / 100.;
                let p = visit_pose(t, duration, true);
                assert!(p.frame < 25);
                if p.visible {
                    assert_eq!(p.doors, 1.);
                }
                if (turn..retreat).contains(&t) {
                    assert_eq!(p.depth, 1.);
                }
                if t >= retreat && t < close {
                    assert_eq!(p.frame, 24);
                }
                if t >= close {
                    assert!(!p.visible);
                }
            }
            let entering = visit_pose(
                OPENING_BELL_SECONDS + DOOR_SECONDS + ENTER_SECONDS / 2.,
                duration,
                true,
            );
            let leaving = visit_pose(retreat + RETREAT_SECONDS / 2., duration, true);
            assert!((entering.depth - 0.5).abs() < 0.001);
            assert!((leaving.depth - 0.5).abs() < 0.001);
            assert!((entering.lateral + 0.5).abs() < 0.001);
            assert!((leaving.lateral - 0.5).abs() < 0.001);
            assert_eq!(leaving.frame, 24);
            assert!(!visit_pose(0., duration, true).visible);
            assert_eq!(visit_pose(duration, duration, true).doors, 0.);
            assert_eq!(
                visit_pose(PERFORMANCE_START + 0.1, duration, false).frame,
                0
            );
        }
    }
    #[test]
    fn quiet_hours_wrap_and_boundaries() {
        assert!(in_time_range(23 * 60, 22 * 60, 8 * 60));
        assert!(in_time_range(0, 22 * 60, 8 * 60));
        assert!(!in_time_range(8 * 60, 22 * 60, 8 * 60));
        assert!(!in_time_range(12 * 60, 22 * 60, 8 * 60));
        assert!(in_time_range(600, 600, 601));
        assert!(!in_time_range(601, 600, 601));
        assert!(in_time_range(900, 0, 0));
    }
    #[test]
    fn shuffled_cycles_are_complete_without_boundary_repeat() {
        let mut b = Bag::default();
        let mut seed = 54321u64;
        let mut prev = None;
        for _ in 0..40 {
            let mut seen = [false; 12];
            for i in 0..12 {
                let x = b.draw(|| {
                    seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
                    seed
                });
                if i == 0 {
                    assert_ne!(Some(x), prev);
                }
                assert!(!seen[x]);
                seen[x] = true;
                prev = Some(x);
            }
            assert!(seen.iter().all(|x| *x));
        }
    }
    #[test]
    fn schedule_no_duplicates_or_wake_catchup() {
        let s = Settings::default();
        let mut timer = Scheduler::new(3599);
        assert_eq!(timer.poll(3600, 0, &s), Some(Slot::Hour));
        assert_eq!(timer.poll(3600, 0, &s), None);
        assert_eq!(timer.poll(3601, 0, &s), None);
        assert_eq!(timer.poll(5400, 30, &s), None); // wake
        timer.poll(5399, 29, &s);
        assert_eq!(timer.poll(5400, 30, &s), Some(Slot::Half));
        timer.poll(3599, 59, &s);
        assert_eq!(timer.poll(3600, 0, &s), Some(Slot::Hour)); // a fresh crossing after resetting time
        assert_eq!(timer.poll(3600, 0, &s), None);
    }
    #[test]
    fn resetting_clock_before_a_fired_boundary_rearms_that_visit() {
        let s = Settings::default();
        for (boundary, minute, slot) in [(3600, 0, Slot::Hour), (5400, 30, Slot::Half)] {
            let mut timer = Scheduler::new(boundary - 1);
            assert_eq!(timer.poll(boundary, minute, &s), Some(slot));
            timer.poll(boundary + 77, minute + 1, &s);
            // The observed Windows reset: roughly 7:01 back to 6:58.
            assert_eq!(timer.poll(boundary - 114, (minute + 58) % 60, &s), None);
            assert_eq!(timer.poll(boundary - 1, (minute + 59) % 60, &s), None);
            assert_eq!(timer.poll(boundary, minute, &s), Some(slot));
            assert_eq!(timer.poll(boundary + 1, minute, &s), None);
            // Small changes within the already-announced minute do not repeat it.
            assert_eq!(timer.poll(boundary, minute, &s), None);
            assert_eq!(timer.poll(boundary + 1, minute, &s), None);
        }
    }
    #[test]
    fn disabled_slots_and_startup_do_not_chime() {
        let s = Settings {
            half_hourly: false,
            ..Settings::default()
        };
        let mut t = Scheduler::new(1800);
        assert_eq!(t.poll(1801, 30, &s), None);
        t.poll(3599, 59, &s);
        assert_eq!(t.poll(3600, 0, &s), Some(Slot::Hour));
        t.poll(5399, 29, &s);
        assert_eq!(t.poll(5400, 30, &s), None);
    }
    #[test]
    fn times_are_validated() {
        assert_eq!(parse_time("22:15"), Some(1335));
        assert_eq!(parse_time("24:00"), None);
        assert_eq!(parse_time("7:99"), None);
    }
    #[test]
    fn mute_quiet_and_snooze_are_independent() {
        let mut s = Settings::default();
        assert!(!s.sound_allowed(0, 100));
        assert!(s.sound_allowed(720, 100));
        s.snooze_until = 200;
        assert!(!s.sound_allowed(720, 100));
        assert!(s.sound_allowed(720, 200));
        s.mode = Mode::Silent;
        assert!(!s.sound_allowed(720, 300));
    }
}
