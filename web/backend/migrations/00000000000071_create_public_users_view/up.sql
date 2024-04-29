-- Create view with the principal informations of the users
-- Additionally, we also find from the documents table two documents that are related to the user
-- with the first being the thumbnail of the user, identified by a path "/api/documents/profile/picture/{users.id}/thumbnail.png" and as
-- creator the user itself, and the second being the profile picture of the user, identified by a path "/api/documents/profile/picture/{users.id}/standard.png" and as
-- creator the user itself. Both of these two latter fields are optional, and if they are not found, the field will be null.

CREATE VIEW public_users AS
SELECT
    users.id,
    users.first_name,
    users.middle_name,
    users.last_name,
    users.created_at,
    users.updated_at,
    thumbnail_documents.id AS thumbnail_id,
    profile_picture_documents.id AS picture_id
FROM users
LEFT JOIN documents AS thumbnail_documents ON thumbnail_documents.author_id = users.id AND thumbnail_documents.path = '/api/documents/profile/picture/' || users.id || '/thumbnail.png'
LEFT JOIN documents AS profile_picture_documents ON profile_picture_documents.author_id = users.id AND profile_picture_documents.path = '/api/documents/profile/picture/' || users.id || '/standard.png';