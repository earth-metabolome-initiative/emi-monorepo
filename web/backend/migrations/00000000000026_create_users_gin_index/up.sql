-- Since the operation needs to be immutable, we need to create a postgresql
-- function that concatenates the three columns with spaces, and that handles
-- the case where the middle_name is null.
CREATE FUNCTION concat_users_name(
  first_name text,
  middle_name text,
  last_name text
) RETURNS text AS $$
BEGIN
  CASE
    WHEN middle_name IS NULL THEN
      RETURN first_name || ' ' || last_name;
    ELSE
      RETURN first_name || ' ' || middle_name || ' ' || last_name;
  END CASE;
END;
$$ LANGUAGE plpgsql IMMUTABLE STRICT PARALLEL SAFE;


CREATE INDEX users_name_trgm_idx ON users USING gin (
  concat_users_name(first_name, middle_name, last_name) gin_trgm_ops
);