CREATE TABLE IF NOT EXISTS procedure_models (
	id SERIAL PRIMARY KEY,
	name TEXT UNIQUE NOT NULL CHECK (must_be_paragraph(name)),
	description TEXT NOT NULL CHECK (must_be_paragraph(description)),
	repeatable BOOLEAN NOT NULL DEFAULT FALSE,
	deprecated BOOLEAN NOT NULL DEFAULT FALSE,
	icon TEXT NOT NULL DEFAULT 'book' CHECK (must_be_font_awesome_class(icon)),
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_by INTEGER NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	CHECK (must_be_distinct(name, description))
);

CREATE TABLE IF NOT EXISTS parent_procedure_models (
	parent_procedure_model_id INTEGER NOT NULL REFERENCES procedure_models(id) ON DELETE CASCADE,
	child_procedure_model_id INTEGER NOT NULL REFERENCES procedure_models(id) ON DELETE CASCADE,
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_by INTEGER NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	PRIMARY KEY (parent_procedure_model_id, child_procedure_model_id),
	CHECK (must_be_distinct_i32(parent_procedure_model_id, child_procedure_model_id))
);

CREATE TABLE IF NOT EXISTS next_procedure_models (
	parent_procedure_model_id INTEGER NOT NULL REFERENCES procedure_models(id) ON DELETE CASCADE,
	successor_id INTEGER NOT NULL REFERENCES procedure_models(id) ON DELETE CASCADE,
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_by INTEGER NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	PRIMARY KEY (parent_procedure_model_id, successor_id),
	FOREIGN KEY (parent_procedure_model_id, successor_id) REFERENCES parent_procedure_models(parent_procedure_model_id, child_procedure_model_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS procedure_model_container_categories (
	id SERIAL PRIMARY KEY,
	procedure_model_id INTEGER NOT NULL REFERENCES procedure_models(id) ON DELETE CASCADE,
	container_category ContainerCategory NOT NULL,
	expected_kelvin REAL NOT NULL CHECK (must_be_strictly_positive_f32(expected_kelvin)) DEFAULT 293.15,
	tolerance_kelvin REAL NOT NULL CHECK (must_be_strictly_positive_f32(tolerance_kelvin)) DEFAULT 20.0,
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_by INTEGER NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	CHECK (must_be_strictly_smaller_than_f32(tolerance_kelvin, expected_kelvin))
);

CREATE TABLE IF NOT EXISTS procedure_model_instrument_categories (
	id SERIAL PRIMARY KEY,
	procedure_model_id INTEGER NOT NULL REFERENCES procedure_models(id) ON DELETE CASCADE,
	instrument_category InstrumentCategory NOT NULL,
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_by INTEGER NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS procedure_model_tool_categories (
	id SERIAL PRIMARY KEY,
	procedure_model_id INTEGER NOT NULL REFERENCES procedure_models(id) ON DELETE CASCADE,
	tool_category ToolCategory NOT NULL,
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_by INTEGER NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS procedure_model_nameplate_categories (
	id SERIAL PRIMARY KEY,
	procedure_model_id INTEGER NOT NULL REFERENCES procedure_models(id) ON DELETE CASCADE,
	nameplate_category NameplateCategory NOT NULL,
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_by INTEGER NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
