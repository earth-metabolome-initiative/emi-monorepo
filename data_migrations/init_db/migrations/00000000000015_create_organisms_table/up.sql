CREATE TABLE IF NOT EXISTS organisms (
  id UUID PRIMARY KEY REFERENCES trackables(id),
  nameplate_category NameplateCategory NOT NULL
);
