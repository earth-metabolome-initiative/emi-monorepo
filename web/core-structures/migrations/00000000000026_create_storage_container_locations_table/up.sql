CREATE TABLE storage_container_locations (
	id UUID PRIMARY KEY,
	storage_container_id UUID NOT NULL REFERENCES storage_containers(id),
	parent_storage_container_location_id UUID REFERENCES storage_container_locations(id),
	room_id UUID REFERENCES rooms(id),
	geolocation GEOGRAPHY(POINT, 4326) NOT NULL,
	inferred BOOLEAN NOT NULL DEFAULT FALSE,
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);