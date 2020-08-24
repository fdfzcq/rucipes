CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE recipe (id text PRIMARY KEY,
                     created_at TIMESTAMP NOT NULL DEFAULT NOW(),
                     updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
                     recipe_name text NOT NULL,
                     -- TODO: create tables for each ingredient --
                     ingredients text[] NOT NULL DEFAULT '{}');
