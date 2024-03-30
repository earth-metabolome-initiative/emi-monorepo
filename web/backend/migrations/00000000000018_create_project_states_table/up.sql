-- SQL defining a state that a project may be in.
CREATE TABLE project_states (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR NOT NULL,
    description TEXT NOT NULL,
    font_awesome_icon VARCHAR NOT NULL,
    icon_color VARCHAR NOT NULL
);
