# Little Timber: clock behavior and sound research

Researched 10 November 2026. This is a research and production brief, not a record of implemented changes. Existing application code was inspected; no executable, settings, or sound assets were changed. Sources below were read as documents, not independently auditioned as audio recordings.

Implementation follow-up (same day): door/movement synthesis, refined voices, distinct modal bell profiles, and a single overlapping visit mix are now implemented. See README.md for the current implementation and live output verification. Sections below preserve the research-stage findings and proposals.

## 1. How real cuckoo clocks behave

### The mechanical sequence

A representative mechanical clock opens its bird door and counts the hour with cuckoo calls: seven calls at seven, twelve at twelve, and one at the half-hour. The Black Forest Cuckoo product description documents this behavior for an eight-day Regula clock. Its music and dancers operate on the hour. This establishes a concrete example, not a universal rule for every model. [Product description](https://www.blackforestcuckoo.com/products/eight-day-musical-cuckoo-clock-with-hand-carved-birds-leaves-and-chicks-in-nest)

The Saarland clock museum explains the two bellows: each feeds a small pipe, with the higher note followed by the lower one. Their separation is central to the recognizable call. The same museum documents a historical clock with gong strikes accompanying the cuckoo. A cuckoo clock's voice is therefore an air-driven musical imitation, rather than necessarily a recording of a wild bird. [Museum mechanism explanation](https://guide.uhrenmuseum.saarland/monitor9/cuckoo/)

Sean C. (Pat) Monk's repair article on a standard Regula 30-hour movement describes a wire connecting bird and door, a preparatory mechanical release before the hour, and a spring returning the bird/door assembly after the strike cycle finishes. The half-hour mechanism permits one strike/cuckoo. Bellows and hammer are driven by pins on the same plate. This supports coordinated motion and sound, rather than unrelated effects. It does not establish a universal audible gong-before-cuckoo order or exact door duration. [AWCI, Horological Times, July 1982, printed pp. 32–33 / PDF pp. 36–37](https://www.awci.com/wp-content/uploads/ht/1982/1982-07-web.pdf)

Music is an additional stage on musical clocks; the published Hönes setup instructions distinguish waiting for the cuckoo sequence and then the music. Musical schedules vary by movement. [Published Hönes instructions](https://www.cuckooclock.de/answers-to-your-questions/)

### Modern clocks are not identical

Trenkle's quartz manual describes hourly calls, a light sensor for night silence, volume adjustment, and music/on/off selection. Modern electronic models therefore provide precedent for adjustable sound and quiet operation. Do not infer that every quartz clock has identical schedules or features. [Trenkle manufacturer manual, hosted by a dealer](https://josef-machhaus.de/mediafiles/Sonstiges/Manual-Trenkle-Uhren.pdf)

### What to keep for Little Timber

Preserve Nick's requested choreography:

**Boundary bell → doors open → solid visitor enters from the left → individual performance and sound → turns around → returns inside to the right → doors close → counted time chimes.**

This is an intentional toy-clock interpretation. The traditional model usually counts with the cuckoo while the bird remains out; Little Timber counts with bells afterward. The walking route, 180-degree turn, and 24 shuffled characters are also Little Timber's own design.

Borrow mechanical causality: a release initiates movement; feet contact the ledge; the character disappears behind the doorway; the door reaches its stop; only then do the counted chimes begin. Keep the opening marker gentle and brief enough that it is perceptually distinct from the closing count.

## 2. Sound design for the whole object

### Production approach

Use a compact set of original or appropriately licensed recordings for material detail, supported by synthesis where it is effective. RØDE recommends clean Foley recordings and performing against picture so effects match the action. For this clock, record small wooden objects and light hardware at their actual scale; choose the takes against the animation. [RØDE Foley guide](https://rode.com/en-ie/about/news-info/a-quick-guide-to-recording-foley-effects)

The following sound choices and timings are artistic recommendations, not measured specifications of antique clocks.

| Event | Proposed sound | Synchronization and restraint |
|---|---|---|
| Idle | Optional very quiet, alternating mechanical tick/tock | Off by default; tied to the pendulum rhythm. No idle character noises. |
| Hour / half-hour boundary | One soft, rounded bell strike | Exactly at the scheduled boundary; let its tail continue naturally. |
| Release | Tiny dry lever click | At the start of door movement. This is a mechanism cue, not an invented visible lock. |
| Doors opening | Short restrained hinge/wood rub with a trace of mechanism movement | Follow the 1.1-second swing. Avoid a large-door creak, slam, or conspicuous motor whine. |
| Doors fully open | Barely audible wooden stop | At the end of travel; omit if the opening texture already communicates the stop. |
| Visitor enters | Two or three small wooden contacts appropriate to its gait | Only where feet actually touch; light birds and heavier animals have different weight. |
| Performance | One recognizable species-inspired phrase or characteristic action | Match beak, paw, or body action. It must remain the focus of the visit. |
| Turn and return | A short pivot scuff, then a few receding wooden contacts | Stop contacts once the visitor is inside. No magical disappearance sound. |
| Doors closing | Related hinge texture, then a soft wooden clack | Closure must sound different from opening. Final clack lands on visible contact. |
| Time count | Warm, clearly separated bell/gong strikes | Begin after closure. Preserve 1–12 strikes on the hour and one at the half-hour. |
| Settled | Natural last resonance decays into silence | No extra flourish or continuous woodland ambience by default. |

Start with mechanism detail substantially quieter than the visitor and time signals. Use a small, consistent room character, rather than a different reverberant environment for each effect. Give wooden impacts a rounded body and modest high-frequency detail. Keep the sound centered initially; stereo travel is optional and should remain subtle on headphones.

Do not randomize critical timing. A handful of alternate foot taps and small voice variations can avoid repetition while preserving clear contact points. The 24-character shuffle already supplies substantial variety.

### Character palette

These are proposed stylized identities, not zoological recordings. Broad labels such as owl, penguin, and toucan do not identify a species; choose a specific reference species before claiming biological accuracy. Retain quiet action Foley when that communicates the toy better than a vocalization.

| Character | Proposed signature |
|---|---|
| Pip — cuckoo | Airy two-note high-to-low wooden-pipe call, with a hint of bellows breath. |
| Rosie — robin | Short clear, rounded song phrase and two light hop taps. |
| Otis — owl | Low breathy hoot pair, coordinated with the head turn. |
| Blue — bluebird | Small liquid whistle phrase; restrained wing-joint clicks. |
| Sunny — cockatoo | Brief softened raspy greeting with the crest lift. |
| Pebble — penguin | Short mellow throaty phrase and alternating broad-foot taps. |
| Mango — toucan | Low hollow croak with the bill nod. |
| Dot — duck | Two rounded nasal quacks with differing intensity. |
| Fern — finch | Light short chirrup with an individual rhythm. |
| Knock — woodpecker | Three or four dry wooden taps synchronized to the pecks. |
| Pearl — dove | Warm breathy coo with a descending finish. |
| Indigo — peacock | Short softened call plus a restrained fan-opening rustle. |
| Clover — fox | Quiet breathy chuff and little paw taps. |
| Coco — rabbit | Soft double foot thump and a tiny nose/sniff gesture. |
| Bruno — bear | Mellow low growl, airy rather than menacing. |
| Clementine — cat | Small warm meow; avoid a long incessant purr. |
| Hazel — hedgehog | Gentle snuffle with small dry contacts. |
| Miso — mouse | Two tiny claw-scratch bursts on wood, synchronized to paws. |
| Moss — frog | Rounded croak pair followed by a light landing tap. |
| Fawn — deer | Delicate paired hoof taps and a quiet breath. |
| Acorn — squirrel | Brief chatter with nut-nibbling clicks. |
| Bandit — raccoon | Soft inquisitive trill/chitter and paw contacts. |
| Bamboo — panda | Small bamboo crunches with a low breath. |
| Olive — otter | Brief chirrup and a tiny wooden fish-handling tap. |

### Bells and Zen mode

Bell identity comes from the strike and its evolving resonances. Karjalainen, Välimäki, and Esquef model bell sounds with decaying resonant modes, including inharmonic components and close frequency pairs that produce beating. This supports different resonance/decay structures for different instruments, rather than only changing the base pitch. [DAFx 2002: Efficient Modeling and Synthesis of Bell-like Sounds](https://www.dafx.de/papers/DAFX02_Karjalainen_Valimaki_Esquef_bell-like_sounds.pdf)

The University of Oldenburg's bowl study specifically identifies paired partials with slightly different frequencies, amplitudes, and decays as a cause of characteristic singing-bowl beating. Its measured bowl is an example, not a tuning template for every bowl. [DAGA 2014: Spectral Directivity of Singing Bowls](https://pub.dega-akustik.de/DAGA_2014/data/articles/000254.pdf)

Recommended distinct choices:

- **Clock gong:** compact, warm metallic body with clear strike definition for counting.
- **Warm singing bowl:** soft mallet onset, low rounded body, slow beating, generous decay.
- **Bright singing bowl:** lighter body and more upper resonance, still softly struck.
- **Temple-style bell:** a separate bell timbre with a clearer attack; select an actual reference instrument before calling it a faithful reproduction.

Zen keeps the same physical choreography and existing counting preference. Substitute the selected meditation tone for the critter's call, reduce mechanism/foot detail, and allow resonances to overlap naturally. A sustained rubbed-bowl drone is a separate experience and is not needed for a brief half-hour visit. Do not silently turn off hour counting in Zen.

## 3. What the current code actually supports

Inspected `src/audio.rs`, `src/model.rs`, and playback in `src/main.rs`:

- `VisitAudio::new` queues opening bell, visitor sound, and closing time chimes. No door or movement Foley is currently queued.
- The common bell generator uses the same four resonance ratios and decay constants for all three options, with base frequencies of 220, 432, and 528 Hz. It adds modulation but does not model individually tuned modal pairs for each instrument.
- `play_wave` explicitly stops the previous sound before starting a new WAV. Adding independent door cues this way could cut off a bell or voice tail.
- Ordinary choreography is 12.7 seconds through closure: boundary bell at 0, opening at 1.6, entry at 2.7, performance at 4.3, turn at 8.8, return at 10.0, closing at 11.6, closed at 12.7. Closing chimes start at 12.95 seconds. Zen duration is computed separately. These are application timings, not historical measurements.

The simplest implementation candidate is to mix each complete visit into one PCM buffer using the same event timings as the animation. That would allow overlapping tails and Foley without adding an audio engine. Preserve cancellation, quiet hours, mute, snooze, and the rule against replaying missed events after suspension. Synchronization and suspend handling still need verification; a pre-mixed buffer alone does not prove those behaviors.

## 4. Production and acceptance checks

1. Build a small reusable material kit: release, hinge opening, hinge closing, two end stops, light/heavy foot taps, pivot scuff, claw scratch, nut/bamboo contacts. Record multiple clean takes without baked-in music or room noise.
2. Produce distinct clock and meditation instruments, then the 24 character signatures. Save source/license records for any external recordings; the research links above do not grant redistribution rights.
3. Audition first with Pip, Dot, Miso, Bruno, and one bowl. They test whistle, quack, scratch, growl, and sustained resonance before extending the entire cast.
4. Check the running app with captured system audio and picture: actual contact synchronization, no clipped tails, correct order, solid return behind the doorway, then closure and count. Listen on the user's speakers and headphones at a comfortable low volume.
5. Verify one, seven, and twelve closing strikes; half-hour one; all 24 visitors; Cuckoo/Zen/Silent; mute or stop mid-visit; settings changes; reduced motion; quiet hours; suspend/resume; backward time adjustment. Use a testable injected clock for time cases rather than changing the user's Windows clock.

Open evidence limit: no reference recording has been auditioned or licensed in this research pass, and no new sound design has been implemented or listening-tested. Exact hinge timbre and final loudness must be decided against the running animation and audible output.
