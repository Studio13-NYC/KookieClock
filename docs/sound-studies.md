# Recorded voices and Foley for all 24 friends

Updated 20 November 2026. All individual character greetings now use edited
recordings or recorded prop Foley in the shared Rust mixer, on Windows and Mac.
The approved Dot, Pearl and Coco PCM assets are unchanged. Recordings retain
their original speed and pitch; no oscillator calls, stretched creaks or
AI-generated character samples remain in the active individual-voice path.

## Research and sound choices

The Academy describes Foley as performing footsteps, fabric and props against
picture. Here, contacts follow the animated toy's feet, paws and props.
[Academy sound activities guide](https://www.oscars.org/sites/oscars/files/complete_sound_and_music_activities_guide.pdf).

Bird greetings favor song, coos and low hoots. Cornell describes the bluebird's
short, soft warbles and the owl's deep hoots; the Woodland Trust distinguishes
robin song from urgent alarm calls. These guided the choice of recordings.
[Eastern bluebird](https://www.allaboutbirds.org/guide/Eastern_Bluebird/sounds),
[great horned owl](https://www.allaboutbirds.org/guide/Great_Horned_Owl/sounds),
[robin song](https://www.woodlandtrust.org.uk/blog/identify-bird-song/).
Mango uses a white-throated toucan recording; Fern uses a European greenfinch.
The toy drawings are stylized and do not establish a biological subspecies.
Sunny and Indigo have feather-movement Foley rather than loud display calls.

For quiet woodland toys, the sounds represent material movement. Cloth supplies
soft foot contacts, paper supplies fine scratches and feather/fan movement,
wood supplies pecks and hoof contacts, and small carrot contacts supply food
nibbles. Bruno, Clover and Hazel use human-breath Foley for an exhale or sniff;
these are not wildlife recordings of those species. Clementine uses a real cat
purr. Moss uses a real frog recording whose source does not identify the species.

The earlier choices remain: Dot uses closing quacks from a diminishing mallard
phrase; Pearl keeps the complete short-long-short coo. Coco uses paw landings
rather than distress or warning sounds.
[Mallard](https://www.allaboutbirds.org/guide/Mallard/sounds),
[collared dove](https://www.allaboutbirds.org/guide/Eurasian_Collared-Dove/sounds),
[RSPCA rabbit behaviour](https://science.rspca.org.uk/en/web/rspca/adviceandwelfare/pets/rabbits/behaviour).

## Timing and mix

All offsets below are seconds after the performance starts at visit time 4.3 s,
once the doors are fully open and the visitor is on the stage.

| Friend | Recorded treatment | Performance timing |
| --- | --- | --- |
| Pip | One complete natural two-note common cuckoo phrase. | 0.15 s; once |
| Rosie | A short European robin song phrase, rather than an alarm tic. | 0.12 s; once |
| Otis | Two rounded great horned owl hoots, with the natural pause shortened. | 0.03, 0.9 s; once |
| Blue | A quiet Eastern bluebird warble, retaining the recorded pitch. | 0.22 s; once |
| Sunny | Paper-brush Foley follows the crest folding and lifting; no cockatoo shriek. | 0.25, 0.75 s; repeats every 1.5 s |
| Pebble | Alternating soft cloth flipper steps. | 0, 0.75 s; repeats every 1.5 s |
| Mango | A short white-throated toucan phrase, aligned with the beak nod. | 0.48 s; once |
| Dot | Two quiet mallard quacks, unchanged. | 0.16 and 0.77 s, once |
| Fern | One small greenfinch twitter, with high-frequency edge softened. | 0.08 s; once |
| Knock | A recorded light wood tap at the deepest peck, atlas frame 6. | 0.75 s; repeats every 1.5 s |
| Pearl | One complete collared-dove coo, unchanged. | 0.08 s, once |
| Indigo | Two broad paper-feather sweeps follow the fan sway. | 0.125, 0.875 s; repeats every 1.5 s |
| Clover | A tiny sniff and soft tail brush, human breath and cloth Foley. | 0.16, 0.87 s; repeats every 1.5 s |
| Coco | Two soft cloth landings, unchanged. | frames 0 and 6, repeating |
| Bruno | One low relaxed exhale and a cloth paw wave. Human breath Foley, not a bear roar. | 0.08, 0.75 s; once |
| Clementine | A real cat purr, with soft head-tilt-length edges. | 0.07 s; once |
| Hazel | Two little sniffles, edited from short human-breath takes as Foley. | 0.08, 0.78 s; repeats every 1.5 s |
| Miso | Six fine paper-friction claw strokes follow the alternating animated paws. | 0.125, 0.375, 0.625, 0.875, 1.125, 1.375 s; repeats every 1.5 s |
| Moss | A small field-recorded frog croak at the grounded start of the leap. | 0 s; once |
| Fawn | A light wooden hoof contact when the stepping leg returns, plus a tiny settling contact. | 0, 0.1 s; repeats every 1.5 s |
| Acorn | Three small nut nibbles from recorded carrot crunches while the acorn is raised. | 0.5, 0.75, 1 s; repeats every 1.5 s |
| Bandit | Quiet cloth paw rubs follow the side-to-side inspection. | 0.25, 1 s; repeats every 1.5 s |
| Bamboo | Two soft bamboo-style nibbles from different carrot contacts, near the raised prop. | 0.625, 0.95 s; repeats every 1.5 s |
| Olive | Two soft cloth-and-paper fish-cradle rubs at the sway extremes. | 0.25, 1 s; repeats every 1.5 s |

The atlas has twelve frames at eight frames per second. Material performances
repeat on that 1.5-second cycle; each loop uses the same integer PCM positions
to avoid drift. The final loop fades only its audio over 40 ms and stops 100 ms
before the turn starts. Solid characters keep their existing turn, retreat and
door ordering. Vocal phrases occur once per visit and finish before the turn.
Reduced motion plays one short greeting. Zen, mute, volume, quiet hours and
snooze retain their existing behavior.

New recorded voices are capped at 0.29 peak amplitude; new movement Foley at
0.24 or lower before master gain. Existing Dot is capped at 0.36. These are
peak ceilings, not claims of identical perceived loudness. Short edge fades,
rounded filtering and space between contacts soften the presentation. No pitch
or time manipulation is used. Alternate cuckoo/whistle voices, clock mechanisms
and meditation bells retain their existing synthesis and settings.

## Recording credits and licenses

The following credits are also included in both app packages. License terms
apply to the identified recordings and their audio adaptations; no endorsement
by any recordist is implied. CC BY-SA adaptations retain the source license.
CC0 and public-domain material remains available under its stated dedication.

- **Dot:** Jonathon Jongsma, [Mallard XC62258](https://xeno-canto.org/62258), [CC BY-SA 3.0](https://creativecommons.org/licenses/by-sa/3.0/). Two closing quacks selected, separated, filtered, faded and leveled.
- **Pearl:** Jonathon Jongsma, [Eurasian Collared Dove XC82758](https://xeno-canto.org/82758), [CC BY-SA 3.0](https://creativecommons.org/licenses/by-sa/3.0/). One intact coo selected, filtered, faded and leveled.
- **Coco:** PelicanPolice, [Fabric flaps](https://freesound.org/people/PelicanPolice/sounds/580967/), [CC0 1.0](https://creativecommons.org/publicdomain/zero/1.0/). Recorded T-shirt contacts selected, filtered, faded, leveled and placed at landing phases.
- **Pip:** Benoît Van Hecke, [Cuculus canorus - Common Cuckoo XC541195.mp3](https://commons.wikimedia.org/wiki/File:Cuculus_canorus_-_Common_Cuckoo_XC541195.mp3). Original recording and the specified adapted PCM audio: **[CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0)**. Original-speed trim, filtering, edge fades, gain and cue placement; exact edits are in `assets/audio/foley/cast/edits.json`.
- **Rosie:** Jan Cibulka, [Erithacus rubecula - European Robin XC441752.mp3](https://commons.wikimedia.org/wiki/File:Erithacus_rubecula_-_European_Robin_XC441752.mp3). Original recording and the specified adapted PCM audio: **[CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0)**. Original-speed trim, filtering, edge fades, gain and cue placement; exact edits are in `assets/audio/foley/cast/edits.json`.
- **Otis:** Jonathon Jongsma, [Bubo virginianus - Great Horned Owl XC428421.mp3](https://commons.wikimedia.org/wiki/File:Bubo_virginianus_-_Great_Horned_Owl_XC428421.mp3). Original recording and the specified adapted PCM audio: **[CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0)**. Original-speed trim, filtering, edge fades, gain and cue placement; exact edits are in `assets/audio/foley/cast/edits.json`.
- **Blue:** Jonathon Jongsma, [Sialia sialis - Eastern Bluebird XC79976.mp3](https://commons.wikimedia.org/wiki/File:Sialia_sialis_-_Eastern_Bluebird_XC79976.mp3). Original recording and the specified adapted PCM audio: **[CC BY-SA 3.0](https://creativecommons.org/licenses/by-sa/3.0)**. Original-speed trim, filtering, edge fades, gain and cue placement; exact edits are in `assets/audio/foley/cast/edits.json`.
- **Fern:** Jan Cibulka, [Chloris chloris - European Greenfinch XC484483.mp3](https://commons.wikimedia.org/wiki/File:Chloris_chloris_-_European_Greenfinch_XC484483.mp3). Original recording and the specified adapted PCM audio: **[CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0)**. Original-speed trim, filtering, edge fades, gain and cue placement; exact edits are in `assets/audio/foley/cast/edits.json`.
- **Clementine:** Mysid, [Purring cat.oga](https://commons.wikimedia.org/wiki/File:Purring_cat.oga). Original recording and the specified adapted PCM audio: **Public domain**. Original-speed trim, filtering, edge fades, gain and cue placement; exact edits are in `assets/audio/foley/cast/edits.json`.
- **Knock, Fawn:** Elandre01, [Freesound - Knocking on wood.wav by Elandre01](https://freesound.org/people/Elandre01/sounds/594389/). Original recording and the specified adapted PCM audio: **[CC0 1.0](https://creativecommons.org/publicdomain/zero/1.0/)**. Original-speed trim, filtering, edge fades, gain and cue placement; exact edits are in `assets/audio/foley/cast/edits.json`.
- **Sunny, Indigo, Miso, Bandit, Olive:** keweldog, [Freesound - rustling paper.wav by keweldog](https://freesound.org/people/keweldog/sounds/181774/). Original recording and the specified adapted PCM audio: **[CC0 1.0](https://creativecommons.org/publicdomain/zero/1.0/)**. Original-speed trim, filtering, edge fades, gain and cue placement; exact edits are in `assets/audio/foley/cast/edits.json`.
- **Acorn, Bamboo:** berzins1, [Freesound - Carrot eating.mp3 by berzins1](https://freesound.org/people/berzins1/sounds/93061/). Original recording and the specified adapted PCM audio: **[CC0 1.0](https://creativecommons.org/publicdomain/zero/1.0/)**. Original-speed trim, filtering, edge fades, gain and cue placement; exact edits are in `assets/audio/foley/cast/edits.json`.
- **Clover, Bruno, Hazel:** Samusasi, [Freesound - Respiración_Suave_s.wav by Samusasi](https://freesound.org/people/Samusasi/sounds/364482/). Original recording and the specified adapted PCM audio: **[CC0 1.0](https://creativecommons.org/publicdomain/zero/1.0/)**. Original-speed trim, filtering, edge fades, gain and cue placement; exact edits are in `assets/audio/foley/cast/edits.json`.
- **Mango:** Niels Krabbe, [Ramphastos tucanus - White-throated Toucan XC251458.mp3](https://commons.wikimedia.org/wiki/File:Ramphastos_tucanus_-_White-throated_Toucan_XC251458.mp3). Original recording and the specified adapted PCM audio: **[CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0)**. Original-speed trim, filtering, edge fades, gain and cue placement; exact edits are in `assets/audio/foley/cast/edits.json`.
- **Moss:** clif_creates, [Small Frog Croaks](https://freesound.org/people/clif_creates/sounds/802737/). Original recording and the specified adapted PCM audio: **[CC0 1.0](https://creativecommons.org/publicdomain/zero/1.0/)**. Original-speed trim, filtering, edge fades, gain and cue placement; exact edits are in `assets/audio/foley/cast/edits.json`.
- **Pebble, Clover, Bruno, Bandit, Olive:** PelicanPolice, [Fabric flaps](https://freesound.org/people/PelicanPolice/sounds/580967/). Original recording and the specified adapted PCM audio: **[CC0 1.0](https://creativecommons.org/publicdomain/zero/1.0/)**. Original-speed trim, filtering, edge fades, gain and cue placement; exact edits are in `assets/audio/foley/cast/edits.json`.

Original recordings, source URLs, SHA-256 checksums and licenses are in
`assets/audio/foley/provenance.json` and
`assets/audio/foley/cast/provenance.json`. The new edit manifest records every
source interval, filter, fade, placement and gain. Source files are included so
preparation works offline. Freesound files are credited high-quality MP3 previews.

## Reproduce and audition

From the repository root, with Rust, uv and ffmpeg installed:

```powershell
uv run --with numpy --with scipy python tools/prepare_foley_studies.py
uv run --with numpy --with scipy python tools/prepare_cast_audio.py
cargo test --locked --release --lib
cargo test --locked --release --lib export_audition_tracks -- --ignored --nocapture
```

Preparation writes measurements and WAVs under `artifacts/cast-audio`. The Rust
exporter writes all 24 actual performance stems as `<name>-solo.wav`, all 24
full visits as `<name>-visit.wav`, and additional bell/door reference mixes
under `artifacts/audio-qa`. Artifacts and loopback captures are not published.

## Verification

Automated coverage includes all 24 sample boundaries, headroom, distinct audio,
silence/Zen behavior, contact repetition without drift, mouse claw phases,
woodpecker peck placement, stopping before the turn, door order and bell counts.

All 24 friends were previewed through the native visitor gallery in the packaged
Windows app. Output-only WASAPI captures had no buffer status flags. Each
complete pre-chime visit matched its exported reference with correlation at
least 0.99999903. Subtracting the bell/mechanism bed and comparing the character
stem separately gave at least 0.99994285, including the very quiet Foley. This
checks the actual character output rather than just matching the common bell.
Settings and clock position compared unchanged; the regular scheduled shuffle
advanced naturally at 11:00. That scheduled visit logged the turn, retreat and
closed doors before the time strikes.

The Windows build and portable tests passed on Windows, Linux, Intel Mac and
Apple Silicon. Both Mac app bundles and the combined universal bundle passed
the existing packaged desktop smoke checks, including audio decode and accepted
playback. [Windows and core CI](https://github.com/Studio13-NYC/KookieClock/actions/runs/44768113962)
and [Mac app checks](https://github.com/Studio13-NYC/KookieClock/actions/runs/44768108756).
These runs test code commit `ff7c006`; the subsequent documentation records
the completed native Windows check. Technical verification cannot replace a
listening review for artistic preference.
