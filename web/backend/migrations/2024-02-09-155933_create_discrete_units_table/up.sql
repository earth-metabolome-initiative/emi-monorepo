-- SQL describing the discrete_units table.
-- The discrete_units table is used as the finite set of units that can be used to measure
-- the discrete quantities of items. For example, a box, a tube, or a vial.
CREATE TABLE discrete_units (
    id INTEGER PRIMARY KEY,
    FOREIGN KEY (id) REFERENCES units(id)
);