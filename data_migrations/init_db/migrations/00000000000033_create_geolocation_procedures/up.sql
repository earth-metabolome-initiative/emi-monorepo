CREATE TABLE IF NOT EXISTS geolocation_procedure_templates (
	id INTEGER PRIMARY KEY REFERENCES procedure_templates(id) ON DELETE CASCADE,
	-- The device used for geolocation.
	geolocated_with_model_id INTEGER NOT NULL REFERENCES positioning_device_models(id),
	procedure_template_geolocated_with_model_id INTEGER NOT NULL REFERENCES procedure_template_asset_models(id) ON DELETE CASCADE,
	geolocated_asset_model_id INTEGER NOT NULL REFERENCES physical_asset_models(id),
	procedure_template_geolocated_asset_model_id INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- We check that the `geolocated_with_model` is indeed an asset model that is compatible with the procedure_id template.
	FOREIGN KEY (
		procedure_template_geolocated_with_model_id,
		geolocated_with_model_id
	) REFERENCES procedure_template_asset_models(id, asset_model_id),
	-- We check that the asset model is indeed a procedure_id asset model.
	FOREIGN KEY (
		procedure_template_geolocated_asset_model_id,
		geolocated_asset_model_id
	) REFERENCES procedure_template_asset_models(id, asset_model_id),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_geolocated_with_model`
	-- for the current `procedure_template`.
	UNIQUE (
		id,
		procedure_template_geolocated_with_model_id
	),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_geolocated_asset_model`
	-- for the current `procedure_template`.
	UNIQUE (
		id,
		procedure_template_geolocated_asset_model_id
	)
);
CREATE TABLE IF NOT EXISTS geolocation_procedures (
	-- Identifier of the geolocation id, which is also a foreign key to the general procedure.
	id UUID PRIMARY KEY REFERENCES procedures(id) ON DELETE CASCADE,
	-- The template of this procedure_id should be a geolocation procedure_id template.
	geolocation_procedure_template_id INTEGER NOT NULL REFERENCES geolocation_procedure_templates(id),
	-- The asset being geolocated, which must be a physical asset.
	geolocated_asset_id UUID NOT NULL REFERENCES physical_assets(id),
	-- The procedure_id template asset model associated to the `geolocated_asset`.
	procedure_template_geolocated_asset_model_id INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The procedure_id asset associated to the `geolocated_asset`.
	procedure_geolocated_asset_id UUID NOT NULL REFERENCES procedure_assets(id) ON DELETE CASCADE,
	-- The positioning device used for geolocation. This field is optional, as the positioning device might not necessarily be tracked.
	geolocated_with_id UUID REFERENCES positioning_devices(id),
	-- The procedure_id asset associated to the `geolocated_with`.
	procedure_geolocated_with_id UUID NOT NULL REFERENCES procedure_assets(id) ON DELETE CASCADE,
	-- The procedure_id template asset model associated to the `geolocated_with_model`.
	procedure_template_geolocated_with_model_id INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The latitude and longitude of the geolocation.
	location GEOGRAPHY(POINT, 4326) NOT NULL,
	-- We enforce that the current `geolocation` has indeed the same `geolocation_template`.
	FOREIGN KEY (id, geolocation_procedure_template_id) REFERENCES procedures(id, procedure_template_id),
	-- The `procedure_template_geolocated_with_model` must be the same as in the `geolocation_procedure_templates`.
	FOREIGN KEY (
		geolocation_procedure_template_id,
		procedure_template_geolocated_with_model_id
	) REFERENCES geolocation_procedure_templates(
		id,
		procedure_template_geolocated_with_model_id
	),
	-- The `procedure_template_geolocated_asset_model` must be the same as in the `geolocation_procedure_templates`.
	FOREIGN KEY (
		geolocation_procedure_template_id,
		procedure_template_geolocated_asset_model_id
	) REFERENCES geolocation_procedure_templates(
		id,
		procedure_template_geolocated_asset_model_id
	),
	-- We check that the `procedure_geolocated_asset` has the same `procedure_template_geolocated_asset_model`.
	FOREIGN KEY (
		procedure_geolocated_asset_id,
		procedure_template_geolocated_asset_model_id
	) REFERENCES procedure_assets(id, procedure_template_asset_model_id),
	-- We check that the `procedure_geolocated_with` has the same `procedure_template_geolocated_with_model`.
	FOREIGN KEY (
		procedure_geolocated_with_id,
		procedure_template_geolocated_with_model_id
	) REFERENCES procedure_assets(id, procedure_template_asset_model_id),
	-- We check that the `procedure_geolocated_asset` is associated to the `geolocated_asset`.
	FOREIGN KEY (procedure_geolocated_asset_id, geolocated_asset_id) REFERENCES procedure_assets(id, asset_id),
	-- We check that the `procedure_geolocated_with` is associated to the `geolocated_with`.
	FOREIGN KEY (procedure_geolocated_with_id, geolocated_with_id) REFERENCES procedure_assets(id, asset_id)
);