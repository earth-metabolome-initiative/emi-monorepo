-- SQL defining a state that a project may be in.
CREATE TABLE project_states (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    font_awesome_icon TEXT NOT NULL,
    icon_color TEXT NOT NULL
);
