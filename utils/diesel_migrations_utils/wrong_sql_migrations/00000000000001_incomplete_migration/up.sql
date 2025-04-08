-- Example of UP migration with errors in it (missing comma)
CREATE TABLE incomplete_migration_table (
	id SERIAL PRIMARY KEY
	name VARCHAR(255) NOT NULL
);