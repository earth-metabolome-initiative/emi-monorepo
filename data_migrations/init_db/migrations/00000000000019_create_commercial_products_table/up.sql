CREATE TABLE IF NOT EXISTS commercial_products (
	id UUID PRIMARY KEY REFERENCES trackables(id),
	deprecation_date TIMESTAMP WITH TIME ZONE,
	brand_id INTEGER NOT NULL REFERENCES brands(id)
);
CREATE TABLE IF NOT EXISTS commercial_weighing_device_models (
	id UUID PRIMARY KEY,
	FOREIGN KEY (id) REFERENCES weighing_device_models(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES commercial_products(id) ON DELETE CASCADE
);