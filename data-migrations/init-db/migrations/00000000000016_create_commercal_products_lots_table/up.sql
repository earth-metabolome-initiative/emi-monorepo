CREATE TABLE IF NOT EXISTS commercial_product_lots (
    id SERIAL PRIMARY KEY,
	lot TEXT NOT NULL,
	product_model_id INTEGER NOT NULL REFERENCES commercial_products(id) ON DELETE CASCADE,
	UNIQUE (lot, product_model_id)
);