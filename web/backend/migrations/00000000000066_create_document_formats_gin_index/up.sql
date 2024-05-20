-- Create index to run approximate search queries on the document_formats table.
-- The search will be case insensitive and will use the trigram index.
CREATE FUNCTION f_concat_document_formats_extension_mime_type(extension text, mime_type text) RETURNS text AS $$ BEGIN
    RETURN extension || ' ' || mime_type;

END;

$$ LANGUAGE plpgsql IMMUTABLE STRICT PARALLEL SAFE;

CREATE INDEX document_formats_extension_mime_type_trgm_idx ON document_formats USING gin (
    f_concat_document_formats_extension_mime_type(extension, mime_type) gin_trgm_ops
);