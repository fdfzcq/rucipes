use super::schema::ingredient_recipe;
use diesel::prelude::*;


#[derive(Insertable, AsChangeset)]
#[table_name="ingredient_recipe"]
pub struct NewIngredientRecipe<'a> {
    pub recipe_id: &'a str,
    pub ingredient: &'a str,
    pub unit: &'a str,
    pub quantity: &'a i32
}

pub fn insert_ingredients_recipe<'a>(conn: &diesel::pg::PgConnection,
                                     ingredients: &'a Vec<super::IngredientRequestBody>,
                                     rid: &'a str) -> QueryResult<usize> {
    let new_ingredients_vec: Vec<NewIngredientRecipe> =
        ingredients.into_iter().map(
            |i| NewIngredientRecipe{
                recipe_id: rid,
                ingredient: &i.ingredient_name,
                unit: &i.unit,
                quantity: &i.quantity
            }).collect();
    return diesel::insert_into(ingredient_recipe::table).values(new_ingredients_vec).execute(conn);
}
