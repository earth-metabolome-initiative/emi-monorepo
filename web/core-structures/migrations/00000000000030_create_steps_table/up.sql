CREATE TABLE IF NOT EXISTS steps (
	id SERIAL PRIMARY KEY,
	step_model_id INT NOT NULL REFERENCES step_models(id),
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
	updated_by INTEGER NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);


-- Table providing the instruments necessary for a given step
CREATE TABLE IF NOT EXISTS step_instruments (
	id SERIAL PRIMARY KEY,
	step_id INT NOT NULL REFERENCES steps(id),
	instrument_id SMALLINT NOT NULL REFERENCES instruments(id),
	created_by INT NOT NULL REFERENCES users(id),
	created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	updated_by INT NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Table providing the nameplate models used within a certain step
CREATE TABLE IF NOT EXISTS step_nameplate_models (
	id SERIAL PRIMARY KEY,
	step_id INT NOT NULL REFERENCES steps(id),
	nameplate_model_id SMALLINT NOT NULL REFERENCES nameplate_models(id),
	created_by INT NOT NULL REFERENCES users(id),
	created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	updated_by INT NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS step_container_models (
	id SERIAL PRIMARY KEY,
	step_id INT NOT NULL REFERENCES steps(id),
	container_model_id INT NOT NULL REFERENCES container_models(id),
	created_by INT NOT NULL REFERENCES users(id),
	created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	updated_by INT NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS step_tool_models (
	id SERIAL PRIMARY KEY,
	step_id INT NOT NULL REFERENCES steps(id),
	tool_model_id INT NOT NULL REFERENCES tool_models(id),
	created_by INT NOT NULL REFERENCES users(id),
	created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	updated_by INT NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);