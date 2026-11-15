//! Rust clock state for the AppKit frontend. Calls stay on the AppKit main thread.

use crate::{
    audio::VisitAudio,
    cast::CAST,
    model::{self, Mode, Saved, Scheduler, Settings, Slot},
};
use chrono::{DateTime, Local, Timelike};
use std::{ptr, slice, time::Instant};

struct Visit {
    id: usize,
    scheduled: bool,
    motion: bool,
    generation: u64,
    start: Option<Instant>,
    elapsed: f32,
    audio: VisitAudio,
}

pub struct Clock {
    saved: Saved,
    scheduler: Scheduler,
    start: Instant,
    visit: Option<Visit>,
    generation: u64,
    revision: u64,
    json: Vec<u8>,
    wav: Vec<u8>,
}

impl Clock {
    fn visit(&mut self, now: DateTime<Local>, slot: Slot, id: Option<usize>, scheduled: bool) {
        let settings = &self.saved.settings;
        let minute = (now.hour() * 60 + now.minute()) as u16;
        if scheduled && settings.quiet_motion && settings.is_quiet(minute) {
            return;
        }
        let id = id.unwrap_or_else(|| {
            self.revision += 1;
            match slot {
                Slot::Hour => self.saved.hour_bag.draw(rand_word),
                Slot::Half => 12 + self.saved.half_bag.draw(rand_word),
            }
        });
        let audible = if scheduled {
            settings.sound_allowed(minute, now.timestamp())
        } else {
            settings.mode != Mode::Silent && settings.volume > 0
        };
        let count = model::closing_chime_count(now.hour(), slot == Slot::Hour, settings.count_hour);
        self.generation += 1;
        self.visit = Some(Visit {
            id,
            scheduled,
            motion: settings.motion,
            generation: self.generation,
            start: None,
            elapsed: 0.,
            audio: VisitAudio::new(id, settings, count, audible),
        });
    }
}

// Darwin's system RNG uses no application state or external dependency.
fn rand_word() -> u64 {
    extern "C" {
        fn arc4random() -> u32;
    }
    unsafe { (u64::from(arc4random()) << 32) | u64::from(arc4random()) }
}

#[repr(C)]
pub struct Frame {
    generation: u64,
    revision: u64,
    hour: i32,
    minute: i32,
    second: i32,
    character: i32,
    visitor: i32,
    door: i32,
    sprite: i32,
    scheduled: i32,
    swing: f32,
    elapsed: f32,
    depth: f32,
    lateral: f32,
}

// The native caller owns this pointer and keeps input byte slices alive during calls.
#[no_mangle]
unsafe extern "C" fn kc_create(bytes: *const u8, length: usize) -> *mut Clock {
    let mut saved: Saved = if length == 0 {
        Saved::default()
    } else {
        serde_json::from_slice(slice::from_raw_parts(bytes, length)).unwrap_or_default()
    };
    saved.settings.normalize();
    saved.hour_bag.validate();
    saved.half_bag.validate();
    Box::into_raw(Box::new(Clock {
        saved,
        scheduler: Scheduler::new(Local::now().timestamp()),
        start: Instant::now(),
        visit: None,
        generation: 0,
        revision: 0,
        json: Vec::new(),
        wav: Vec::new(),
    }))
}

#[no_mangle]
unsafe extern "C" fn kc_destroy(clock: *mut Clock) {
    drop(Box::from_raw(clock));
}

// Returned bytes are borrowed until the next JSON or audio call, respectively.
#[no_mangle]
extern "C" fn kc_json(clock: &mut Clock, kind: i32, length: &mut usize) -> *const u8 {
    clock.json = match kind {
        0 => serde_json::to_vec(&clock.saved.settings),
        1 => serde_json::to_vec(&clock.saved),
        _ => serde_json::to_vec(&CAST.map(|c| (c.name, c.kind, c.action))),
    }
    .expect("clock state serializes");
    *length = clock.json.len();
    clock.json.as_ptr()
}

#[no_mangle]
unsafe extern "C" fn kc_settings(clock: &mut Clock, bytes: *const u8, length: usize) -> bool {
    let Ok(mut settings) = serde_json::from_slice::<Settings>(slice::from_raw_parts(bytes, length))
    else {
        return false;
    };
    settings.normalize();
    clock.saved.settings = settings;
    clock.revision += 1;
    true
}

#[no_mangle]
extern "C" fn kc_visit(clock: &mut Clock, id: i32, half: bool, scheduled: bool) {
    let id = usize::try_from(id).ok().filter(|&id| id < CAST.len());
    let slot = if id.map_or(half, |id| id >= 12) {
        Slot::Half
    } else {
        Slot::Hour
    };
    clock.visit(Local::now(), slot, id, scheduled);
}

#[no_mangle]
extern "C" fn kc_stop(clock: &mut Clock) {
    clock.visit = None;
}

#[no_mangle]
extern "C" fn kc_resume(clock: &mut Clock) {
    kc_stop(clock);
    clock.scheduler = Scheduler::new(Local::now().timestamp());
}

#[no_mangle]
extern "C" fn kc_audio(clock: &mut Clock, length: &mut usize) -> *const u8 {
    clock.wav.clear();
    if let Some(cue) = clock.visit.as_mut().and_then(|v| v.audio.take_due(0.)) {
        clock.wav = cue.wav;
    }
    *length = clock.wav.len();
    if clock.wav.is_empty() {
        ptr::null()
    } else {
        clock.wav.as_ptr()
    }
}

#[no_mangle]
extern "C" fn kc_started(clock: &mut Clock) {
    if let Some(visit) = &mut clock.visit {
        visit.start = Some(Instant::now());
    }
}

#[no_mangle]
extern "C" fn kc_tick(clock: &mut Clock, frame: &mut Frame, playback_seconds: f64) {
    let now = Local::now();
    if let Some(slot) = clock
        .scheduler
        .poll(now.timestamp(), now.minute(), &clock.saved.settings)
    {
        clock.visit(now, slot, None, true);
    }
    if clock.scheduler.take_time_change().is_some() {
        kc_stop(clock);
    }
    *frame = Frame {
        generation: 0,
        revision: clock.revision,
        hour: now.hour() as i32,
        minute: now.minute() as i32,
        second: now.second() as i32,
        character: -1,
        visitor: -1,
        door: 0,
        sprite: 0,
        scheduled: 0,
        swing: if clock.saved.settings.motion {
            (clock.start.elapsed().as_secs_f64() * 2.8).sin() as f32 * 3.5
        } else {
            0.
        },
        elapsed: 0.,
        depth: 0.,
        lateral: 0.,
    };
    if let Some(v) = &mut clock.visit {
        // AVAudioPlayer runs independently of AppKit's timer. If the UI thread
        // misses a tick, draw the current pose instead of cutting off the audio.
        // Sleep and wall-clock discontinuities still cancel the visit.
        let elapsed = v.start.map_or(0., |start| {
            if playback_seconds >= 0. {
                playback_seconds as f32
            } else {
                start.elapsed().as_secs_f32()
            }
        });
        let elapsed = elapsed.max(v.elapsed);
        v.elapsed = elapsed;
        if elapsed >= v.audio.finish {
            kc_stop(clock);
            return;
        }
        let pose = model::visit_pose(elapsed, v.audio.visual_duration, v.motion);
        frame.generation = v.generation;
        frame.character = v.id as i32;
        frame.scheduled = i32::from(v.scheduled);
        frame.door = (pose.doors * 16.).round() as i32;
        frame.visitor = if pose.visible { v.id as i32 } else { -1 };
        frame.sprite = pose.frame as i32;
        frame.elapsed = elapsed;
        frame.depth = pose.depth;
        frame.lateral = pose.lateral;
    }
}
