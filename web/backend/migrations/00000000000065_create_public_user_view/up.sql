-- SQL defining the view for a public user, i.e. the set of information on
-- a user that is safe to expose to the public.
CREATE VIEW public_user AS
SELECT
    id,
    first_name,
    middle_name,
    last_name,
    created_at,
    updated_at
FROM
    users;