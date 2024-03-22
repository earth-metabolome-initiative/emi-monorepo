-- SQL defining the view that provides combined information regarding the available document formats.
-- 
-- It combines the information from the describables table to provide the title and description of the format.
CREATE VIEW formats_view AS
SELECT
    document_formats.id,
    describables.name AS extension,
    describables.description AS format_description
FROM document_formats
JOIN describables ON document_formats.id = describables.id;