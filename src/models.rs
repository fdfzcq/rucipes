use chrono::NaiveDateTime;
use super::schema::recipe;
use diesel::prelude::*;

#[derive(Insertable)]
#[table_name="recipe"]
pub struct NewRecipe<'a> {
    pub id: &'a str,
    pub recipe_name: &'a str,
    pub ingredients: &'a Vec<String>,
}

#[derive(Queryable)]
pub struct Recipe {
    pub id: String,
    pub recipe_name: String,
    pub ingredients: Vec<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub fn create_recipe<'a>(conn: &diesel::pg::PgConnection, id: &'a str, recipe_name: &'a str, ingredients: &'a Vec<String>) -> NewRecipe<'a> {
    let new_recipe = NewRecipe {
        id: id,
        recipe_name: recipe_name,
        ingredients: ingredients,
    };

    diesel::insert_into(recipe::table)
        .values(&new_recipe)
        .execute(conn)
        .expect("Error saving new post");

    return new_recipe;
}