-- Your SQL goes here
CREATE TABLE edits (
  id BIGINT PRIMARY KEY REFERENCES editables(id) ON DELETE CASCADE REFERENCES describables(id) ON DELETE CASCADE,
  edited_by INTEGER NOT NULL REFERENCES users(id),
  edited_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
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
    ON edits FOR EACH ROW EXECUTE FUNCTION delete_editables();
