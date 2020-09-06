#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
extern crate chrono;
extern crate dotenv;
extern crate uuid;
#[macro_use] extern crate rocket_contrib;
extern crate serde;

use uuid::Uuid;
use rocket_contrib::json::{Json, JsonValue};
use serde::{Serialize, Deserialize};

// mods

mod database;

pub mod schema;
pub mod models;

// request bodies

#[derive(Serialize, Deserialize)]
struct RecipeRequestBody {
    recipe_name: String,
    ingredients: Vec<String>
}

// endpoints

#[get("/ping")]
fn ping() -> &'static str {
    "pong"
}

#[post("/v1/create_recipe", format = "json", data = "<recipe_body>")]
fn create_recipe(recipe_body: Json<RecipeRequestBody>) -> JsonValue {
    let recipe_id = Uuid::new_v4().to_string();
    let connection = database::init_db();
    // TODO: verify input
    // handle errors
    models::create_recipe(&connection, &recipe_id, &recipe_body.recipe_name, &recipe_body.ingredients);
    json!({
        "status": "201",
        "recipe_id": recipe_id,
    })
}

#[get("/v1/recipe/<rid>", format = "json")]
fn recipe(rid: Option<String>) -> JsonValue {
    let connection = database::init_db();
    let result = models::read_recipe(&connection, rid.unwrap());
    match result {
        Ok(recipe) => json!(recipe),
        Err(_) => json!({"status": "404"})
    }
}

fn main() {
    // TODO: enrich database (separate process)
    // TODO: more endpoints
    rocket::ignite().mount("/", routes![ping, create_recipe, recipe]).launch();
}