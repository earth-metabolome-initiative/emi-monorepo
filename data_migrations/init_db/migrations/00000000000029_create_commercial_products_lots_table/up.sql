CREATE TABLE IF NOT EXISTS commercial_product_lots (
	id INTEGER PRIMARY KEY REFERENCES physical_asset_models(id) ON DELETE CASCADE,
	lot TEXT NOT NULL,
	product_model_id INTEGER NOT NULL REFERENCES commercial_products(id) ON DELETE CASCADE,
	-- A lot must be unique within the scope of a product model.
	UNIQUE (lot, product_model_id),
	-- The parent_id product model must be a commercial product.
	FOREIGN KEY (id, product_model_id) REFERENCES asset_models(id, parent_model_id)
);
CREATE TABLE IF NOT EXISTS commercial_weighing_device_lots (
	id INTEGER PRIMARY KEY,
	FOREIGN KEY (id) REFERENCES commercial_product_lots(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES weighing_device_models(id) ON DELETE CASCADE,
	commercial_weighing_device_model_id INTEGER NOT NULL REFERENCES commercial_weighing_device_models(id),
	FOREIGN KEY (id, commercial_weighing_device_model_id) REFERENCES asset_models(id, parent_model_id)
);
CREATE TABLE IF NOT EXISTS commercial_volume_measuring_device_lots (
	id INTEGER PRIMARY KEY,
	FOREIGN KEY (id) REFERENCES commercial_product_lots(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES volume_measuring_device_models(id) ON DELETE CASCADE,
	commercial_volume_measuring_device_model_id INTEGER NOT NULL REFERENCES commercial_volume_measuring_device_models(id),
	FOREIGN KEY (id, commercial_volume_measuring_device_model_id) REFERENCES asset_models(id, parent_model_id)
);
CREATE TABLE IF NOT EXISTS commercial_pipette_lots (
	id INTEGER PRIMARY KEY,
	FOREIGN KEY (id) REFERENCES commercial_product_lots(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES pipette_models(id) ON DELETE CASCADE,
	commercial_pipette_model_id INTEGER NOT NULL REFERENCES commercial_pipette_models(id),
	FOREIGN KEY (id, commercial_pipette_model_id) REFERENCES asset_models(id, parent_model_id)
);
CREATE TABLE IF NOT EXISTS commercial_pipette_tip_lots (
	id INTEGER PRIMARY KEY,
	FOREIGN KEY (id) REFERENCES commercial_product_lots(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES pipette_tip_models(id) ON DELETE CASCADE,
	commercial_pipette_tip_model_id INTEGER NOT NULL REFERENCES commercial_pipette_tip_models(id),
	FOREIGN KEY (id, commercial_pipette_tip_model_id) REFERENCES asset_models(id, parent_model_id)
);
CREATE TABLE IF NOT EXISTS commercial_packaging_lots (
	id INTEGER PRIMARY KEY,
	FOREIGN KEY (id) REFERENCES commercial_product_lots(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES packaging_models(id) ON DELETE CASCADE,
	commercial_packaging_model_id INTEGER NOT NULL REFERENCES commercial_packaging_models(id),
	FOREIGN KEY (id, commercial_packaging_model_id) REFERENCES asset_models(id, parent_model_id)
);
CREATE TABLE IF NOT EXISTS commercial_bead_lots (
	id INTEGER PRIMARY KEY,
	FOREIGN KEY (id) REFERENCES commercial_product_lots(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES bead_models(id) ON DELETE CASCADE,
	commercial_bead_model_id INTEGER NOT NULL REFERENCES commercial_bead_models(id),
	FOREIGN KEY (id, commercial_bead_model_id) REFERENCES asset_models(id, parent_model_id)
);
CREATE TABLE IF NOT EXISTS commercial_cap_lots (
	id INTEGER PRIMARY KEY,
	FOREIGN KEY (id) REFERENCES commercial_product_lots(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES cap_models(id) ON DELETE CASCADE,
	commercial_cap_model_id INTEGER NOT NULL REFERENCES commercial_cap_models(id),
	FOREIGN KEY (id, commercial_cap_model_id) REFERENCES asset_models(id, parent_model_id)
);
CREATE TABLE IF NOT EXISTS commercial_ball_mill_machine_lots (
	id INTEGER PRIMARY KEY,
	FOREIGN KEY (id) REFERENCES commercial_product_lots(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES ball_mill_machine_models(id) ON DELETE CASCADE,
	commercial_ball_mill_machine_model_id INTEGER NOT NULL REFERENCES commercial_ball_mill_machine_models(id),
	FOREIGN KEY (id, commercial_ball_mill_machine_model_id) REFERENCES asset_models(id, parent_model_id)
);
CREATE TABLE IF NOT EXISTS commercial_centrifuge_lots (
	id INTEGER PRIMARY KEY,
	FOREIGN KEY (id) REFERENCES commercial_product_lots(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES centrifuge_models(id) ON DELETE CASCADE,
	commercial_centrifuge_model_id INTEGER NOT NULL REFERENCES commercial_centrifuge_models(id),
	FOREIGN KEY (id, commercial_centrifuge_model_id) REFERENCES asset_models(id, parent_model_id)
);
CREATE TABLE IF NOT EXISTS commercial_freezer_lots (
	id INTEGER PRIMARY KEY,
	FOREIGN KEY (id) REFERENCES commercial_product_lots(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES freezer_models(id) ON DELETE CASCADE,
	commercial_freezer_model_id INTEGER NOT NULL REFERENCES commercial_freezer_models(id),
	FOREIGN KEY (id, commercial_freezer_model_id) REFERENCES asset_models(id, parent_model_id)
);
CREATE TABLE IF NOT EXISTS commercial_freeze_dryer_lots (
	id INTEGER PRIMARY KEY,
	FOREIGN KEY (id) REFERENCES commercial_product_lots(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES freeze_dryer_models(id) ON DELETE CASCADE,
	commercial_freeze_dryer_model_id INTEGER NOT NULL REFERENCES commercial_freeze_dryer_models(id),
	FOREIGN KEY (id, commercial_freeze_dryer_model_id) REFERENCES asset_models(id, parent_model_id)
);
CREATE TABLE IF NOT EXISTS commercial_positioning_device_lots (
	id INTEGER PRIMARY KEY,
	FOREIGN KEY (id) REFERENCES commercial_product_lots(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES positioning_device_models(id) ON DELETE CASCADE,
	commercial_positioning_device_model_id INTEGER NOT NULL REFERENCES commercial_positioning_device_models(id),
	FOREIGN KEY (id, commercial_positioning_device_model_id) REFERENCES asset_models(id, parent_model_id)
);
CREATE TABLE IF NOT EXISTS commercial_camera_lots (
	id INTEGER PRIMARY KEY,
	FOREIGN KEY (id) REFERENCES commercial_product_lots(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES camera_models(id) ON DELETE CASCADE,
	commercial_camera_model_id INTEGER NOT NULL REFERENCES commercial_camera_models(id),
	FOREIGN KEY (id, commercial_camera_model_id) REFERENCES asset_models(id, parent_model_id)
);