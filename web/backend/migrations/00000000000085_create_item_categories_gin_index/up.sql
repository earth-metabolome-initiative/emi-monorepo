-- Create index to run approximate search queries on the item_categories table.
-- The search will be case insensitive and will use the trigram index.

CREATE EXTENSION IF NOT EXISTS pg_trgm;

CREATE FUNCTION f_concat_item_categories_name_description(
name text,
description text
) RETURNS text AS $$
BEGIN
  CASE
    WHEN description IS NULL THEN
      RETURN name;
    ELSE
      RETURN name || ' ' || description;
  END CASE;
END;
$$ LANGUAGE plpgsql IMMUTABLE STRICT PARALLEL SAFE;

CREATE INDEX item_categories_name_description_trgm_idx ON item_categories USING gin (
f_concat_item_categories_name_description(
name,
description
) gin_trgm_ops
);
