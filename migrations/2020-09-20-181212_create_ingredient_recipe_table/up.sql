CREATE TABLE ingredient_recipe (recipe_id text,
                                ingredient text,
                                unit text DEFAULT 'unit',
                                quantity INTEGER DEFAULT 0,
                                created_at TIMESTAMP NOT NULL DEFAULT NOW(),
                                updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
                                PRIMARY KEY(recipe_id, ingredient));