CREATE TABLE IF NOT EXISTS procedures (
	id SERIAL PRIMARY KEY,
	procedure_model_id INTEGER NOT NULL REFERENCES procedure_models(id),
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
	updated_by INTEGER NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
