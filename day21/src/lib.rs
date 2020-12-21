//Looked up some solutions in other languages to help with understanding the problem better and checkout some possible ideas for solving it.

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::hash_map::Entry;

fn ingredient_no_allergen(input:&String) -> (HashMap<String,HashSet<String>>,HashMap<String,usize>){
    let mut allergen_map:HashMap<String,HashSet<String>> = HashMap::new();
    let mut ingredient_occurrences:HashMap<String,usize> = HashMap::new();
    for l in input.lines(){
        let contains = l.find(" (contains ").unwrap();
        let ingredients = l[..contains].split(' ').map(String::from).collect::<HashSet<String>>();
        let allergens = l[(contains+11)..(l.len()-1)].split(", ").map(String::from).collect::<Vec<String>>();

        for allergen in allergens.iter(){
            match allergen_map.entry(allergen.clone()){
                Entry::Vacant(entry) => {entry.insert(ingredients.clone());},
                Entry::Occupied(mut entry) => {
                    *entry.get_mut() = entry.get()
                        .intersection(&ingredients)
                        .map(|i| i.clone())
                        .collect()
                }
            }
        }
        for ingredient in ingredients.iter(){
            *ingredient_occurrences.entry(ingredient.clone()).or_insert(0)+=1
        }
    }

    for (_,ingredients) in allergen_map.iter(){
        for ingredient in ingredients.iter(){
            ingredient_occurrences.remove(ingredient);
        }
    }
    (allergen_map,ingredient_occurrences)
}

pub fn count_ingredients_no_allergen(input:&String) -> usize{
    let (_,ingredient_occurrences) = ingredient_no_allergen(input);

    let mut sum = 0;
    for (_, occurrences) in ingredient_occurrences.iter() {
        sum += *occurrences;
    }

    sum
}

pub fn danger_list(input:&String) -> String{
    let (mut allergen_map,_) = ingredient_no_allergen(input);
    let mut danger:Vec<(String,String)> = Vec::new();

    while !allergen_map.is_empty(){
        for (allergen,ingredient_set) in allergen_map.iter(){
            if ingredient_set.len() == 1{
                let ingredient = ingredient_set.iter().next().unwrap();
                danger.push((allergen.clone(),ingredient.clone()))
            }
        }

        for(allergen,_) in danger.iter(){
            allergen_map.remove(allergen);
        }

        for (_,ingredient_set) in allergen_map.iter_mut(){
            for (_, ingredient) in danger.iter(){
                ingredient_set.remove(ingredient);
            }
        }
    }
    danger.sort_by_key(|(allergen,_)| allergen.clone());
    let mut canonical_danger_list = String::new();
    for (i,(_,ingredient)) in danger.iter().enumerate(){
        canonical_danger_list += ingredient;
        if i != danger.len()-1{
            canonical_danger_list += ",";
        }
    }
    canonical_danger_list
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day21_test() {
        let input = String::from("mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)");
        assert_eq!(5,count_ingredients_no_allergen(&input))
    }

    #[test]
    fn day21_test_2() {
        let input = String::from("mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)");
        assert_eq!("mxmxvkd,sqjhc,fvjkl",danger_list(&input))
    }
}