-- SQL creating a view that collects the information regarding the edits.
--
-- In includes the following joins:
-- 1. edits JOIN editables ON edits.id = editables.id (To retrieve when and who made the edit)
-- 2. users ON editables.created_by = users.id (To retrieve the user who made the edit)
-- 3. describables ON editables.id = describables.id (To retrieve the reason of the edit)

CREATE VIEW edits_view AS
SELECT
    edits.id,
    edits.editable_id AS editable_id,
    editables.created_at AS edited_at,
    users.id AS editor_id,
    users.first_name AS editor_first_name,
    users.middle_name AS editor_middle_name,
    users.last_name AS editor_last_name,
    describables.name AS edit_title,
    describables.description AS edit_extended_reason
FROM edits
JOIN editables ON edits.id = editables.id
JOIN users ON editables.created_by = users.id
JOIN describables ON editables.id = describables.id;
