-- SQL defining the item_quantities table.
-- Some items may be counted discretely, as opposed to a weight in grams.
-- This table defines the discrete quantities of items that we have in stock.
-- The quantity of an item may change over time, so multiple quantity entries
-- may be inserted for the same item.
-- While these quantities are discrete, in order to facilitate the semantics of
-- the system, we will still allow for the user to specify the counter unit for 
-- the item. For example, a counter unit may be a box, a tube, or a vial.
CREATE TABLE item_discrete_quantities (
    id BIGINT PRIMARY KEY REFERENCES editables(id) ON DELETE CASCADE,
    item_id BIGINT REFERENCES items(id) ON DELETE CASCADE,
    quantity INTEGER NOT NULL,
    unit_id BIGINT REFERENCES units(id) ON DELETE CASCADE,
    measured_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    measured_by INTEGER REFERENCES users(id) ON DELETE SET NULL,
    UNIQUE (item_id, unit_id),
    FOREIGN KEY (item_id, unit_id) REFERENCES item_units(item_id, unit_id) ON DELETE CASCADE,
    FOREIGN KEY (unit_id) REFERENCES discrete_units(id)
);