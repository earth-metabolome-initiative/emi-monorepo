-- This is a no-op SQL statement
COPY colors(
    name,
    hexadecimal_value,
    description
)
FROM '/app/colors.csv' DELIMITER ',' CSV HEADER;