CREATE TABLE IF NOT EXISTS products(
    id SERIAL PRIMARY KEY,
    product_category_id SMALLINT NOT NULL,
    company_id INTEGER NOT NULL,
    ean_number EAN13 NOT NULL UNIQUE,
    name TEXT NOT NULL CHECK (must_not_be_empty(name)),
    price MONEY NOT NULL CHECK(must_be_strictly_positive_double(price)),
    FOREIGN KEY (company_id) REFERENCES companies(id),
    FOREIGN KEY (product_category_id) REFERENCES product_categories(id)
);