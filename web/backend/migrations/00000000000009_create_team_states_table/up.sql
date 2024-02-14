-- SQL defining the team states table.
-- A team state is a state that a team may be in.
-- A team state is used to describe the state of a team, such as active, inactive, or archived.
CREATE TABLE team_states (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    font_awesome_icon VARCHAR(255),
    editable_id INTEGER NOT NULL REFERENCES editables(id) ON DELETE CASCADE
);