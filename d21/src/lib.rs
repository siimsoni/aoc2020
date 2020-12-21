extern crate rustc_hash;
mod tokenizer;
use rustc_hash::{FxHashMap, FxHashSet};
use std::io::BufRead;
use std::str::from_utf8;
use tokenizer::{TokenKind, Tokenizer};

type ParserResult = Box<[(FxHashSet<Box<[u8]>>, FxHashSet<Box<[u8]>>)]>;

pub fn parse<R>(mut reader: R) -> ParserResult
where
    R: BufRead,
{
    let mut input = Vec::new();
    let mut tokenizer = Tokenizer::new();

    let mut result = Vec::new();
    let mut page: [u8; 4096] = [0; 4096];
    while let Ok(page_len) = reader.read(&mut page) {
        if page_len == 0 {
            break;
        }
        tokenizer.tokenize(&mut (page[..page_len].iter()));
        input.extend_from_slice(&page[..page_len]);
    }
    tokenizer.flush();

    let mut token_iter = tokenizer.tokens.iter();
    let mut pos = 0;

    while pos != input.len() {
        let mut ingredients = FxHashSet::default();
        for token in &mut token_iter {
            match token.kind {
                TokenKind::Literal => {
                    ingredients.insert(Box::from(&input[pos..pos + token.len]));
                    pos += token.len;
                }
                TokenKind::ParenthesesOpen => {
                    pos += token.len;
                    break;
                }
                _ => {
                    pos += token.len;
                }
            }
        }

        let mut allergens = FxHashSet::default();

        for token in &mut token_iter {
            match token.kind {
                TokenKind::Literal => {
                    let slice = &input[pos..pos + token.len];
                    if slice != b"contains" {
                        allergens.insert(Box::from(slice));
                    }
                    pos += token.len;
                }
                TokenKind::ParenthesesClose => {
                    pos += token.len;
                    break;
                }
                _ => {
                    pos += token.len;
                }
            }
        }

        result.push((ingredients, allergens));
    }

    result.into_boxed_slice()
}

fn remove_ingredient(
    possible_ingredients_by_allergen: &mut FxHashMap<&[u8], FxHashSet<Box<[u8]>>>,
    ingredient: &[u8],
) {
    for possible_ingredients in possible_ingredients_by_allergen.values_mut() {
        possible_ingredients.remove(ingredient);
    }
}

fn map_ingredients(parsed: &ParserResult) -> FxHashMap<&[u8], Box<[u8]>> {
    let mut possible_ingredients_by_allergen: FxHashMap<&[u8], FxHashSet<Box<[u8]>>> =
        FxHashMap::default();

    for (ingredients, allergens) in parsed.iter() {
        for allergen in allergens {
            let possible_ingredients = possible_ingredients_by_allergen
                .entry(allergen)
                .or_insert_with(|| ingredients.clone());
            possible_ingredients.retain(|i| ingredients.contains(i));
        }
    }

    let mut mapping: FxHashMap<&[u8], Box<[u8]>> = FxHashMap::default();

    let mut mapped_in_iter = Vec::new();
    loop {
        for (allergen, possible_ingredients) in possible_ingredients_by_allergen.iter() {
            if possible_ingredients.len() == 1 {
                let ingredient = possible_ingredients.iter().next().unwrap();
                mapping.insert(allergen, ingredient.clone());
                mapped_in_iter.push(ingredient.clone());
            }
        }

        for ingredient in &mapped_in_iter {
            remove_ingredient(&mut possible_ingredients_by_allergen, ingredient);
        }

        if mapped_in_iter.is_empty() {
            break;
        }

        mapped_in_iter.clear();
    }

    mapping
}

pub fn p1_solve(parsed: &ParserResult) -> Option<String> {
    let mapping = map_ingredients(parsed);

    let allergens: FxHashSet<&Box<[u8]>> =
        mapping.iter().map(|(_, ingredient)| ingredient).collect();

    Some(
        parsed
            .iter()
            .flat_map(|(ingredients, _)| ingredients)
            .filter(|i| !allergens.contains(i))
            .count()
            .to_string(),
    )
}

pub fn p2_solve(parsed: &ParserResult) -> Option<String> {
    let mapping = map_ingredients(parsed);

    let str_mapping: FxHashMap<&str, &str> = mapping
        .iter()
        .map(|(allergen, ingredient)| {
            (from_utf8(allergen).unwrap(), from_utf8(ingredient).unwrap())
        })
        .collect();

    let mut ordered: Vec<&str> = str_mapping.iter().map(|(allergen, _)| *allergen).collect();
    ordered.sort_unstable();
    let result = ordered
        .iter()
        .map(|allergen| *str_mapping.get(allergen).unwrap())
        .collect::<Vec<&str>>()
        .join(",");

    Some(result)
}
