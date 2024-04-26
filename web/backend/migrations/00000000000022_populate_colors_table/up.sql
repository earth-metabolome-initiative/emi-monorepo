-- This is a no-op SQL statement
COPY colors(
    name,
    hexadecimal_value
)
FROM '/app/colors.csv' DELIMITER ',' CSV HEADER;