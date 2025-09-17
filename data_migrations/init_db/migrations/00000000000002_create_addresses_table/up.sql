CREATE TABLE IF NOT EXISTS addresses (
	id SERIAL PRIMARY KEY,
	city_id INTEGER NOT NULL REFERENCES cities(id),
	street_name TEXT NOT NULL,
	house_number TEXT NOT NULL,
	postal_code TEXT NOT NULL,
	geolocation GEOGRAPHY(Point, 4326) NOT NULL,
	CONSTRAINT unique_address UNIQUE (city_id, street_name, house_number)
);