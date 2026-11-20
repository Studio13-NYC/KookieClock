"""Build the remaining 21 original-speed recorded voices and movement Foley.

    uv run --with numpy --with scipy python tools/prepare_cast_audio.py

The approved Dot, Pearl and Coco assets are not rewritten. Each edit and source
is in assets/audio/foley/cast/edits.json and provenance.json. No network access,
pitch shifting, time stretching, oscillators or generative audio are used.
"""
from pathlib import Path
import hashlib
import json

import numpy as np
from scipy.io import wavfile

from prepare_foley_studies import RATE, read, cut, level, phrase

ROOT = Path(__file__).resolve().parents[1]
ASSETS = ROOT / "assets/audio/foley/cast"
OUT = ROOT / "artifacts/cast-audio"


def main():
    OUT.mkdir(parents=True, exist_ok=True)
    sources = json.loads((ASSETS / "provenance.json").read_text(encoding="utf-8"))["sources"]
    decoded = {}
    for name, source in sources.items():
        path = ASSETS / "sources" / source["downloaded"]
        if hashlib.sha256(path.read_bytes()).hexdigest() != source["sha256"]:
            raise ValueError(f"Source checksum changed: {path}")
        decoded[name] = read(path)
    edits = json.loads((ASSETS / "edits.json").read_text(encoding="utf-8"))
    stats = {}
    for edit in edits:
        events = []
        for event in edit["events"]:
            clip = cut(decoded[event["source"]], event["start"], event["end"],
                       event["low"], event["high"], event["attack"], event["release"])
            # Level separate prop takes before placement so accidental source
            # peaks cannot overpower a softer material in the same performance.
            clip = level(clip, 1, .25)
            events.append((event["at"], clip, event["gain"]))
        samples = level(phrase(1.5, events), edit["peak"], edit["rms"])
        pcm = np.rint(samples * 32767).astype("<i2")
        (ASSETS / f'{edit["name"]}.pcm').write_bytes(pcm.tobytes())
        wavfile.write(OUT / f'{edit["name"]}.wav', RATE, pcm)
        stats[edit["name"]] = {
            "seconds": len(samples) / RATE,
            "peak_dbfs": round(float(20 * np.log10(max(abs(samples).max(), 1e-9))), 2),
            "rms_dbfs": round(float(20 * np.log10(max(np.sqrt(np.mean(samples**2)), 1e-9))), 2),
            "sha256": hashlib.sha256(pcm.tobytes()).hexdigest(),
        }
    (OUT / "measurements.json").write_text(json.dumps(stats, indent=2) + "\n", encoding="utf-8")
    print(json.dumps(stats, indent=2))


if __name__ == "__main__":
    main()
