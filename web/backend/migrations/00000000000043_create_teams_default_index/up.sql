-- In this SQL up migration, we change the id column of the teams table
-- from INTEGER to SERIAL, which will automatically generate unique
-- values for each row.

CREATE SEQUENCE teams_id_seq;
ALTER TABLE teams ALTER COLUMN id SET DEFAULT nextval('teams_id_seq');
ALTER SEQUENCE teams_id_seq OWNED BY teams.id;