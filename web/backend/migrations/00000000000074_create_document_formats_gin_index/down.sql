-- Drop index on the document_formats table.
-- The index was used to run approximate search queries on the table.
DROP INDEX document_formats_extension_mime_type_trgm_idx;

DROP FUNCTION concat_document_formats_extension_mime_type(extension text, mime_type text);