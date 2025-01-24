-- Changes the type of the column "observations"."picture" from "BYTEA" to "jpeg"
-- The reason the type is not originally "jpeg" is because the "jpeg" type
-- is not supported in the GlueSQL dialect which we employ in the frontend,
-- and we want to maintain as much as possible of the definition of the table
-- in the backend exactly identical to the frontend.

-- We DROP the column "observations"."picture" to be able to change its type
ALTER TABLE
    observations
DROP COLUMN
    picture;

-- We ADD the column "observations"."picture" with the type "jpeg"
ALTER TABLE
    observations
ADD COLUMN
    picture jpeg NOT NULL;