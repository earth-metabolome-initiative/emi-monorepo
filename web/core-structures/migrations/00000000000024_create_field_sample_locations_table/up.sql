CREATE TABLE IF NOT EXISTS field_sample_locations (
	id UUID PRIMARY KEY,
	field_sample_id UUID NOT NULL REFERENCES field_samples(id),
	storage_container_id UUID REFERENCES storage_containers(id),
	geolocation GEOGRAPHY(POINT, 4326) NOT NULL,
	inferred BOOLEAN NOT NULL DEFAULT FALSE,
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	created_by INTEGER NOT NULL REFERENCES users(id)
);