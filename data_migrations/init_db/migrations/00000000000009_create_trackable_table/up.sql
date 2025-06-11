CREATE TABLE IF NOT EXISTS trackables (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) UNIQUE CHECK (must_be_paragraph(description)),
    description TEXT CHECK (must_be_paragraph(description)),
    photograph_id UUID REFERENCES documents(id),
    parent_id UUID REFERENCES trackables(id) ON DELETE CASCADE,
    created_by INTEGER NOT NULL REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by INTEGER NOT NULL REFERENCES users(id),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CHECK (must_be_distinct(name, description)),
    CHECK (must_be_distinct_uuid(id, parent_id)),
    CHECK (must_be_smaller_than_utc(created_at, updated_at))
);

CREATE TABLE IF NOT EXISTS trackable_locations (
	id UUID PRIMARY KEY,
	trackable_id UUID NOT NULL REFERENCES trackables(id),
	container_id UUID REFERENCES trackables(id),
	geolocation GEOGRAPHY(POINT, 4326) NOT NULL,
	inferred BOOLEAN NOT NULL DEFAULT FALSE,
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	created_by INTEGER NOT NULL REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS container_models (
    -- We need to use as a parent a trackable and not a commercial product
    -- because unfortunately in several experimental setups the containers
    -- are not from known brands.
	id UUID PRIMARY KEY REFERENCES trackables(id),
	liters REAL NOT NULL CHECK (must_be_strictly_positive_f32(liters))
);