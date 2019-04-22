#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

use std::path::{PathBuf, Path};
use std::env::args;
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
    id: i32,
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
        "liter" => Unit::Liter(amount),
        "milliliter" => Unit::Milliliter(amount),
        "tablespoon" => Unit::TableSpoon(amount),
        "teaspoon" => Unit::TeaSpoon(amount),
        "coffeespoon" => Unit::CoffeeSpoon(amount),

        // Mass
        "gram" => Unit::Gram(amount),
        "kilogram" => Unit::Kilogram(amount),

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
        .map(|x| x.to_string_lossy().trim().to_lowercase())
        .collect::<Vec<String>>();

    println!("[DEBUG] Searching for [1] {:?}", search_params);

    let search_string_map: HashMap<String, String> = zip_to_map(
        search_params.iter().step_by(2),
        search_params.iter().skip(1).step_by(2),
    );

    println!("[DEBUG] Searching for [2] {:?}", search_string_map);

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

pub fn save_recipes(recipes: &Vec<Recipe>) {
    let result_json = serde_json::to_string(recipes);

    if result_json.is_ok() {
        let res = std::fs::write(
            Path::new("persistence.json"),
            result_json.unwrap(),
        );

        if res.is_ok() {
            println!("[INFO ] Saved recipes!");
        } else {
            println!("[ERROR] Failed when trying to save recipes.");
        }
    } else {
        println!("[ERROR] Failed serializing the recipes.")
    }
}

/// WEB SERVER STUFF ///
#[get("/search/<query..>")]
pub fn api_search(recipes_sync: State<Recipes>, query: PathBuf) -> Json<Vec<Recipe>>
{
    let recipes = recipes_sync.read().unwrap();

    if query.iter().count() % 2 == 1 {
        println!("[api][search] Odd number of arguments, skipping");
        return Json(Vec::new());
    }

    println!("[DEBUG] query {:?}", query);
    let search_map: HashMap<String, f32> = parse_parameters(query);
    println!("[DEBUG] Search map [Final] {:?}", search_map);

    if has_invalid_amounts(&search_map) || search_map.is_empty() {
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
        let recipe_iterator = if can_make_any_recipe {
            println!("[ INFO] Can make some recipes");
            can_make_now.into_iter()
        } else {
            println!("[ INFO] Can't make any recipes");
            scored_recipes.into_iter()
        };

        return Json(recipe_iterator.map(|x| x.1.clone()).collect());
    }

    return Json(Vec::new());
}

#[get("/all")]
pub fn api_all_recipes(state: State<Recipes>) -> Json<Vec<Recipe>>
{
    return Json(state.read().unwrap().to_vec());
}

#[post("/add_recipe", format = "json", data = "<recipe>")]
pub fn api_add_recipe(recipes_sync: State<Recipes>, recipe: Json<Recipe>) -> Json<bool>
{
    let mut real_recipe: Recipe = recipe.into_inner();
    real_recipe.name = real_recipe.name.trim().to_string();
    let is_name_taken = recipes_sync.read().unwrap().iter()
        .find(|&r| r.name == real_recipe.name)
        .is_some();

    if !is_name_taken {
        let mut recipes = recipes_sync.write().unwrap();

        let maybe_max_id = recipes.iter().map(|r| r.id).max();
        real_recipe.id = maybe_max_id.unwrap_or_else(|| 0) + 1;
        real_recipe.ingredients.iter_mut().for_each(
            |(_name, amount)| {
                *amount = amount.to_si()
            }
        );

        let mut ingredients_trimmed: HashMap<String, Unit> = HashMap::new();
        for (name, amount) in real_recipe.ingredients {
            ingredients_trimmed.insert(name.trim().to_lowercase(), amount);
        }

        real_recipe.ingredients = ingredients_trimmed;


        println!("[ INFO][api][add_recipe] Adding new recipe {}", real_recipe);
        (*recipes).push(real_recipe);

        save_recipes(&recipes);
    }

    return Json(!is_name_taken);
}

#[allow(dead_code)]
fn main()
{
    let mut port = 80u16;
    let mut webroot = r"C:\Stuff\Projects\CookingBook_Web".to_string();

    for arg in args() {
        if arg.starts_with("--port=") {
            port = arg[7..].to_string().parse::<u16>().unwrap();
        } else if arg.starts_with("--webroot=") {
            webroot = arg[10..].to_string();
        }
    }

    let cfg = Config::build(Environment::Development)
        .address("0.0.0.0")
        .port(port)
        .unwrap();

    let mut recipes: Vec<Recipe> = Vec::new();

    let maybe_saved_recipes_str
        = std::fs::read_to_string(Path::new("persistence.json"));

    if maybe_saved_recipes_str.is_ok() {
        let maybe_saved_recipes: Result<Vec<Recipe>, _> =
            serde_json::from_str(maybe_saved_recipes_str.unwrap().as_str());

        if maybe_saved_recipes.is_ok() {
            recipes = maybe_saved_recipes.unwrap();
            println!("[ INFO] Loaded {} recipes", recipes.len());
        }
    }

    println!("{}", webroot);
    let rocket = rocket::custom(cfg)
        .manage(RwLock::new(recipes))
        .mount("/", StaticFiles::new(webroot, Options::Index))
        .mount("/api/", routes![api_search])
        .mount("/api/", routes![api_add_recipe])
        ;

    rocket.launch();
}
