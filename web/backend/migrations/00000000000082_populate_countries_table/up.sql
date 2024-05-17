COPY countries(
    ISO,
    emoji,
    unicode,
    name
)

FROM '/app/countries.csv' DELIMITER ',' CSV HEADER;