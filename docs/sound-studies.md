# Three character sound studies

Prepared 19 November 2026. These are first listening studies for Dot, Pearl, and
Coco. The shared Rust mixer uses them in both native frontends. All three retain
the original speed and pitch of their source recordings.

## Research that shaped the edit

The Academy describes Foley as performance against picture, especially footsteps,
breath, fabric, and props. For this clock, each material sound should occur where
the toy touches or moves. Short separate contacts leave space for the character's
identity and the clock bells. [Academy sound activities guide](https://www.oscars.org/sites/oscars/files/complete_sound_and_music_activities_guide.pdf)

Cornell describes a female mallard's quacks as a phrase that diminishes in
loudness. Dot uses two closing quacks from a real recording. Their nasal, textured
attack remains, with filtering and short fades to soften the edges.
[Cornell Mallard sound guide](https://www.allaboutbirds.org/guide/Mallard/sounds)
and [Audubon Mallard guide](https://www.audubon.org/field-guide/bird/mallard).

Cornell describes the collared-dove's short–long–short coo and distinguishes it
from the excited or alarmed call. Pearl uses one intact three-part coo.
[Cornell collared-dove sound guide](https://www.allaboutbirds.org/guide/Eurasian_Collared-Dove/sounds).

The RSPCA describes happy rabbit jumping and explains that thumping, grunting,
and loud tooth grinding can occur with fear or discomfort. Coco's greeting
therefore uses two light paw landings. A real rabbit-thump recording was located
and kept as a research reference, while the final landing texture comes from
cloth Foley. [RSPCA rabbit behaviour](https://science.rspca.org.uk/en/web/rspca/adviceandwelfare/pets/rabbits/behaviour)
and [kessir's rabbit-thump recording, CC0](https://freesound.org/people/kessir/sounds/386007/).

## Timing and mix

| Friend | Treatment | Placement |
| --- | --- | --- |
| Dot | Two edited field-recorded quacks | 0.16 and 0.77 seconds after the performance starts |
| Pearl | One complete recorded coo | Begins 0.08 seconds into the performance and ends at 1.46 seconds |
| Coco | Two separate cloth contacts | Atlas frames 0 and 6, every 0.75 seconds during the hopping performance |

The performance starts 4.3 seconds into the visit, once the doors are open and
the visitor has arrived. Coco's contact sequence repeats with the 1.5-second
animation cycle and stops before the turn. Reduced motion plays a short
two-contact greeting. Master volume, quiet hours, silent mode, and Zen mode use
the existing settings.

Only these three character sounds are part of this study. The shared opening
and closing door effects and bells still use the existing mix, so solo previews
are useful for assessing the character sound itself.

## Sources and audio licensing

**Dot.** Jonathon Jongsma, *Mallard (Anas platyrhynchos), XC62258*, recorded
14 September 2010. Adapted by selecting two quacks, spacing, filtering, fades,
and gain. Original recording and adapted Dot audio use
[CC BY-SA 3.0](https://creativecommons.org/licenses/by-sa/3.0/).
[Original source](https://xeno-canto.org/62258) and
[Wikimedia copy](https://commons.wikimedia.org/wiki/File:Anas_platyrhynchos_-_Mallard_-_XC62258.ogg).

**Pearl.** Jonathon Jongsma, *Eurasian Collared Dove (Streptopelia decaocto),
XC82758*, recorded 3 July 2011. Adapted by selecting a phrase, filtering,
fades, and gain. Original recording and adapted Pearl audio use
[CC BY-SA 3.0](https://creativecommons.org/licenses/by-sa/3.0/).
[Original source](https://xeno-canto.org/82758) and
[Wikimedia license record](https://commons.wikimedia.org/wiki/File:Streptopelia_decaocto_-_Eurasian_Collared_Dove_-_XC82758.ogg).

**Coco.** PelicanPolice, *Fabric flaps*, recorded 22 July 2021. The source is
recorded T-shirt movement. Adapted by selecting two contacts, filtering,
fades, gain, and synchronization. Original and Coco audio use
[CC0 1.0](https://creativecommons.org/publicdomain/zero/1.0/).
[Source recording and license](https://freesound.org/people/PelicanPolice/sounds/580967/).

## Reproduce and audition

From the repository root, with Rust, uv, and ffmpeg installed:

```powershell
uv run --with numpy --with scipy python tools/prepare_foley_studies.py
cargo test --locked --release --lib export_audition_tracks -- --ignored --nocapture
```

The preparation script writes isolated studies and measurements under
`artifacts/foley-studies`. The Rust exporter writes the actual performance
stems (`dot-solo.wav`, `pearl-solo.wav`, and `coco-solo.wav`), full visit WAVs,
and timeline CSVs under `artifacts/audio-qa`. Coco's Rust stem includes the
repeated contacts used by the running clock.

## Verification on Windows

The 17 automated tests passed, including sample boundaries, silence and Zen
behavior, contact timing, and headroom. All three were previewed through the
native visitor gallery in the packaged Windows app. Output-only WASAPI captures
matched the first 12.7 seconds of their reference visit mixes with correlations
above 0.999998. The captures reported no audio status flags, and the saved
settings were unchanged. This checks playback and timing. The sound studies
still need Nick's listening review for character, softness, and preference.
