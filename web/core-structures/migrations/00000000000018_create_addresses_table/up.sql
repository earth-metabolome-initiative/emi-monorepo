CREATE TABLE IF NOT EXISTS addresses (
	id SERIAL PRIMARY KEY,
	iso TEXT NOT NULL REFERENCES countries(iso),
	city_code TEXT NOT NULL REFERENCES cities(code),
	street_name TEXT NOT NULL,
	street_number TEXT NOT NULL,
	postal_code TEXT NOT NULL,
	geolocation GEOGRAPHY(Point, 4326) NOT NULL,
	CONSTRAINT unique_address UNIQUE (iso, city_code, street_name, street_number)
);