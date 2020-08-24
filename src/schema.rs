table! {
    recipe (id) {
        id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        recipe_name -> Text,
        ingredients -> Array<Text>,
    }
}
