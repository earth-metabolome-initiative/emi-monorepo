-- Table defining the step models of a procedure model and the order of the steps.
CREATE TABLE IF NOT EXISTS procedure_step_models (
	id SERIAL PRIMARY KEY,
	procedure_model_id INT NOT NULL REFERENCES procedure_models(id),
	step_model_id INT NOT NULL REFERENCES step_models(id),
	next_procedure_step_model_id INT REFERENCES procedure_step_models(id),
	prev_procedure_step_model_id INT REFERENCES procedure_step_models(id),
	created_by INT NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
	updated_by INT NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
	CONSTRAINT next_check CHECK (must_be_distinct_i32(next_procedure_step_model_id, id)),
	CONSTRAINT prev_check CHECK (must_be_distinct_i32(prev_procedure_step_model_id, id)),
	CONSTRAINT next_prev_check CHECK (must_be_distinct_i32(next_procedure_step_model_id, prev_procedure_step_model_id))
);
