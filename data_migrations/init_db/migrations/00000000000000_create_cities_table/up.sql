CREATE TABLE IF NOT EXISTS cities (
	id SERIAL PRIMARY KEY,
	name TEXT NOT NULL,
	country_id CountryCode NOT NULL REFERENCES countries(id)
);