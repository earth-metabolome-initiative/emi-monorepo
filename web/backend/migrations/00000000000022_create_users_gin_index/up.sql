-- Since the operation needs to be immutable, we need to create a postgresql
-- function that concatenates the three columns with spaces, and that handles
-- the case where the middle_value is null.
CREATE FUNCTION f_concat_users_name(
  first_value text,
  middle_value text,
  last_value text
) RETURNS text AS $$
BEGIN
  CASE
    WHEN middle_value IS NULL THEN
      RETURN first_value || ' ' || last_value;
    ELSE
      RETURN first_value || ' ' || middle_value || ' ' || last_value;
  END CASE;
END;
$$ LANGUAGE plpgsql IMMUTABLE STRICT PARALLEL SAFE;


CREATE INDEX users_name_trgm_idx ON users USING gin (
  f_concat_users_name(first_name, middle_name, last_name) gin_trgm_ops
);