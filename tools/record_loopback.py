"""Record the default Windows output for clock QA; never opens a microphone.

Optional developer dependency: PyAudioWPatch (not shipped with Little Timber).
python tools/record_loopback.py artifacts/audio-qa/live.wav --seconds 60
"""
import argparse
import datetime
import json
from pathlib import Path
import sys
import time
import wave

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "artifacts/qa-python"))
import pyaudiowpatch as pa

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("output", type=Path)
parser.add_argument("--seconds", type=float, default=60)
args = parser.parse_args()
args.output.parent.mkdir(parents=True, exist_ok=True)
with pa.PyAudio() as p:
    api = p.get_host_api_info_by_type(pa.paWASAPI)
    default = p.get_device_info_by_index(api["defaultOutputDevice"])
    device = next(d for d in p.get_loopback_device_info_generator()
                  if default["name"] in d["name"])
    frames = []
    statuses = []
    def callback(data, count, timing, status):
        frames.append(data)
        if status:
            statuses.append(status)
        return None, pa.paContinue
    rate = int(device["defaultSampleRate"])
    channels = int(device["maxInputChannels"])
    started = datetime.datetime.now(datetime.timezone.utc).isoformat()
    with p.open(format=pa.paInt16, channels=channels, rate=rate, input=True,
                input_device_index=device["index"], frames_per_buffer=512,
                stream_callback=callback) as stream:
        print(f"Capturing output only: {device['name']}", flush=True)
        end = time.monotonic() + args.seconds
        while time.monotonic() < end:
            time.sleep(0.1)
        stream.stop_stream()
    with wave.open(str(args.output), "wb") as wav:
        wav.setnchannels(channels)
        wav.setsampwidth(2)
        wav.setframerate(rate)
        wav.writeframes(b"".join(frames))
    metadata = dict(started_utc=started, device=device["name"], rate=rate,
                    channels=channels, status_flags=statuses,
                    captured_seconds=sum(map(len, frames))/(2*channels*rate))
    args.output.with_suffix(".json").write_text(json.dumps(metadata, indent=2))
    print(json.dumps(metadata), flush=True)
