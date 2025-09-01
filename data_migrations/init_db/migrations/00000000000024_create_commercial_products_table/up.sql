CREATE TABLE IF NOT EXISTS commercial_products (
	id INTEGER PRIMARY KEY REFERENCES physical_asset_models(id),
	deprecation_date TIMESTAMP WITH TIME ZONE,
	brand_id INTEGER NOT NULL REFERENCES brands(id)
);
CREATE TABLE IF NOT EXISTS commercial_weighing_device_models (
	id INTEGER PRIMARY KEY,
	FOREIGN KEY (id) REFERENCES weighing_device_models(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES commercial_products(id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS commercial_volume_measuring_device_models (
	id INTEGER PRIMARY KEY,
	FOREIGN KEY (id) REFERENCES volume_measuring_device_models(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES commercial_products(id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS commercial_pipette_models (
	id INTEGER PRIMARY KEY,
	FOREIGN KEY (id) REFERENCES pipette_models(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES commercial_products(id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS commercial_pipette_tip_models (
	id INTEGER PRIMARY KEY,
	FOREIGN KEY (id) REFERENCES pipette_tip_models(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES commercial_products(id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS commercial_packaging_models (
	id INTEGER PRIMARY KEY,
	FOREIGN KEY (id) REFERENCES packaging_models(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES commercial_products(id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS commercial_beads_models (
	id INTEGER PRIMARY KEY,
	FOREIGN KEY (id) REFERENCES beads_models(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES commercial_products(id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS commercial_ball_mill_machine_models (
	id INTEGER PRIMARY KEY,
	FOREIGN KEY (id) REFERENCES ball_mill_machine_models(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES commercial_products(id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS commercial_centrifuge_models (
	id INTEGER PRIMARY KEY,
	FOREIGN KEY (id) REFERENCES centrifuge_models(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES commercial_products(id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS commercial_freezer_models (
	id INTEGER PRIMARY KEY,
	FOREIGN KEY (id) REFERENCES freezer_models(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES commercial_products(id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS commercial_freeze_dryer_models (
	id INTEGER PRIMARY KEY,
	FOREIGN KEY (id) REFERENCES freeze_dryer_models(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES commercial_products(id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS commercial_positioning_device_models (
	id INTEGER PRIMARY KEY,
	FOREIGN KEY (id) REFERENCES positioning_device_models(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES commercial_products(id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS commercial_camera_models (
	id INTEGER PRIMARY KEY,
	FOREIGN KEY (id) REFERENCES camera_models(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES commercial_products(id) ON DELETE CASCADE
);