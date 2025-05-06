CREATE TABLE IF NOT EXISTS organisms (
  id UUID PRIMARY KEY REFERENCES trackables(id),
  name TEXT CHECK (must_not_be_empty(name)),
  nameplate_category NameplateCategory NOT NULL
);
