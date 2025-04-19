CREATE TABLE IF NOT EXISTS packaging_models (
	id SERIAL PRIMARY KEY REFERENCES commercial_products(id),
	material_id INT NOT NULL REFERENCES materials(id),
	created_by INT NOT NULL REFERENCES users(id),
	created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	updated_by INT NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);