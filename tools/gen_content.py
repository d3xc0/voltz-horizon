#!/usr/bin/env python3
"""Generates the Voltz Core content mod (content/voltz/): blocks, items,
recipes, loot, tags, models, lang and original 16x16 textures.
Run from anywhere; rewrites content/voltz/ whole. The textures are drawn here
(seeded noise + shapes), so nothing comes from Minecraft or the Voltz mods."""
import json, random, shutil
from pathlib import Path
from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parent.parent / "content" / "voltz"
NS = "voltz"

MACHINES = {
    "coal_generator": ("Coal Generator", (90, 90, 96), (230, 120, 40)),
    "battery_box": ("Battery Box", (70, 80, 100), (80, 200, 255)),
    "electric_furnace": ("Electric Furnace", (150, 150, 160), (255, 170, 60)),
    "crusher": ("Macerator", (120, 110, 100), (200, 200, 210)),
    "geothermal_generator": ("Geothermal Generator", (110, 70, 60), (255, 110, 30)),
    "solar_panel": ("Solar Panel", (40, 60, 110), (120, 180, 255)),
    "mfe": ("MFE", (90, 100, 120), (100, 220, 255)),
    "mfsu": ("MFSU", (60, 60, 70), (180, 120, 255)),
    "compressor": ("Compressor", (130, 130, 140), (220, 220, 230)),
    "extractor": ("Extractor", (120, 130, 110), (180, 230, 140)),
    # Mekanism
    "heat_generator": ("Heat Generator", (120, 80, 60), (255, 140, 40)),
    "wind_generator": ("Wind Generator", (170, 180, 190), (200, 240, 255)),
    "basic_energy_cube": ("Basic Energy Cube", (90, 90, 90), (220, 220, 220)),
    "advanced_energy_cube": ("Advanced Energy Cube", (110, 40, 40), (255, 80, 80)),
    "elite_energy_cube": ("Elite Energy Cube", (40, 70, 120), (80, 160, 255)),
    "ultimate_energy_cube": ("Ultimate Energy Cube", (80, 40, 110), (190, 110, 255)),
    "enrichment_chamber": ("Enrichment Chamber", (150, 150, 160), (120, 220, 120)),
    "mekanism_crusher": ("Crusher", (150, 150, 160), (220, 120, 120)),
    "energized_smelter": ("Energized Smelter", (150, 150, 160), (255, 160, 60)),
    "osmium_compressor": ("Osmium Compressor", (150, 150, 160), (120, 160, 255)),
    "metallurgic_infuser": ("Metallurgic Infuser", (150, 150, 160), (255, 220, 80)),
    # Ender IO
    "alloy_smelter": ("Alloy Smelter", (70, 70, 75), (255, 150, 50)),
    "sag_mill": ("SAG Mill", (70, 70, 75), (200, 200, 210)),
    "slice_and_splice": ("Slice'N'Splice", (70, 70, 75), (220, 60, 60)),
    "soul_binder": ("Soul Binder", (60, 50, 45), (120, 255, 200)),
    "stirling_generator": ("Stirling Generator", (70, 70, 75), (255, 120, 30)),
    "basic_capacitor_bank": ("Basic Capacitor Bank", (90, 90, 95), (200, 60, 60)),
    "capacitor_bank": ("Capacitor Bank", (90, 90, 95), (60, 200, 60)),
    "vibrant_capacitor_bank": ("Vibrant Capacitor Bank", (90, 90, 95), (120, 255, 120)),
    # Atomic Science
    "chemical_extractor": ("Chemical Extractor", (120, 130, 120), (200, 255, 80)),
    "centrifuge": ("Centrifuge", (120, 130, 120), (120, 255, 120)),
    "fission_reactor": ("Fission Reactor Cell", (70, 90, 70), (100, 255, 100)),
    # Tinkers' Construct, Railcraft
    "smeltery": ("Smeltery Controller", (70, 60, 55), (255, 120, 30)),
    "coke_oven": ("Coke Oven", (150, 80, 60), (255, 140, 40)),
    "blast_furnace": ("Blast Furnace", (110, 100, 90), (255, 100, 30)),
}
EXPLOSIVES = {
    "condensed_explosive": ("Condensed Explosive", (200, 40, 40)),
    "incendiary_explosive": ("Incendiary Explosive", (255, 120, 20)),
    "shrapnel_explosive": ("Shrapnel Explosive", (170, 170, 180)),
    "breaching_explosive": ("Breaching Explosive", (60, 140, 220)),
    "fragmentation_explosive": ("Fragmentation Explosive", (140, 140, 150)),
    "thermobaric_explosive": ("Thermobaric Explosive", (230, 90, 30)),
    "exothermic_explosive": ("Exothermic Explosive", (255, 60, 0)),
    "endothermic_explosive": ("Endothermic Explosive", (120, 200, 255)),
    "sonic_explosive": ("Sonic Explosive", (200, 200, 60)),
    "nuclear_explosive": ("Nuclear Explosive", (80, 220, 60)),
    "smoke_explosive": ("Smoke Explosive", (90, 90, 90)),
    "chemical_explosive": ("Chemical Explosive", (120, 200, 40)),
    "debilitation_explosive": ("Debilitation Explosive", (150, 90, 180)),
    "contagious_explosive": ("Contagious Explosive", (90, 130, 40)),
    "emp_explosive": ("EMP Explosive", (60, 120, 255)),
    "anvil_explosive": ("Anvil Explosive", (70, 70, 80)),
}
DUSTS = {"iron_dust": ("Iron Dust", (215, 205, 195)), "gold_dust": ("Gold Dust", (250, 210, 70)),
         "copper_dust": ("Copper Dust", (225, 120, 80)),
         # Mekanism materials (drawn as a heap; the import swaps in Mekanism's own)
         "osmium_dust": ("Osmium Dust", (150, 170, 200)), "osmium_ingot": ("Osmium Ingot", (160, 180, 210)),
         "steel_dust": ("Steel Dust", (120, 120, 125)), "steel_ingot": ("Steel Ingot", (110, 110, 118)),
         "obsidian_dust": ("Obsidian Dust", (60, 40, 90)), "diamond_dust": ("Diamond Dust", (120, 230, 230)),
         "refined_obsidian_dust": ("Refined Obsidian Dust", (110, 70, 150)),
         "refined_obsidian_ingot": ("Refined Obsidian Ingot", (120, 80, 170)),
         "refined_glowstone_ingot": ("Refined Glowstone Ingot", (250, 220, 110)),
         "enriched_alloy": ("Enriched Alloy", (220, 60, 60)), "reinforced_alloy": ("Reinforced Alloy", (60, 200, 220)),
         "atomic_alloy": ("Atomic Alloy", (170, 60, 220)),
         "basic_control_circuit": ("Basic Control Circuit", (60, 180, 60)),
         "advanced_control_circuit": ("Advanced Control Circuit", (220, 60, 60)),
         "elite_control_circuit": ("Elite Control Circuit", (60, 120, 230)),
         "ultimate_control_circuit": ("Ultimate Control Circuit", (170, 60, 220)),
         # Ender IO
         "electrical_steel_ingot": ("Electrical Steel Ingot", (170, 170, 175)),
         "energetic_alloy_ingot": ("Energetic Alloy Ingot", (255, 170, 60)),
         "vibrant_alloy_ingot": ("Vibrant Alloy Ingot", (180, 240, 90)),
         "redstone_alloy_ingot": ("Redstone Alloy Ingot", (200, 40, 40)),
         "conductive_iron_ingot": ("Conductive Iron Ingot", (220, 160, 160)),
         "pulsating_iron_ingot": ("Pulsating Iron Ingot", (110, 220, 160)),
         "dark_steel_ingot": ("Dark Steel Ingot", (70, 70, 80)),
         "soularium_ingot": ("Soularium Ingot", (120, 90, 60)),
         "end_steel_ingot": ("End Steel Ingot", (230, 230, 180)),
         "basic_capacitor": ("Basic Capacitor", (90, 160, 90)),
         "double_layer_capacitor": ("Double-Layer Capacitor", (90, 90, 200)),
         "octadic_capacitor": ("Octadic Capacitor", (200, 90, 200)),
         "machine_chassis": ("Machine Chassis", (90, 90, 95)),
         # Atomic Science
         "yellow_cake": ("Yellowcake", (230, 220, 60)), "uranium_238": ("Uranium-238", (80, 140, 60)),
         "uranium_235": ("Uranium-235", (120, 230, 80)), "empty_cell": ("Empty Cell", (200, 210, 220)),
         "water_cell": ("Water Cell", (80, 120, 230)), "fissile_fuel_cell": ("Fissile Fuel Cell", (120, 230, 80)),
         "toxic_waste": ("Toxic Waste", (90, 110, 40)),
         # Tinkers' Construct
         "cobalt_ingot": ("Cobalt Ingot", (40, 110, 230)), "ardite_ingot": ("Ardite Ingot", (220, 110, 30)),
         "manyullyn_ingot": ("Manyullyn Ingot", (160, 80, 200)), "blank_pattern": ("Blank Pattern", (200, 170, 110)),
         "tool_rod": ("Tool Rod", (140, 100, 60)), "binding": ("Binding", (140, 100, 60)),
         # Railcraft
         "coal_coke": ("Coal Coke", (60, 60, 60))}
CHESTS = {"copper_chest": ("Copper Chest", (190, 110, 60)), "iron_chest": ("Iron Chest", (200, 200, 205)),
          "gold_chest": ("Gold Chest", (240, 200, 60)), "diamond_chest": ("Diamond Chest", (100, 220, 220)),
          "crystal_chest": ("Crystal Chest", (220, 240, 255)), "obsidian_chest": ("Obsidian Chest", (50, 30, 70))}
MODULES = {"cube_module": "Cube Module", "sphere_module": "Sphere Module", "wall_module": "Wall Module",
           "tube_module": "Tube Module", "scale_module": "Scale Module"}
AE_ITEMS = {"storage_cell_1k": "1k ME Storage Cell", "storage_cell_4k": "4k ME Storage Cell",
            "storage_cell_16k": "16k ME Storage Cell", "storage_cell_64k": "64k ME Storage Cell"}
# Tinkers' Construct: head material -> (name, colour, vanilla tier, durability, ingot)
TINKER_MATERIALS = {
    "stone": ("Stone", (130, 130, 130), "stone", 180, "minecraft:cobblestone"),
    "iron": ("Iron", (210, 210, 215), "iron", 350, "minecraft:iron_ingot"),
    "cobalt": ("Cobalt", (40, 110, 230), "diamond", 900, "voltz:cobalt_ingot"),
    "ardite": ("Ardite", (220, 110, 30), "diamond", 1100, "voltz:ardite_ingot"),
    "manyullyn": ("Manyullyn", (160, 80, 200), "netherite", 2200, "voltz:manyullyn_ingot"),
}
TINKER_TOOLS = {"pickaxe": ("Pickaxe", "pickaxe_head", "Pickaxe Head"), "hatchet": ("Hatchet", "axe_head", "Axe Head"),
                "shovel": ("Shovel", "shovel_head", "Shovel Head"), "broadsword": ("Broadsword", "sword_blade", "Sword Blade")}
VANILLA_KIND = {"pickaxe": "pickaxe", "hatchet": "axe", "shovel": "shovel", "broadsword": "sword"}
PIPES = {"wooden_pipe": "Wooden Transport Pipe", "cobblestone_pipe": "Cobblestone Transport Pipe",
         "stone_pipe": "Stone Transport Pipe", "gold_pipe": "Gold Transport Pipe"}
CONDUITS = {"energy_conduit": ("Energy Conduit", "voltz:conductive_iron_ingot"),
            "enhanced_energy_conduit": ("Enhanced Energy Conduit", "voltz:energetic_alloy_ingot"),
            "ender_energy_conduit": ("Ender Energy Conduit", "voltz:vibrant_alloy_ingot")}
ORES = {"osmium_ore": ("Osmium Ore", "minecraft:iron_ore", (150, 170, 200)),
        "deepslate_osmium_ore": ("Deepslate Osmium Ore", "minecraft:deepslate_iron_ore", (150, 170, 200)),
        "uranium_ore": ("Uranium Ore", "minecraft:diamond_ore", (90, 220, 60)),
        "cobalt_ore": ("Cobalt Ore", "minecraft:nether_gold_ore", (40, 110, 230)),
        "ardite_ore": ("Ardite Ore", "minecraft:nether_gold_ore", (220, 110, 30)),
        "deepslate_uranium_ore": ("Deepslate Uranium Ore", "minecraft:deepslate_diamond_ore", (90, 220, 60))}
# Mekanism Tools: material -> (name, tool template tier, durability, repair ingot, texture prefix in the jar)
TOOL_MATERIALS = {
    "osmium": ("Osmium", "iron", 500, "voltz:osmium_ingot", "osmium"),
    "steel": ("Steel", "iron", 750, "voltz:steel_ingot", "steel"),
    "refined_obsidian": ("Refined Obsidian", "diamond", 2500, "voltz:refined_obsidian_ingot", "obsidian"),
    "refined_glowstone": ("Refined Glowstone", "diamond", 1200, "voltz:refined_glowstone_ingot", "glowstone"),
    "lapis": ("Lapis Lazuli", "iron", 250, "minecraft:lapis_lazuli", "lapislazuli"),
    "dark_steel": ("Dark Steel", "diamond", 2000, "voltz:dark_steel_ingot", "enderio"),
}
# Kinds a material doesn't have (Ender IO's dark steel: no shovel or hoe).
TOOL_SKIP = {"dark_steel": {"shovel", "hoe"}}
TOOLS = {"pickaxe": ["XXX", " S ", " S "], "axe": ["XX", "XS", " S"], "shovel": ["X", "S", "S"],
         "hoe": ["XX", " S", " S"], "sword": ["X", "X", "S"]}
ARMOR = {"helmet": ["XXX", "X X"], "chestplate": ["X X", "XXX", "XXX"], "leggings": ["XXX", "X X", "X X"],
         "boots": ["X X", "X X"]}


def w(path, obj):
    p = ROOT / path
    p.parent.mkdir(parents=True, exist_ok=True)
    p.write_text(json.dumps(obj, indent=2) + "\n")


def noise(img, base, spread, seed):
    rnd = random.Random(seed)
    px = img.load()
    for y in range(16):
        for x in range(16):
            d = rnd.randint(-spread, spread)
            px[x, y] = tuple(max(0, min(255, c + d)) for c in base) + (255,)


def plate(base, seed):
    img = Image.new("RGBA", (16, 16))
    noise(img, base, 8, seed)
    d = ImageDraw.Draw(img)
    dark = tuple(max(0, c - 45) for c in base)
    light = tuple(min(255, c + 35) for c in base)
    d.rectangle([0, 0, 15, 15], outline=dark)
    d.line([1, 1, 14, 1], fill=light)
    d.line([1, 1, 1, 14], fill=light)
    for p in [(2, 2), (13, 2), (2, 13), (13, 13)]:
        d.point(p, fill=light)
    return img


def save(img, rel):
    p = ROOT / "assets" / NS / "textures" / rel
    p.parent.mkdir(parents=True, exist_ok=True)
    img.save(p)


def textures():
    for i, (mid, (_, base, glow)) in enumerate(MACHINES.items()):
        save(plate(base, 10 + i), f"block/{mid}_side.png")
        top = plate(tuple(c + 15 for c in base), 20 + i)
        d = ImageDraw.Draw(top)
        d.rectangle([5, 5, 10, 10], outline=(40, 40, 40))
        d.rectangle([6, 6, 9, 9], fill=(200, 120, 60))  # copper terminal
        save(top, f"block/{mid}_top.png")
        save(plate(tuple(c - 10 for c in base), 40 + i), f"block/{mid}_bottom.png")
        for lit in (False, True):
            f = plate(base, 30 + i)
            d = ImageDraw.Draw(f)
            d.rectangle([3, 3, 12, 12], fill=(30, 30, 34), outline=(20, 20, 20))
            col = glow if lit else (60, 60, 66)
            if mid == "battery_box":
                for k in range(4):
                    d.rectangle([5, 10 - 2 * k, 10, 10 - 2 * k], fill=col if (lit or k == 0) else (60, 60, 66))
            elif mid == "crusher":
                for k in range(3):
                    d.line([4 + 3 * k, 5, 6 + 3 * k, 10], fill=col)
            else:
                d.rectangle([5, 7, 10, 10], fill=col)
                d.line([5, 5, 10, 5], fill=(110, 110, 120))
            save(f, f"block/{mid}_front{'_on' if lit else ''}.png")
    for i, (eid, (_, accent)) in enumerate(EXPLOSIVES.items()):
        for armed in (False, True):
            img = Image.new("RGBA", (16, 16))
            noise(img, (60, 55, 50), 6, 70 + i)
            d = ImageDraw.Draw(img)
            for k in range(-16, 16, 4):  # hazard stripes, top and bottom bands
                d.line([k, 0, k + 3, 3], fill=(230, 200, 30))
                d.line([k, 12, k + 3, 15], fill=(230, 200, 30))
            d.rectangle([2, 5, 13, 10], fill=accent, outline=(25, 25, 25))
            d.rectangle([6, 6, 9, 9], fill=(255, 240, 120) if armed else (35, 35, 35))
            save(img, f"block/{eid}{'_armed' if armed else ''}.png")
        top = Image.new("RGBA", (16, 16))
        noise(top, (70, 65, 60), 6, 90 + i)
        d = ImageDraw.Draw(top)
        d.ellipse([4, 4, 11, 11], fill=accent, outline=(25, 25, 25))
        save(top, f"block/{eid}_top.png")
    wire = Image.new("RGBA", (16, 16))
    noise(wire, (200, 110, 70), 14, 99)
    d = ImageDraw.Draw(wire)
    for x in range(0, 16, 4):
        d.line([x, 0, x, 15], fill=(160, 80, 50))
    save(wire, "block/copper_wire.png")
    for i, (did, (_, col)) in enumerate(DUSTS.items()):
        img = Image.new("RGBA", (16, 16), (0, 0, 0, 0))
        rnd = random.Random(50 + i)
        px = img.load()
        for y in range(6, 14):
            for x in range(3, 13):
                if abs(x - 7.5) + (13 - y) * 0.9 < 6.5 and rnd.random() < 0.85:
                    s = rnd.randint(-30, 20)
                    px[x, y] = tuple(max(0, min(255, c + s)) for c in col) + (255,)
        save(img, f"item/{did}.png")


def machine(mid, name):
    w(f"data/{NS}/horizon/block/{mid}.json", {
        "template": "minecraft:furnace",
        "properties": {"hardness": 3.5, "explosion_resistance": 6.0, "sounds": "minecraft:iron_block",
                       "requires_tool": True, "light": 0},
        "item": True,
    })
    for lit in (False, True):
        sfx = "_on" if lit else ""
        w(f"assets/{NS}/models/block/{mid}{sfx}.json", {
            "parent": "minecraft:block/orientable_with_bottom",
            "textures": {"top": f"{NS}:block/{mid}_top", "bottom": f"{NS}:block/{mid}_bottom",
                         "side": f"{NS}:block/{mid}_side",
                         "front": f"{NS}:block/{mid}_front{sfx}"},
        })
    rot = {"north": 0, "east": 90, "south": 180, "west": 270}
    w(f"assets/{NS}/blockstates/{mid}.json", {"variants": {
        f"facing={f},lit={l}": {"model": f"{NS}:block/{mid}{'_on' if l == 'true' else ''}", **({"y": r} if r else {})}
        for f, r in rot.items() for l in ("false", "true")}})
    w(f"assets/{NS}/items/{mid}.json", {"model": {"type": "minecraft:model", "model": f"{NS}:block/{mid}"}})
    loot(mid)


def loot(bid):
    w(f"data/{NS}/loot_table/blocks/{bid}.json", {
        "type": "minecraft:block",
        "pools": [{"rolls": 1, "conditions": [{"condition": "minecraft:survives_explosion"}],
                   "entries": [{"type": "minecraft:item", "name": f"{NS}:{bid}"}]}],
        "random_sequence": f"{NS}:blocks/{bid}",
    })


def wire(wid="copper_wire"):
    # A thin cable along its axis (placed like a chain).
    w(f"data/{NS}/horizon/block/{wid}.json", {
        "template": "minecraft:iron_chain",
        "properties": {"hardness": 0.5, "sounds": "minecraft:white_wool", "requires_tool": False},
        "shapes": {"property": "axis", "values": {
            "x": [[0, 0.375, 0.375, 1, 0.625, 0.625]],
            "y": [[0.375, 0, 0.375, 0.625, 1, 0.625]],
            "z": [[0.375, 0.375, 0, 0.625, 0.625, 1]]}},
        "item": True,
    })
    w(f"assets/{NS}/models/block/{wid}.json", {
        "parent": "minecraft:block/block",
        "textures": {"particle": f"{NS}:block/{wid}", "wire": f"{NS}:block/{wid}"},
        "elements": [{"from": [6, 6, 0], "to": [10, 10, 16], "faces": {
            f: {"texture": "#wire", "uv": [0, 0, 16, 4] if f not in ("north", "south") else [6, 6, 10, 10]}
            for f in ("north", "south", "east", "west", "up", "down")}}],
    })
    w(f"assets/{NS}/blockstates/{wid}.json", {"variants": {
        "axis=z": {"model": f"{NS}:block/{wid}"},
        "axis=x": {"model": f"{NS}:block/{wid}", "y": 90},
        "axis=y": {"model": f"{NS}:block/{wid}", "x": 90}}})
    w(f"assets/{NS}/items/{wid}.json", {"model": {"type": "minecraft:model", "model": f"{NS}:block/{wid}"}})
    loot(wid)
    if wid != "copper_wire":
        save(Image.open(ROOT / "assets" / NS / "textures" / "block" / "copper_wire.png"), f"block/{wid}.png")


def explosive(eid):
    w(f"data/{NS}/horizon/block/{eid}.json", {
        "template": "minecraft:furnace",
        "properties": {"hardness": 0.5, "explosion_resistance": 0.0, "sounds": "minecraft:tnt",
                       "requires_tool": False, "light": 0},
        "item": True,
    })
    for armed in (False, True):
        sfx = "_armed" if armed else ""
        w(f"assets/{NS}/models/block/{eid}{sfx}.json", {
            "parent": "minecraft:block/cube_bottom_top",
            "textures": {"top": f"{NS}:block/{eid}_top", "bottom": f"{NS}:block/{eid}_top",
                         "side": f"{NS}:block/{eid}{sfx}"},
        })
    w(f"assets/{NS}/blockstates/{eid}.json", {"variants": {
        f"facing={f},lit={l}": {"model": f"{NS}:block/{eid}{'_armed' if l == 'true' else ''}"}
        for f in ("north", "east", "south", "west") for l in ("false", "true")}})
    w(f"assets/{NS}/items/{eid}.json", {"model": {"type": "minecraft:model", "model": f"{NS}:block/{eid}"}})
    loot(eid)


def ore(oid, name, template, col):
    w(f"data/{NS}/horizon/block/{oid}.json", {"template": template, "item": True})
    img = Image.new("RGBA", (16, 16))
    noise(img, (125, 125, 125) if "deepslate" not in oid else (70, 70, 78), 12, sum(map(ord, oid)))
    d = ImageDraw.Draw(img)
    rnd = random.Random(oid)
    for _ in range(6):
        x, y = rnd.randint(2, 12), rnd.randint(2, 12)
        d.rectangle([x, y, x + 2, y + 1], fill=col)
    save(img, f"block/{oid}.png")
    w(f"assets/{NS}/models/block/{oid}.json", {"parent": "minecraft:block/cube_all", "textures": {"all": f"{NS}:block/{oid}"}})
    w(f"assets/{NS}/blockstates/{oid}.json", {"variants": {"": {"model": f"{NS}:block/{oid}"}}})
    w(f"assets/{NS}/items/{oid}.json", {"model": {"type": "minecraft:model", "model": f"{NS}:block/{oid}"}})
    loot(oid)


def tools():
    lang = {}
    for mat, (mname, tier, dur, ingot, _) in TOOL_MATERIALS.items():
        for kind, pattern in {**TOOLS, **ARMOR}.items():
            if kind in TOOL_SKIP.get(mat, ()):
                continue
            iid = f"{mat}_{kind}"
            template = f"minecraft:{tier}_{kind}"
            w(f"data/{NS}/horizon/item/{iid}.json", {
                "template": template,
                "components": {"minecraft:max_damage": dur if kind in TOOLS else dur // 4 * {"helmet": 11, "chestplate": 16, "leggings": 15, "boots": 13}[kind] // 10,
                               "minecraft:repairable": {"items": ingot}}})
            img = Image.new("RGBA", (16, 16), (0, 0, 0, 0))
            d = ImageDraw.Draw(img)
            d.line([3, 12, 12, 3], fill=(120, 90, 50), width=2)
            d.rectangle([9, 2, 13, 6], fill=(150, 150, 170))
            save(img, f"item/{iid}.png")
            w(f"assets/{NS}/models/item/{iid}.json", {"parent": "minecraft:item/handheld" if kind in TOOLS else "minecraft:item/generated",
                                                      "textures": {"layer0": f"{NS}:item/{iid}"}})
            w(f"assets/{NS}/items/{iid}.json", {"model": {"type": "minecraft:model", "model": f"{NS}:item/{iid}"}})
            key = {"X": ingot}
            if "S" in "".join(pattern):
                key["S"] = "minecraft:stick"
            shaped(iid, pattern, key, f"{NS}:{iid}")
            lang[f"item.{NS}.{iid}"] = f"{mname} {kind.capitalize()}"
    return lang


def chest(cid, name, col):
    w(f"data/{NS}/horizon/block/{cid}.json", {
        "template": "minecraft:barrel",
        "properties": {"hardness": 3.0, "explosion_resistance": 1200.0 if "obsidian" in cid else 6.0,
                       "sounds": "minecraft:iron_block", "requires_tool": False},
        "item": True})
    for face in ("side", "top"):
        img = plate(col, sum(map(ord, cid + face)))
        d = ImageDraw.Draw(img)
        if face == "side":
            d.line([0, 5, 15, 5], fill=tuple(max(0, c - 60) for c in col))
            d.rectangle([7, 4, 8, 7], fill=(60, 60, 60))
        save(img, f"block/{cid}_{face}.png")
    w(f"assets/{NS}/models/block/{cid}.json", {"parent": "minecraft:block/cube_bottom_top",
        "textures": {"top": f"{NS}:block/{cid}_top", "bottom": f"{NS}:block/{cid}_top", "side": f"{NS}:block/{cid}_side"}})
    w(f"assets/{NS}/blockstates/{cid}.json", {"variants": {
        f"facing={f},open={o}": {"model": f"{NS}:block/{cid}"}
        for f in ("north", "east", "south", "west", "up", "down") for o in ("false", "true")}})
    w(f"assets/{NS}/items/{cid}.json", {"model": {"type": "minecraft:model", "model": f"{NS}:block/{cid}"}})
    loot(cid)


def forcefield():
    w(f"data/{NS}/horizon/block/force_field.json", {
        "template": "minecraft:light_blue_stained_glass",
        "properties": {"hardness": -1.0, "explosion_resistance": 3600000.0, "light": 6, "requires_tool": False}})
    img = Image.new("RGBA", (16, 16), (90, 170, 255, 90))
    d = ImageDraw.Draw(img)
    d.rectangle([0, 0, 15, 15], outline=(160, 220, 255, 200))
    save(img, "block/force_field.png")
    w(f"assets/{NS}/models/block/force_field.json", {"parent": "minecraft:block/cube_all",
        "render_type": "minecraft:translucent", "textures": {"all": f"{NS}:block/force_field"}})
    w(f"assets/{NS}/blockstates/force_field.json", {"variants": {"": {"model": f"{NS}:block/force_field"}}})
    machine("force_field_projector", "Force Field Projector")
    for mid in MODULES:
        w(f"data/{NS}/horizon/item/{mid}.json", {"template": "minecraft:iron_ingot"})
        img = Image.new("RGBA", (16, 16), (0, 0, 0, 0))
        ImageDraw.Draw(img).rectangle([3, 3, 12, 12], fill=(70, 70, 90), outline=(150, 200, 255))
        save(img, f"item/{mid}.png")
        w(f"assets/{NS}/models/item/{mid}.json", {"parent": "minecraft:item/generated", "textures": {"layer0": f"{NS}:item/{mid}"}})
        w(f"assets/{NS}/items/{mid}.json", {"model": {"type": "minecraft:model", "model": f"{NS}:item/{mid}"}})


def simple_block(bid, template, faces, extra_props=None):
    """A block with a BE-capable or plain template and per-face textures."""
    w(f"data/{NS}/horizon/block/{bid}.json", {"template": template,
        "properties": {"hardness": 2.0, "sounds": "minecraft:iron_block", "requires_tool": False, **(extra_props or {})},
        "item": True})
    for face, col in faces.items():
        save(plate(col, sum(map(ord, bid + face))), f"block/{bid}_{face}.png")
    w(f"assets/{NS}/models/block/{bid}.json", {"parent": "minecraft:block/cube_bottom_top",
        "textures": {"top": f"{NS}:block/{bid}_top", "bottom": f"{NS}:block/{bid}_top", "side": f"{NS}:block/{bid}_side"}})
    loot(bid)


def ae2():
    for bid, template, col in [("me_drive", "minecraft:barrel", (60, 60, 70)), ("me_import_bus", "minecraft:barrel", (70, 70, 90)),
                               ("me_terminal", "minecraft:iron_block", (40, 40, 60))]:
        simple_block(bid, template, {"side": col, "top": tuple(c + 20 for c in col)})
        if template == "minecraft:barrel":
            w(f"assets/{NS}/blockstates/{bid}.json", {"variants": {
                f"facing={f},open={o}": {"model": f"{NS}:block/{bid}"}
                for f in ("north", "east", "south", "west", "up", "down") for o in ("false", "true")}})
        else:
            w(f"assets/{NS}/blockstates/{bid}.json", {"variants": {"": {"model": f"{NS}:block/{bid}"}}})
        w(f"assets/{NS}/items/{bid}.json", {"model": {"type": "minecraft:model", "model": f"{NS}:block/{bid}"}})
    machine("me_controller", "ME Controller")
    wire("me_cable")
    for iid in AE_ITEMS:
        w(f"data/{NS}/horizon/item/{iid}.json", {"template": "minecraft:iron_ingot", "max_stack_size": 1})
        img = Image.new("RGBA", (16, 16), (0, 0, 0, 0))
        ImageDraw.Draw(img).rectangle([3, 2, 12, 13], fill=(80, 80, 90), outline=(160, 120, 255))
        save(img, f"item/{iid}.png")
        w(f"assets/{NS}/models/item/{iid}.json", {"parent": "minecraft:item/generated", "textures": {"layer0": f"{NS}:item/{iid}"}})
        w(f"assets/{NS}/items/{iid}.json", {"model": {"type": "minecraft:model", "model": f"{NS}:item/{iid}"}})


def tinkers():
    lang = {}
    for mat, (mname, col, tier, dur, ingot) in TINKER_MATERIALS.items():
        for tool, (tname, part, pname) in TINKER_TOOLS.items():
            pid = f"{mat}_{part}"
            w(f"data/{NS}/horizon/item/{pid}.json", {"template": "minecraft:iron_ingot"})
            save(plate(col, sum(map(ord, pid))), f"item/{pid}.png")
            w(f"assets/{NS}/models/item/{pid}.json", {"parent": "minecraft:item/generated", "textures": {"layer0": f"{NS}:item/{pid}"}})
            w(f"assets/{NS}/items/{pid}.json", {"model": {"type": "minecraft:model", "model": f"{NS}:item/{pid}"}})
            lang[f"item.{NS}.{pid}"] = f"{mname} {pname}"
            shaped(f"part/{pid}", {"pickaxe_head": ["XXX", " P "], "axe_head": ["XX", "XP"], "shovel_head": ["X", "P"],
                                   "sword_blade": ["X", "X", "P"]}[part], {"X": ingot, "P": f"{NS}:blank_pattern"}, f"{NS}:{pid}")
            iid = f"{mat}_{tool}"
            w(f"data/{NS}/horizon/item/{iid}.json", {"template": f"minecraft:{tier}_{VANILLA_KIND[tool]}",
                                                     "components": {"minecraft:max_damage": dur,
                                                                    "minecraft:repairable": {"items": ingot}}})
            save(plate(col, sum(map(ord, iid))), f"item/{iid}.png")
            w(f"assets/{NS}/models/item/{iid}.json", {"parent": "minecraft:item/handheld", "textures": {"layer0": f"{NS}:item/{iid}"}})
            w(f"assets/{NS}/items/{iid}.json", {"model": {"type": "minecraft:model", "model": f"{NS}:item/{iid}"}})
            lang[f"item.{NS}.{iid}"] = f"{mname} {tname}"
            w(f"data/{NS}/recipe/tool/{iid}.json", {"type": "minecraft:crafting_shapeless", "category": "equipment",
                "ingredients": [f"{NS}:{pid}", f"{NS}:binding", f"{NS}:tool_rod"], "result": {"id": f"{NS}:{iid}"}})
    return lang


def items():
    for did in DUSTS:
        w(f"data/{NS}/horizon/item/{did}.json", {"template": "minecraft:iron_ingot"})
        w(f"assets/{NS}/models/item/{did}.json", {"parent": "minecraft:item/generated",
                                                   "textures": {"layer0": f"{NS}:item/{did}"}})
        w(f"assets/{NS}/items/{did}.json", {"model": {"type": "minecraft:model", "model": f"{NS}:item/{did}"}})


def shaped(name, pattern, key, result, count=1):
    w(f"data/{NS}/recipe/{name}.json", {"type": "minecraft:crafting_shaped", "category": "misc",
                                        "pattern": pattern, "key": key, "result": {"id": result, "count": count}})


def recipes():
    I, C, R = "minecraft:iron_ingot", "minecraft:copper_ingot", "minecraft:redstone"
    shaped("copper_wire", ["WWW", "CCC", "WWW"], {"W": "#minecraft:wool", "C": C}, f"{NS}:copper_wire", 6)
    shaped("coal_generator", ["III", "IFI", "CRC"], {"I": I, "F": "minecraft:furnace", "C": C, "R": R}, f"{NS}:coal_generator")
    shaped("battery_box", ["ICI", "RBR", "ICI"], {"I": I, "C": C, "R": R, "B": "minecraft:redstone_block"}, f"{NS}:battery_box")
    shaped("electric_furnace", ["ICI", "CFC", "IRI"], {"I": I, "C": C, "F": "minecraft:furnace", "R": R}, f"{NS}:electric_furnace")
    shaped("crusher", ["IPI", "CRC", "III"], {"I": I, "P": "minecraft:piston", "C": C, "R": R}, f"{NS}:crusher")
    G, Wi = "minecraft:glass", f"{NS}:copper_wire"
    shaped("geothermal_generator", ["GBG", "GBG", "IXI"], {"G": G, "B": "minecraft:bucket", "I": I, "X": f"{NS}:coal_generator"}, f"{NS}:geothermal_generator")
    shaped("solar_panel", ["GGG", "CDC", "IXI"], {"G": G, "C": C, "D": "minecraft:daylight_detector", "I": I, "X": f"{NS}:coal_generator"}, f"{NS}:solar_panel")
    shaped("mfe", ["WBW", "BIB", "WBW"], {"W": Wi, "B": f"{NS}:battery_box", "I": "minecraft:iron_block"}, f"{NS}:mfe")
    shaped("mfsu", ["DMD", "MIM", "DMD"], {"D": "minecraft:diamond", "M": f"{NS}:mfe", "I": "minecraft:iron_block"}, f"{NS}:mfsu")
    shaped("compressor", ["IPI", "PCP", "IRI"], {"I": I, "P": "minecraft:piston", "C": C, "R": R}, f"{NS}:compressor")
    shaped("extractor", ["ISI", "CRC", "III"], {"I": I, "S": "minecraft:sticky_piston", "C": C, "R": R}, f"{NS}:extractor")
    O, BC = f"{NS}:osmium_ingot", f"{NS}:basic_control_circuit"
    mek = lambda name, center: shaped(name, ["RCR", "OXO", "RCR"], {"R": R, "C": BC, "O": O, "X": center}, f"{NS}:{name}")
    mek("enrichment_chamber", "minecraft:iron_block")
    mek("mekanism_crusher", f"{NS}:crusher")
    mek("energized_smelter", f"{NS}:electric_furnace")
    mek("osmium_compressor", f"{NS}:compressor")
    mek("metallurgic_infuser", "minecraft:furnace")
    shaped("heat_generator", ["III", "OFO", "CXC"], {"I": I, "O": O, "F": "minecraft:furnace", "C": C, "X": f"{NS}:coal_generator"}, f"{NS}:heat_generator")
    shaped("wind_generator", [" O ", "OXO", " I "], {"O": O, "X": f"{NS}:enriched_alloy", "I": "minecraft:iron_block"}, f"{NS}:wind_generator")
    shaped("empty_cell", [" G ", "GIG", " G "], {"G": "minecraft:glass_pane", "I": I}, f"{NS}:empty_cell", 4)
    w(f"data/{NS}/recipe/water_cell.json", {"type": "minecraft:crafting_shapeless", "category": "misc",
        "ingredients": [f"{NS}:empty_cell", "minecraft:water_bucket"], "result": {"id": f"{NS}:water_cell"}})
    shaped("fissile_fuel_cell", ["UUU", "UCU", "UUU"], {"U": f"{NS}:uranium_235", "C": f"{NS}:empty_cell"}, f"{NS}:fissile_fuel_cell")
    shaped("chemical_extractor", ["SCS", "BXB", "SCS"], {"S": f"{NS}:steel_ingot", "C": f"{NS}:advanced_control_circuit", "B": "minecraft:bucket", "X": f"{NS}:enrichment_chamber"}, f"{NS}:chemical_extractor")
    shaped("centrifuge", ["SCS", "IXI", "SCS"], {"S": f"{NS}:steel_ingot", "C": f"{NS}:advanced_control_circuit", "I": f"{NS}:reinforced_alloy", "X": f"{NS}:mekanism_crusher"}, f"{NS}:centrifuge")
    shaped("fission_reactor", ["SGS", "GUG", "SCS"], {"S": f"{NS}:steel_ingot", "G": "minecraft:glass", "U": f"{NS}:uranium_235", "C": f"{NS}:elite_control_circuit"}, f"{NS}:fission_reactor")
    prev = "minecraft:chest"
    for cid, ingot in [("copper_chest", "minecraft:copper_ingot"), ("iron_chest", I), ("gold_chest", "minecraft:gold_ingot"),
                       ("diamond_chest", "minecraft:diamond"), ("crystal_chest", "minecraft:glass"), ("obsidian_chest", "minecraft:obsidian")]:
        shaped(cid, ["XXX", "XCX", "XXX"], {"X": ingot, "C": prev}, f"{NS}:{cid}")
        prev = f"{NS}:{cid}" if cid != "crystal_chest" else f"{NS}:diamond_chest"
    shaped("force_field_projector", ["SDS", "CXC", "SDS"], {"S": f"{NS}:steel_ingot", "D": "minecraft:diamond", "C": f"{NS}:advanced_control_circuit", "X": "minecraft:beacon"}, f"{NS}:force_field_projector")
    for mid, center in [("cube_module", "minecraft:iron_block"), ("sphere_module", "minecraft:slime_ball"),
                        ("wall_module", "minecraft:iron_bars"), ("tube_module", "minecraft:hopper"), ("scale_module", "minecraft:redstone")]:
        shaped(mid, ["SSS", "SXS", "SSS"], {"S": f"{NS}:steel_ingot", "X": center}, f"{NS}:{mid}", 1 if mid != "scale_module" else 4)
    for pid, mat in [("wooden_pipe", "#minecraft:planks"), ("cobblestone_pipe", "minecraft:cobblestone"),
                     ("stone_pipe", "minecraft:stone"), ("gold_pipe", "minecraft:gold_ingot")]:
        shaped(pid, ["XGX"], {"X": mat, "G": "minecraft:glass"}, f"{NS}:{pid}", 8)
    shaped("quarry", ["IGI", "GCG", "DPD"], {"I": I, "G": "minecraft:gold_ingot", "C": f"{NS}:advanced_control_circuit", "D": "minecraft:diamond", "P": "minecraft:diamond_pickaxe"}, f"{NS}:quarry")
    Q = "minecraft:quartz"
    shaped("me_cable", ["QGQ"], {"Q": Q, "G": "minecraft:glass"}, f"{NS}:me_cable", 8)
    shaped("me_controller", ["IQI", "QCQ", "IQI"], {"I": I, "Q": "minecraft:quartz_block", "C": f"{NS}:advanced_control_circuit"}, f"{NS}:me_controller")
    shaped("me_drive", ["IQI", "C C", "IQI"], {"I": I, "Q": Q, "C": f"{NS}:basic_control_circuit"}, f"{NS}:me_drive")
    shaped("me_terminal", ["GGG", "QCQ", "III"], {"G": "minecraft:glass_pane", "Q": Q, "C": f"{NS}:basic_control_circuit", "I": I}, f"{NS}:me_terminal")
    shaped("me_import_bus", ["QHQ", "ICI"], {"Q": Q, "H": "minecraft:hopper", "I": I, "C": f"{NS}:basic_control_circuit"}, f"{NS}:me_import_bus")
    shaped("storage_cell_1k", ["GRG", "RQR", "III"], {"G": "minecraft:glass", "R": R, "Q": Q, "I": I}, f"{NS}:storage_cell_1k")
    for prev, cur, x in [("1k", "4k", "minecraft:gold_block"), ("4k", "16k", "minecraft:diamond"), ("16k", "64k", "minecraft:emerald_block")]:
        shaped(f"storage_cell_{cur}", ["CRC", "RXR", "CRC"], {"C": f"{NS}:storage_cell_{prev}", "R": R, "X": x}, f"{NS}:storage_cell_{cur}")
    CH = f"{NS}:machine_chassis"
    shaped("machine_chassis", ["IBI", "BCB", "IBI"], {"I": "minecraft:iron_bars", "B": I, "C": f"{NS}:basic_capacitor"}, CH)
    shaped("basic_capacitor", [" N ", "NRN", "G G"], {"N": "minecraft:gold_nugget", "R": R, "G": "minecraft:copper_ingot"}, f"{NS}:basic_capacitor")
    shaped("double_layer_capacitor", [" E ", "CXC", " E "], {"E": f"{NS}:energetic_alloy_ingot", "C": f"{NS}:basic_capacitor", "X": "minecraft:coal"}, f"{NS}:double_layer_capacitor")
    shaped("octadic_capacitor", [" V ", "CGC", " V "], {"V": f"{NS}:vibrant_alloy_ingot", "C": f"{NS}:double_layer_capacitor", "G": "minecraft:glowstone"}, f"{NS}:octadic_capacitor")
    shaped("alloy_smelter", ["IFI", "FCF", "IKI"], {"I": I, "F": "minecraft:furnace", "C": CH, "K": "minecraft:cauldron"}, f"{NS}:alloy_smelter")
    shaped("sag_mill", ["FFF", "ICI", "IPI"], {"F": "minecraft:flint", "I": I, "C": CH, "P": "minecraft:piston"}, f"{NS}:sag_mill")
    shaped("slice_and_splice", ["SSS", "AXA", "SSS"], {"S": f"{NS}:soularium_ingot", "A": "minecraft:iron_axe", "X": CH}, f"{NS}:slice_and_splice")
    shaped("soul_binder", ["SES", "ZXZ", "SES"], {"S": f"{NS}:soularium_ingot", "E": "minecraft:ender_eye", "Z": "minecraft:zombie_head", "X": CH}, f"{NS}:soul_binder")
    shaped("stirling_generator", ["BBB", "BFB", "IPI"], {"B": "minecraft:stone_bricks", "F": "minecraft:furnace", "I": I, "P": "minecraft:piston"}, f"{NS}:stirling_generator")
    shaped("basic_capacitor_bank", ["IRI", "CBC", "IRI"], {"I": I, "R": "minecraft:redstone_block", "C": f"{NS}:basic_capacitor", "B": "minecraft:redstone_block"}, f"{NS}:basic_capacitor_bank")
    shaped("capacitor_bank", ["IDI", "DXD", "IDI"], {"I": I, "D": f"{NS}:double_layer_capacitor", "X": f"{NS}:basic_capacitor_bank"}, f"{NS}:capacitor_bank")
    shaped("vibrant_capacitor_bank", ["IOI", "OXO", "IOI"], {"I": f"{NS}:electrical_steel_ingot", "O": f"{NS}:octadic_capacitor", "X": f"{NS}:capacitor_bank"}, f"{NS}:vibrant_capacitor_bank")
    for cid, (_, ingot) in CONDUITS.items():
        shaped(cid, ["CCC", "III", "CCC"], {"C": "minecraft:clay_ball", "I": ingot}, f"{NS}:{cid}", 8)
    shaped("basic_energy_cube", ["ROR", "IBI", "ROR"], {"R": R, "O": O, "I": I, "B": f"{NS}:battery_box"}, f"{NS}:basic_energy_cube")
    for prev, name, alloy in [("basic", "advanced", "enriched_alloy"), ("advanced", "elite", "reinforced_alloy"), ("elite", "ultimate", "atomic_alloy")]:
        shaped(f"{name}_energy_cube", [" A ", "AXA", " A "], {"A": f"{NS}:{alloy}", "X": f"{NS}:{prev}_energy_cube"}, f"{NS}:{name}_energy_cube")
    T, X = "minecraft:tnt", f"{NS}:condensed_explosive"
    shaped("condensed_explosive", ["TRT"], {"T": T, "R": R}, X, 1)
    shaped("incendiary_explosive", [" B ", "BXB", " B "], {"B": "minecraft:blaze_powder", "X": X}, f"{NS}:incendiary_explosive")
    shaped("shrapnel_explosive", [" A ", "AXA", " A "], {"A": "minecraft:arrow", "X": X}, f"{NS}:shrapnel_explosive")
    shaped("breaching_explosive", ["X", "X", "P"], {"X": X, "P": "minecraft:iron_pickaxe"}, f"{NS}:breaching_explosive")
    ring = lambda name, around, base: shaped(name, [" A ", "AXA", " A "], {"A": around, "X": base}, f"{NS}:{name}")
    ring("fragmentation_explosive", "minecraft:flint", f"{NS}:shrapnel_explosive")
    ring("thermobaric_explosive", "minecraft:gunpowder", X)
    ring("exothermic_explosive", "minecraft:magma_block", f"{NS}:incendiary_explosive")
    ring("endothermic_explosive", "minecraft:ice", X)
    ring("sonic_explosive", "minecraft:note_block", X)
    ring("nuclear_explosive", "minecraft:redstone_block", f"{NS}:thermobaric_explosive")
    ring("smoke_explosive", "minecraft:charcoal", X)
    ring("chemical_explosive", "minecraft:spider_eye", X)
    ring("debilitation_explosive", "minecraft:fermented_spider_eye", X)
    ring("contagious_explosive", "minecraft:rotten_flesh", X)
    ring("emp_explosive", "minecraft:copper_block", X)
    ring("anvil_explosive", "minecraft:iron_block", X)
    crush = {
        "iron_ore": ("iron_dust", 2), "deepslate_iron_ore": ("iron_dust", 2), "raw_iron": ("iron_dust", 1),
        "gold_ore": ("gold_dust", 2), "deepslate_gold_ore": ("gold_dust", 2), "raw_gold": ("gold_dust", 1),
        "copper_ore": ("copper_dust", 4), "deepslate_copper_ore": ("copper_dust", 4), "raw_copper": ("copper_dust", 1),
        "cobblestone": ("minecraft:gravel", 1), "gravel": ("minecraft:sand", 1), "bone": ("minecraft:bone_meal", 5),
    }
    for src, (out, n) in crush.items():
        oid = out if ":" in out else f"{NS}:{out}"
        w(f"data/{NS}/recipe/crushing/{src}.json", {"type": f"{NS}:crushing", "ingredient": f"minecraft:{src}",
                                                    "result": {"id": oid, "count": n}})
    M = lambda x: x if ":" in x else f"{NS}:{x}"
    def typed(kind, src, out, n=1, secondary=None):
        r = {"type": f"{NS}:{kind}", "ingredient": M(src) if ":" in src or src in DUSTS else f"minecraft:{src}",
             "result": {"id": M(out) if ":" in out or out in DUSTS or out in ORES else f"minecraft:{out}", "count": n}}
        if secondary:
            r["secondary"] = secondary if ":" in secondary or secondary not in DUSTS else f"{NS}:{secondary}"
        w(f"data/{NS}/recipe/{kind}/{src.split(':')[-1]}.json", r)
    for src, (out, n) in {"iron_ore": ("iron_dust", 2), "deepslate_iron_ore": ("iron_dust", 2), "raw_iron": ("iron_dust", 2),
                           "gold_ore": ("gold_dust", 2), "deepslate_gold_ore": ("gold_dust", 2), "raw_gold": ("gold_dust", 2),
                           "copper_ore": ("copper_dust", 6), "deepslate_copper_ore": ("copper_dust", 6), "raw_copper": ("copper_dust", 2),
                           "voltz:osmium_ore": ("osmium_dust", 2), "voltz:deepslate_osmium_ore": ("osmium_dust", 2),
                           "redstone_ore": ("redstone", 12), "diamond_ore": ("diamond", 2), "lapis_lazuli_ore": ("lapis_lazuli", 12),
                           "coal_ore": ("coal", 2), "emerald_ore": ("emerald", 2)}.items():
        typed("enriching", src, out, n)
    for src, (out, n) in {"iron_ingot": ("iron_dust", 1), "gold_ingot": ("gold_dust", 1), "copper_ingot": ("copper_dust", 1),
                           "voltz:osmium_ingot": ("osmium_dust", 1), "voltz:steel_ingot": ("steel_dust", 1),
                           "obsidian": ("obsidian_dust", 4), "diamond": ("diamond_dust", 1), "cobblestone": ("gravel", 1),
                           "gravel": ("sand", 1), "stone": ("cobblestone", 1)}.items():
        typed("mekanism_crushing", src, out, n)
    for src, sec, out in [("iron_ingot", "minecraft:redstone", "enriched_alloy"),
                          ("voltz:enriched_alloy", "voltz:diamond_dust", "reinforced_alloy"),
                          ("voltz:reinforced_alloy", "voltz:obsidian_dust", "atomic_alloy"),
                          ("voltz:osmium_ingot", "minecraft:redstone", "basic_control_circuit"),
                          ("voltz:basic_control_circuit", "voltz:enriched_alloy", "advanced_control_circuit"),
                          ("voltz:advanced_control_circuit", "voltz:reinforced_alloy", "elite_control_circuit"),
                          ("voltz:elite_control_circuit", "voltz:atomic_alloy", "ultimate_control_circuit"),
                          ("voltz:iron_dust", "minecraft:coal", "steel_dust"),
                          ("voltz:obsidian_dust", "voltz:diamond_dust", "refined_obsidian_dust")]:
        typed("infusing", src, out, 1, sec)
    for src, sec, out, n in [("iron_ingot", "minecraft:redstone", "conductive_iron_ingot", 1),
                             ("iron_ingot", "minecraft:coal", "electrical_steel_ingot", 1),
                             ("redstone", "minecraft:sand", "redstone_alloy_ingot", 1),
                             ("gold_ingot", "minecraft:glowstone_dust", "energetic_alloy_ingot", 1),
                             ("voltz:energetic_alloy_ingot", "minecraft:ender_pearl", "vibrant_alloy_ingot", 1),
                             ("iron_ingot", "minecraft:ender_pearl", "pulsating_iron_ingot", 1),
                             ("iron_ingot", "minecraft:obsidian", "dark_steel_ingot", 1),
                             ("gold_ingot", "minecraft:soul_sand", "soularium_ingot", 1),
                             ("voltz:dark_steel_ingot", "minecraft:end_stone", "end_steel_ingot", 1),
                             ("sand", "minecraft:sand", "glass", 2)]:
        r = {"type": f"{NS}:alloy_smelting", "ingredient": src if ":" in src else f"minecraft:{src}", "secondary": sec,
             "result": {"id": out if out == "glass" and False else (f"minecraft:{out}" if out == "glass" else f"{NS}:{out}"), "count": n}}
        w(f"data/{NS}/recipe/alloy_smelting/{out}.json", r)
    for src, (out, n) in {"iron_ore": ("iron_dust", 2), "deepslate_iron_ore": ("iron_dust", 2), "gold_ore": ("gold_dust", 2),
                           "deepslate_gold_ore": ("gold_dust", 2), "copper_ore": ("copper_dust", 6),
                           "voltz:osmium_ore": ("osmium_dust", 2), "nether_quartz_ore": ("quartz", 3),
                           "cobblestone": ("gravel", 1), "gravel": ("flint", 1), "bone": ("bone_meal", 6),
                           "blaze_rod": ("blaze_powder", 5), "wheat": ("wheat_seeds", 3)}.items():
        typed("sag_milling", src, out, n)
    for src, sec, out, n in [("skeleton_skull", "voltz:soularium_ingot", "wither_skeleton_skull", 1),
                             ("zombie_head", "minecraft:gunpowder", "creeper_head", 1)]:
        typed("slicing", src, out, n, sec)
    for src, sec, out, n in [("ender_pearl", "voltz:soularium_ingot", "ender_eye", 2),
                             ("emerald", "voltz:soularium_ingot", "experience_bottle", 8)]:
        typed("soul_binding", src, out, n, sec)
    for src, sec, out, n in [("voltz:uranium_ore", "voltz:water_cell", "yellow_cake", 3),
                             ("voltz:deepslate_uranium_ore", "voltz:water_cell", "yellow_cake", 3)]:
        typed("chemical_extracting", src, out, n, sec)
    typed("centrifuging", "voltz:yellow_cake", "uranium_238", 1)
    typed("centrifuging", "voltz:uranium_238", "uranium_235", 1, "voltz:uranium_238")
    typed("osmium_compressing", "voltz:refined_obsidian_dust", "refined_obsidian_ingot", 1, "voltz:osmium_ingot")
    typed("osmium_compressing", "glowstone_dust", "refined_glowstone_ingot", 1, "voltz:osmium_ingot")
    for src, (out, n) in {"iron_ore": ("iron_ingot", 2), "deepslate_iron_ore": ("iron_ingot", 2), "raw_iron": ("iron_ingot", 2),
                           "gold_ore": ("gold_ingot", 2), "deepslate_gold_ore": ("gold_ingot", 2), "raw_gold": ("gold_ingot", 2),
                           "copper_ore": ("copper_ingot", 4), "raw_copper": ("copper_ingot", 2),
                           "voltz:cobalt_ore": ("cobalt_ingot", 2), "voltz:ardite_ore": ("ardite_ingot", 2),
                           "voltz:osmium_ore": ("osmium_ingot", 2)}.items():
        typed("smeltery_melting", src, out, n)
    typed("smeltery_melting", "voltz:cobalt_ingot", "manyullyn_ingot", 1, "voltz:ardite_ingot")
    typed("coking", "coal", "coal_coke", 1)
    typed("coking", "oak_log", "charcoal", 1)
    typed("blast_smelting", "iron_ingot", "steel_ingot", 1, "voltz:coal_coke")
    typed("blast_smelting", "iron_block", "steel_ingot", 9, "minecraft:coal_block")
    shaped("blank_pattern", ["PS", "SP"], {"P": "#minecraft:planks", "S": "minecraft:stick"}, f"{NS}:blank_pattern", 4)
    shaped("tool_rod", ["S", "S"], {"S": "minecraft:stick"}, f"{NS}:tool_rod", 2)
    shaped("binding", ["S S", " S ", "S S"], {"S": "minecraft:stick"}, f"{NS}:binding", 2)
    shaped("smeltery", ["BBB", "BFB", "BBB"], {"B": "minecraft:bricks", "F": "minecraft:blast_furnace"}, f"{NS}:smeltery")
    shaped("coke_oven", ["BSB", "S S", "BSB"], {"B": "minecraft:bricks", "S": "minecraft:sand"}, f"{NS}:coke_oven")
    shaped("blast_furnace", ["NBN", "BMB", "NBN"], {"N": "minecraft:nether_bricks", "B": "minecraft:blaze_powder", "M": "minecraft:magma_block"}, f"{NS}:blast_furnace")
    for src, (out, n) in {"snow_block": ("ice", 1), "ice": ("packed_ice", 1), "packed_ice": ("blue_ice", 1),
                           "sand": ("sandstone", 1), "red_sand": ("red_sandstone", 1), "coal": ("coal_block", 1),
                           "nether_wart": ("nether_wart_block", 1), "clay_ball": ("clay", 1)}.items():
        w(f"data/{NS}/recipe/compressing/{src}.json", {"type": f"{NS}:compressing", "ingredient": f"minecraft:{src}",
                                                       "result": {"id": f"minecraft:{out}", "count": n}})
    for src, (out, n) in {"gravel": ("flint", 1), "sugar_cane": ("sugar", 2), "slime_block": ("slime_ball", 9),
                           "bone_block": ("bone_meal", 9), "blaze_rod": ("blaze_powder", 4),
                           "glowstone": ("glowstone_dust", 4), "magma_block": ("magma_cream", 2)}.items():
        w(f"data/{NS}/recipe/extracting/{src}.json", {"type": f"{NS}:extracting", "ingredient": f"minecraft:{src}",
                                                      "result": {"id": f"minecraft:{out}", "count": n}})
    smelt = {f"{NS}:iron_dust": "minecraft:iron_ingot", f"{NS}:gold_dust": "minecraft:gold_ingot",
             f"{NS}:copper_dust": "minecraft:copper_ingot", "minecraft:raw_iron": "minecraft:iron_ingot",
             "minecraft:raw_gold": "minecraft:gold_ingot", "minecraft:raw_copper": "minecraft:copper_ingot",
             "minecraft:sand": "minecraft:glass", "minecraft:cobblestone": "minecraft:stone",
             "minecraft:clay_ball": "minecraft:brick", "minecraft:beef": "minecraft:cooked_beef",
             "minecraft:porkchop": "minecraft:cooked_porkchop", "minecraft:chicken": "minecraft:cooked_chicken",
             "minecraft:mutton": "minecraft:cooked_mutton", "minecraft:potato": "minecraft:baked_potato",
             "minecraft:cod": "minecraft:cooked_cod", "minecraft:salmon": "minecraft:cooked_salmon",
             "minecraft:kelp": "minecraft:dried_kelp", "minecraft:wet_sponge": "minecraft:sponge",
             f"{NS}:osmium_dust": f"{NS}:osmium_ingot", f"{NS}:steel_dust": f"{NS}:steel_ingot",
             f"{NS}:osmium_ore": f"{NS}:osmium_ingot", f"{NS}:deepslate_osmium_ore": f"{NS}:osmium_ingot"}
    for src, out in smelt.items():
        name = src.split(":")[1]
        w(f"data/{NS}/recipe/electric_smelting/{name}.json", {"type": f"{NS}:electric_smelting",
                                                               "ingredient": src, "result": {"id": out}})
    for did, ingot in [("iron_dust", "minecraft:iron_ingot"), ("gold_dust", "minecraft:gold_ingot"),
                       ("copper_dust", "minecraft:copper_ingot"), ("osmium_dust", f"{NS}:osmium_ingot"),
                       ("osmium_ore", f"{NS}:osmium_ingot"), ("deepslate_osmium_ore", f"{NS}:osmium_ingot"),
                       ("steel_dust", f"{NS}:steel_ingot")]:
        w(f"data/{NS}/recipe/smelting/{did}.json", {"type": "minecraft:smelting", "category": "misc",
                                                     "ingredient": f"{NS}:{did}",
                                                     "result": {"id": ingot},
                                                     "experience": 0.5, "cookingtime": 200})


def main():
    if ROOT.exists():
        shutil.rmtree(ROOT)
    (ROOT).mkdir(parents=True)
    (ROOT / "horizon-mod.toml").write_text(
        'id = "voltz"\nname = "Voltz Core"\nversion = "0.1.0"\n'
        'description = "Voltz remade for Horizon: power, wires, generators and machines (content half; pair with the voltz_core native mod)"\n'
        'authors = ["Arthur Black"]\napi = "0.2"\nside = "both"\ncapabilities = ["registries"]\n')
    textures()
    MACHINES_EXTRA = {"force_field_projector": ("Force Field Projector", (60, 70, 90), (120, 200, 255)),
                      "me_controller": ("ME Controller", (60, 60, 80), (160, 120, 255)),
                      "quarry": ("Quarry", (110, 100, 60), (255, 220, 80))}
    for i, (mid, (_, base, glow)) in enumerate(MACHINES_EXTRA.items()):
        save(plate(base, 300 + i), f"block/{mid}_side.png")
        save(plate(base, 310 + i), f"block/{mid}_top.png")
        save(plate(base, 320 + i), f"block/{mid}_bottom.png")
        for lit in (False, True):
            f = plate(base, 330 + i)
            ImageDraw.Draw(f).ellipse([4, 4, 11, 11], fill=glow if lit else (40, 40, 50))
            save(f, f"block/{mid}_front{'_on' if lit else ''}.png")
    lang = {}
    for mid, (name, _, _) in MACHINES.items():
        machine(mid, name)
        lang[f"block.{NS}.{mid}"] = name
    for eid, (name, _) in EXPLOSIVES.items():
        explosive(eid)
        lang[f"block.{NS}.{eid}"] = name
    for cid, (name, col) in CHESTS.items():
        chest(cid, name, col)
        lang[f"block.{NS}.{cid}"] = name
    forcefield()
    lang[f"block.{NS}.force_field_projector"] = "Force Field Projector"
    lang[f"block.{NS}.force_field"] = "Force Field"
    for mid, name in MODULES.items():
        lang[f"item.{NS}.{mid}"] = name
    for oid, (name, template, col) in ORES.items():
        ore(oid, name, template, col)
        lang[f"block.{NS}.{oid}"] = name
    lang.update(tools())
    lang.update(tinkers())
    wire()
    lang[f"block.{NS}.copper_wire"] = "Copper Wire"
    for cid, (name, _) in CONDUITS.items():
        wire(cid)
        lang[f"block.{NS}.{cid}"] = name
    for pid, name in PIPES.items():
        wire(pid)
        lang[f"block.{NS}.{pid}"] = name
    machine("quarry", "Quarry")
    lang[f"block.{NS}.quarry"] = "Quarry"
    ae2()
    for bid, name in [("me_drive", "ME Drive"), ("me_import_bus", "ME Import Bus"), ("me_terminal", "ME Terminal"),
                      ("me_controller", "ME Controller"), ("me_cable", "ME Glass Cable")]:
        lang[f"block.{NS}.{bid}"] = name
    for iid, name in AE_ITEMS.items():
        lang[f"item.{NS}.{iid}"] = name
    items()
    for did, (name, _) in DUSTS.items():
        lang[f"item.{NS}.{did}"] = name
    recipes()
    # One creative tab per original mod, in Voltz's own order.
    TABS = [
        ("industrialcraft", "IndustrialCraft 2", "coal_generator",
         ["copper_wire", "coal_generator", "geothermal_generator", "solar_panel", "battery_box", "mfe", "mfsu",
          "electric_furnace", "crusher", "compressor", "extractor", "iron_dust", "gold_dust", "copper_dust"]),
        ("icbm", "ICBM", "nuclear_explosive", list(EXPLOSIVES)),
        ("mekanism", "Mekanism", "enrichment_chamber",
         ["enrichment_chamber", "mekanism_crusher", "energized_smelter", "osmium_compressor", "metallurgic_infuser",
          "heat_generator", "wind_generator", "basic_energy_cube", "advanced_energy_cube", "elite_energy_cube",
          "ultimate_energy_cube", "osmium_ore", "deepslate_osmium_ore", "osmium_dust", "osmium_ingot", "steel_dust",
          "steel_ingot", "obsidian_dust", "diamond_dust", "refined_obsidian_dust", "refined_obsidian_ingot",
          "refined_glowstone_ingot", "enriched_alloy", "reinforced_alloy", "atomic_alloy", "basic_control_circuit",
          "advanced_control_circuit", "elite_control_circuit", "ultimate_control_circuit"]),
        ("mekanismtools", "Mekanism Tools", "refined_obsidian_pickaxe",
         [f"{m}_{k}" for m in TOOL_MATERIALS if m != "dark_steel" for k in [*TOOLS, *ARMOR]]),
        ("enderio", "Ender IO", "alloy_smelter",
         ["alloy_smelter", "sag_mill", "slice_and_splice", "soul_binder", "stirling_generator", "basic_capacitor_bank",
          "capacitor_bank", "vibrant_capacitor_bank", *CONDUITS, "electrical_steel_ingot", "energetic_alloy_ingot",
          "vibrant_alloy_ingot", "redstone_alloy_ingot", "conductive_iron_ingot", "pulsating_iron_ingot",
          "dark_steel_ingot", "soularium_ingot", "end_steel_ingot", "basic_capacitor", "double_layer_capacitor",
          "octadic_capacitor", "machine_chassis"]
         + [f"dark_steel_{k}" for k in [*TOOLS, *ARMOR] if k not in TOOL_SKIP["dark_steel"]]),
        ("atomicscience", "Atomic Science", "fission_reactor",
         ["chemical_extractor", "centrifuge", "fission_reactor", "uranium_ore", "deepslate_uranium_ore", "yellow_cake",
          "uranium_238", "uranium_235", "empty_cell", "water_cell", "fissile_fuel_cell", "toxic_waste"]),
        ("mffs", "Modular Force Field System", "force_field_projector", ["force_field_projector", *MODULES]),
        ("ironchest", "Iron Chests", "diamond_chest", list(CHESTS)),
        ("buildcraft", "BuildCraft", "quarry", ["quarry", *PIPES]),
        ("appliedenergistics2", "Applied Energistics 2", "me_terminal",
         ["me_controller", "me_drive", "me_terminal", "me_import_bus", "me_cable", *AE_ITEMS]),
        ("tconstruct", "Tinkers' Construct", "manyullyn_pickaxe",
         ["smeltery", "cobalt_ore", "ardite_ore", "cobalt_ingot", "ardite_ingot", "manyullyn_ingot", "blank_pattern",
          "tool_rod", "binding"]
         + [f"{m}_{p}" for m in TINKER_MATERIALS for p in [v[1] for v in TINKER_TOOLS.values()]]
         + [f"{m}_{t}" for m in TINKER_MATERIALS for t in TINKER_TOOLS]),
        ("railcraft", "Railcraft", "coke_oven", ["coke_oven", "blast_furnace", "coal_coke"]),
    ]
    placed = set()
    for key, title, icon, members in TABS:
        w(f"data/{NS}/horizon/creative_tab/{key}.json", {
            "title": f"itemGroup.{NS}.{key}", "icon": f"{NS}:{icon}", "items": [f"{NS}:{x}" for x in members]})
        lang[f"itemGroup.{NS}.{key}"] = title
        placed.update(members)
    # Every item is in exactly one tab: fail the build if one was missed.
    all_items = {p.stem for p in (ROOT / "data" / NS / "horizon" / "item").glob("*.json")}
    all_items |= {p.stem for p in (ROOT / "data" / NS / "horizon" / "block").glob("*.json")
                  if json.loads(p.read_text()).get("item")}
    missing = sorted(all_items - placed)
    assert not missing, f"items in no creative tab: {missing}"
    w(f"assets/{NS}/lang/en_us.json", lang)
    w("data/minecraft/tags/block/mineable/pickaxe.json", {"values": [f"{NS}:{m}" for m in [*MACHINES, *ORES, "quarry", "force_field_projector", *CHESTS]]})
    w("data/minecraft/tags/block/needs_stone_tool.json", {"values": [f"{NS}:{o}" for o in ORES]})
    print("wrote", sum(1 for _ in ROOT.rglob("*") if _.is_file()), "files to", ROOT)


main()
