-- Example of UP migration without the curresponding down migration
CREATE TABLE incomplete_migration_table (
	id SERIAL PRIMARY KEY,
	name VARCHAR(255) NOT NULL
);