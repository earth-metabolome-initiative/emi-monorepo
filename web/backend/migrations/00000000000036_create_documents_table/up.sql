CREATE TABLE documents (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    author_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    path VARCHAR(255) NOT NULL,
    format_id INTEGER NOT NULL REFERENCES document_formats(id) ON
    DELETE
        CASCADE,
        bytes INTEGER NOT NULL
);