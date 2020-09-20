table! {
    ingredient_recipe (recipe_id, ingredient) {
        recipe_id -> Text,
        ingredient -> Text,
        unit -> Nullable<Text>,
        quantity -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    recipe (id) {
        id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        recipe_name -> Text,
        ingredients -> Array<Text>,
    }
}

allow_tables_to_appear_in_same_query!(
    ingredient_recipe,
    recipe,
);
