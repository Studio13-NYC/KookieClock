"""Check that tracked repository text is UTF-8 without a byte-order mark."""
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]
EXTENSIONS = {".rs", ".md", ".toml", ".lock", ".json", ".yml", ".yaml",
              ".py", ".ps1", ".rc", ".manifest", ".txt", ".sh"}
NAMES = {".gitignore", ".gitattributes", ".editorconfig", "LICENSE"}
paths = subprocess.check_output(["git", "ls-files", "-z"], cwd=ROOT).decode("utf-8").split("\0")
checked = 0
errors = []
for name in filter(None, paths):
    path = ROOT / name
    if path.suffix not in EXTENSIONS and path.name not in NAMES:
        continue
    try:
        data = path.read_bytes()
        data.decode("utf-8")
        if data.startswith(b"\xef\xbb\xbf"):
            raise ValueError("UTF-8 byte-order mark; save without BOM")
        checked += 1
    except (UnicodeError, ValueError) as error:
        errors.append(f"{name}: {error}")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print(f"Verified {checked} tracked text files: UTF-8 without BOM.")
