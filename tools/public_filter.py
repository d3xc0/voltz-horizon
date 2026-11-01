#!/usr/bin/env python3
"""Turns the generated content mod into the public one: drops what may not be
published yet (IndustrialCraft 2 and Railcraft remakes, and mods whose licence
is still being checked), every recipe and tab entry that names them, and
renames the few IC2-specific names on generic machines. Run after
gen_content.py, without the jar-based steps (the public build uses only the
textures gen_content.py draws)."""
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent / "content" / "voltz"
NS = "voltz"
# Nothing is held back any more: everything in the public build is our own
# work (new code, drawn art, generic names). Brand-specific names are renamed.
EXCLUDE: set = set()
RENAME = {"crusher": "Crusher", "mfe": "Energy Storage Unit", "mfsu": "Large Energy Storage Unit",
          "fission_reactor": "Fission Reactor Cell"}


def main():
    gone = set()
    for kind in ("item", "block"):
        for p in (ROOT / "data" / NS / "horizon" / kind).glob("*.json"):
            if p.stem in EXCLUDE:
                p.unlink()
                gone.add(p.stem)
    for p in list((ROOT / "assets" / NS).rglob("*")):
        if p.is_file() and any(p.stem == x or p.stem.startswith(x + "_") for x in gone) and "textures" not in p.parts[-2:]:
            p.unlink()
    for p in (ROOT / "data" / NS / "loot_table" / "blocks").glob("*.json"):
        if p.stem in gone:
            p.unlink()
    ids = {f"{NS}:{x}" for x in gone}
    dropped = 0
    for p in list((ROOT / "data" / NS / "recipe").rglob("*.json")):
        if any(i in p.read_text() for i in ids):
            p.unlink()
            dropped += 1
    for p in (ROOT / "data" / NS / "horizon" / "creative_tab").glob("*.json"):
        d = json.loads(p.read_text())
        d["items"] = [i for i in d["items"] if i not in ids]
        if not d["items"]:
            p.unlink()
            continue
        if d.get("icon") in ids:
            d["icon"] = d["items"][0]
        p.write_text(json.dumps(d, indent=2) + "\n")
    lang_p = ROOT / "assets" / NS / "lang" / "en_us.json"
    lang = json.loads(lang_p.read_text())
    lang = {k: v for k, v in lang.items() if k.split(".")[-1] not in gone}
    for iid, name in RENAME.items():
        for k in (f"block.{NS}.{iid}", f"item.{NS}.{iid}"):
            if k in lang:
                lang[k] = name
    # Tabs are named for what they hold, not for the mods that inspired them.
    for tab, title in {"industrialcraft": "Voltz Industrial", "railcraft": "Voltz Railworks",
                       "atomicscience": "Voltz Atomics", "mffs": "Voltz Force Fields"}.items():
        if f"itemGroup.{NS}.{tab}" in lang:
            lang[f"itemGroup.{NS}.{tab}"] = title
    lang_p.write_text(json.dumps(lang, indent=2) + "\n")
    tags = ROOT / "data" / "minecraft" / "tags" / "block"
    for p in tags.rglob("*.json"):
        d = json.loads(p.read_text())
        d["values"] = [v for v in d["values"] if v not in ids]
        p.write_text(json.dumps(d, indent=2) + "\n")
    print(f"public: removed {len(gone)} items/blocks, {dropped} recipes")


main()
