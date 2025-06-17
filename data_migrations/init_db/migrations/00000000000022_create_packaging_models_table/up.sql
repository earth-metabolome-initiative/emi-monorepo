CREATE TABLE IF NOT EXISTS packaging_models (
	trackable_id UUID PRIMARY KEY REFERENCES trackables(id),
	material_id SMALLINT NOT NULL REFERENCES materials(id)
);