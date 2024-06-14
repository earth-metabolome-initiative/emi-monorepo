-- SQL defining the document_formats table creation.
-- The taple specifies the set of possible document formats that can be used in the system.
CREATE TABLE IF NOT EXISTS document_formats (
  id INTEGER PRIMARY KEY,
  extension text NOT NULL UNIQUE,
  mime_type text NOT NULL,
  description TEXT NOT NULL,
  icon_id INTEGER NOT NULL,
  color_id INTEGER NOT NULL,
  FOREIGN KEY (icon_id) REFERENCES font_awesome_icons(id),
  FOREIGN KEY (color_id) REFERENCES colors(id)
);
