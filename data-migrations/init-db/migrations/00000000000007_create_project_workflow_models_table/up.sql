CREATE TABLE IF NOT EXISTS project_workflow_models (
	id SERIAL PRIMARY KEY,
	name VARCHAR(255) NOT NULL CHECK (must_be_paragraph(name)),
	description TEXT NOT NULL CHECK (must_be_paragraph(description)),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	created_by INTEGER NOT NULL REFERENCES users(id),
	updated_by INTEGER NOT NULL REFERENCES users(id)
);