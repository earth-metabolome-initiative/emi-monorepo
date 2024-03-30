-- SQL defining the container_vertical_rules table.
-- The container rules define whether an item type can contain another item type.
-- For instance a rack can contain tubes, but a tube cannot contain a rack.
-- We define such rules in an allow-list fashion, meaning that if a rule is not defined,
-- then the item type cannot contain another item type. The rules are defined by an admin
-- user, and are used to enforce the containment rules when creating or updating items.
-- Some containers may only contain items that are within a certain temperature, humidity,
-- or pressure range. These constraints are also defined in the container rules.
CREATE TABLE container_vertical_rules (
    id UUID PRIMARY KEY REFERENCES describables(id) ON DELETE CASCADE,
    container_item_type_id UUID REFERENCES item_categories(id) ON
    DELETE
        CASCADE,
        contained_item_type_id UUID REFERENCES item_categories(id) ON
    DELETE
        CASCADE,
        minimum_temperature FLOAT DEFAULT NULL,
        maximum_temperature FLOAT DEFAULT NULL,
        minimum_humidity FLOAT DEFAULT NULL,
        maximum_humidity FLOAT DEFAULT NULL,
        minimum_pressure FLOAT DEFAULT NULL,
        maximum_pressure FLOAT DEFAULT NULL,
        CHECK (
            minimum_temperature IS NULL
            OR maximum_temperature IS NULL
            OR minimum_temperature <= maximum_temperature
        ),
        CHECK (
            minimum_humidity IS NULL
            OR maximum_humidity IS NULL
            OR minimum_humidity <= maximum_humidity
        ),
        CHECK (
            minimum_pressure IS NULL
            OR maximum_pressure IS NULL
            OR minimum_pressure <= maximum_pressure
        )
);