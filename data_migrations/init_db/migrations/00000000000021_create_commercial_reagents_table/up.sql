CREATE TABLE IF NOT EXISTS commercial_reagents (
    id UUID PRIMARY KEY REFERENCES processables(id),
    commercial_product_lot_id UUID NOT NULL REFERENCES commercial_product_lots(id)
);