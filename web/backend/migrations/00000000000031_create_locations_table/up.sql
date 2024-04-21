-- SQL defining the locations table.
-- A location is a physical place where items can be stored.
-- This table defines the locations where items can be stored.
-- It contains the following columns:
-- - id: the unique identifier of the location.
-- - name: the name of the location.
-- - description: a description of the location.
-- - latitude: the latitude of the location.
-- - longitude: the longitude of the location.
-- - altitude: the altitude of the location.
-- - address: the address of the location, at the best precision available.
-- - parent_location_id: the unique identifier of the parent location, if any.
-- - state: the state of the location, such as "active", "under construction", "decommissioned", "abandoned", "destroyed", "lost", "stolen", "missing", "unknown", "other".
-- - created_at: the date and time when the location was created.
-- - updated_at: the date and time when the location was last updated.
-- - created_by: the unique identifier of the user who created the location.
-- - updated_by: the unique identifier of the user who last updated the location.
CREATE TABLE locations (
    id UUID PRIMARY KEY REFERENCES describables(id) ON
    DELETE
        CASCADE,
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
        NULL,
        state_id UUID NOT NULL REFERENCES location_states(id)
);

-- We also need to add a bi-directional cascade delete constraint to the editables
-- table, so that when a location is deleted, the corresponding editable is also deleted.
-- Since the editables table is referenced by several other tables, we cannot add a cascade
-- delete constraint to the editables table. Instead, we add a trigger to delete the corresponding
-- record in the editables table when a location is deleted.
CREATE
OR REPLACE FUNCTION delete_editables() RETURNS TRIGGER AS $$ BEGIN
    DELETE FROM
        editables
    WHERE
        id = OLD .id;

RETURN OLD;

END;

$$ LANGUAGE plpgsql;

CREATE TRIGGER delete_editables AFTER
DELETE
    ON locations FOR EACH ROW EXECUTE FUNCTION delete_editables();