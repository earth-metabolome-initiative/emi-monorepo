CREATE TABLE IF NOT EXISTS procedures (
	id SERIAL PRIMARY KEY,
	procedure_model_id INTEGER NOT NULL REFERENCES procedure_models(id),
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
	updated_by INTEGER NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);


-- Table providing the instruments necessary for a given procedure
CREATE TABLE IF NOT EXISTS procedure_instruments (
	id SERIAL PRIMARY KEY,
	procedure_id INT NOT NULL REFERENCES procedures(id),
	instrument_id SMALLINT NOT NULL REFERENCES instruments(id),
	created_by INT NOT NULL REFERENCES users(id),
	created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	updated_by INT NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Table providing the nameplate models used within a certain procedure
CREATE TABLE IF NOT EXISTS procedure_nameplate_models (
	id SERIAL PRIMARY KEY,
	procedure_id INT NOT NULL REFERENCES procedures(id),
	nameplate_model_id SMALLINT NOT NULL REFERENCES nameplate_models(id),
	created_by INT NOT NULL REFERENCES users(id),
	created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	updated_by INT NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS procedure_container_models (
	id SERIAL PRIMARY KEY,
	procedure_id INT NOT NULL REFERENCES procedures(id),
	container_model_id INT NOT NULL REFERENCES container_models(id),
	created_by INT NOT NULL REFERENCES users(id),
	created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	updated_by INT NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS procedure_tool_models (
	id SERIAL PRIMARY KEY,
	procedure_id INT NOT NULL REFERENCES procedures(id),
	tool_model_id INT NOT NULL REFERENCES tool_models(id),
	created_by INT NOT NULL REFERENCES users(id),
	created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	updated_by INT NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);