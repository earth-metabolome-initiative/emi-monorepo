-- UP MIGRATION
CREATE TABLE IF NOT EXISTS samples (
    id UUID PRIMARY KEY,
    container_id INTEGER NOT NULL UNIQUE,
    notes TEXT,
    project_id INTEGER NOT NULL,
    created_by INTEGER NOT NULL,
    sampled_by INTEGER NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by INTEGER NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    state_id SMALLINT NOT NULL DEFAULT 1,
    FOREIGN KEY (container_id) REFERENCES sample_containers(id) ON DELETE CASCADE,
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
    FOREIGN KEY (created_by) REFERENCES users(id),
    FOREIGN KEY (sampled_by) REFERENCES users(id),
    FOREIGN KEY (updated_by) REFERENCES users(id),
    FOREIGN KEY (state_id) REFERENCES sample_states(id)
);