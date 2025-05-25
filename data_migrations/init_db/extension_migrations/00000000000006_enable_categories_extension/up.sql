-- Enable several of the PGRX extensions.
CREATE EXTENSION IF NOT EXISTS media_types;
CREATE EXTENSION IF NOT EXISTS tool_categories;
CREATE EXTENSION IF NOT EXISTS instrument_categories;
CREATE EXTENSION IF NOT EXISTS container_categories;
CREATE EXTENSION IF NOT EXISTS nameplate_categories;
CREATE EXTENSION IF NOT EXISTS step_model_categories;

-- We create a custom type called 'CustomCaTeGoRy' with a string field.
CREATE TYPE CustomCaTeGoRy AS (
	name TEXT
);

CREATE TYPE "My_Type" AS ENUM ('foo', 'bar');
CREATE TYPE My_Type AS ENUM ('foo', 'bar');