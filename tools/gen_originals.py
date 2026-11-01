#!/usr/bin/env python3
"""The public build's own versions of what the private build takes from the
players' jars: a nuclear reactor and its parts, plates and scrap, more
machines, missiles and their launcher, turrets, guns and ammo. All names are
generic and all art is drawn here. Run after gen_content.py (public build)."""
import json
import random
from pathlib import Path

from PIL import Image, ImageDraw

import gen_content as gc

ROOT, NS, w, save, plate, shaped = gc.ROOT, gc.NS, gc.w, gc.save, gc.plate, gc.shaped


def icon(iid, draw, tab, name, lang, tabs, template="minecraft:iron_ingot", stack=64, handheld=False):
    img = Image.new("RGBA", (16, 16), (0, 0, 0, 0))
    draw(ImageDraw.Draw(img), img)
    save(img, f"item/{iid}.png")
    data = {"template": template}
    if stack != 64:
        data["max_stack_size"] = stack
    w(f"data/{NS}/horizon/item/{iid}.json", data)
    w(f"assets/{NS}/models/item/{iid}.json", {"parent": "minecraft:item/handheld" if handheld else "minecraft:item/generated",
                                              "textures": {"layer0": f"{NS}:item/{iid}"}})
    w(f"assets/{NS}/items/{iid}.json", {"model": {"type": "minecraft:model", "model": f"{NS}:item/{iid}"}})
    lang[f"item.{NS}.{iid}"] = name
    tabs.setdefault(tab, []).append(iid)


def machine_block(mid, name, base, glow, tab, lang, tabs, seed):
    gc.save(plate(base, seed), f"block/{mid}_side.png")
    gc.save(plate(tuple(min(255, c + 15) for c in base), seed + 1), f"block/{mid}_top.png")
    gc.save(plate(tuple(max(0, c - 10) for c in base), seed + 2), f"block/{mid}_bottom.png")
    for lit in (False, True):
        f = plate(base, seed + 3)
        d = ImageDraw.Draw(f)
        d.rectangle([3, 3, 12, 12], fill=(30, 30, 34), outline=(20, 20, 20))
        d.rectangle([5, 6, 10, 10], fill=glow if lit else (60, 60, 66))
        gc.save(f, f"block/{mid}_front{'_on' if lit else ''}.png")
    gc.machine(mid, name)
    lang[f"block.{NS}.{mid}"] = name
    tabs.setdefault(tab, []).append(mid)


def plate_icon(col):
    def d(dr, img):
        dr.rectangle([2, 3, 13, 12], fill=col, outline=tuple(max(0, c - 60) for c in col))
        dr.line([3, 4, 12, 4], fill=tuple(min(255, c + 50) for c in col))
    return d


def rod_icon(cells, col):
    def d(dr, img):
        xs = {1: [7], 2: [5, 9], 4: [3, 6, 9, 12]}[cells]
        for x in xs:
            dr.rectangle([x - 1, 2, x + 1, 13], fill=(140, 140, 150), outline=(70, 70, 80))
            dr.rectangle([x - 1, 5, x + 1, 10], fill=col)
    return d


def vent_icon(col):
    def d(dr, img):
        dr.rectangle([2, 2, 13, 13], fill=(90, 90, 100), outline=(40, 40, 50))
        for y in range(4, 13, 3):
            dr.line([4, y, 11, y], fill=col)
    return d


def exch_icon(col):
    def d(dr, img):
        dr.rectangle([2, 2, 13, 13], fill=(90, 90, 100), outline=(40, 40, 50))
        dr.line([4, 4, 11, 11], fill=col, width=2)
        dr.line([11, 4, 4, 11], fill=col, width=2)
    return d


def cell_icon(col):
    def d(dr, img):
        dr.rectangle([5, 2, 10, 13], fill=(200, 210, 220), outline=(90, 100, 110))
        dr.rectangle([6, 5, 9, 12], fill=col)
    return d


def missile_icon(col):
    def d(dr, img):
        dr.polygon([(12, 2), (13, 3), (5, 11), (4, 10)], fill=(200, 200, 205))
        dr.polygon([(12, 2), (14, 1), (13, 3)], fill=col)
        dr.polygon([(4, 10), (2, 10), (5, 13), (5, 11)], fill=(90, 90, 95))
        dr.point((3, 13), fill=(255, 140, 30))
        dr.point((2, 14), fill=(255, 200, 60))
        dr.line([10, 4, 11, 5], fill=col)
    return d


def gun_icon(col):
    def d(dr, img):
        dr.rectangle([2, 5, 13, 8], fill=col)
        dr.rectangle([4, 8, 6, 13], fill=(60, 50, 40))
        dr.rectangle([12, 4, 13, 5], fill=(40, 40, 40))
    return d


def ammo_icon(col):
    def d(dr, img):
        for x in (4, 8, 12):
            dr.rectangle([x - 1, 5, x, 12], fill=col)
            dr.rectangle([x - 1, 3, x, 4], fill=(200, 140, 60))
    return d


def main():
    lang_p = ROOT / "assets" / NS / "lang" / "en_us.json"
    lang = json.loads(lang_p.read_text())
    tabs = {}
    I, C, R, S = "minecraft:iron_ingot", "minecraft:copper_ingot", "minecraft:redstone", f"{NS}:steel_ingot"
    BC, AC, EC = f"{NS}:basic_control_circuit", f"{NS}:advanced_control_circuit", f"{NS}:elite_control_circuit"

    # Plates and scrap.
    for metal, ingot, col in [("iron", I, (200, 200, 205)), ("copper", C, (215, 120, 80)),
                              ("gold", "minecraft:gold_ingot", (240, 200, 60)), ("steel", S, (120, 120, 128))]:
        iid = f"{metal}_plate"
        icon(iid, plate_icon(col), "industrialcraft", f"{metal.capitalize()} Plate", lang, tabs)
        for kind in ("compressing", "metal_forming", "rolling"):
            w(f"data/{NS}/recipe/{kind}/orig_{iid}.json", {"type": f"{NS}:{kind}", "ingredient": ingot, "result": {"id": f"{NS}:{iid}"}})
    icon("scrap", lambda d, i: [d.rectangle([3 + 3 * k, 4 + (k % 2) * 3, 6 + 3 * k, 8 + (k % 2) * 3], fill=(110 + 20 * k, 90, 70)) for k in range(3)],
         "industrialcraft", "Scrap", lang, tabs)
    for junk in ["cobblestone", "dirt", "gravel", "sand", "netherrack", "rotten_flesh", "stick", "cobbled_deepslate"]:
        w(f"data/{NS}/recipe/recycling/orig_{junk}.json", {"type": f"{NS}:recycling", "ingredient": f"minecraft:{junk}", "result": {"id": f"{NS}:scrap"}})

    # More machines (their power use and speed are in the native mod's PROCESSORS table).
    for i, (mid, name, base, glow, centre) in enumerate([
        ("induction_furnace", "Induction Furnace", (140, 120, 110), (255, 120, 40), f"{NS}:electric_furnace"),
        ("recycler", "Recycler", (110, 110, 100), (180, 160, 120), f"{NS}:compressor"),
        ("metal_former", "Metal Former", (130, 130, 140), (200, 200, 220), "minecraft:anvil"),
        ("rolling_machine", "Rolling Machine", (120, 100, 80), (220, 200, 160), "minecraft:piston"),
    ]):
        machine_block(mid, name, base, glow, "industrialcraft" if mid != "rolling_machine" else "railcraft", lang, tabs, 500 + 10 * i)
        shaped(f"orig/{mid}", ["ICI", "RXR", "ICI"], {"I": I, "C": BC, "R": R, "X": centre}, f"{NS}:{mid}")

    # The nuclear reactor: a block with a 9x6 grid, and its parts (ids the native mod's reactor knows).
    machine_block("nuclear_reactor", "Nuclear Reactor", (70, 80, 70), (120, 255, 120), "industrialcraft", lang, tabs, 600)
    shaped("orig/nuclear_reactor", ["PCP", "CGC", "PCP"], {"P": f"{NS}:reactor_plating", "C": AC, "G": f"{NS}:coal_generator"}, f"{NS}:nuclear_reactor")
    U8, U5 = f"{NS}:uranium_238", f"{NS}:uranium_235"
    parts = [
        ("empty_fuel_rod", "Empty Fuel Rod", cell_icon((60, 60, 60)), (["P", "P", "P"], {"P": f"{NS}:iron_plate"}), 1),
        ("uranium_fuel_rod", "Uranium Fuel Rod", rod_icon(1, (90, 220, 60)), None, 1),
        ("dual_uranium_fuel_rod", "Dual Uranium Fuel Rod", rod_icon(2, (90, 220, 60)), (["RPR"], {"R": f"{NS}:uranium_fuel_rod", "P": f"{NS}:iron_plate"}), 1),
        ("quad_uranium_fuel_rod", "Quad Uranium Fuel Rod", rod_icon(4, (90, 220, 60)), ([" R ", "PCP", " R "], {"R": f"{NS}:dual_uranium_fuel_rod", "P": f"{NS}:iron_plate", "C": f"{NS}:copper_plate"}), 1),
        ("depleted_uranium_fuel_rod", "Depleted Uranium Fuel Rod", rod_icon(1, (70, 90, 60)), None, 1),
        ("depleted_dual_uranium_fuel_rod", "Depleted Dual Uranium Fuel Rod", rod_icon(2, (70, 90, 60)), None, 1),
        ("depleted_quad_uranium_fuel_rod", "Depleted Quad Uranium Fuel Rod", rod_icon(4, (70, 90, 60)), None, 1),
        ("heat_vent", "Heat Vent", vent_icon((200, 200, 210)), (["PBP", "B B", "PBP"], {"P": f"{NS}:iron_plate", "B": "minecraft:iron_bars"}), 1),
        ("advanced_heat_vent", "Advanced Heat Vent", vent_icon((120, 220, 255)), (["BVB", "BDB", "BVB"], {"B": "minecraft:iron_bars", "V": f"{NS}:heat_vent", "D": "minecraft:diamond"}), 1),
        ("casing_heat_vent", "Casing Heat Vent", vent_icon((255, 160, 60)), (["CVC"], {"C": f"{NS}:copper_plate", "V": f"{NS}:advanced_heat_vent"}), 1),
        ("high_flow_heat_vent", "High-Flow Heat Vent", vent_icon((255, 80, 80)), ([" G ", "GVG", " G "], {"G": f"{NS}:gold_plate", "V": f"{NS}:casing_heat_vent"}), 1),
        ("spreading_heat_vent", "Spreading Heat Vent", vent_icon((160, 255, 160)), ([" B ", "BVB", " B "], {"B": "minecraft:iron_bars", "V": f"{NS}:heat_vent"}), 1),
        ("heat_exchanger", "Heat Exchanger", exch_icon((255, 160, 60)), (["CPC", "PCP", "CPC"], {"C": f"{NS}:copper_plate", "P": f"{NS}:iron_plate"}), 1),
        ("advanced_heat_exchanger", "Advanced Heat Exchanger", exch_icon((120, 220, 255)), (["LXL", "CPC", "LXL"], {"L": "minecraft:lapis_lazuli", "X": f"{NS}:heat_exchanger", "C": BC, "P": f"{NS}:copper_plate"}), 1),
        ("casing_heat_exchanger", "Casing Heat Exchanger", exch_icon((255, 200, 80)), (["CCC", "CXC", "CCC"], {"C": f"{NS}:copper_plate", "X": f"{NS}:heat_exchanger"}), 1),
        ("spreading_heat_exchanger", "Spreading Heat Exchanger", exch_icon((160, 255, 160)), ([" G ", "GXG", " G "], {"G": f"{NS}:gold_plate", "X": f"{NS}:heat_exchanger"}), 1),
        ("coolant_cell_10k", "10k Coolant Cell", cell_icon((80, 160, 230)), ([" P ", "PWP", " P "], {"P": f"{NS}:iron_plate", "W": "minecraft:water_bucket"}), 1),
        ("coolant_cell_30k", "30k Coolant Cell", cell_icon((60, 120, 230)), (["PPP", "CCC", "PPP"], {"P": f"{NS}:iron_plate", "C": f"{NS}:coolant_cell_10k"}), 1),
        ("coolant_cell_60k", "60k Coolant Cell", cell_icon((40, 80, 220)), (["PCP", "PXP", "PCP"], {"P": f"{NS}:iron_plate", "C": f"{NS}:coolant_cell_30k", "X": f"{NS}:copper_plate"}), 1),
        ("redstone_condenser", "Redstone Condenser", cell_icon((220, 40, 40)), (["RRR", "RVR", "RXR"], {"R": R, "V": f"{NS}:heat_vent", "X": f"{NS}:heat_exchanger"}), 1),
        ("lapis_condenser", "Lapis Condenser", cell_icon((40, 80, 220)), (["RLR", "LCL", "RXR"], {"R": "minecraft:redstone_block", "L": "minecraft:lapis_block", "C": f"{NS}:redstone_condenser", "X": f"{NS}:casing_heat_exchanger"}), 1),
        ("reactor_plating", "Reactor Plating", plate_icon((90, 90, 100)), (["SS", "SS"], {"S": f"{NS}:steel_plate"}), 1),
        ("heat_plating", "Heat-Capacity Reactor Plating", plate_icon((150, 90, 70)), (["CCC", "CPC", "CCC"], {"C": f"{NS}:copper_plate", "P": f"{NS}:reactor_plating"}), 1),
        ("containment_plating", "Containment Reactor Plating", plate_icon((70, 70, 120)), (["SPS"], {"S": S, "P": f"{NS}:reactor_plating"}), 1),
        ("neutron_reflector", "Neutron Reflector", plate_icon((200, 200, 120)), (["CPC", "PCP", "CPC"], {"C": "minecraft:coal", "P": f"{NS}:copper_plate"}), 1),
        ("thick_neutron_reflector", "Thick Neutron Reflector", plate_icon((220, 220, 140)), ([" P ", "PRP", " P "], {"P": f"{NS}:copper_plate", "R": f"{NS}:neutron_reflector"}), 1),
        ("reinforced_neutron_reflector", "Reinforced Neutron Reflector", plate_icon((240, 240, 180)), (["RRR", "RDR", "RRR"], {"R": f"{NS}:thick_neutron_reflector", "D": "minecraft:diamond_block"}), 1),
    ]
    for iid, name, draw, recipe, stack in parts:
        icon(iid, draw, "industrialcraft", name, lang, tabs, stack=1 if "rod" in iid or "cell" in iid else 64)
        if recipe:
            shaped(f"orig/{iid}", recipe[0], recipe[1], f"{NS}:{iid}")
    w(f"data/{NS}/recipe/orig/uranium_fuel_rod.json", {"type": "minecraft:crafting_shapeless", "category": "misc",
        "ingredients": [f"{NS}:empty_fuel_rod", U5, U8, U8], "result": {"id": f"{NS}:uranium_fuel_rod"}})

    # ICBM-style missiles, launcher, designator.
    cols = {"condensed": (200, 40, 40), "incendiary": (255, 120, 20), "shrapnel": (170, 170, 180), "fragmentation": (140, 140, 150),
            "thermobaric": (230, 90, 30), "exothermic": (255, 60, 0), "endothermic": (120, 200, 255), "chemical": (120, 200, 40),
            "debilitation": (150, 90, 180), "emp": (60, 120, 255), "anvil": (70, 70, 80), "nuclear": (80, 220, 60)}
    for kind, col in cols.items():
        name = ("EMP" if kind == "emp" else kind.capitalize()) + " Missile"
        icon(f"{kind}_missile", missile_icon(col), "icbm", name, lang, tabs, stack=16, handheld=True)
        shaped(f"orig/{kind}_missile", [" X ", "IRI", "IFI"], {"X": f"{NS}:{kind}_explosive", "I": I, "R": R, "F": "minecraft:fire_charge"}, f"{NS}:{kind}_missile")
    icon("target_designator", lambda d, i: (d.rectangle([3, 6, 12, 10], fill=(70, 70, 80)), d.rectangle([11, 7, 13, 9], fill=(255, 40, 40)), d.rectangle([5, 10, 7, 13], fill=(50, 50, 55))),
         "icbm", "Target Designator", lang, tabs, stack=1, handheld=True)
    shaped("orig/target_designator", ["  G", "IRI", "I  "], {"G": "minecraft:glass", "I": I, "R": R}, f"{NS}:target_designator")
    w(f"data/{NS}/horizon/block/missile_launcher.json", {"template": "minecraft:barrel",
        "properties": {"hardness": 5.0, "explosion_resistance": 30.0, "sounds": "minecraft:iron_block", "requires_tool": True}, "item": True})
    side = plate((80, 110, 70), 700)
    ImageDraw.Draw(side).line([0, 8, 15, 8], fill=(230, 200, 30))
    gc.save(side, "block/missile_launcher_side.png")
    top = plate((80, 110, 70), 701)
    ImageDraw.Draw(top).ellipse([4, 4, 11, 11], outline=(230, 200, 30))
    gc.save(top, "block/missile_launcher_top.png")
    w(f"assets/{NS}/models/block/missile_launcher.json", {"parent": "minecraft:block/cube_bottom_top",
        "textures": {"top": f"{NS}:block/missile_launcher_top", "bottom": f"{NS}:block/missile_launcher_side", "side": f"{NS}:block/missile_launcher_side"}})
    w(f"assets/{NS}/blockstates/missile_launcher.json", {"variants": {
        f"facing={f},open={o}": {"model": f"{NS}:block/missile_launcher"} for f in ("north", "east", "south", "west", "up", "down") for o in ("false", "true")}})
    w(f"assets/{NS}/items/missile_launcher.json", {"model": {"type": "minecraft:model", "model": f"{NS}:block/missile_launcher"}})
    gc.loot("missile_launcher")
    lang[f"block.{NS}.missile_launcher"] = "Missile Launcher"
    tabs.setdefault("icbm", []).append("missile_launcher")
    shaped("orig/missile_launcher", ["I I", "ICI", "SSS"], {"I": I, "C": AC, "S": S}, f"{NS}:missile_launcher")

    # Turrets and guns.
    for i, (tid, name, col, centre) in enumerate([("machine_gun_turret", "Machine Gun Turret", (120, 120, 130), "minecraft:crossbow"),
                                                  ("incendiary_turret", "Incendiary Turret", (200, 90, 40), "minecraft:fire_charge"),
                                                  ("laser_turret", "Laser Turret", (90, 160, 220), "minecraft:diamond"),
                                                  ("rocket_turret", "Rocket Turret", (90, 120, 80), f"{NS}:condensed_explosive")]):
        machine_block(tid, name, col, (255, 60, 60), "openmodularturrets", lang, tabs, 800 + 10 * i)
        shaped(f"orig/{tid}", [" X ", "ICI", "IBI"], {"X": centre, "I": I, "C": BC, "B": f"{NS}:battery_box"}, f"{NS}:{tid}")
    for gid, name, col, ammo, aname, acol, recipe in [
        ("pistol", "Pistol", (90, 90, 95), "pistol_rounds", "Pistol Rounds", (180, 150, 60), (["II ", " IR"], {"I": I, "R": R})),
        ("assault_rifle", "Assault Rifle", (60, 70, 60), "rifle_rounds", "Rifle Rounds", (160, 140, 60), (["SSS", " CR"], {"S": S, "C": BC, "R": R})),
        ("minigun", "Minigun", (110, 110, 120), "minigun_drum", "Minigun Drum", (140, 140, 150), (["SSS", "SXC"], {"S": S, "X": f"{NS}:basic_energy_cube", "C": AC})),
        ("rocket_launcher", "Rocket Launcher", (70, 90, 60), "rocket", "Rocket", (200, 60, 50), (["SSS", " CT"], {"S": S, "C": AC, "T": "minecraft:tnt"})),
    ]:
        icon(gid, gun_icon(col), "techguns", name, lang, tabs, stack=1, handheld=True)
        shaped(f"orig/{gid}", recipe[0], recipe[1], f"{NS}:{gid}")
        icon(ammo, ammo_icon(acol), "techguns", aname, lang, tabs)
        shaped(f"orig/{ammo}", ["C", "G", "I"] if ammo != "rocket" else [" I ", "ITI", "IGI"],
               {"C": C, "G": "minecraft:gunpowder", "I": I, "T": "minecraft:tnt"} if ammo != "rocket" else {"I": I, "T": "minecraft:tnt", "G": "minecraft:gunpowder"},
               f"{NS}:{ammo}", 16 if ammo != "rocket" else 2)

    titles = {"techguns": ("Voltz Arms", "voltz:assault_rifle"), "openmodularturrets": ("Voltz Turrets", "voltz:laser_turret")}
    for tab, ids in tabs.items():
        p = ROOT / "data" / NS / "horizon" / "creative_tab" / f"{tab}.json"
        if p.exists():
            d = json.loads(p.read_text())
        else:
            title, icon_id = titles.get(tab, (tab, f"{NS}:{ids[0]}"))
            d = {"title": f"itemGroup.{NS}.{tab}", "icon": icon_id, "items": []}
            lang[f"itemGroup.{NS}.{tab}"] = title
        d["items"] += [f"{NS}:{i}" for i in ids if f"{NS}:{i}" not in d["items"]]
        p.write_text(json.dumps(d, indent=2) + "\n")
    lang_p.write_text(json.dumps(lang, indent=2) + "\n")
    tags = ROOT / "data" / "minecraft" / "tags" / "block" / "mineable" / "pickaxe.json"
    t = json.loads(tags.read_text())
    t["values"] += [f"{NS}:{m}" for m in ["induction_furnace", "recycler", "metal_former", "rolling_machine", "nuclear_reactor",
                                          "missile_launcher", "machine_gun_turret", "incendiary_turret", "laser_turret", "rocket_turret"]]
    tags.write_text(json.dumps(t, indent=2) + "\n")
    print("originals:", sum(len(v) for v in tabs.values()), "items/blocks")


main()
