-- SQL query creating a mockup of the users table.
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username TEXT NOT NULL,
    email TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (username),
    UNIQUE (email),
    UNIQUE (username, email)
);

CREATE TABLE projects (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    parent_project_id INTEGER REFERENCES projects(id) ON DELETE CASCADE,
    created_by INTEGER NOT NULL REFERENCES users(id),
    updated_by INTEGER NOT NULL REFERENCES users(id),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE teams (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    created_by INTEGER NOT NULL REFERENCES users(id),
    updated_by INTEGER NOT NULL REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS team_members (
    member_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    team_id INTEGER NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    PRIMARY KEY (member_id, team_id)
);

CREATE TABLE IF NOT EXISTS team_projects (
    project_id INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    team_id INTEGER NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    PRIMARY KEY (project_id, team_id)
);

CREATE TABLE IF NOT EXISTS trackables (
	id SERIAL PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS procedure_models (
	id SERIAL PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS trackable_procedure_models (
	id INTEGER PRIMARY KEY,
	FOREIGN KEY (id) REFERENCES trackables(id),
	FOREIGN KEY (id) REFERENCES procedure_models(id)
);

CREATE TABLE IF NOT EXISTS weighing_procedure_models (
	id INTEGER PRIMARY KEY REFERENCES procedure_models(id)
);

CREATE TABLE IF NOT EXISTS weighing_trackable_procedure_models (
	id INTEGER PRIMARY KEY,
	FOREIGN KEY (id) REFERENCES trackable_procedure_models(id),
	FOREIGN KEY (id) REFERENCES weighing_procedure_models(id)
);