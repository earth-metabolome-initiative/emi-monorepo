CREATE TABLE second_table (
	id SERIAL PRIMARY KEY,
	first_id INTEGER NOT NULL REFERENCES first_table(id),
	third_id INTEGER NOT NULL REFERENCES third_table(id)
);