-- SQL defining a state that a project may be in.
CREATE TABLE project_states (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    font_awesome_icon TEXT NOT NULL,
    icon_color TEXT NOT NULL
);
