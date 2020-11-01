use chrono::NaiveDateTime;
use super::schema::recipe;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Insertable, AsChangeset)]
#[table_name="recipe"]
pub struct NewRecipe<'a> {
    pub id: &'a str,
    pub recipe_name: &'a str,
    pub ingredients: &'a Vec<String>,
}

#[derive(Queryable, Identifiable, Serialize, Deserialize)]
#[table_name="recipe"]
pub struct Recipe {
    pub id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub recipe_name: String,
    pub ingredients: Vec<String>
}

// db interface

pub fn create_recipe<'a>(conn: &diesel::pg::PgConnection,
                         rid: &'a str,
                         recipe_name: &'a str,
                         ingrs: &'a Vec<super::IngredientRequestBody>) -> QueryResult<usize> {
    let new_recipe = NewRecipe {
        id: rid,
        recipe_name: recipe_name,
        ingredients: &(ingrs.into_iter().map(|i| i.ingredient_name.clone()).collect()),
    };

    let ingredient_recipe_insertion_result = super::ingredients::insert_ingredients_recipe(conn, ingrs, rid);

    match ingredient_recipe_insertion_result {
        Ok(_) => return diesel::insert_into(recipe::table).values(&new_recipe).execute(conn),
        Err(_) => return ingredient_recipe_insertion_result
    }
}

pub fn read_recipe<'a>(conn: &diesel::pg::PgConnection, rid: String) -> QueryResult<Recipe> {
    return recipe::table.filter(recipe::id.eq(rid)).first(conn);
}

pub fn delete_recipe<'a>(conn: &diesel::pg::PgConnection, rid: String) -> QueryResult<usize> {
    return diesel::delete(recipe::table.filter(recipe::id.eq(rid))).execute(conn);
}

pub fn update_recipe<'a>(conn: &diesel::pg::PgConnection,
                         rid: &'a str,
                         rname: &'a str,
                         ingrs: &'a Vec<super::IngredientRequestBody>) -> QueryResult<usize> {
    let updated_recipe = NewRecipe {
        id: rid,
        recipe_name: rname,
        ingredients: &(ingrs.into_iter().map(|i| i.ingredient_name.clone()).collect()),
    };
    return diesel::update(recipe::table).set(updated_recipe).execute(conn);
}

pub fn all_recipes<'a>(conn: &diesel::pg::PgConnection,
                       page_size: i64,
                       page: i64) -> QueryResult<Vec<Recipe>> {
    return recipe::table.order(recipe::created_at)
                        .offset(page - 1)
                        .limit(page_size)
                        .load::<Recipe>(conn);                     
}
