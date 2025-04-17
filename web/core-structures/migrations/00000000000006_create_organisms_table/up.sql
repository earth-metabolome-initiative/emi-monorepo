CREATE TABLE IF NOT EXISTS organisms (
  id UUID PRIMARY KEY REFERENCES trackables(id),
  name TEXT CHECK (must_not_be_empty(name)),
  nameplate_category_id SMALLINT NOT NULL REFERENCES nameplate_categories(id)
);
