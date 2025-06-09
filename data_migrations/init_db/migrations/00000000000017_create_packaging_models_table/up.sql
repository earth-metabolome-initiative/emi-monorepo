CREATE TABLE IF NOT EXISTS packaging_models (
	id SERIAL PRIMARY KEY REFERENCES commercial_products(id),
	material_id INTEGER NOT NULL REFERENCES materials(id),
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_by INTEGER NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	CHECK (must_be_smaller_than_utc(created_at, updated_at))
);