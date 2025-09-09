CREATE TABLE IF NOT EXISTS commercial_products (
	id INTEGER PRIMARY KEY REFERENCES asset_models(id),
	deprecation_date TIMESTAMP WITH TIME ZONE,
	brand_id INTEGER NOT NULL REFERENCES brands(id)
);
CREATE TABLE IF NOT EXISTS commercial_weighing_device_models (
	id INTEGER PRIMARY KEY,
	weighing_device_model INTEGER NOT NULL REFERENCES weighing_device_models(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES weighing_device_models(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES commercial_products(id) ON DELETE CASCADE,
	FOREIGN KEY (id, weighing_device_model) REFERENCES asset_models(id, parent_model)
);
CREATE TABLE IF NOT EXISTS commercial_volume_measuring_device_models (
	id INTEGER PRIMARY KEY,
	volume_measuring_device_model INTEGER NOT NULL REFERENCES volume_measuring_device_models(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES volume_measuring_device_models(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES commercial_products(id) ON DELETE CASCADE,
	FOREIGN KEY (id, volume_measuring_device_model) REFERENCES asset_models(id, parent_model)
);
CREATE TABLE IF NOT EXISTS commercial_pipette_models (
	id INTEGER PRIMARY KEY,
	pipette_model INTEGER NOT NULL REFERENCES pipette_models(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES pipette_models(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES commercial_products(id) ON DELETE CASCADE,
	FOREIGN KEY (id, pipette_model) REFERENCES asset_models(id, parent_model)
);
CREATE TABLE IF NOT EXISTS commercial_pipette_tip_models (
	id INTEGER PRIMARY KEY,
	pipette_tip_model INTEGER NOT NULL REFERENCES pipette_tip_models(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES pipette_tip_models(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES commercial_products(id) ON DELETE CASCADE,
	FOREIGN KEY (id, pipette_tip_model) REFERENCES asset_models(id, parent_model)
);
CREATE TABLE IF NOT EXISTS commercial_packaging_models (
	id INTEGER PRIMARY KEY,
	packaging_model INTEGER NOT NULL REFERENCES packaging_models(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES packaging_models(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES commercial_products(id) ON DELETE CASCADE,
	FOREIGN KEY (id, packaging_model) REFERENCES asset_models(id, parent_model)
);
CREATE TABLE IF NOT EXISTS commercial_bead_models (
	id INTEGER PRIMARY KEY,
	bead_model INTEGER NOT NULL REFERENCES bead_models(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES bead_models(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES commercial_products(id) ON DELETE CASCADE,
	FOREIGN KEY (id, bead_model) REFERENCES asset_models(id, parent_model)
);
CREATE TABLE IF NOT EXISTS commercial_cap_models (
	id INTEGER PRIMARY KEY,
	cap_model INTEGER NOT NULL REFERENCES cap_models(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES cap_models(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES commercial_products(id) ON DELETE CASCADE,
	FOREIGN KEY (id, cap_model) REFERENCES asset_models(id, parent_model)
);
CREATE TABLE IF NOT EXISTS commercial_ball_mill_machine_models (
	id INTEGER PRIMARY KEY,
	ball_mill_machine_model INTEGER NOT NULL REFERENCES ball_mill_machine_models(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES ball_mill_machine_models(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES commercial_products(id) ON DELETE CASCADE,
	FOREIGN KEY (id, ball_mill_machine_model) REFERENCES asset_models(id, parent_model)
);
CREATE TABLE IF NOT EXISTS commercial_centrifuge_models (
	id INTEGER PRIMARY KEY,
	centrifuge_model INTEGER NOT NULL REFERENCES centrifuge_models(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES centrifuge_models(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES commercial_products(id) ON DELETE CASCADE,
	FOREIGN KEY (id, centrifuge_model) REFERENCES asset_models(id, parent_model)
);
CREATE TABLE IF NOT EXISTS commercial_freezer_models (
	id INTEGER PRIMARY KEY,
	freezer_model INTEGER NOT NULL REFERENCES freezer_models(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES freezer_models(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES commercial_products(id) ON DELETE CASCADE,
	FOREIGN KEY (id, freezer_model) REFERENCES asset_models(id, parent_model)
);
CREATE TABLE IF NOT EXISTS commercial_freeze_dryer_models (
	id INTEGER PRIMARY KEY,
	freeze_dryer_model INTEGER NOT NULL REFERENCES freeze_dryer_models(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES freeze_dryer_models(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES commercial_products(id) ON DELETE CASCADE,
	FOREIGN KEY (id, freeze_dryer_model) REFERENCES asset_models(id, parent_model)
);
CREATE TABLE IF NOT EXISTS commercial_positioning_device_models (
	id INTEGER PRIMARY KEY,
	positioning_device_model INTEGER NOT NULL REFERENCES positioning_device_models(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES positioning_device_models(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES commercial_products(id) ON DELETE CASCADE,
	FOREIGN KEY (id, positioning_device_model) REFERENCES asset_models(id, parent_model)
);
CREATE TABLE IF NOT EXISTS commercial_camera_models (
	id INTEGER PRIMARY KEY,
	camera_model INTEGER NOT NULL REFERENCES camera_models(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES camera_models(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES commercial_products(id) ON DELETE CASCADE,
	FOREIGN KEY (id, camera_model) REFERENCES asset_models(id, parent_model)
);