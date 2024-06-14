-- We revert the changes of types made in the up.sql file
-- We revert the type of the column "users"."picture" from "jpeg" to "BYTEA"
ALTER TABLE
    users
ALTER COLUMN
    picture
SET
    DATA TYPE BYTEA USING (picture).value;