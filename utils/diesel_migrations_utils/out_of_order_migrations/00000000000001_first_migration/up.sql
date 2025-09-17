CREATE TABLE first_table (
	id SERIAL PRIMARY KEY,
	parent INTEGER NOT NULL REFERENCES first_table(id),
	name TEXT NOT NULL
);