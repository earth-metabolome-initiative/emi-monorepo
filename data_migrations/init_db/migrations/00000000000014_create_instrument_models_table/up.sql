CREATE TABLE IF NOT EXISTS weighing_device_models (
	id INTEGER PRIMARY KEY REFERENCES physical_asset_models (id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS volume_measuring_device_models (
	id INTEGER PRIMARY KEY REFERENCES physical_asset_models (id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS positioning_device_models (
	id INTEGER PRIMARY KEY REFERENCES physical_asset_models (id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS camera_models (
	id INTEGER PRIMARY KEY REFERENCES physical_asset_models (id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS pipette_models (
	id INTEGER PRIMARY KEY REFERENCES physical_asset_models (id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS pipette_tip_models (
	id INTEGER PRIMARY KEY REFERENCES physical_asset_models (id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS phone_models (
	id INTEGER PRIMARY KEY,
	CONSTRAINT phone_models_camera FOREIGN KEY (id) REFERENCES camera_models (id) ON DELETE CASCADE,
	CONSTRAINT phone_models_positioning FOREIGN KEY (id) REFERENCES positioning_device_models (id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS freezer_models (
	id INTEGER PRIMARY KEY REFERENCES physical_asset_models (id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS freeze_dryer_models (
	id INTEGER PRIMARY KEY REFERENCES physical_asset_models (id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS centrifuge_models (
	id INTEGER PRIMARY KEY REFERENCES physical_asset_models (id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS ball_mill_machine_models (
	id INTEGER PRIMARY KEY REFERENCES physical_asset_models (id) ON DELETE CASCADE
);