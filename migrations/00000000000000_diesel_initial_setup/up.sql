CREATE TABLE recipe (id text PRIMARY KEY,
                     created_at TIMESTAMP NOT NULL DEFAULT NOW(),
                     updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
                     recipe_name text NOT NULL,
                     ingredients text[] NOT NULL DEFAULT '{}');
