"""Prepare the three generated voices for offline, sample-accurate clock playback.

Requires ffmpeg, numpy, scipy. Sources and generation provenance live in
assets/audio/source and assets/audio/provenance.json. No API calls are made.
"""
from pathlib import Path
import json
import subprocess
import numpy as np
from scipy import signal
from scipy.io import wavfile

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / "assets/audio"
QA = ROOT / "artifacts/natural-audio"
RATE = 24000
QA.mkdir(parents=True, exist_ok=True)

def source(name):
    path = QA / f"{name}-source.wav"
    subprocess.run(["ffmpeg", "-hide_banner", "-loglevel", "error", "-y", "-i",
                    str(OUT / "source" / f"{name}.mp3"), "-ac", "1", "-ar", str(RATE),
                    str(path)], check=True)
    _, samples = wavfile.read(path)
    return samples.astype(np.float64) / 32768

def soften(samples, low, high, attack=0.012, release=0.025):
    samples = signal.sosfiltfilt(signal.butter(2, [low, high], "bandpass", fs=RATE, output="sos"), samples)
    samples -= samples.mean()
    for length, reverse in [(attack, False), (release, True)]:
        n = min(len(samples)//2, round(length * RATE))
        ramp = np.sin(np.linspace(0, np.pi/2, n)) ** 2
        if reverse:
            samples[-n:] *= ramp[::-1]
        else:
            samples[:n] *= ramp
    return samples

stats = {}
def save(name, samples, peak_limit, rms_limit):
    peak = np.max(np.abs(samples))
    rms = np.sqrt(np.mean(samples**2))
    samples *= min(peak_limit / max(peak, 1e-9), rms_limit / max(rms, 1e-9))
    pcm = np.rint(samples * 32767).astype("<i2")
    (OUT / f"{name}.pcm").write_bytes(pcm.tobytes())
    wavfile.write(QA / f"{name}.wav", RATE, pcm)
    stats[name] = dict(seconds=len(pcm)/RATE, peak=float(np.max(np.abs(samples))),
                       rms=float(np.sqrt(np.mean(samples**2))))
    return samples

duck = source("duck")
phrase = np.zeros(round(0.76 * RATE))
for start, end, at, gain in [(0.18, 0.40, 0., 1.), (0.49, 0.72, 0.46, 0.76)]:
    quack = soften(duck[round(start*RATE):round(end*RATE)], 140, 3900)
    offset = round(at * RATE)
    phrase[offset:offset+len(quack)] += quack * gain
save("duck", phrase, 0.48, 0.12)

mouse = source("mouse")
for i, start in enumerate([0.035, 0.15, 0.27]):
    stroke = soften(mouse[round(start*RATE):round((start+0.11)*RATE)], 300, 5500,
                    attack=0.008, release=0.018)
    save(f"mouse-{i}", stroke, 0.24, 0.060)

bear = soften(source("bear")[:round(1.35*RATE)], 55, 1900, attack=0.075, release=0.17)
save("bear", bear, 0.42, 0.085)
(QA / "asset-levels.json").write_text(json.dumps(stats, indent=2))
print(json.dumps(stats, indent=2))
