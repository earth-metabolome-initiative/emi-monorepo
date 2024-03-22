-- Your SQL goes here
CREATE TABLE edits (
    -- The id of the edit, inserted as an editable created by some user.
    id UUID PRIMARY KEY REFERENCES editables(id) ON DELETE CASCADE REFERENCES describables(id) ON DELETE CASCADE,
    -- The id of the editable object that was edited
    editable_id UUID NOT NULL REFERENCES editables(id) ON DELETE CASCADE
);
