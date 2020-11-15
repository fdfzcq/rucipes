CREATE TRIGGER set_timestamp
BEFORE UPDATE ON ingredient_recipe
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();
CREATE TRIGGER set_timestamp
BEFORE UPDATE ON recipe_step
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();