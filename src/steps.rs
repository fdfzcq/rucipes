use super::schema::recipe_step;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Insertable, AsChangeset)]
#[table_name="recipe_step"]
pub struct NewRecipeStep<'a> {
    pub recipe_id: &'a str,
    pub step_no: &'a i32,
    pub content: &'a str
}

#[derive(Queryable, Identifiable, Serialize, Deserialize)]
#[primary_key(recipe_id, step_no)]
#[table_name="recipe_step"]
pub struct RecipeStep {
    pub recipe_id: String,
    pub step_no: i32,
    pub content: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

pub fn add_recipe_step<'a>(conn: &diesel::pg::PgConnection,
                           step_number: &'a i32,
                           content_str: &'a str,
                           rid: &'a str) -> QueryResult<usize> {
    let new_step = NewRecipeStep{
                     recipe_id: rid,
                     step_no: step_number,
                     content: content_str
                   };
    return diesel::insert_into(recipe_step::table).values(new_step).execute(conn);
}

pub fn read_step<'a>(conn: &diesel::pg::PgConnection, rid: String, step_number: i32) -> QueryResult<RecipeStep> {
    return recipe_step::table
            .find((rid, step_number))
            .first(conn);
}

pub fn delete_step<'a>(conn: &diesel::pg::PgConnection, rid: String, step_number: i32) -> QueryResult<usize> {
    return diesel::delete(recipe_step::table.find((rid, step_number))).execute(conn);
}
