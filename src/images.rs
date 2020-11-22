use super::schema::recipe_image;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Insertable, AsChangeset)]
#[table_name="recipe_image"]
pub struct NewRecipeImage<'a> {
    pub recipe_id: &'a str,
    pub image_byte: &'a Vec<u8>
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct RecipeImage {
    pub recipe_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub recipe_image: Vec<u8>
}

pub fn add_recipe_image<'a>(conn: &diesel::pg::PgConnection,
                            image: &'a Vec<u8>,
                            rid: &'a str) -> QueryResult<usize> {
    let new_image = NewRecipeImage{
        recipe_id: rid,
        image_byte: image
    };
    return diesel::insert_into(recipe_image::table).values(new_image).execute(conn);
}

pub fn read_image<'a>(conn: &diesel::pg::PgConnection, rid: String) -> QueryResult<RecipeImage> {
    return recipe_image::table.find(rid).first(conn);
}

pub fn delete_image<'a>(conn: &diesel::pg::PgConnection, rid: String) -> QueryResult<usize> {
    return diesel::delete(recipe_image::table.find(rid)).execute(conn);
}
