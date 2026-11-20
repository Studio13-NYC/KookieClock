# Recorded character sounds

The original Dot, Pearl and Coco studies were prepared on 19 November 2026.
All 24 friends now use recorded voices or Foley; the remaining 21 are in
[`cast/`](cast/), with exact edits and source provenance. The original recordings
and their prepared PCM derivatives are identified below. Preparation is
reproducible with `tools/prepare_foley_studies.py` and uses original speed and
pitch. The PCM format is mono, 24 kHz, signed 16-bit little-endian.

## Dot the duck

- Original recording, `mallard-xc62258.ogg`, **Mallard (Anas platyrhynchos), XC62258**, by **Jonathon Jongsma**, 14 September 2010.
- [Source recording](https://xeno-canto.org/62258) and [Wikimedia copy](https://commons.wikimedia.org/wiki/File:Anas_platyrhynchos_-_Mallard_-_XC62258.ogg).
- Original and adapted `dot.pcm` are licensed **CC BY-SA 3.0**. [License](https://creativecommons.org/licenses/by-sa/3.0/)
- Changes made for KookieClock: selected the last two quacks, separated them, reduced background with gentle filtering, applied short fades, and adjusted gain.

## Pearl the dove

- Original recording, `collared-dove-xc82758.mp3`, **Eurasian Collared Dove (Streptopelia decaocto), XC82758**, by **Jonathon Jongsma**, 3 July 2011.
- [Source recording](https://xeno-canto.org/82758) and [Wikimedia license record](https://commons.wikimedia.org/wiki/File:Streptopelia_decaocto_-_Eurasian_Collared_Dove_-_XC82758.ogg).
- Original and adapted `pearl.pcm` are licensed **CC BY-SA 3.0**. [License](https://creativecommons.org/licenses/by-sa/3.0/)
- Changes made for KookieClock: selected one complete three-part coo, filtered background, applied fades, and adjusted gain.

## Coco the rabbit

- Original recording, `fabric-flaps-580967.mp3`, **Fabric flaps**, by **PelicanPolice**, 22 July 2021. The source is a T-shirt being shaken, recorded on an iPhone. The downloaded file is the site's high-quality MP3 preview.
- [Source and license](https://freesound.org/people/PelicanPolice/sounds/580967/).
- Original and adapted `coco.pcm`, `coco-paw-a.pcm`, and `coco-paw-b.pcm` use **CC0 1.0**. [Public-domain dedication](https://creativecommons.org/publicdomain/zero/1.0/)
- Changes made for KookieClock: selected two different contacts, filtered and softened them, and placed them at rabbit landing phases. This is prop Foley used for feet touching down.

The CC BY-SA license applies to the specified recordings and their audio
adaptations. The credits do not claim an endorsement by the recordists.
The research and timing notes are in [the sound study](../../../docs/sound-studies.md).
