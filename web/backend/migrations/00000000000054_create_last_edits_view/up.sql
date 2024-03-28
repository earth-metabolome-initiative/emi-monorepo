-- SQL defining the last_edits_view, which builds on the last_edits table and
-- for each document, i.e. the editable_id, only takes into account the latest
-- edit, ordered by the edited_at timestamp.
CREATE VIEW last_edits_view AS
SELECT
    last_edits.id,
    last_edits.editable_id,
    last_edits.edited_at,
    last_edits.editor_id,
    last_edits.editor_first_name,
    last_edits.editor_middle_name,
    last_edits.editor_last_name,
    last_edits.edit_title,
    last_edits.edit_extended_reason
FROM (
    SELECT
        id,
        editable_id,
        edited_at,
        editor_id,
        editor_first_name,
        editor_middle_name,
        editor_last_name,
        edit_title,
        edit_extended_reason,
        ROW_NUMBER() OVER (PARTITION BY editable_id ORDER BY edited_at DESC) AS rn
    FROM edits_view
) AS last_edits
WHERE last_edits.rn = 1;
