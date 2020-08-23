table! {
    recipe (id) {
        id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        recipe_name -> Text,
        ingredients -> Array<Text>,
    }
}
