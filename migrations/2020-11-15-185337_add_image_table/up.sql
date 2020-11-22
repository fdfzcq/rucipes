CREATE TABLE recipe_image (recipe_id text PRIMARY KEY,
                           created_at TIMESTAMP NOT NULL DEFAULT NOW(),
                           updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
                           image_byte bytea NOT NULL,
                           CONSTRAINT recipe_id_constraint
                                FOREIGN KEY(recipe_id)
                                    REFERENCES recipe(id));
CREATE TRIGGER set_timestamp
BEFORE UPDATE ON recipe_image
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();