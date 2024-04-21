CREATE TABLE items (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  parent_id UUID REFERENCES items(id) ON DELETE SET NULL
);