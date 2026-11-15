"""Check baked toy frames before packaging. Requires Pillow; never edits assets."""
from pathlib import Path
from PIL import Image

root = Path(__file__).resolve().parents[1] / "assets" / "woodland"
tile = 192
for character in range(24):
    with Image.open(root / f"visitor-atlas-{character:02d}.png") as atlas:
        assert atlas.mode == "RGBA" and atlas.size == (tile * 5, tile * 5)
        frames = []
        for frame in range(25):
            x, y = frame % 5 * tile, frame // 5 * tile
            image = atlas.crop((x, y, x + tile, y + tile))
            alpha = image.getchannel("A")
            bounds = alpha.getbbox()
            assert bounds, (character, frame, "empty frame")
            assert alpha.getextrema()[1] == 255, (character, frame, "not opaque")
            assert bounds[0] > 0 and bounds[1] > 0 and bounds[2] < tile and bounds[3] < tile, (
                character, frame, "toy clipped by sprite boundary", bounds
            )
            frames.append(image.tobytes())
        assert len(set(frames[:12])) >= 5, (character, "missing articulation")
        assert frames[12] != frames[18] != frames[24], (character, "missing turn views")
print("Validated 24 articulated toys, 600 populated opaque poses, and all sprite bounds.")
