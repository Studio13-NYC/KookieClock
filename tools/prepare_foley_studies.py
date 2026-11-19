"""Prepare Dot, Pearl, and Coco from the credited recordings in assets/audio/foley.

Run with ffmpeg, numpy, and scipy installed:
    uv run --with numpy --with scipy python tools/prepare_foley_studies.py

No pitching, time stretching, oscillators, or network requests. All edits are
reproducible. Audition files and measurements go in artifacts/foley-studies.
"""
from pathlib import Path
import hashlib
import json
import subprocess

import numpy as np
from scipy import signal
from scipy.io import wavfile

ROOT = Path(__file__).resolve().parents[1]
ASSETS = ROOT / "assets/audio/foley"
OUT = ROOT / "artifacts/foley-studies"
RATE = 24000


def read(name):
    result = subprocess.run(
        ["ffmpeg", "-hide_banner", "-loglevel", "error", "-i", str(ASSETS / name),
         "-ac", "1", "-ar", str(RATE), "-f", "f32le", "-"],
        check=True, capture_output=True,
    )
    return np.frombuffer(result.stdout, dtype="<f4").astype(np.float64)


def cut(samples, start, end, low, high, attack, release):
    clip = samples[round(start * RATE):round(end * RATE)].copy()
    clip = signal.sosfiltfilt(
        signal.butter(2, [low, high], btype="bandpass", fs=RATE, output="sos"), clip,
    )
    clip -= clip.mean()
    for seconds, tail in [(attack, False), (release, True)]:
        n = min(len(clip) // 2, round(seconds * RATE))
        ramp = np.sin(np.linspace(0, np.pi / 2, n)) ** 2
        if tail:
            clip[-n:] *= ramp[::-1]
        else:
            clip[:n] *= ramp
    return clip


def level(samples, peak, rms):
    gain = min(peak / max(abs(samples).max(), 1e-9),
               rms / max(np.sqrt(np.mean(samples ** 2)), 1e-9))
    return samples * gain


def phrase(seconds, events):
    samples = np.zeros(round(seconds * RATE))
    for at, clip, gain in events:
        offset = round(at * RATE)
        samples[offset:offset + len(clip)] += clip * gain
    return samples


def save(name, samples):
    pcm = np.rint(samples * 32767).astype("<i2")
    (ASSETS / f"{name}.pcm").write_bytes(pcm.tobytes())
    wavfile.write(OUT / f"{name}.wav", RATE, pcm)
    return {
        "seconds": len(pcm) / RATE,
        "peak_dbfs": float(20 * np.log10(max(abs(samples).max(), 1e-9))),
        "rms_dbfs": float(20 * np.log10(max(np.sqrt(np.mean(samples ** 2)), 1e-9))),
        "sha256": hashlib.sha256(pcm.tobytes()).hexdigest(),
    }


def main():
    OUT.mkdir(parents=True, exist_ok=True)
    duck = read("mallard-xc62258.ogg")
    dove = read("collared-dove-xc82758.mp3")
    fabric = read("fabric-flaps-580967.mp3")

    # Use the two quiet closing quacks of a real decrescendo, not the loud lead.
    quack_a = cut(duck, 2.075, 2.38, 180, 4300, .014, .045)
    quack_b = cut(duck, 2.445, 2.635, 180, 4300, .012, .040)
    dot = level(phrase(1.45, [(.16, quack_a, 1), (.77, quack_b, .90)]), .36, .075)

    # Keep an intact short–long–short coo, including its breath and natural pitch.
    pearl = level(cut(dove, .78, 2.16, 240, 1350, .055, .10), .30, .09)
    pearl = phrase(1.55, [(.08, pearl, 1)])

    # Two distinct cloth contacts give the rabbit's feet weight and softness.
    # These are movement Foley, not a claim that a rabbit vocalizes this way.
    paw_a = level(cut(fabric, 1.81, 2.04, 100, 2300, .012, .065), .22, .047)
    paw_b = level(cut(fabric, 5.72, 5.97, 100, 2300, .014, .075), .20, .043)
    # The atlas lands on frames 0 and 6 of each twelve-frame/1.5-second cycle.
    coco = phrase(1.5, [(0, paw_a, .68), (.75, paw_b, 1)])
    stats = {
        "dot": save("dot", dot),
        "pearl": save("pearl", pearl),
        "coco-paw-a": save("coco-paw-a", paw_a),
        "coco-paw-b": save("coco-paw-b", paw_b),
        "coco": save("coco", coco),
    }
    reel = np.concatenate([dot, np.zeros(RATE), pearl, np.zeros(RATE),
                           np.tile(coco, 3), np.zeros(RATE // 2)])
    wavfile.write(OUT / "three-studies.wav", RATE, np.rint(reel * 32767).astype("<i2"))
    (OUT / "measurements.json").write_text(json.dumps(stats, indent=2) + "\n", encoding="utf-8")
    print(json.dumps(stats, indent=2))


if __name__ == "__main__":
    main()
