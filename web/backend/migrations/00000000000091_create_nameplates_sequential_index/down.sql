-- This is a no-op SQL statement
ALTER SEQUENCE nameplates_id_seq OWNED BY NONE;
ALTER TABLE nameplates ALTER COLUMN id DROP DEFAULT;
DROP SEQUENCE nameplates_id_seq;