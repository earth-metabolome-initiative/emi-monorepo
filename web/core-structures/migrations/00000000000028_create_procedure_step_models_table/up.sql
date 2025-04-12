CREATE TABLE IF NOT EXISTS procedure_step_models (
	id SERIAL PRIMARY KEY,
	procedure_model_id INT NOT NULL REFERENCES procedure_models(id) ON DELETE CASCADE,
	previous_step_id INT REFERENCES procedure_step_models(id) ON DELETE CASCADE,
	procedure_step_model_type_id SMALLINT NOT NULL REFERENCES procedure_step_model_types(id),
	created_by INT NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
	updated_by INT NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
	CONSTRAINT CHECK (must_be_distinct_i32(id, previous_step_id))
);