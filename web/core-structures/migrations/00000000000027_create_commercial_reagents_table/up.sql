CREATE TABLE IF NOT EXISTS commercial_reagents (
    id INTEGER PRIMARY KEY REFERENCES processables(id),
    commercial_product_lot_id INTEGER NOT NULL REFERENCES commercial_product_lots(id)
);