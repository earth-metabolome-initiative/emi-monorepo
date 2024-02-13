-- SQL describing the continuous_units table.
-- The continuous_units table is used as the finite set of units that can be used to measure
-- the continuous quantities of items. For example, a box, a tube, or a vial.
CREATE TABLE continuous_units (
    id INTEGER PRIMARY KEY,
    FOREIGN KEY (id) REFERENCES units(id)
);