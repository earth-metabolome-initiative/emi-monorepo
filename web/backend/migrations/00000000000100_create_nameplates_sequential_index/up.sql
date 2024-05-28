-- This is a no-op SQL statement
CREATE SEQUENCE nameplates_id_seq;
ALTER TABLE nameplates ALTER COLUMN id SET DEFAULT nextval('nameplates_id_seq');
ALTER TABLE nameplates ALTER COLUMN id SET NOT NULL;
ALTER SEQUENCE nameplates_id_seq OWNED BY nameplates.id;
