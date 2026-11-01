// Machine recipes, read from the content mod's own recipe types
// (`data/voltz/recipe/*.json`, `"type": "voltz:crushing"` or
// `"voltz:electric_smelting"`), so a data pack can add or change them.

/// One input item to one output stack.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Recipe {
    pub input: String,
    pub output: String,
    pub count: u32,
    /// A second item the machine's other slot must hold, used up with the
    /// input (a metallurgic infuser's redstone, a combiner's cobblestone).
    pub secondary: Option<String>,
}

/// A stack as the engine sees it: item id and count.
pub type Stack = Option<(String, u32)>;

pub fn find<'a>(recipes: &'a [Recipe], input: &Stack) -> Option<&'a Recipe> {
    find_with(recipes, input, &None)
}

/// The recipe for `input`, with `second` in the other slot.
pub fn find_with<'a>(recipes: &'a [Recipe], input: &Stack, second: &Stack) -> Option<&'a Recipe> {
    let (id, n) = input.as_ref()?;
    if *n == 0 {
        return None;
    }
    let second = second.as_ref().filter(|(_, n)| *n > 0).map(|(id, _)| id.as_str());
    recipes.iter().find(|r| &r.input == id && r.secondary.as_deref().is_none_or(|s| Some(s) == second))
}

/// Whether `out` can take `recipe`'s result, `max` being the item's stack size.
pub fn fits(out: &Stack, recipe: &Recipe, max: u32) -> bool {
    match out {
        None => recipe.count <= max,
        Some((id, n)) => *id == recipe.output && n + recipe.count <= max,
    }
}

/// Reads one recipe file the way the glue hands it over: `{"type": ...,
/// "ingredient": "minecraft:iron_ore", "result": {"id": ..., "count": 2}}`.
/// Kept free of a JSON crate: the glue passes the three fields it read.
pub fn from_fields(ingredient: &str, result_id: &str, count: Option<u32>) -> Option<Recipe> {
    let norm = |s: &str| if s.contains(':') { s.to_string() } else { format!("minecraft:{s}") };
    if ingredient.is_empty() || ingredient.starts_with('#') || result_id.is_empty() {
        return None;
    }
    Some(Recipe { input: norm(ingredient), output: norm(result_id), count: count.unwrap_or(1).clamp(1, 64), secondary: None })
}

/// `from_fields` with a second ingredient (`"secondary"` in the file).
pub fn from_fields2(ingredient: &str, secondary: Option<&str>, result_id: &str, count: Option<u32>) -> Option<Recipe> {
    let mut r = from_fields(ingredient, result_id, count)?;
    r.secondary = secondary.filter(|s| !s.is_empty()).map(|s| if s.contains(':') { s.to_string() } else { format!("minecraft:{s}") });
    Some(r)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn r(i: &str, o: &str, c: u32) -> Recipe {
        from_fields(i, o, Some(c)).unwrap()
    }

    #[test]
    fn finds_and_fits() {
        let rs = vec![r("iron_ore", "voltz:iron_dust", 2), r("cobblestone", "gravel", 1)];
        let ore: Stack = Some(("minecraft:iron_ore".into(), 3));
        let rec = find(&rs, &ore).unwrap();
        assert_eq!(rec.output, "voltz:iron_dust");
        assert!(fits(&None, rec, 64));
        assert!(fits(&Some(("voltz:iron_dust".into(), 62)), rec, 64));
        assert!(!fits(&Some(("voltz:iron_dust".into(), 63)), rec, 64));
        assert!(!fits(&Some(("minecraft:gravel".into(), 1)), rec, 64));
        assert!(find(&rs, &Some(("minecraft:dirt".into(), 1))).is_none());
        assert!(find(&rs, &None).is_none());
    }

    #[test]
    fn secondary_must_be_there() {
        let rs = vec![from_fields2("iron_ingot", Some("redstone"), "voltz:enriched_alloy", None).unwrap()];
        let iron: Stack = Some(("minecraft:iron_ingot".into(), 1));
        assert!(find_with(&rs, &iron, &None).is_none());
        assert!(find_with(&rs, &iron, &Some(("minecraft:coal".into(), 1))).is_none());
        assert!(find_with(&rs, &iron, &Some(("minecraft:redstone".into(), 1))).is_some());
    }

    #[test]
    fn tags_are_refused() {
        assert!(from_fields("#minecraft:logs", "charcoal", None).is_none());
    }
}
