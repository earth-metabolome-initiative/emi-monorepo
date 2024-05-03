-- This is a no-op SQL statement

COPY document_formats(extension, mime_type) FROM '/app/document_formats.csv' DELIMITER ',' CSV HEADER;
