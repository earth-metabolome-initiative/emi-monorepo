-- Table defining the step models of a procedure model and the order of the steps.
CREATE TABLE IF NOT EXISTS procedure_step_models (
	id SERIAL PRIMARY KEY,
	procedure_model_id INTEGER NOT NULL REFERENCES procedure_models(id),
	step_model_id INTEGER NOT NULL REFERENCES step_models(id),
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_by INTEGER NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
