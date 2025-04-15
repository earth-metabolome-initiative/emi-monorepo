CREATE TABLE second_table (
	id SERIAL PRIMARY KEY,
	first_id INTEGER NOT NULL REFERENCES first_table(id)
);