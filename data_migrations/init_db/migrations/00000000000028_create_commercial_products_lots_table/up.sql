CREATE TABLE IF NOT EXISTS commercial_product_lots (
	id INTEGER PRIMARY KEY REFERENCES physical_asset_models(id) ON DELETE CASCADE,
	lot TEXT NOT NULL,
	product_model INTEGER NOT NULL REFERENCES commercial_products(id) ON DELETE CASCADE,
	-- A lot must be unique within the scope of a product model.
	UNIQUE (lot, product_model),
	-- The parent product model must be a commercial product.
	FOREIGN KEY (id, product_model) REFERENCES asset_models(id, parent_model)
);
CREATE TABLE IF NOT EXISTS commercial_weighing_device_lots (
	id INTEGER PRIMARY KEY,
	FOREIGN KEY (id) REFERENCES commercial_product_lots(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES weighing_device_models(id) ON DELETE CASCADE,
	product_model INTEGER NOT NULL REFERENCES commercial_weighing_device_models(id),
	FOREIGN KEY (id, product_model) REFERENCES asset_models(id, parent_model)
);
CREATE TABLE IF NOT EXISTS commercial_volume_measuring_device_lots (
	id INTEGER PRIMARY KEY,
	FOREIGN KEY (id) REFERENCES commercial_product_lots(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES volume_measuring_device_models(id) ON DELETE CASCADE,
	product_model INTEGER NOT NULL REFERENCES commercial_volume_measuring_device_models(id),
	FOREIGN KEY (id, product_model) REFERENCES asset_models(id, parent_model)
);
CREATE TABLE IF NOT EXISTS commercial_pipette_lots (
	id INTEGER PRIMARY KEY,
	FOREIGN KEY (id) REFERENCES commercial_product_lots(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES pipette_models(id) ON DELETE CASCADE,
	product_model INTEGER NOT NULL REFERENCES commercial_pipette_models(id),
	FOREIGN KEY (id, product_model) REFERENCES asset_models(id, parent_model)
);
CREATE TABLE IF NOT EXISTS commercial_pipette_tip_lots (
	id INTEGER PRIMARY KEY,
	FOREIGN KEY (id) REFERENCES commercial_product_lots(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES pipette_tip_models(id) ON DELETE CASCADE,
	product_model INTEGER NOT NULL REFERENCES commercial_pipette_tip_models(id),
	FOREIGN KEY (id, product_model) REFERENCES asset_models(id, parent_model)
);
CREATE TABLE IF NOT EXISTS commercial_packaging_lots (
	id INTEGER PRIMARY KEY,
	FOREIGN KEY (id) REFERENCES commercial_product_lots(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES packaging_models(id) ON DELETE CASCADE,
	product_model INTEGER NOT NULL REFERENCES commercial_packaging_models(id),
	FOREIGN KEY (id, product_model) REFERENCES asset_models(id, parent_model)
);
CREATE TABLE IF NOT EXISTS commercial_beads_lots (
	id INTEGER PRIMARY KEY,
	FOREIGN KEY (id) REFERENCES commercial_product_lots(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES beads_models(id) ON DELETE CASCADE,
	product_model INTEGER NOT NULL REFERENCES commercial_beads_models(id),
	FOREIGN KEY (id, product_model) REFERENCES asset_models(id, parent_model)
);
CREATE TABLE IF NOT EXISTS commercial_ball_mill_machine_lots (
	id INTEGER PRIMARY KEY,
	FOREIGN KEY (id) REFERENCES commercial_product_lots(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES ball_mill_machine_models(id) ON DELETE CASCADE,
	product_model INTEGER NOT NULL REFERENCES commercial_ball_mill_machine_models(id),
	FOREIGN KEY (id, product_model) REFERENCES asset_models(id, parent_model)
);
CREATE TABLE IF NOT EXISTS commercial_centrifuge_lots (
	id INTEGER PRIMARY KEY,
	FOREIGN KEY (id) REFERENCES commercial_product_lots(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES centrifuge_models(id) ON DELETE CASCADE,
	product_model INTEGER NOT NULL REFERENCES commercial_centrifuge_models(id),
	FOREIGN KEY (id, product_model) REFERENCES asset_models(id, parent_model)
);
CREATE TABLE IF NOT EXISTS commercial_freezer_lots (
	id INTEGER PRIMARY KEY,
	FOREIGN KEY (id) REFERENCES commercial_product_lots(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES freezer_models(id) ON DELETE CASCADE,
	product_model INTEGER NOT NULL REFERENCES commercial_freezer_models(id),
	FOREIGN KEY (id, product_model) REFERENCES asset_models(id, parent_model)
);
CREATE TABLE IF NOT EXISTS commercial_freeze_dryer_lots (
	id INTEGER PRIMARY KEY,
	FOREIGN KEY (id) REFERENCES commercial_product_lots(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES freeze_dryer_models(id) ON DELETE CASCADE,
	product_model INTEGER NOT NULL REFERENCES commercial_freeze_dryer_models(id),
	FOREIGN KEY (id, product_model) REFERENCES asset_models(id, parent_model)
);
CREATE TABLE IF NOT EXISTS commercial_positioning_device_lots (
	id INTEGER PRIMARY KEY,
	FOREIGN KEY (id) REFERENCES commercial_product_lots(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES positioning_device_models(id) ON DELETE CASCADE,
	product_model INTEGER NOT NULL REFERENCES commercial_positioning_device_models(id),
	FOREIGN KEY (id, product_model) REFERENCES asset_models(id, parent_model)
);
CREATE TABLE IF NOT EXISTS commercial_camera_lots (
	id INTEGER PRIMARY KEY,
	FOREIGN KEY (id) REFERENCES commercial_product_lots(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES camera_models(id) ON DELETE CASCADE,
	product_model INTEGER NOT NULL REFERENCES commercial_camera_models(id),
	FOREIGN KEY (id, product_model) REFERENCES asset_models(id, parent_model)
);