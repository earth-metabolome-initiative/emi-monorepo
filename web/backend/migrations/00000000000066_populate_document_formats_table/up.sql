-- This is a no-op SQL statement

-- Loads from the file document_formats.csv the table document_formats
--
-- The file has headers:
--     name,font_awesome_icon
CREATE TEMPORARY TABLE tmp_document_formats(extension TEXT, mime_type TEXT, description TEXT, font_awesome_icon TEXT, color TEXT);

COPY tmp_document_formats
FROM
    '/app/document_formats.csv' DELIMITER ',' CSV HEADER;

INSERT INTO
    document_formats(extension, mime_type, description, icon_id, color_id)
SELECT
    tmp_document_formats.extension,
    tmp_document_formats.mime_type,
    tmp_document_formats.description,
    font_awesome_icons.id,
    colors.id
FROM
    tmp_document_formats
    JOIN font_awesome_icons ON tmp_document_formats.font_awesome_icon = font_awesome_icons.name
    JOIN colors ON tmp_document_formats.color = colors.name;

-- now we want to assert that the number of lines in the sample_states table is the same as the number 
-- of lines in the tmp_sample_states table
DO $$ DECLARE tmp_document_formats_count INTEGER;

document_formats_count INTEGER;

BEGIN
SELECT
    COUNT(*) INTO tmp_document_formats_count
FROM
    tmp_document_formats;

SELECT
    COUNT(*) INTO document_formats_count
FROM
    document_formats;

IF tmp_document_formats_count <> document_formats_count THEN RAISE EXCEPTION 'The number of rows in the tmp_document_formats table is different from the number of rows in the document_formats table';

END IF;

END $$;

DROP TABLE tmp_document_formats;