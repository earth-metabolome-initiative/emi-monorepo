-- SQL defining the organization states table.
-- A organization state is a state that a organization may be in.
-- A organization state is used to describe the state of a organization, such as active, inactive, or archived.
CREATE TABLE organization_states (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    font_awesome_icon VARCHAR(255),
    editable_id INTEGER NOT NULL REFERENCES editables(id) ON DELETE CASCADE
);