CREATE TABLE IF NOT EXISTS sample_packaging_step_models (
	id SERIAL PRIMARY KEY,
	packaging_model_id INT NOT NULL REFERENCES sample_packaging_models(id),
	created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
	updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
	created_by INT NOT NULL REFERENCES users(id),
	updated_by INT NOT NULL REFERENCES users(id)
);