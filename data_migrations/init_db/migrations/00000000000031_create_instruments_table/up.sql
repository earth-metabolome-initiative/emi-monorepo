CREATE TABLE IF NOT EXISTS weighing_devices (
	id UUID PRIMARY KEY REFERENCES physical_assets (id) ON DELETE CASCADE,
	model INTEGER NOT NULL REFERENCES commercial_weighing_device_lots (id),
	FOREIGN KEY (id, model) REFERENCES assets (id, model)
);
CREATE TABLE IF NOT EXISTS volume_measuring_devices (
	id UUID PRIMARY KEY REFERENCES physical_assets (id) ON DELETE CASCADE,
	model INTEGER NOT NULL REFERENCES commercial_volume_measuring_device_lots (id),
	FOREIGN KEY (id, model) REFERENCES assets (id, model)
);
CREATE TABLE IF NOT EXISTS pipettes (
	id UUID PRIMARY KEY REFERENCES physical_assets (id) ON DELETE CASCADE,
	model INTEGER NOT NULL REFERENCES commercial_pipette_lots (id),
	FOREIGN KEY (id, model) REFERENCES assets (id, model)
);
CREATE TABLE IF NOT EXISTS ball_mill_machines (
	id UUID PRIMARY KEY REFERENCES physical_assets (id) ON DELETE CASCADE,
	model INTEGER NOT NULL REFERENCES commercial_ball_mill_machine_lots (id),
	FOREIGN KEY (id, model) REFERENCES assets (id, model)
);
CREATE TABLE IF NOT EXISTS centrifuges (
	id UUID PRIMARY KEY REFERENCES physical_assets (id) ON DELETE CASCADE,
	model INTEGER NOT NULL REFERENCES commercial_centrifuge_lots (id),
	FOREIGN KEY (id, model) REFERENCES assets (id, model)
);
CREATE TABLE IF NOT EXISTS freezers (
	id UUID PRIMARY KEY REFERENCES physical_assets (id) ON DELETE CASCADE,
	model INTEGER NOT NULL REFERENCES commercial_freezer_lots (id),
	FOREIGN KEY (id, model) REFERENCES assets (id, model)
);
CREATE TABLE IF NOT EXISTS freeze_dryers (
	id UUID PRIMARY KEY REFERENCES physical_assets (id) ON DELETE CASCADE,
	model INTEGER NOT NULL REFERENCES commercial_freeze_dryer_lots (id),
	FOREIGN KEY (id, model) REFERENCES assets (id, model)
);
CREATE TABLE IF NOT EXISTS positioning_devices (
	id UUID PRIMARY KEY REFERENCES physical_assets (id) ON DELETE CASCADE,
	model INTEGER NOT NULL REFERENCES commercial_positioning_device_lots (id),
	FOREIGN KEY (id, model) REFERENCES assets (id, model)
);
CREATE TABLE IF NOT EXISTS cameras (
	id UUID PRIMARY KEY REFERENCES physical_assets (id) ON DELETE CASCADE,
	model INTEGER NOT NULL REFERENCES commercial_camera_lots (id),
	FOREIGN KEY (id, model) REFERENCES assets (id, model)
);