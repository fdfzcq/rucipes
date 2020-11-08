CREATE TABLE recipe_step (recipe_id text,
                          step_no INTEGER,
                          content text,
                          created_at TIMESTAMP NOT NULL DEFAULT NOW(),
                          updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
                          PRIMARY KEY(recipe_id, step_no));