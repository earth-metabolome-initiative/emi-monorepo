-- We revert the changes of types made in the up.sql file
-- We revert the type of the column "observations"."picture" from "jpeg" to "BYTEA"
ALTER TABLE
    observations
ALTER COLUMN
    picture
SET
    DATA TYPE BYTEA USING (picture).value;