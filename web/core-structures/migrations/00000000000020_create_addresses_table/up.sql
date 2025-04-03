CREATE TABLE IF NOT EXISTS addresses (
	id SERIAL PRIMARY KEY,
	street_id INTEGER NOT NULL REFERENCES streets(id),
	street_number TEXT NOT NULL,
	postal_code_id INTEGER NOT NULL REFERENCES postal_codes(id),
	geolocation GEOMETRY(Point, 4326) NOT NULL
);