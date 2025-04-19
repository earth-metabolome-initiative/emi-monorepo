-- UP MIGRATION
CREATE TABLE IF NOT EXISTS trackables (
    id UUID PRIMARY KEY,
    container_model_id INTEGER NOT NULL REFERENCES container_models(id),
    project_id INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    trackable_state_id INTEGER NOT NULL REFERENCES trackable_states(id)
    created_by INTEGER NOT NULL REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by INTEGER NOT NULL REFERENCES users(id),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
);

CREATE TABLE IF NOT EXISTS trackable_locations (
	id UUID PRIMARY KEY,
	mixture_id UUID NOT NULL REFERENCES trackables(id),
	storage_container_id UUID REFERENCES storage_containers(id),
	geolocation GEOGRAPHY(POINT, 4326) NOT NULL,
	inferred BOOLEAN NOT NULL DEFAULT FALSE,
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	created_by INTEGER NOT NULL REFERENCES users(id)
);
