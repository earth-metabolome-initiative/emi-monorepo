CREATE TABLE IF NOT EXISTS photographs (
	id UUID PRIMARY KEY REFERENCES digital_assets(id)
);