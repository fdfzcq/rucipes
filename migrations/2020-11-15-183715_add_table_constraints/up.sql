ALTER TABLE ingredient_recipe
    ADD CONSTRAINT recipe_table_to_ingredient_recipe_table FOREIGN KEY (recipe_id) REFERENCES recipe (id);
ALTER TABLE recipe_step
    ADD CONSTRAINT recipe_table_to_recipe_step_table FOREIGN KEY (recipe_id) REFERENCES recipe (id);