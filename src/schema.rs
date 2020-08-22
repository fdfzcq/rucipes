table! {
    recipe (id) {
        id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        recipe_name -> Nullable<Text>,
        ingredients -> Nullable<Array<Text>>,
    }
}
