-- Create the index to search approximately the composite columns of
-- project states, including name.
CREATE INDEX font_awesome_icons_name_trgm_idx ON font_awesome_icons USING gin (name gin_trgm_ops);