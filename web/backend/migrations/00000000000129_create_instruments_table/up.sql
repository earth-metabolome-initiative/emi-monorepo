-- This is a no-op SQL statement
CREATE TABLE IF NOT EXISTS instruments(
    id INTEGER PRIMARY KEY,
    instrument_category_id INTEGER NOT NULL,
    organization_id INTEGER NOT NULL,
    FOREIGN KEY (instrument_category_id) REFERENCES instrument_categories(id),
    FOREIGN KEY (organization_id) REFERENCES organizations(id)
);