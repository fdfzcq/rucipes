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
pub mod recipes;
pub mod ingredients;
pub mod steps;

// const

// TODO: change this magic number to something more reasonable
const MAX_PAGE_SIZE: i64 = 500;

// request bodies

#[derive(Serialize, Deserialize)]
pub struct RecipeRequestBody {
    recipe_name: String,
    ingredients: Vec<IngredientRequestBody>
}

#[derive(Serialize, Deserialize)]
pub struct IngredientRequestBody {
    ingredient_name: String,
    unit: String,
    quantity: i32
}

#[derive(Serialize, Deserialize)]
pub struct StepRequestBody {
    content: String
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
    let result = recipes::create_recipe(&connection,
                                       &recipe_id,
                                       &recipe_body.recipe_name,
                                       &recipe_body.ingredients);
    match result {
        Ok(_) =>
            json!({"status": 201,
                   "recipe_id": recipe_id}),
        Err(_) =>
            // TODO: more granular errors
            json!({"status": 400})
    }
}

#[get("/v1/recipe/<rid>")]
fn get_recipe(rid: Option<String>) -> JsonValue {
    let connection = database::init_db();
    let result = recipes::read_recipe(&connection, rid.unwrap());
    match result {
        Ok(recipe) => json!(recipe),
        Err(_) => json!({"status": 404})
    }
}

#[delete("/v1/recipe/<rid>")]
fn delete_recipe(rid: Option<String>) -> JsonValue {
    let connection = database::init_db();
    let result = recipes::delete_recipe(&connection, rid.unwrap());
    match result {
        Ok(_) => json!({"status": 200}),
        Err(_) => json!({"status": 404})
    }
}

#[post("/v1/recipe/<rid>", format = "json", data = "<recipe_body>")]
fn update_recipe(recipe_body: Json<RecipeRequestBody>, rid: Option<String>) -> JsonValue {
    let connection = database::init_db();
    let rid_str = rid.unwrap();
    let result = recipes::update_recipe(&connection,
                                       &rid_str,
                                       &recipe_body.recipe_name,
                                       &recipe_body.ingredients);
    match result {
        Ok(_) =>
            json!({"status": 200,
                   "recipe_id": rid_str}),
        Err(_) =>
            // TODO: more granular errors
            json!({"status": 400})
        }
}

#[post("/v1/recipe/<rid>/step/<step>", format = "json", data = "<step_body>")]
fn create_update_step(step_body: Json<StepRequestBody>, rid: Option<String>, step: Option<i32>) -> JsonValue {
    let connection = database::init_db();
    let rid_str = rid.unwrap();
    let step_number = step.unwrap();
    let result = steps::add_recipe_step(&connection,
                                        &step_number,
                                        &step_body.content,
                                        &rid_str);
    match result {
        Ok(_) =>
            json!({"status": 201,
                   "recipe_id": rid_str}),
        Err(_) =>
            // TODO: more granular errors
            json!({"status": 400})
        }
}

#[get("/v1/recipe/<rid>/step/<step>")]
fn get_recipe_step(rid: Option<String>, step: Option<i32>) -> JsonValue {
    let connection = database::init_db();
    let result = steps::read_step(&connection, rid.unwrap(), step.unwrap());
    match result {
        Ok(step) => json!(step),
        Err(_) => json!({"status": 404})
    }
}

#[delete("/v1/recipe/<rid>/step/<step>")]
fn delete_recipe_step(rid: Option<String>, step: Option<i32>) -> JsonValue {
    let connection = database::init_db();
    let result = steps::delete_step(&connection, rid.unwrap(), step.unwrap());
    match result {
        Ok(_) => json!({"status": 200}),
        Err(_) => json!({"status": 404})
    }
}

#[get("/v1/all_recipes?<page_size>&<page>")]
fn all_recipes(page_size: Option<i64>, page: Option<i64>) -> JsonValue {
    let connection = database::init_db();
    let result = recipes::all_recipes(&connection, page_size.unwrap_or(MAX_PAGE_SIZE), page.unwrap_or(1));
    match result {
        Ok(recipes) => json!(recipes),
        Err(_) => json!({"status": 400})
    }
}

fn main() {
    // TODO: enrich database (separate process)
    // TODO: more endpoints
    rocket::ignite().mount("/",
        routes![
            ping,
            create_recipe,
            get_recipe,
            delete_recipe,
            update_recipe,
            create_update_step,
            get_recipe_step,
            delete_recipe_step,
            all_recipes
        ]).launch();
}