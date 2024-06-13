-- This is a no-op SQL statement
ALTER TABLE 
    nameplates 
DROP COLUMN 
    geolocation;

ALTER TABLE
    nameplates
ADD COLUMN
    geolocation geometry(POINT, 4326) NOT NULL;