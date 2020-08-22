CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE recipe (id uuid PRIMARY KEY,
                     created_at TIMESTAMP NOT NULL,
                     updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
                     recipe_name text,
                     -- TODO: create tables for each ingredient --
                     ingredients text[]);
