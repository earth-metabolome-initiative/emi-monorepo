CREATE TABLE IF NOT EXISTS storage_containers (
	id UUID PRIMARY KEY,
	container_model_id INTEGER NOT NULL REFERENCES container_models(id),
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);