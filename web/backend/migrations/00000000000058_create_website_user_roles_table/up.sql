-- SQL defining the assigned role of users in the website.

CREATE TABLE website_user_roles (
    id BIGINT PRIMARY KEY REFERENCES editables(id) ON DELETE cascade,
    website_role_id BIGINT NOT NULL REFERENCES website_roles(id) ON DELETE cascade,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE cascade,
    UNIQUE (website_role_id, user_id)
);

-- Add a trigger to delete the corresponding record in the editables table when a website_user_role is deleted.
CREATE OR REPLACE FUNCTION delete_editables() RETURNS TRIGGER AS $$
BEGIN
    DELETE FROM
        editables
    WHERE
        id = OLD.id;

    RETURN OLD;

END;

$$ LANGUAGE plpgsql;

CREATE TRIGGER delete_editables AFTER
DELETE
    ON website_user_roles FOR EACH ROW EXECUTE FUNCTION delete_editables();