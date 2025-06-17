CREATE TABLE IF NOT EXISTS commercial_products (
    id UUID PRIMARY KEY REFERENCES trackables(id),
	deprecation_date TIMESTAMP WITH TIME ZONE,
	brand_id INTEGER NOT NULL REFERENCES brands(id)
);

