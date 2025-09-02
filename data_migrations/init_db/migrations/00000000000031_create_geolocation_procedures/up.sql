CREATE TABLE IF NOT EXISTS geolocation_procedure_templates (
	procedure_template INTEGER PRIMARY KEY REFERENCES procedure_templates(procedure_template) ON DELETE CASCADE,
	-- The device used for geolocation.
	geolocated_with_model INTEGER NOT NULL REFERENCES positioning_device_models(id),
	procedure_template_geolocated_with_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id) ON DELETE CASCADE,
	geolocated_asset_model INTEGER NOT NULL REFERENCES physical_asset_models(id),
	foreign_procedure_template INTEGER NOT NULL REFERENCES procedure_templates(procedure_template) CHECK (
		must_be_distinct_i32(
			procedure_template,
			foreign_procedure_template
		)
	),
	procedure_template_geolocated_asset_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- We check that the `geolocated_with_model` is indeed an asset model that is compatible with the procedure template.
	FOREIGN KEY (
		procedure_template_geolocated_with_model,
		geolocated_with_model
	) REFERENCES procedure_template_asset_models(id, asset_model),
	-- We check that the `procedure_template_geolocated_with_model` is indeed an asset model that is compatible with the procedure template.
	FOREIGN KEY (
		procedure_template_geolocated_with_model,
		procedure_template
	) REFERENCES procedure_template_asset_models(id, procedure_template),
	-- We check that the asset model is indeed a procedure asset model.
	FOREIGN KEY (
		procedure_template_geolocated_asset_model,
		geolocated_asset_model
	) REFERENCES procedure_template_asset_models(id, asset_model),
	-- We check that the procedure asset model is associated with the foreign procedure template.
	FOREIGN KEY (
		procedure_template_geolocated_asset_model,
		foreign_procedure_template
	) REFERENCES procedure_template_asset_models(id, procedure_template),
	-- We define a same-as index to allow for foreign key references to check whether a `geolocation_procedure_template`
	-- is associated with a given `geolocation_foreign_procedure_template`.
	UNIQUE (procedure_template, foreign_procedure_template)
);
CREATE TABLE IF NOT EXISTS geolocation_procedures (
	-- Identifier of the geolocation procedure, which is also a foreign key to the general procedure.
	procedure UUID PRIMARY KEY REFERENCES procedures(procedure) ON DELETE CASCADE,
	-- The template of this procedure should be a geolocation procedure template.
	procedure_template INTEGER NOT NULL REFERENCES geolocation_procedure_templates(procedure_template),
	-- The procedure template associated with the foreign procedure template.
	foreign_procedure_template INTEGER NOT NULL REFERENCES procedure_templates(procedure_template) CHECK (
		must_be_distinct_i32(
			procedure_template,
			foreign_procedure_template
		)
	),
	-- The foreign procedure that has first defined the asset being geolocated (e.g., a sampling or fractioning procedure).
	foreign_procedure UUID NOT NULL REFERENCES procedures(procedure) CHECK (
		must_be_distinct_uuid(procedure, foreign_procedure)
	),
	-- The asset being geolocated, which must be a physical asset.
	geolocated_asset UUID NOT NULL REFERENCES physical_assets(id),
	-- The positioning device used for geolocation. This field is optional, as the positioning device might not necessarily be tracked.
	geolocated_with UUID REFERENCES positioning_devices(id),
	-- The model of the positioning device used for geolocation, which must be a positioning device model.
	geolocated_with_model INTEGER NOT NULL REFERENCES positioning_device_models(id),
	-- We enforce that the current `geolocation` has indeed the same `geolocation_template`.
	FOREIGN KEY (procedure, procedure_template) REFERENCES procedures(procedure, procedure_template),
	-- We enforce that the `foreign_procedure` has as `procedure_template` the specified `foreign_procedure_template`.
	FOREIGN KEY (foreign_procedure, foreign_procedure_template) REFERENCES procedures(procedure, procedure_template),
	-- Additionally, we enforce that the `geolocated_asset` is indeed a procedure asset of the correct model.
	FOREIGN KEY (foreign_procedure, geolocated_asset) REFERENCES procedure_assets(procedure, asset),
	-- We enforce that the `geolocated_with_model` is indeed a procedure asset model.
	FOREIGN KEY (procedure, geolocated_with_model) REFERENCES procedure_assets(procedure, asset_model),
	-- And that the `geolocated_with` is indeed a procedure asset of the correct model.
	FOREIGN KEY (procedure, geolocated_with) REFERENCES procedure_assets(procedure, asset),
	-- We enforce that the `geolocated_with` is indeed a weighing device of the correct model.
	FOREIGN KEY (geolocated_with, geolocated_with_model) REFERENCES assets(id, model),
	-- We enforce that the associated procedure template requires the provided foreign procedure template.
	FOREIGN KEY (procedure_template, foreign_procedure_template) REFERENCES geolocation_procedure_templates(procedure_template, foreign_procedure_template)
);