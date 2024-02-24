-- Your SQL goes here
CREATE TABLE project_continuous_requirements (
  id BIGINT PRIMARY KEY REFERENCES editables(id) ON DELETE CASCADE,
  project_id BIGINT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
  item_id BIGINT NOT NULL REFERENCES item_categories(id) ON DELETE CASCADE,
  quantity FLOAT NOT NULL,
  unit_id BIGINT REFERENCES units(id) ON DELETE CASCADE,
  UNIQUE (project_id, item_id),
  FOREIGN KEY (item_id, unit_id) REFERENCES item_units(item_id, unit_id) ON DELETE CASCADE,
  FOREIGN KEY (unit_id) REFERENCES continuous_units(id) ON DELETE CASCADE
);

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
    ON project_continuous_requirements FOR EACH ROW EXECUTE FUNCTION delete_editables();
