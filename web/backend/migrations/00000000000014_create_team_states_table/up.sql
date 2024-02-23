-- SQL defining the team states table.
-- A team state is a state that a team may be in.
-- A team state is used to describe the state of a team, such as active, inactive, or archived.
CREATE TABLE team_states (
    id BIGINT PRIMARY KEY REFERENCES editables(id) ON DELETE CASCADE REFERENCES describables(id) ON DELETE CASCADE,
    font_awesome_icon VARCHAR(255)
);