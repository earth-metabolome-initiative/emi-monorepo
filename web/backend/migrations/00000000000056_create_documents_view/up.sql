-- SQL to create a view that combines the documents, editables, users, edits, describables and document_formats tables
-- so to provide a single view of the documents in the system.
-- The view is read-only and is used by the backend to provide a single view of the documents
-- in the system.
--
-- By building on top of the other available views, specifically:
-- 1. last_edits_view, which provides the latest edit for each document (if it exists)
-- 2. formats_view, which provides information regarding the document format
--
-- We run the following join statements:
-- 1. documents JOIN editables ON documents.id = editables.id, so to retrieve when the document was created and by who
-- 2. editables JOIN users ON editables.created_by = users.id, so to retrieve the creator of the document
-- 3. documents JOIN last_edits_view ON documents.id = last_edits_view.editable_id, so to retrieve the latest edit for each document, if it exists
-- 4. documents JOIN format_view ON documents.format_id = format_view.id, so to retrieve the format of the document
-- 5. documents JOIN describables ON documents.id = describables.id, so to retrieve the description of the document

CREATE VIEW documents_view AS
SELECT
    documents.id,
    documents.format_id,
    documents.path AS document_path,
    documents.bytes AS bytes,
    describables.name AS document_name,
    describables.description,
    editables.created_by AS creator_id,
    users.first_name AS creator_first_name,
    users.middle_name AS creator_middle_name,
    users.last_name AS creator_last_name,
    last_edits_view.edit_id AS last_edit_id,
    last_edits_view.edited_at AS last_edit_at,
    last_edits_view.editor_id AS last_editor_id,
    last_edits_view.editor_first_name AS last_editor_first_name,
    last_edits_view.editor_middle_name AS last_editor_middle_name,
    last_edits_view.editor_last_name AS last_editor_last_name,
    last_edits_view.edit_title AS last_edit_title,
    last_edits_view.edit_extended_reason AS last_edit_extended_reason,
    formats_view.extension
FROM documents
JOIN editables ON documents.id = editables.id
JOIN users ON editables.created_by = users.id
LEFT JOIN last_edits_view ON documents.id = last_edits_view.editable_id
JOIN formats_view ON documents.format_id = formats_view.id
JOIN describables ON documents.id = describables.id;

