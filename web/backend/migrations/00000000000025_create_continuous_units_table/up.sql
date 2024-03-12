-- SQL describing the continuous_units table.
-- The continuous_units table is used as the finite set of units that can be used to measure
-- the continuous quantities of items. For example, a box, a tube, or a vial.
CREATE TABLE continuous_units (
    id UUID PRIMARY KEY REFERENCES units(id) ON DELETE CASCADE
);

-- We also need to add a bi-directional cascade delete constraint to the units
-- table, so that when a continuous unit is deleted, the corresponding unit is also deleted.
-- We do not expect to delete continuous units, but we add the constraint for consistency.
-- Since the units table is referenced by several other tables, we cannot add a cascade
-- delete constraint to the units table. Instead, we add a trigger to delete the corresponding
-- record in the units table when a continuous unit is deleted.
CREATE OR REPLACE FUNCTION delete_units() RETURNS TRIGGER AS $$
BEGIN
    DELETE FROM
        units
    WHERE
        id = OLD.id;

    RETURN OLD;

END;

$$ LANGUAGE plpgsql;

CREATE TRIGGER delete_units AFTER
DELETE
    ON continuous_units FOR EACH ROW EXECUTE FUNCTION delete_units();
