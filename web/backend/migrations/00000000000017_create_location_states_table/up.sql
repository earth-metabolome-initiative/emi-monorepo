-- SQL defining the location states table.
-- A location state is a state that a location may be in.
-- A location state is used to describe the state of a location, such as destroyed, at risk, or safe.
CREATE TABLE location_states (
    id BIGINT PRIMARY KEY REFERENCES editables(id) ON DELETE CASCADE REFERENCES describables(id) ON DELETE CASCADE,
    font_awesome_icon VARCHAR(255)
);