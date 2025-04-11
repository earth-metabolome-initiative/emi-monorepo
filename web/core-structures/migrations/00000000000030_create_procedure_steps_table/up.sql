CREATE TABLE IF NOT EXISTS procedure_steps (
	id SERIAL PRIMARY KEY,
	procedure_id INT NOT NULL REFERENCES procedures(id),
	procedure_step_model_id INT NOT NULL REFERENCES procedure_step_models(id),
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
	updated_by INTEGER NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);