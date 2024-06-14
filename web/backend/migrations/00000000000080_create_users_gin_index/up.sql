-- Since the operation needs to be immutable, we need to create a postgresql
-- function that concatenates the three columns with spaces, and that handles
-- the case where the middle_name is null.
CREATE FUNCTION concat_users_name(
  first_name text,
  last_name text
) RETURNS text AS $$
BEGIN
  RETURN first_name || ' ' || last_name;
END;
$$ LANGUAGE plpgsql IMMUTABLE STRICT PARALLEL SAFE;


CREATE INDEX users_name_trgm_idx ON users USING gin (
  concat_users_name(first_name, last_name) gin_trgm_ops
);