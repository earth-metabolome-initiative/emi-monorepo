-- This is a no-op SQL statement
ALTER TABLE 
    nameplates
ALTER COLUMN
    geolocation
SET
    DATA TYPE POINT USING geolocation::POINT;