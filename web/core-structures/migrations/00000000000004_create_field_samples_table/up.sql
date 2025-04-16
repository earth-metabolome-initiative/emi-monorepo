-- UP MIGRATION
CREATE TABLE IF NOT EXISTS field_samples (
    id UUID PRIMARY KEY,
    container_model_id INTEGER NOT NULL REFERENCES container_models(id),
    project_id INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    created_by INTEGER NOT NULL REFERENCES users(id),
    sampled_by INTEGER NOT NULL REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by INTEGER NOT NULL REFERENCES users(id),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    state_id SMALLINT NOT NULL DEFAULT 1 REFERENCES sample_states(id)
);

CREATE TABLE IF NOT EXISTS field_sample_locations (
	id UUID PRIMARY KEY,
	field_sample_id UUID NOT NULL REFERENCES field_samples(id),
	storage_container_id UUID REFERENCES storage_containers(id),
	geolocation GEOGRAPHY(POINT, 4326) NOT NULL,
	inferred BOOLEAN NOT NULL DEFAULT FALSE,
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	created_by INTEGER NOT NULL REFERENCES users(id)
);