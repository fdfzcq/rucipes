extern crate chrono;
use chrono::{DateTime, Local, NaiveDateTime, TimeZone, Utc};

#[derive(Queryable)]
pub struct Recipe {
    id uuid PRIMARY KEY,
                     created_at TIMESTAMP NOT NULL,
                     updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
                     recipe_name text,
                     -- TODO: create tables for each ingredient --
                     ingredients text[]);
    pub id: String,
    pub created_at: chrono::DateTime<Utc>,
    pub recipe_name: String,
    pub ingredients: String[],
}