CREATE TABLE IF NOT EXISTS step_models (
	id SERIAL PRIMARY KEY,
	name TEXT NOT NULL CHECK (must_not_be_empty(name)),
	description TEXT NOT NULL CHECK (must_not_be_empty(description)),
	-- Image illustrating what the step looks like
	photograph_id INT NOT NULL REFERENCES photographs(id),
	procedure_step_model_category_id SMALLINT NOT NULL REFERENCES procedure_step_model_categories(id),
	created_by INT NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
	updated_by INT NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Table providing the instrument types necessary for a given abstract procedure model
CREATE TABLE IF NOT EXISTS step_model_instrument_categories (
	id SERIAL PRIMARY KEY,
	step_model_id INT NOT NULL REFERENCES step_models(id),
	instrument_category_id SMALLINT NOT NULL REFERENCES instrument_categories(id),
	created_by INT NOT NULL REFERENCES users(id),
	created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	updated_by INT NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Table providing the nameplate categories used within a certain procedure model
CREATE TABLE IF NOT EXISTS step_model_nameplate_categories (
	id SERIAL PRIMARY KEY,
	step_model_id INT NOT NULL REFERENCES step_models(id),
	nameplate_category_id SMALLINT NOT NULL REFERENCES nameplate_categories(id),
	created_by INT NOT NULL REFERENCES users(id),
	created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	updated_by INT NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS step_model_container_categories (
	id SERIAL PRIMARY KEY,
	step_model_id INT NOT NULL REFERENCES step_models(id),
	container_category_id SMALLINT NOT NULL REFERENCES container_categories(id),
	created_by INT NOT NULL REFERENCES users(id),
	created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	updated_by INT NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS step_model_tool_categories (
	id SERIAL PRIMARY KEY,
	step_model_id INT NOT NULL REFERENCES step_models(id),
	tool_category_id SMALLINT NOT NULL REFERENCES tool_categories(id),
	created_by INT NOT NULL REFERENCES users(id),
	created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	updated_by INT NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);