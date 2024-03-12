-- SQL defining the procedure_continuous_requirements table.
-- A procedure may require one or more items to be performed. This table defines the
-- requirements of a procedure in terms of the number of items of a given category.
CREATE TABLE procedure_continuous_requirements (
  id UUID PRIMARY KEY REFERENCES editables(id) ON DELETE CASCADE,
  procedure_id UUID NOT NULL REFERENCES procedures(id) ON DELETE CASCADE,
  item_category_id UUID NOT NULL REFERENCES item_categories(id) ON DELETE CASCADE,
  quantity FLOAT NOT NULL,
  unit_id UUID REFERENCES units(id) ON DELETE CASCADE,
  UNIQUE (procedure_id, item_category_id),
  FOREIGN KEY (item_category_id, unit_id) REFERENCES item_category_units(item_category_id, unit_id) ON DELETE CASCADE,
  FOREIGN KEY (unit_id) REFERENCES continuous_units(id) ON DELETE CASCADE
);

-- We also need to add a bi-directional cascade delete constraint to the editables
-- table, so that when a procedure continuous requirement is deleted, the corresponding editable is also deleted.
-- Since the editables table is referenced by several other tables, we cannot add a cascade
-- delete constraint to the editables table. Instead, we add a trigger to delete the corresponding
-- record in the editables table when a procedure continuous requirement is deleted.
CREATE OR REPLACE FUNCTION delete_editables() RETURNS TRIGGER AS $$
BEGIN
    DELETE FROM
        editables
    WHERE
        id = OLD.id;

    RETURN OLD;

END;

$$ LANGUAGE plpgsql;

CREATE TRIGGER delete_editables AFTER
DELETE
    ON procedure_continuous_requirements FOR EACH ROW EXECUTE FUNCTION delete_editables();
