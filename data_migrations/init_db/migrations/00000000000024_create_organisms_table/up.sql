CREATE TABLE IF NOT EXISTS organisms (
  id UUID PRIMARY KEY REFERENCES trackables(id),
  name TEXT CHECK (must_be_paragraph(name)),
  nameplate_category NameplateCategory NOT NULL
);
