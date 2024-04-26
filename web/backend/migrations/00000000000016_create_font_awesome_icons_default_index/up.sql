-- In this SQL up migration, we change the id column of the taxa table
-- from INTEGER to SERIAL, which will automatically generate unique
-- values for each row.

CREATE SEQUENCE font_awesome_icons_id_seq;
ALTER TABLE font_awesome_icons ALTER COLUMN id SET DEFAULT nextval('font_awesome_icons_id_seq');
ALTER SEQUENCE font_awesome_icons_id_seq OWNED BY font_awesome_icons.id;