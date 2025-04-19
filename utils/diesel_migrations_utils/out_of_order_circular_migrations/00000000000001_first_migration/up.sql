CREATE TABLE first_table (
	id SERIAL PRIMARY KEY,
	parent_id INTEGER REFERENCES first_table(id),
	name TEXT NOT NULL
);