-- This is a no-op SQL statement
COPY font_awesome_icons(
    name
)
FROM '/app/font_awesome_icons.csv' DELIMITER ',' CSV HEADER;