CREATE TABLE third_table (
	id SERIAL PRIMARY KEY,
	first_id INTEGER NOT NULL REFERENCES first_table(id),
	second_id INTEGER NOT NULL REFERENCES second_table(id)
);