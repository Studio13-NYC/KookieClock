"""Build the original 32px toy-clock icon from geometry; no external packages."""
import struct
from pathlib import Path

n = 32
pixels = []
for y in range(n):
    for x in range(n):
        c = (0, 0, 0, 0)
        if 6 <= x <= 25 and 12 <= y <= 28:
            c = (225, 189, 143, 255)
        if 3 <= y <= 15 and abs(x - 15.5) < (y - 1) * 1.1:
            c = (94, 139, 143, 255)
        if (x - 15.5) ** 2 + (y - 21) ** 2 <= 41:
            c = (255, 243, 217, 255)
        if (15 <= x <= 16 and 16 <= y <= 22) or (15 <= x <= 20 and y == 21):
            c = (52, 75, 76, 255)
        pixels.append(bytes((c[2], c[1], c[0], c[3])))
raw = b"".join(b"".join(pixels[y*n:(y+1)*n]) for y in range(n-1, -1, -1))
mask = b"\0" * (4*n)
bitmap = struct.pack("<IiiHHIIIIII", 40, n, n*2, 1, 32, 0, len(raw), 0, 0, 0, 0) + raw + mask
ico = struct.pack("<HHH", 0, 1, 1) + struct.pack("<BBBBHHII", n, n, 0, 0, 1, 32, len(bitmap), 22) + bitmap
Path(__file__).resolve().parents[1].joinpath("assets/clock.ico").write_bytes(ico)
