CREATE TABLE IF NOT EXISTS procedure_models (
	id SERIAL PRIMARY KEY,
	name TEXT UNIQUE NOT NULL CHECK (must_not_be_empty(name)),
	description TEXT NOT NULL CHECK (must_not_be_empty(description)),
	icon TEXT NOT NULL DEFAULT 'book' CHECK (must_be_font_awesome_class(icon)),
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_by INTEGER NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	CHECK (must_be_distinct(name, description))
);

-- Table providing the instrument types necessary for a given abstract procedure model
CREATE TABLE IF NOT EXISTS procedure_model_instrument_categories (
	id SERIAL PRIMARY KEY,
	procedure_model_id INTEGER NOT NULL REFERENCES procedure_models(id),
	instrument_category InstrumentCategory NOT NULL,
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_by INTEGER NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Table providing the nameplate categories used within a certain procedure model
CREATE TABLE IF NOT EXISTS procedure_model_nameplate_categories (
	id SERIAL PRIMARY KEY,
	procedure_model_id INTEGER NOT NULL REFERENCES procedure_models(id),
	nameplate_category NameplateCategory NOT NULL,
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_by INTEGER NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS procedure_model_container_categories (
	id SERIAL PRIMARY KEY,
	procedure_model_id INTEGER NOT NULL REFERENCES procedure_models(id),
	container_category ContainerCategory NOT NULL,
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_by INTEGER NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS procedure_model_tool_categories (
	id SERIAL PRIMARY KEY,
	quantity INTEGER DEFAULT 1 NOT NULL CHECK (must_be_strictly_positive_i32(quantity)),
	procedure_model_id INTEGER NOT NULL REFERENCES procedure_models(id),
	tool_category ToolCategory NOT NULL,
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_by INTEGER NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);