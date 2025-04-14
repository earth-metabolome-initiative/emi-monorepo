-- Table defining the steps of a procedure.
CREATE TABLE IF NOT EXISTS procedure_steps (
	id SERIAL PRIMARY KEY,
	procedure_step_model_id INT NOT NULL REFERENCES procedure_step_models(id),
	procedure_id INT NOT NULL REFERENCES procedures(id),
	step_id INT NOT NULL REFERENCES steps(id),
	created_by INT NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
	updated_by INT NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS procedure_step_instruments (
	id SERIAL PRIMARY KEY,
	procedure_step_id INT NOT NULL REFERENCES procedure_steps(id),
	procedure_instrument_id INT NOT NULL REFERENCES procedure_instruments(id),
	step_instrument_id INT NOT NULL REFERENCES step_instruments(id),
	created_by INT NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
	updated_by INT NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS procedure_step_nameplate (
	id SERIAL PRIMARY KEY,
	procedure_step_id INT NOT NULL REFERENCES procedure_steps(id),
	procedure_nameplate_model_id INT NOT NULL REFERENCES procedure_nameplate_models(id),
	step_nameplate_model_id INT NOT NULL REFERENCES step_nameplate_models(id),
	created_by INT NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
	updated_by INT NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS procedure_step_container_models(
	id SERIAL PRIMARY KEY,
	procedure_step_id INT NOT NULL REFERENCES procedure_steps(id),
	procedure_container_model_id INT NOT NULL REFERENCES procedure_container_models(id),
	step_container_model_id INT NOT NULL REFERENCES step_container_models(id),
	created_by INT NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
	updated_by INT NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);