CREATE TABLE IF NOT EXISTS locations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    latitude_degrees INTEGER,
    latitude_minutes INTEGER,
    latitude_seconds INTEGER,
    longitude_degrees INTEGER,
    longitude_minutes INTEGER,
    longitude_seconds INTEGER,
    altitude INTEGER,
    address TEXT,
    geolocalization_device_id UUID REFERENCES items(id) ON
    DELETE
    SET
        NULL,
        altitude_device_id UUID REFERENCES items(id) ON
    DELETE
    SET
        NULL,
        parent_location_id UUID REFERENCES locations(id) ON
    DELETE
    SET
        NULL
);