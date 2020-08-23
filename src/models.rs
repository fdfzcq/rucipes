use chrono::NaiveDateTime;

#[derive(Queryable)]
pub struct Recipe {
    pub id: i32,
    pub recipe_name: String,
    pub ingredients: Vec<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}