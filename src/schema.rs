table! {
    recipe (id) {
        id -> Text,
        recipe_name -> Text,
        ingredients -> Array<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
