CREATE TABLE IF NOT EXISTS instruments (
	id UUID PRIMARY KEY REFERENCES trackables(id) ON DELETE CASCADE,
	instrument_model_id UUID NOT NULL REFERENCES instrument_models(id) ON DELETE CASCADE
);
