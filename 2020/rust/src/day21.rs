use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug)]
struct Food {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}

#[derive(Debug)]
struct Input {
    foods: Vec<Food>,
    ingredients: HashMap<String, usize>,
    allergens: HashSet<String>,
}

impl FromStr for Input {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut ingredients: HashMap<String, usize> = HashMap::new();
        let mut allergens: HashSet<String> = HashSet::new();
        let mut foods: Vec<Food> = Vec::new();

        for line in input.lines() {
            let mut parts = line.split(" (contains ");
            let mut food_ingredients: HashSet<String> = HashSet::new();

            for ingredient in parts.next().unwrap().split(' ') {
                *ingredients.entry(ingredient.to_owned()).or_insert(0) += 1;

                food_ingredients.insert(ingredient.to_owned());
            }

            let mut food_allergens: HashSet<String> = HashSet::new();

            for allergen in parts.next().unwrap().strip_suffix(")").unwrap().split(", ") {
                allergens.insert(allergen.to_owned());
                food_allergens.insert(allergen.to_owned());
            }

            foods.push(Food {
                ingredients: food_ingredients,
                allergens: food_allergens,
            });
        }

        Ok(Self {
            foods,
            ingredients,
            allergens,
        })
    }
}

#[must_use]
pub fn part1(input_string: &str) -> usize {
    let input: Input = input_string.parse().unwrap();
    let allergen_map: HashMap<&String, &String> = map_ingredients_to_allergens(&input);

    input
        .ingredients
        .iter()
        .filter_map(|(ingredient, count)| {
            if allergen_map.contains_key(ingredient) {
                None
            } else {
                Some(count)
            }
        })
        .sum()
}

fn map_ingredients_to_allergens(input: &Input) -> HashMap<&String, &String> {
    let mut possibles: HashMap<&String, Vec<&String>> = HashMap::new();
    let mut definites: HashMap<&String, &String> = HashMap::new();

    for allergen in &input.allergens {
        possibles.insert(allergen, find_overlap(allergen, &input.foods));
    }

    loop {
        if possibles.is_empty() {
            break;
        }

        let unmatched_allergens: Vec<&String> = possibles.keys().copied().collect();

        for allergen in &unmatched_allergens {
            if let Some(ingredients) = possibles.get(allergen) {
                if let [definite] = ingredients[..] {
                    definites.insert(definite, allergen);
                    possibles.remove(allergen);
                    for ingredients in possibles.values_mut() {
                        ingredients.retain(|&ingredient| ingredient != definite);
                    }
                }
            }
        }
    }

    definites
}

fn find_overlap<'a>(allergen: &'a str, foods: &'a [Food]) -> Vec<&'a String> {
    let mut matching_foods = foods
        .iter()
        .filter(|food| food.allergens.contains(allergen));

    let mut common: HashSet<&String> = matching_foods.next().unwrap().ingredients.iter().collect();

    for food in matching_foods {
        common = common
            .intersection(&food.ingredients.iter().collect())
            .copied()
            .collect();
    }

    common.into_iter().collect()
}

#[must_use]
pub fn part2(input_string: &str) -> String {
    let input: Input = input_string.parse().unwrap();
    let mut allergen_list: Vec<(&String, &String)> =
        map_ingredients_to_allergens(&input).into_iter().collect();

    allergen_list.sort_by(|a, b| a.1.cmp(b.1));

    allergen_list
        .into_iter()
        .map(|(ingredient, _allergen)| ingredient.as_str())
        .collect::<Vec<&str>>()
        .join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "\
mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)
"
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(input()), 5);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(input()), "mxmxvkd,sqjhc,fvjkl");
    }
}
