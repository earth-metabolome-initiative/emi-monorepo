CREATE TABLE IF NOT EXISTS procedure_models (
	id SERIAL PRIMARY KEY,
	name TEXT NOT NULL CHECK (must_not_be_empty(name))
	description TEXT NOT NULL CHECK (must_not_be_empty(description)),
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
	updated_by INTEGER NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);