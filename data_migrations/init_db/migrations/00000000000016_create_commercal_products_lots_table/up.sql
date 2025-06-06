CREATE TABLE IF NOT EXISTS commercial_product_lots (
    id UUID PRIMARY KEY REFERENCES trackables(id) ON DELETE CASCADE,
	lot TEXT NOT NULL,
	product_model_id UUID NOT NULL REFERENCES commercial_products(id) ON DELETE CASCADE,
	UNIQUE (lot, product_model_id)
);