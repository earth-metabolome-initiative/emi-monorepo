-- SQL describing the real_units table.
-- The real_units table is used as the finite set of units that can be used to measure
-- the real quantities of items. For example, a box, a tube, or a vial.
CREATE TABLE real_units (
    id INTEGER PRIMARY KEY,
    FOREIGN KEY (id) REFERENCES units(id)
);