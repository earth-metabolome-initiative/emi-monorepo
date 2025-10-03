CREATE TABLE IF NOT EXISTS personal_protective_equipment_models (
	id INTEGER PRIMARY KEY REFERENCES physical_asset_models (id) ON DELETE CASCADE
);