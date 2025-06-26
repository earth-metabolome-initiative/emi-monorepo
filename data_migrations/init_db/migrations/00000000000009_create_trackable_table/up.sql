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
	id UUID PRIMARY KEY REFERENCES trackables(id)
);

CREATE TABLE IF NOT EXISTS volumetric_container_models (
	id UUID PRIMARY KEY REFERENCES container_models(id),
    -- The maximum volume of the container in liters.
	liters REAL NOT NULL CHECK (must_be_strictly_positive_f32(liters))
);

CREATE TABLE IF NOT EXISTS compatibility_rules (
    left_trackable_id UUID NOT NULL REFERENCES trackables(id),
    right_trackable_id UUID NOT NULL REFERENCES trackables(id),
    -- The maximal quantity of the right trackable that can be associated with the left trackable.
    quantity SMALLINT CHECK (must_be_strictly_positive_i16(quantity)),
    created_by INTEGER NOT NULL REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (left_trackable_id, right_trackable_id),
    CHECK (must_be_distinct_uuid(left_trackable_id, right_trackable_id))
);
