CREATE TABLE
	IF NOT EXISTS weighing_devices (
		id UUID PRIMARY KEY REFERENCES trackables (id) ON DELETE CASCADE,
		model_id UUID NOT NULL REFERENCES commercial_weighing_device_models (id),
		FOREIGN KEY (id, model_id) REFERENCES trackables (id, parent_id) ON DELETE CASCADE
	);
