use crate::{
    cast::CAST,
    model::{Mode, Settings},
};
use std::f32::consts::TAU;
const RATE: u32 = 24000;
// Original-speed field recordings and recorded prop Foley for all 24 friends.
// All playback stays offline. Sources, licenses and exact edits are in assets/audio/foley.
const VOICES: [&[u8]; 24] = [
    include_bytes!("../assets/audio/foley/cast/pip.pcm"),
    include_bytes!("../assets/audio/foley/cast/rosie.pcm"),
    include_bytes!("../assets/audio/foley/cast/otis.pcm"),
    include_bytes!("../assets/audio/foley/cast/blue.pcm"),
    include_bytes!("../assets/audio/foley/cast/sunny.pcm"),
    include_bytes!("../assets/audio/foley/cast/pebble.pcm"),
    include_bytes!("../assets/audio/foley/cast/mango.pcm"),
    include_bytes!("../assets/audio/foley/dot.pcm"),
    include_bytes!("../assets/audio/foley/cast/fern.pcm"),
    include_bytes!("../assets/audio/foley/cast/knock.pcm"),
    include_bytes!("../assets/audio/foley/pearl.pcm"),
    include_bytes!("../assets/audio/foley/cast/indigo.pcm"),
    include_bytes!("../assets/audio/foley/cast/clover.pcm"),
    include_bytes!("../assets/audio/foley/coco.pcm"),
    include_bytes!("../assets/audio/foley/cast/bruno.pcm"),
    include_bytes!("../assets/audio/foley/cast/clementine.pcm"),
    include_bytes!("../assets/audio/foley/cast/hazel.pcm"),
    include_bytes!("../assets/audio/foley/cast/miso.pcm"),
    include_bytes!("../assets/audio/foley/cast/moss.pcm"),
    include_bytes!("../assets/audio/foley/cast/fawn.pcm"),
    include_bytes!("../assets/audio/foley/cast/acorn.pcm"),
    include_bytes!("../assets/audio/foley/cast/bandit.pcm"),
    include_bytes!("../assets/audio/foley/cast/bamboo.pcm"),
    include_bytes!("../assets/audio/foley/cast/olive.pcm"),
];
// Material contacts repeat with the twelve-frame / 8 fps articulated performance.
// Vocal phrases (including Bruno's single exhale) happen once per visit.
const MOVEMENT_FOLEY: &[usize] = &[4, 5, 9, 11, 12, 13, 16, 17, 19, 20, 21, 22, 23];

// Headerless 24 kHz mono signed little-endian PCM prepared by the asset tool.
fn sample_at(pcm: &[u8], t: f32) -> f32 {
    if t < 0. || t >= pcm.len() as f32 / (2. * RATE as f32) {
        return 0.;
    }
    let position = t * RATE as f32;
    let i = position.floor() as usize;
    let read = |index: usize| {
        pcm.get(index * 2..index * 2 + 2).map_or(0., |bytes| {
            i16::from_le_bytes([bytes[0], bytes[1]]) as f32 / 32768.
        })
    };
    let fraction = position - i as f32;
    read(i) * (1. - fraction) + read(i + 1) * fraction
}
pub fn duration(settings: &Settings, count: usize) -> f32 {
    let count = count.clamp(1, 12);
    if settings.mode == Mode::Zen {
        6.0 + (count - 1) as f32 * 2.4
    } else if settings.voice == 0 {
        1.6 + (count - 1) as f32 * 1.85
    } else {
        count as f32 * 1.05 + 0.3
    }
}

fn envelope(t: f32, length: f32, attack: f32) -> f32 {
    if !(0.0..length).contains(&t) {
        return 0.;
    }
    (t / attack).min(1.) * ((length - t) / (length * 0.35)).min(1.)
}

fn noise(t: f32, seed: u32) -> f32 {
    let mut x = (t.max(0.) * RATE as f32) as u32 ^ seed;
    x = x.wrapping_mul(747796405).wrapping_add(2891336453);
    x = ((x >> ((x >> 28) + 4)) ^ x).wrapping_mul(277803737);
    ((x >> 22) ^ x) as f32 / u32::MAX as f32 * 2. - 1.
}

// Smooth random pressure rather than harsh full-band digital hiss.
fn air(t: f32, seed: u32, bandwidth: f32) -> f32 {
    let p = t.max(0.) * bandwidth;
    let i = p.floor();
    let f = p - i;
    let f = f * f * (3. - 2. * f);
    noise(i / RATE as f32, seed) * (1. - f) + noise((i + 1.) / RATE as f32, seed) * f
}

fn scratch(t: f32, length: f32, seed: u32) -> f32 {
    // Short high-passed friction strokes, with a faint wooden resonance.
    let hiss = air(t, seed, 4200.) - air(t, seed, 700.);
    envelope(t, length, 0.012)
        * (hiss * 0.36 + (TAU * 1250. * t).sin() * 0.035)
        * (0.7 + 0.3 * (TAU * 47. * t).sin())
}

fn creature(id: usize, t: f32) -> f32 {
    sample_at(VOICES[id % VOICES.len()], t)
}

fn voice_samples(character: usize, settings: &Settings, count: usize) -> Vec<f32> {
    let c = CAST[character % 24];
    let zen = settings.mode == Mode::Zen;
    let count = count.clamp(1, 12);
    let spacing = if zen {
        2.4
    } else if settings.voice == 0 {
        1.85
    } else {
        1.05
    };
    let duration = duration(settings, count);
    let mut samples = vec![0f32; (duration * RATE as f32) as usize];
    for strike in 0..count {
        let offset = (strike as f32 * spacing * RATE as f32) as usize;
        for (i, sample) in samples.iter_mut().enumerate().skip(offset) {
            let t = (i - offset) as f32 / RATE as f32;
            let value = if zen {
                bell_sample(character, settings.bell, t)
            } else if settings.voice == 0 {
                if t >= 1.6 {
                    break;
                }
                creature(character, t)
            } else {
                if t >= 0.9 {
                    break;
                }
                let mut v = 0.;
                let voice = settings.voice;
                let base = if voice == 0 {
                    c.pitch
                } else {
                    [0., 760., 550., 980.][voice]
                };
                let notes = if voice == 0 && character >= 12 { 3 } else { 2 };
                for note in 0..notes {
                    let nt = t - note as f32 * 0.29;
                    if !(0.0..0.27).contains(&nt) {
                        continue;
                    }
                    let env = (nt / 0.022).min(1.) * ((0.27 - nt) / 0.085).min(1.);
                    let pitch = base
                        * if note == 1 {
                            0.79
                        } else if note == 2 {
                            1.12
                        } else {
                            1.
                        };
                    let flutter = if voice == 3 || voice == 0 {
                        0.035 * (TAU * (6. + (character % 5) as f32) * nt).sin()
                    } else {
                        0.
                    };
                    let phase = TAU * pitch * (nt + flutter * nt * 0.15);
                    let overtone = if voice == 2 {
                        0.035
                    } else {
                        0.12 + (character % 4) as f32 * 0.03
                    };
                    v += env
                        * (phase.sin() + overtone * (2. * phase).sin() + 0.03 * (3. * phase).sin())
                        * 0.65;
                }
                v
            };
            *sample += value;
        }
    }
    samples
}

#[cfg(test)]
fn make_wav(character: usize, settings: &Settings, count: usize) -> Vec<u8> {
    encode_wav(
        &voice_samples(character, settings, count),
        settings.volume as f32 / 100. * 0.65,
    )
}

fn encode_wav(samples: &[f32], gain: f32) -> Vec<u8> {
    let length = (samples.len() * 2) as u32;
    let mut wav = Vec::with_capacity(length as usize + 44);
    wav.extend(b"RIFF");
    wav.extend((length + 36).to_le_bytes());
    wav.extend(b"WAVEfmt ");
    wav.extend(16u32.to_le_bytes());
    wav.extend(1u16.to_le_bytes());
    wav.extend(1u16.to_le_bytes());
    wav.extend(RATE.to_le_bytes());
    wav.extend((RATE * 2).to_le_bytes());
    wav.extend(2u16.to_le_bytes());
    wav.extend(16u16.to_le_bytes());
    wav.extend(b"data");
    wav.extend(length.to_le_bytes());
    for (i, s) in samples.iter().enumerate() {
        let fade = ((samples.len() - i) as f32 / (RATE as f32 * 0.04)).min(1.);
        let pcm = ((s * gain * fade).clamp(-0.98, 0.98) * i16::MAX as f32) as i16;
        wav.extend(pcm.to_le_bytes());
    }
    wav
}
fn bell_sample(character: usize, bell: usize, t: f32) -> f32 {
    let bell = if bell == 3 {
        character % 3
    } else {
        bell.min(2)
    };
    // Each instrument has its own measured-style modal structure, not a
    // transposition of one oscillator. Close pairs create natural beating.
    let (base, modes, attack) = match bell {
        0 => (
            196.,
            [
                (1., 0.48, 3.8, 0.65),
                (2.71, 0.22, 2.8, 1.1),
                (5.18, 0.10, 1.8, 1.8),
                (8.63, 0.035, 0.9, 2.7),
                (12.1, 0.012, 0.45, 3.2),
            ],
            0.018,
        ),
        1 => (
            329.6,
            [
                (1., 0.40, 3.1, 1.0),
                (2.58, 0.24, 2.7, 1.6),
                (4.94, 0.13, 1.5, 2.3),
                (8.21, 0.06, 0.8, 3.4),
                (11.7, 0.02, 0.4, 4.2),
            ],
            0.009,
        ),
        _ => (
            261.6,
            [
                (0.5, 0.19, 3.6, 0.4),
                (1., 0.35, 3.0, 0.8),
                (1.49, 0.18, 2.4, 1.3),
                (2.03, 0.12, 1.4, 2.0),
                (3.96, 0.07, 0.65, 3.3),
            ],
            0.006,
        ),
    };
    let mut value = 0.;
    for (ratio, amplitude, decay, split) in modes {
        let f = base * ratio;
        value += amplitude
            * ((TAU * f * t).sin() * 0.62 * (-t / decay).exp()
                + (TAU * (f + split) * t).sin() * 0.38 * (-t / (decay * 0.86)).exp());
    }
    let mallet = air(t, 501 + bell as u32, 1800.) * (-t * 65.).exp() * 0.045;
    (t / attack).min(1.) * (value + mallet)
}

const BELL_SPACING: f32 = 1.3;
const BELL_TAIL: f32 = 6.0;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Sound {
    OpeningBell,
    DoorOpen,
    OpenStop,
    Step,
    Voice,
    Turn,
    DoorClose,
    CloseStop,
    Chime,
}
impl Sound {
    fn name(self) -> &'static str {
        match self {
            Self::OpeningBell => "boundary bell",
            Self::DoorOpen => "doors opening",
            Self::OpenStop => "doors open",
            Self::Step => "wooden foot contact",
            Self::Voice => "visitor performance",
            Self::Turn => "turning inward",
            Self::DoorClose => "doors closing",
            Self::CloseStop => "doors closed",
            Self::Chime => "time strike",
        }
    }
}
#[derive(Clone, Copy)]
struct Event {
    at: f32,
    sound: Sound,
}

fn score(settings: &Settings, count: usize, visual: f32) -> Vec<Event> {
    use crate::model::*;
    let timing = VisitTiming::new(visual);
    let mut events = vec![
        Event {
            at: 0.,
            sound: Sound::OpeningBell,
        },
        Event {
            at: PERFORMANCE_START,
            sound: Sound::Voice,
        },
    ];
    if settings.motion {
        events.extend([
            Event {
                at: OPENING_BELL_SECONDS,
                sound: Sound::DoorOpen,
            },
            Event {
                at: OPENING_BELL_SECONDS + DOOR_SECONDS,
                sound: Sound::OpenStop,
            },
            Event {
                at: timing.turn,
                sound: Sound::Turn,
            },
            Event {
                at: timing.close,
                sound: Sound::DoorClose,
            },
            Event {
                at: timing.closed,
                sound: Sound::CloseStop,
            },
        ]);
        // Contact occurs at each zero of the renderer's absolute sine bounce.
        let step_interval = std::f32::consts::PI / STEP_RADIANS_PER_SECOND;
        for n in 1..(visual / step_interval) as usize {
            let at = n as f32 * step_interval;
            let pose = visit_pose(at, visual, true);
            if (at < PERFORMANCE_START || at >= timing.retreat)
                && pose.visible
                && pose.depth > 0.12
                && pose.depth < 0.94
            {
                events.push(Event {
                    at,
                    sound: Sound::Step,
                });
            }
        }
    } else {
        // Reduced-motion doors jump; there is no audible travel without travel.
        events.extend([
            Event {
                at: OPENING_BELL_SECONDS + DOOR_SECONDS,
                sound: Sound::OpenStop,
            },
            Event {
                at: timing.close,
                sound: Sound::CloseStop,
            },
        ]);
    }
    for n in 0..count.clamp(1, 12) {
        events.push(Event {
            at: timing.closed + 0.25 + n as f32 * BELL_SPACING,
            sound: Sound::Chime,
        });
    }
    events.sort_by(|a, b| a.at.total_cmp(&b.at));
    events
}

fn wood(t: f32, base: f32) -> f32 {
    if !(0.0..0.22).contains(&t) {
        return 0.;
    }
    (t / 0.002).min(1.)
        * ((TAU * base * t).sin() * (-t * 36.).exp() * 0.65
            + (TAU * base * 2.83 * t).sin() * (-t * 62.).exp() * 0.23
            + air(t, 341, 3600.) * (-t * 95.).exp() * 0.12)
}

fn event_samples(event: Event, id: usize, s: &Settings, visual: f32) -> Vec<f32> {
    use crate::model::*;
    if event.sound == Sound::Voice {
        // The articulated poses repeat every 1.5s. Contact-based performances
        // follow those poses, while vocal phrases happen once per visit.
        if s.motion
            && s.mode != Mode::Zen
            && s.voice == 0
            && MOVEMENT_FOLEY.contains(&(id % VOICES.len()))
        {
            let length = VisitTiming::new(visual).turn - PERFORMANCE_START - 0.1;
            let cycle = PERFORMANCE_FRAMES as f32 / PERFORMANCE_FPS;
            let cycle_samples = (cycle * RATE as f32) as usize;
            return (0..(length * RATE as f32) as usize)
                .map(|i| {
                    let t = i as f32 / RATE as f32;
                    creature(id, (i % cycle_samples) as f32 / RATE as f32)
                        * ((length - t) / 0.04).min(1.)
                })
                .collect();
        }
        return voice_samples(id, s, 1);
    }
    let bell = matches!(event.sound, Sound::OpeningBell | Sound::Chime);
    let door = matches!(event.sound, Sound::DoorOpen | Sound::DoorClose);
    let length = if bell {
        BELL_TAIL
    } else if door {
        DOOR_SECONDS
    } else {
        0.25
    };
    let heavy = matches!(id, 14 | 18 | 22);
    let base = if heavy {
        260.
    } else if id >= 12 {
        460.
    } else {
        680.
    };
    let detail = if s.mode == Mode::Zen { 0.35 } else { 1. };
    (0..(length * RATE as f32) as usize)
        .map(|i| {
            let t = i as f32 / RATE as f32;
            match event.sound {
                Sound::OpeningBell | Sound::Chime => {
                    bell_sample(id, s.bell, t)
                        * if event.sound == Sound::OpeningBell {
                            0.38
                        } else {
                            0.65
                        }
                        * ((length - t) / 1.2).min(1.)
                }
                Sound::DoorOpen | Sound::DoorClose => {
                    let closing = event.sound == Sound::DoorClose;
                    let friction =
                        envelope(t, length, 0.08) * (std::f32::consts::PI * t / length).sin();
                    detail
                        * (friction
                            * (air(t, if closing { 83 } else { 71 }, 1800.) * 0.075
                                + air(t, 42, 180.) * 0.035)
                            + wood(t, if closing { 810. } else { 950. }) * 0.075)
                }
                Sound::OpenStop => detail * wood(t, 620.) * 0.10,
                Sound::CloseStop => detail * (wood(t, 440.) * 0.20 + wood(t - 0.035, 710.) * 0.055),
                Sound::Step => detail * wood(t, base) * 0.16,
                Sound::Turn => detail * scratch(t, 0.20, 391) * 0.14,
                Sound::Voice => unreachable!(),
            }
        })
        .collect()
}

pub struct Cue {
    pub name: &'static str,
    pub wav: Vec<u8>,
}
pub struct VisitAudio {
    pub visual_duration: f32,
    pub finish: f32,
    pending: Option<Cue>,
    markers: std::collections::VecDeque<Event>,
    last_elapsed: f32,
}
impl VisitAudio {
    pub fn new(id: usize, settings: &Settings, count: usize, audible: bool) -> Self {
        let visual_duration = crate::model::visit_duration(duration(settings, 1));
        let mut sequence = Self {
            visual_duration,
            finish: visual_duration,
            pending: None,
            markers: Default::default(),
            last_elapsed: 0.,
        };
        if audible && settings.mode != Mode::Silent && settings.volume > 0 {
            let events = score(settings, count, visual_duration);
            sequence.finish = events.last().unwrap().at + BELL_TAIL;
            let mut samples = vec![0f32; (sequence.finish * RATE as f32).ceil() as usize];
            for &event in &events {
                let offset = (event.at * RATE as f32).round() as usize;
                for (destination, source) in samples[offset..].iter_mut().zip(event_samples(
                    event,
                    id,
                    settings,
                    visual_duration,
                )) {
                    *destination += source;
                }
            }
            // Only reduce gain when necessary; never flatten overlapping peaks.
            let peak = samples.iter().fold(0f32, |p, x| p.max(x.abs()));
            let gain = 0.65f32.min(0.92 / peak.max(0.001)) * settings.volume as f32 / 100.;
            sequence.pending = Some(Cue {
                name: "synchronized visit soundtrack",
                wav: encode_wav(&samples, gain),
            });
            sequence.markers = events.into();
        }
        sequence
    }
    pub fn cancel(&mut self) {
        self.pending = None;
        self.markers.clear();
    }
    pub fn take_due(&mut self, elapsed: f32) -> Option<Cue> {
        let cue = self.pending.take();
        if elapsed < 0.75 {
            cue
        } else {
            None
        }
    }
    pub fn take_marker(&mut self, elapsed: f32) -> Option<(&'static str, f32)> {
        if self
            .markers
            .front()
            .is_some_and(|event| elapsed >= event.at)
        {
            self.markers
                .pop_front()
                .map(|event| (event.sound.name(), event.at))
        } else {
            None
        }
    }
    pub fn stalled(&mut self, elapsed: f32) -> bool {
        let gap = elapsed - self.last_elapsed;
        self.last_elapsed = elapsed;
        gap > 0.75
    }
}
#[cfg(test)]
fn wav_seconds(wav: &[u8]) -> f32 {
    (wav.len() - 44) as f32 / (RATE * 2) as f32
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn natural_samples_are_gentle_and_fit_the_existing_actions() {
        for pcm in VOICES {
            assert_eq!(pcm.len() % 2, 0);
            let length = pcm.len() as f32 / (2. * RATE as f32);
            assert!((1.45..=1.56).contains(&length));
            let samples: Vec<_> = pcm
                .as_chunks::<2>()
                .0
                .iter()
                .map(|p| i16::from_le_bytes([p[0], p[1]]) as f32 / 32768.)
                .collect();
            assert!(samples.iter().any(|x| x.abs() > 0.02));
            assert!(samples.iter().all(|x| x.abs() < 0.37));
            assert_eq!(samples[0], 0.);
            assert_eq!(*samples.last().unwrap(), 0.);
            assert_eq!(sample_at(pcm, -0.01), 0.);
            assert_eq!(sample_at(pcm, length + 0.01), 0.);
        }
        // Coco's two grounded atlas poses are 0 and 6. Leave space between
        // contacts instead of running a scrape or an animal cry across the hop.
        assert!(creature(13, 0.06).abs() > 0.0001);
        assert_eq!(creature(13, 0.5), 0.);
        assert!(creature(13, 0.81).abs() > 0.0001);
        assert_eq!(creature(13, 1.4), 0.);
        // Six paw strokes at the existing animation contact phases, with
        // silence between strokes instead of a continuous pitched scratching bed.
        for n in 0..6 {
            let contact = 0.125 + n as f32 * 0.25;
            let energy: f32 = (0..2400)
                .map(|i| creature(17, contact + i as f32 / RATE as f32).powi(2))
                .sum();
            assert!(energy > 0.01);
            assert_eq!(creature(17, contact - 0.02), 0.);
            assert_eq!(creature(17, contact + 0.12), 0.);
        }
    }
    #[test]
    fn movement_foley_repeats_with_atlas_and_stops_before_the_turn() {
        use crate::model::*;
        let s = Settings::default();
        let visual = visit_duration(duration(&s, 1));
        for (id, character) in CAST.iter().enumerate() {
            let samples = event_samples(
                Event {
                    at: PERFORMANCE_START,
                    sound: Sound::Voice,
                },
                id,
                &s,
                visual,
            );
            if MOVEMENT_FOLEY.contains(&id) {
                let cycle = (RATE as f32 * PERFORMANCE_FRAMES as f32 / PERFORMANCE_FPS) as usize;
                assert!(
                    samples[..cycle]
                        .iter()
                        .zip(&samples[cycle..cycle * 2])
                        .all(|(a, b)| a == b),
                    "{}",
                    character.name
                );
                assert!(
                    PERFORMANCE_START + samples.len() as f32 / (RATE as f32)
                        < VisitTiming::new(visual).turn
                );
            } else {
                assert_eq!(samples.len(), (1.6 * RATE as f32) as usize);
            }
        }
        // The recorded wood tap must occur at the visible deepest peck (frame 6).
        assert_eq!(performance_frame(0.75), 6);
        let peck: f32 = (0..4800)
            .map(|i| creature(9, 0.75 + i as f32 / RATE as f32).powi(2))
            .sum();
        assert!(peck > 0.01);
        assert!((0..18000).all(|i| creature(9, i as f32 / RATE as f32) == 0.));
    }
    #[test]
    fn score_follows_visible_contacts_and_counts_only_after_closure() {
        use crate::model::*;
        for mode in [Mode::Cuckoo, Mode::Zen] {
            for motion in [false, true] {
                let s = Settings {
                    mode,
                    motion,
                    ..Settings::default()
                };
                let visual = visit_duration(duration(&s, 1));
                for count in [1, 7, 12] {
                    let events = score(&s, count, visual);
                    let chimes: Vec<_> =
                        events.iter().filter(|e| e.sound == Sound::Chime).collect();
                    assert_eq!(chimes.len(), count);
                    for (n, e) in chimes.iter().enumerate() {
                        assert!((e.at - (visual + 0.25 + n as f32 * BELL_SPACING)).abs() < 0.001);
                        let pose = visit_pose(e.at, visual, motion);
                        assert!(!pose.visible && pose.doors == 0.);
                    }
                    for e in &events {
                        let pose = visit_pose(e.at, visual, motion);
                        match e.sound {
                            Sound::Step => {
                                assert!(motion && pose.visible);
                                assert!((e.at * STEP_RADIANS_PER_SECOND).sin().abs() < 0.0001);
                            }
                            Sound::CloseStop => assert!(!pose.visible && pose.doors < 0.001),
                            Sound::Voice => assert!(pose.visible && pose.doors == 1.),
                            _ => {}
                        }
                    }
                    if !motion {
                        assert!(!events.iter().any(|e| matches!(
                            e.sound,
                            Sound::Step | Sound::Turn | Sound::DoorOpen | Sound::DoorClose
                        )));
                    }
                }
            }
        }
    }
    #[test]
    fn mixed_track_overlaps_bell_tail_with_door_without_restarting_playback() {
        let s = Settings::default();
        let mut sequence = VisitAudio::new(7, &s, 7, true);
        let cue = sequence.take_due(0.).unwrap();
        assert!((wav_seconds(&cue.wav) - sequence.finish).abs() < 0.001);
        assert!(sequence.take_due(1.6).is_none());
        let visual = sequence.visual_duration;
        let bell = event_samples(
            Event {
                at: 0.,
                sound: Sound::OpeningBell,
            },
            7,
            &s,
            visual,
        );
        let door = event_samples(
            Event {
                at: 1.6,
                sound: Sound::DoorOpen,
            },
            7,
            &s,
            visual,
        );
        assert!(bell[(2. * RATE as f32) as usize].abs() > 0.00001);
        assert!(door.iter().any(|x| x.abs() > 0.01));
        let i = (2. * RATE as f32) as usize;
        let expected = ((bell[i] + door[i - (1.6 * RATE as f32) as usize]) * 0.65 * s.volume as f32
            / 100.
            * i16::MAX as f32) as i16;
        let actual = i16::from_le_bytes(cue.wav[44 + i * 2..46 + i * 2].try_into().unwrap());
        assert!((actual as i32 - expected as i32).abs() <= 2);
    }
    #[test]
    fn cancelled_muted_and_stalled_visits_do_not_restart_sound() {
        let s = Settings::default();
        let mut cancelled = VisitAudio::new(17, &s, 12, true);
        cancelled.cancel();
        assert!(cancelled.take_due(0.).is_none());
        assert!(cancelled.take_marker(100.).is_none());
        for settings in [
            Settings {
                mode: Mode::Silent,
                ..s.clone()
            },
            Settings {
                volume: 0,
                ..s.clone()
            },
        ] {
            assert!(VisitAudio::new(17, &settings, 12, true)
                .take_due(0.)
                .is_none());
        }
        assert!(VisitAudio::new(17, &s, 12, false).take_due(0.).is_none());
        let mut slept = VisitAudio::new(17, &s, 12, true);
        assert!(slept.take_due(20.).is_none());
        assert!(!slept.stalled(0.05));
        assert!(slept.stalled(2.));
    }
    #[test]
    fn full_mix_has_headroom_for_all_characters_and_bells() {
        for id in 0..24 {
            let s = Settings {
                volume: 100,
                ..Settings::default()
            };
            let mut audio = VisitAudio::new(id, &s, 12, true);
            let wav = audio.take_due(0.).unwrap().wav;
            assert!(wav[44..]
                .as_chunks::<2>()
                .0
                .iter()
                .all(|p| (i16::from_le_bytes(*p) as i32).abs() < 31000));
        }
        for bell in 0..3 {
            let s = Settings {
                volume: 100,
                mode: Mode::Zen,
                bell,
                ..Settings::default()
            };
            let mut audio = VisitAudio::new(7, &s, 12, true);
            let wav = audio.take_due(0.).unwrap().wav;
            assert!(wav[44..]
                .as_chunks::<2>()
                .0
                .iter()
                .all(|p| (i16::from_le_bytes(*p) as i32).abs() < 31000));
        }
    }
    #[test]
    #[ignore = "Writes reference WAVs and cue sheets for live loopback comparison"]
    fn export_audition_tracks() {
        let dir = std::path::Path::new("artifacts/audio-qa");
        std::fs::create_dir_all(dir).unwrap();
        for (id, mode, bell, count, label) in [
            (0, Mode::Cuckoo, 0, 7, "pip"),
            (6, Mode::Cuckoo, 0, 8, "mango-eight"),
            (7, Mode::Cuckoo, 0, 7, "duck"),
            (7, Mode::Cuckoo, 0, 2, "duck-two"),
            (10, Mode::Cuckoo, 0, 1, "pearl"),
            (13, Mode::Cuckoo, 0, 1, "coco"),
            (17, Mode::Cuckoo, 0, 1, "mouse"),
            (14, Mode::Cuckoo, 0, 1, "bear"),
            (9, Mode::Cuckoo, 0, 7, "woodpecker"),
            (17, Mode::Zen, 0, 1, "mouse-zen"),
            (7, Mode::Zen, 0, 7, "warm-bowl"),
            (7, Mode::Zen, 1, 7, "bright-bowl"),
            (7, Mode::Zen, 2, 7, "temple-bell"),
        ] {
            let s = Settings {
                mode,
                bell,
                volume: 55,
                ..Settings::default()
            };
            let before = std::time::Instant::now();
            let mut audio = VisitAudio::new(id, &s, count, true);
            println!(
                "{} render {:.0}ms",
                label,
                before.elapsed().as_secs_f64() * 1000.
            );
            std::fs::write(
                dir.join(format!("{label}.wav")),
                audio.take_due(0.).unwrap().wav,
            )
            .unwrap();
            let rows = audio
                .markers
                .iter()
                .map(|e| format!("{:.4},{}", e.at, e.sound.name()))
                .collect::<Vec<_>>()
                .join("\n");
            std::fs::write(
                dir.join(format!("{label}.csv")),
                format!("seconds,event\n{rows}\n"),
            )
            .unwrap();
        }
        let settings = Settings {
            volume: 100,
            ..Settings::default()
        };
        for (id, character) in CAST.iter().enumerate() {
            let label = character.name.to_lowercase();
            let visual = crate::model::visit_duration(duration(&settings, 1));
            let samples = event_samples(
                Event {
                    at: 0.,
                    sound: Sound::Voice,
                },
                id,
                &settings,
                visual,
            );
            std::fs::write(
                dir.join(format!("{label}-solo.wav")),
                encode_wav(&samples, 0.65),
            )
            .unwrap();
            let mut visit = VisitAudio::new(id, &settings, 1, true);
            std::fs::write(
                dir.join(format!("{label}-visit.wav")),
                visit.take_due(0.).unwrap().wav,
            )
            .unwrap();
        }
    }
    #[test]
    fn every_character_has_a_unique_valid_sound() {
        let s = Settings::default();
        let clips: Vec<_> = (0..24).map(|i| make_wav(i, &s, 1)).collect();
        for (i, w) in clips.iter().enumerate() {
            assert_eq!(&w[..4], b"RIFF");
            assert_eq!(
                u32::from_le_bytes(w[40..44].try_into().unwrap()) as usize,
                w.len() - 44
            );
            assert!(w[44..].iter().any(|&x| x != 0));
            for other in &clips[..i] {
                assert_ne!(w, other);
            }
        }
    }
    #[test]
    fn silence_and_bells() {
        let mut s = Settings {
            volume: 0,
            ..Settings::default()
        };
        assert!(make_wav(0, &s, 1)[44..].iter().all(|&x| x == 0));
        s.mode = Mode::Zen;
        s.volume = 35;
        let mut clips = vec![];
        for bell in 0..3 {
            s.bell = bell;
            clips.push(make_wav(0, &s, 1));
        }
        assert_ne!(clips[0], clips[1]);
        assert_ne!(clips[1], clips[2]);
    }
    #[test]
    fn calls_have_headroom_and_counted_audio_fits_the_visit() {
        let s = Settings {
            volume: 100,
            ..Settings::default()
        };
        for id in 0..24 {
            let wav = make_wav(id, &s, 1);
            let pcm: Vec<_> = wav[44..]
                .as_chunks::<2>()
                .0
                .iter()
                .map(|p| i16::from_le_bytes([p[0], p[1]]) as i32)
                .collect();
            assert!(pcm.iter().all(|x| x.abs() < 32000));
            assert_eq!(pcm[0], 0);
            assert!(pcm.last().unwrap().abs() < 50);
        }
        for mode in [Mode::Cuckoo, Mode::Zen] {
            let settings = Settings { mode, ..s.clone() };
            let wav = make_wav(7, &settings, 12);
            let seconds = (wav.len() - 44) as f32 / (2 * RATE) as f32;
            assert!((seconds - duration(&settings, 12)).abs() < 0.001);
            assert!(
                crate::model::PERFORMANCE_START + seconds
                    < crate::model::visit_duration(seconds)
                        - crate::model::DOOR_SECONDS
                        - crate::model::RETREAT_SECONDS
                        - crate::model::TURN_SECONDS
            );
        }
    }
}
