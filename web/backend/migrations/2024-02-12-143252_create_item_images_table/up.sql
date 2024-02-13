-- SQL defining the table for item images.
CREATE TABLE item_images (
    id SERIAL PRIMARY KEY,
    format VARCHAR(3) NOT NULL,
    item_id INTEGER NOT NULL REFERENCES items(id),
);