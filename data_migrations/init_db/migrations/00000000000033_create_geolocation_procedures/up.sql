CREATE TABLE IF NOT EXISTS geolocation_procedure_templates (
	procedure_template INTEGER PRIMARY KEY REFERENCES procedure_templates(procedure_template) ON DELETE CASCADE,
	-- The device used for geolocation.
	geolocated_with_model INTEGER NOT NULL REFERENCES positioning_device_models(id),
	procedure_template_geolocated_with_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id) ON DELETE CASCADE,
	geolocated_asset_model INTEGER NOT NULL REFERENCES physical_asset_models(id),
	procedure_template_geolocated_asset_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- We check that the `geolocated_with_model` is indeed an asset model that is compatible with the procedure template.
	FOREIGN KEY (
		procedure_template_geolocated_with_model,
		geolocated_with_model
	) REFERENCES procedure_template_asset_models(id, asset_model),
	-- We check that the asset model is indeed a procedure asset model.
	FOREIGN KEY (
		procedure_template_geolocated_asset_model,
		geolocated_asset_model
	) REFERENCES procedure_template_asset_models(id, asset_model),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_geolocated_with_model`
	-- for the current `procedure_template`.
	UNIQUE (
		procedure_template,
		procedure_template_geolocated_with_model
	),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_geolocated_asset_model`
	-- for the current `procedure_template`.
	UNIQUE (
		procedure_template,
		procedure_template_geolocated_asset_model
	)
);
CREATE TABLE IF NOT EXISTS geolocation_procedures (
	-- Identifier of the geolocation procedure, which is also a foreign key to the general procedure.
	procedure UUID PRIMARY KEY REFERENCES procedures(procedure) ON DELETE CASCADE,
	-- The template of this procedure should be a geolocation procedure template.
	procedure_template INTEGER NOT NULL REFERENCES geolocation_procedure_templates(procedure_template),
	-- The asset being geolocated, which must be a physical asset.
	geolocated_asset UUID REFERENCES physical_assets(id),
	-- The procedure template asset model associated to the `geolocated_asset`.
	procedure_template_geolocated_asset_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The procedure asset associated to the `geolocated_asset`.
	procedure_geolocated_asset UUID NOT NULL REFERENCES procedure_assets(id) ON DELETE CASCADE,
	-- The positioning device used for geolocation. This field is optional, as the positioning device might not necessarily be tracked.
	geolocated_with UUID REFERENCES positioning_devices(id),
	-- The procedure asset associated to the `geolocated_with`.
	procedure_geolocated_with UUID NOT NULL REFERENCES procedure_assets(id) ON DELETE CASCADE,
	-- The procedure template asset model associated to the `geolocated_with_model`.
	procedure_template_geolocated_with_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The latitude and longitude of the geolocation.
	location GEOGRAPHY(POINT, 4326) NOT NULL,
	-- We enforce that the current `geolocation` has indeed the same `geolocation_template`.
	FOREIGN KEY (procedure, procedure_template) REFERENCES procedures(procedure, procedure_template),
	-- The `procedure_template_geolocated_with_model` must be the same as in the `geolocation_procedure_templates`.
	FOREIGN KEY (
		procedure_template,
		procedure_template_geolocated_with_model
	) REFERENCES geolocation_procedure_templates(
		procedure_template,
		procedure_template_geolocated_with_model
	),
	-- The `procedure_template_geolocated_asset_model` must be the same as in the `geolocation_procedure_templates`.
	FOREIGN KEY (
		procedure_template,
		procedure_template_geolocated_asset_model
	) REFERENCES geolocation_procedure_templates(
		procedure_template,
		procedure_template_geolocated_asset_model
	),
	-- We check that the `procedure_geolocated_asset` has the same `procedure_template_geolocated_asset_model`.
	FOREIGN KEY (
		procedure_geolocated_asset,
		procedure_template_geolocated_asset_model
	) REFERENCES procedure_assets(id, procedure_template_asset_model),
	-- We check that the `procedure_geolocated_with` has the same `procedure_template_geolocated_with_model`.
	FOREIGN KEY (
		procedure_geolocated_with,
		procedure_template_geolocated_with_model
	) REFERENCES procedure_assets(id, procedure_template_asset_model),
	-- We check that the `procedure_geolocated_asset` is associated to the `geolocated_asset`.
	FOREIGN KEY (procedure_geolocated_asset, geolocated_asset) REFERENCES procedure_assets(id, asset),
	-- We check that the `procedure_geolocated_with` is associated to the `geolocated_with`.
	FOREIGN KEY (procedure_geolocated_with, geolocated_with) REFERENCES procedure_assets(id, asset)
);