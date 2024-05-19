-- This is a no-op SQL statement
COPY font_awesome_icons(
    name,
    description
)
FROM '/app/font_awesome_icons.csv' DELIMITER ',' CSV HEADER;