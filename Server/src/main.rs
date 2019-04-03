#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

use std::io::Cursor;
use std::path::PathBuf;
use std::iter::*;
use std::ffi::OsStr;
use std::hash::Hash;
use std::collections::HashMap;
use std::cmp::Ordering;
use json::{JsonValue, Array};
use json::object::Object;
use rayon::slice::ParallelSliceMut;
use rayon::iter::ParallelIterator;
use rocket::{Request, Response, Config, State};
use rocket::http::Status;
use rocket::response::Responder;
use rocket::config::Environment;
use rocket_contrib::serve::{Options, StaticFiles};

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

pub struct Recipe
{
    name: String,
    description: String,
    guide: String,
    image_path: String,
    ingredients: HashMap<String, Unit>,
}

pub struct JsonResponse
{
    value: json::JsonValue
}

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

    pub fn name(&self) -> &'static str
    {
        match self {
            // Volume
            Unit::Liter(_) => "liters",
            Unit::Milliliter(_) => "milliliters",
            Unit::TableSpoon(_) => "tablespoons",
            Unit::TeaSpoon(_) => "teaspoons",
            Unit::CoffeeSpoon(_) => "coffeespoon",

            // Mass
            Unit::Gram(_) => "grams",
            Unit::Kilogram(_) => "kilograms",

            // Other
            Unit::Count(_) => "count"
        }
    }

    pub fn to_json(&self) -> JsonValue
    {
        let mut obj = Object::new();
        obj.insert(self.name(), JsonValue::from(self.value()));
        JsonValue::from(obj)
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

    fn to_short_string(&self) -> String
    {
        match *self {
            // Volume
            Unit::Liter(x) => format!("{:00} л.", x),
            Unit::Milliliter(x) => format!("{:00} мл.", x),
            Unit::TableSpoon(x) => format!("{:00} с.л.", x),
            Unit::TeaSpoon(x) => format!("{:00} ч.л.", x),
            Unit::CoffeeSpoon(x) => format!("{:00} к.л.", x),

            // Mass
            Unit::Gram(x) => format!("{} г.", x),
            Unit::Kilogram(x) => format!("{} кг.", x),

            // Other
            Unit::Count(x) => format!("{} бр.", x),
        }
    }
}

impl Recipe
{
    pub fn to_json(&self) -> JsonValue
    {
        let mut tree = Object::new();
        tree.insert("name", JsonValue::String(self.name.clone()));
        tree.insert("description", JsonValue::String(self.description.clone()));
        tree.insert("guide", JsonValue::String(self.guide.clone()));
        tree.insert("image_path", JsonValue::String(self.image_path.clone()));


        let mut ingredients = Object::new();
        for (ingredient, units) in &self.ingredients {
            ingredients.insert(ingredient, units.to_json());
        }

        tree.insert("ingredients", JsonValue::from(ingredients));
        JsonValue::Object(tree)
    }
}

impl JsonResponse
{
    fn new(json_value: json::JsonValue) -> Self
    {
        JsonResponse { value: json_value }
    }
}

impl From<JsonValue> for JsonResponse
{
    fn from(json_value: JsonValue) -> Self
    {
        JsonResponse::new(json_value)
    }
}

impl ToString for JsonResponse
{
    fn to_string(&self) -> String
    {
        self.value.to_string()
    }
}

impl<'r> Responder<'r> for JsonResponse
{
    fn respond_to(self, _request: &Request) -> Result<Response<'r>, Status>
    {
        Response::build()
            .header(rocket::http::ContentType::JSON)
            .sized_body(Cursor::new(self.value.to_string()))
            .ok()
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
pub fn api_search(recipes: State<Vec<Recipe>>, query: PathBuf) -> JsonResponse
{
    let search_map: HashMap<String, f32> = parse_parameters(query);

    if has_invalid_amounts(&search_map) {
        return JsonResponse { value: JsonValue::new_object() };
    }

    let scored_recipes: Vec<(f32, &Recipe)>
        = score_all_recipes(&search_map, recipes.inner());

    let mut response = Object::new();
    let mut found_recipes: Vec<JsonValue> = Array::new();
    if !scored_recipes.is_empty() {
        let can_make_now: Vec<_> = scored_recipes
            .iter()
            .take_while(|&&r| r.0 <= 0.0)
            .map(|&r| r)
            .collect();


        let iter = if can_make_now.is_empty() {
            response.insert("can_make_any", JsonValue::Boolean(false));
            scored_recipes.iter()
        } else {
            response.insert("can_make_any", JsonValue::Boolean(true));
            can_make_now.iter()
        };

        for &(score, recipe) in iter.take(128) {
            found_recipes.push(recipe.to_json());
        }
    }

    response.insert("recipes", JsonValue::Array(found_recipes));
    JsonResponse { value: JsonValue::Object(response) }
}

/// WEB SERVER STUFF ///
#[get("/all")]
pub fn api_all_recipes(state: State<Vec<Recipe>>) -> JsonResponse
{
    JsonResponse { value: JsonValue::from("{}") }
}

#[post("/addrecipe/<recipe>")]
pub fn api_addrecipe(state: State<Vec<Recipe>>, recipe: String)
{
    let maybe_recipe_object = json::parse(&recipe);
    if maybe_recipe_object.is_err() {
        println!("Couldn't parse json {}", recipe);
    }

//    JsonResponse::from(JsonValue::new_object());
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


    musaka_ingredients.iter_mut().for_each(|(x, y)| {
        *y = y.to_si();
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
    recipes.push(Recipe {
        name: "Мусака2".to_string(),
        description: "Best food ever.".to_string(),
        guide: "I have no idea how to make it, but it is incredible. Only women know, and women don't tell.".to_string(),
        image_path: "/res/img/instructionArea.png".to_string(),
        ingredients: musaka_ingredients,
    });


    let rocket = rocket::custom(cfg)
        .manage(recipes)
        .mount("/", StaticFiles::new(r"C:\Stuff\Projects\CookingBook", Options::Index))
        .mount("/api/", routes![api_search])
        .mount("/api/", routes![api_addrecipe])
        ;

    rocket.launch();
}