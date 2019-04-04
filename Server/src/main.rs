#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

use std::path::PathBuf;
use std::iter::*;
use std::sync::RwLock;
use std::hash::Hash;
use std::fmt::{Formatter, Error};
use std::collections::HashMap;
use rocket::{Config, State};
use rocket::config::Environment;
use rocket_contrib::json::Json;
use rocket_contrib::serve::{Options, StaticFiles};
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Unit
{
    Liter(f32),
    Milliliter(f32),
    TableSpoon(f32),
    TeaSpoon(f32),
    CoffeeSpoon(f32),

    Gram(f32),
    Kilogram(f32),

    Count(i32),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Recipe
{
    name: String,
    description: String,
    guide: String,
    image_path: String,
    ingredients: HashMap<String, Unit>,
}

pub type Recipes = RwLock<Vec<Recipe>>;

impl Unit
{
    pub fn value(&self) -> f32
    {
        match self {
            // Volume
            Unit::Liter(x) => *x,
            Unit::Milliliter(x) => *x,
            Unit::TableSpoon(x) => *x,
            Unit::TeaSpoon(x) => *x,
            Unit::CoffeeSpoon(x) => *x,

            // Mass
            Unit::Gram(x) => *x,
            Unit::Kilogram(x) => *x,
            Unit::Count(x) => *x as f32,
        }
    }

    pub fn to_si(&self) -> Self
    {
        match *self {
            // Volume
            Unit::Liter(x) => Unit::Liter(x),
            Unit::Milliliter(x) => Unit::Liter(x * 1e-3_f32),
            Unit::TableSpoon(x) => Unit::Liter(x * 15e-3_f32),
            Unit::TeaSpoon(x) => Unit::Liter(x * 5e-3_f32),
            Unit::CoffeeSpoon(x) => Unit::Liter(x * 2e-3_f32),

            // Mass
            Unit::Gram(x) => Unit::Gram(x),
            Unit::Kilogram(x) => Unit::Gram(x * 1000f32),

            // Other
            Unit::Count(x) => Unit::Count(x),
        }
    }
}

impl std::fmt::Display for Recipe {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let formatted = format!(
            "Recipe[name={};description={};guide={}]",
            self.name,
            self.description,
            self.guide,
        );

        f.write_str(formatted.as_str())
    }
}

pub fn zip_to_map<'a, T, KeyIter, ValueIter>(
    key_iter: KeyIter,
    value_iter: ValueIter,
) -> HashMap<T, T>
    where
        T: 'a + Hash + Eq + Clone,
        KeyIter: Iterator<Item=&'a T>,
        ValueIter: Iterator<Item=&'a T>,
{
    key_iter.cloned().zip(value_iter.cloned()).collect::<HashMap<T, T>>()
}

pub fn score_recipe(query: &HashMap<String, f32>, recipe: &Recipe) -> f32
{
    let mut score = 0f32;

    for (needed_name, needed_amount) in &recipe.ingredients {
        if !query.contains_key(needed_name) {
            score += needed_amount.value();
        } else {
            let diff = query[needed_name] - needed_amount.value();

            if diff < 0f32 {
                score += diff;
            }
        }
    }

    score
}

pub fn parse_parameter_tuple(parameter: (String, String)) -> Option<(String, f32)>
{
    parse_parameter(parameter.0, parameter.1)
}

pub fn parse_parameter(name: String, amount_str: String) -> Option<(String, f32)>
{
    let maybe_space_position = amount_str.find('_');
    let space_pos = if maybe_space_position.is_none() {
        return None;
    } else {
        maybe_space_position.unwrap()
    };

    let maybe_amount = amount_str[..space_pos].parse::<f32>();
    let amount = if maybe_amount.is_err() {
        // Error
        return None;
    } else {
        maybe_amount.unwrap()
    };

    let units_amount = match &amount_str[(space_pos + 1)..] {
        // Volume
        "liters" => Unit::Liter(amount),
        "milliliters" => Unit::Milliliter(amount),
        "tablespoons" => Unit::TableSpoon(amount),
        "teaspoons" => Unit::TeaSpoon(amount),
        "coffeespoon" => Unit::CoffeeSpoon(amount),

        // Mass
        "grams" => Unit::Gram(amount),
        "kilograms" => Unit::Kilogram(amount),

        // Other
        "count" => Unit::Count(amount as i32),

        // Error
        _ => return None
    };

    if units_amount.value() == -1f32 {
        return None;
    }

    Some((name, units_amount.to_si().value()))
}

pub fn parse_parameters(query: PathBuf) -> HashMap<String, f32>
{
    // PathBuf provides OsStr objects, that are clunky. Convert
    // to normal Strings.
    let search_params = query.iter()
        .map(|x| x.to_string_lossy().to_string())
        .collect::<Vec<String>>();

    let search_string_map: HashMap<String, String> = zip_to_map(
        search_params.iter().step_by(2),
        search_params.iter().skip(1).step_by(2),
    );

    return search_string_map
        .into_iter()
        .filter_map(parse_parameter_tuple)
        .collect();
}

pub fn has_invalid_amounts(query: &HashMap<String, f32>) -> bool
{
    return query.values().find(|&&x| x == -1f32).is_some();
}

pub fn score_all_recipes<'r>(
    query: &HashMap<String, f32>,
    recipes: &'r Vec<Recipe>,
) -> Vec<(f32, &'r Recipe)>
{
    let mut scored_recipes = recipes
        .iter()
        .map(|r| (score_recipe(&query, r), r))
        .collect::<Vec<_>>();

    scored_recipes.sort_by_key(|r| (r.0 * 1000f32) as i32);
    scored_recipes
}

#[get("/search/<query..>")]
pub fn api_search(recipes_sync: State<Recipes>, query: PathBuf) -> Json<Vec<Recipe>>
{
    let recipes = recipes_sync.read().unwrap();

    if query.iter().count() % 2 == 1 {
        println!("[api][search] Odd number of arguments, skipping");
        return Json(Vec::new());
    }

    let search_map: HashMap<String, f32> = parse_parameters(query);

    if has_invalid_amounts(&search_map) {
        return Json(Vec::new());
    }
    let scored_recipes: Vec<(f32, &Recipe)>
        = score_all_recipes(&search_map, &*recipes);

    if !scored_recipes.is_empty() {
        let can_make_now: Vec<_> = scored_recipes
            .iter()
            .take_while(|&&r| r.0 <= 0.0)
            .map(|&r| r)
            .collect();

        let can_make_any_recipe = !can_make_now.is_empty();
        let recipe_iterator = if !can_make_any_recipe {
            scored_recipes.into_iter()
        } else {
            can_make_now.into_iter()
        };

        return Json(recipe_iterator.map(|x| x.1.clone()).collect());
    }

    return Json(Vec::new());
}


/// WEB SERVER STUFF ///
#[get("/all")]
pub fn api_all_recipes(state: State<Recipes>) -> Json<Vec<Recipe>>
{
    return Json(state.read().unwrap().to_vec());
}

#[post("/add_recipe", format = "json", data = "<recipe>")]
pub fn api_add_recipe(recipes_sync: State<Recipes>, recipe: Json<Recipe>) -> Json<bool>
{
    let real_recipe: Recipe = recipe.into_inner();
    let is_name_taken = recipes_sync.read().unwrap().iter()
        .find(|&r| r.name == real_recipe.name)
        .is_some();

    if is_name_taken {
        let mut recipes = recipes_sync.write().unwrap();
        println!("[api][add_recipe] Adding new recipe {}", real_recipe);
        (*recipes).push(real_recipe);
    }


    return Json(!is_name_taken);
}

#[allow(dead_code)]
fn main()
{
    let cfg = Config::build(Environment::Development)
        .address("0.0.0.0")
        .port(80)
        .unwrap();

    let mut musaka_ingredients: HashMap<String, Unit> = HashMap::new();
    musaka_ingredients.insert("Кайма".to_lowercase(), Unit::Gram(500f32));
    musaka_ingredients.insert("Картофи".to_lowercase(), Unit::Kilogram(1f32));
    musaka_ingredients.insert("Лук".to_lowercase(), Unit::Count(2i32));
    musaka_ingredients.insert("Домати".to_lowercase(), Unit::Count(2i32));
    musaka_ingredients.insert("Кисело мляко".to_lowercase(), Unit::Gram(400f32));
    musaka_ingredients.insert("Яйца".to_lowercase(), Unit::Gram(900f32));
    musaka_ingredients.insert("Кашкавал".to_lowercase(), Unit::Gram(900f32));
    musaka_ingredients.insert("Брашно".to_lowercase(), Unit::Gram(900f32));
    musaka_ingredients.insert("Олио".to_lowercase(), Unit::Gram(900f32));
    musaka_ingredients.insert("Червен пипер".to_lowercase(), Unit::TeaSpoon(1f32));
    musaka_ingredients.insert("Чубрица".to_lowercase(), Unit::TeaSpoon(1f32));


    musaka_ingredients.iter_mut().for_each(|(_name, amount)| {
        *amount = amount.to_si();
    });

    let mut recipes: Vec<Recipe> = Vec::new();
    recipes.push(Recipe {
        name: "Мусака".to_string(),
        description: "Best food ever.".to_string(),
        guide: "I have no idea how to make it, but it is incredible. Only women know, and women don't tell.".to_string(),
        image_path: "/res/img/instructionArea.png".to_string(),
        ingredients: musaka_ingredients,
    });

    let mut musaka_ingredients: HashMap<String, Unit> = HashMap::new();
    musaka_ingredients.insert("чушки".to_lowercase(), Unit::Count(4));

    musaka_ingredients
        .iter_mut()
        .for_each(|(_ingredient, amount)| {
            *amount = amount.to_si();
        });
    recipes.push(Recipe {
        name: "Мусака2".to_string(),
        description: "Best food ever.".to_string(),
        guide: "I have no idea how to make it, but it is incredible. Only women know, and women don't tell.".to_string(),
        image_path: "/res/img/instructionArea.png".to_string(),
        ingredients: musaka_ingredients,
    });


    let rocket = rocket::custom(cfg)
        .manage(RwLock::new(recipes))
        .mount("/", StaticFiles::new(r"C:\Stuff\Projects\CookingBook_Web", Options::Index))
        .mount("/api/", routes![api_search])
        .mount("/api/", routes![api_add_recipe])
        ;

    rocket.launch();
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::thread::JoinHandle;

    fn start_web_server() -> JoinHandle<()> {
        thread::spawn(main)
    }

    #[test]
    fn test_web_server() {
        let server = start_web_server();

//        use rocket_contrib::helmet::
    }
}