-- SQL defining the organization states table.
-- A organization state is a state that a organization may be in.
-- A organization state is used to describe the state of a organization, such as active, inactive, or archived.
CREATE TABLE organization_states (
    id BIGINT PRIMARY KEY REFERENCES editables(id) ON DELETE CASCADE REFERENCES describables(id) ON DELETE CASCADE,
    font_awesome_icon VARCHAR(255)
);