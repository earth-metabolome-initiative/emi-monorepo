CREATE TABLE IF NOT EXISTS project_workflow_models (
	id SERIAL PRIMARY KEY,
	name VARCHAR(255) NOT NULL CHECK (must_not_be_empty(name)),
	description TEXT NOT NULL CHECK (must_not_be_empty(description)),
	created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
	updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
	created_by INT NOT NULL REFERENCES users(id),
	updated_by INT NOT NULL REFERENCES users(id)
);