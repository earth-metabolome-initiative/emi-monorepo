CREATE TABLE IF NOT EXISTS freeze_drying_procedure_templates (
	procedure_template INTEGER PRIMARY KEY REFERENCES procedure_templates(procedure_template) ON DELETE CASCADE,
	-- The storage temperature in Kelvin.
	kelvin REAL NOT NULL DEFAULT 203.15 CHECK (must_be_strictly_positive_f32(kelvin)),
	-- Tolerance percentage for the storage temperature.
	kelvin_tolerance_percentage REAL NOT NULL DEFAULT 5.0 CHECK (
		must_be_strictly_positive_f32(kelvin_tolerance_percentage)
		AND must_be_smaller_than_f32(kelvin_tolerance_percentage, 100.0)
	),
	-- We use a default of 4 Pa for the pressure in the freeze-drying chamber.
	pascal REAL NOT NULL DEFAULT 4.0 CHECK (
		must_be_strictly_positive_f32(pascal)
		AND must_be_smaller_than_f32(pascal, 500.0)
	),
	-- We use a default of 3 days (259200 seconds) for the freeze-drying procedure.
	seconds REAL NOT NULL DEFAULT 259200.0 CHECK (
		must_be_strictly_greater_than_f32(seconds, 7200.0)
		AND must_be_strictly_smaller_than_f32(seconds, 604800.0)
	),
	-- The device used for the freeze drying procedure.
	freeze_dried_with_model INTEGER NOT NULL REFERENCES freeze_dryer_models(id),
	procedure_template_freeze_dried_with_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id) ON DELETE CASCADE,
	-- The container that is being freeze_dried.
	freeze_dried_container_model INTEGER NOT NULL REFERENCES volumetric_container_models(id),
	procedure_template_freeze_dried_container_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- We check that the `freeze_dried_with_model` is indeed a container that is compatible with the procedure template.
	FOREIGN KEY (
		procedure_template_freeze_dried_with_model,
		freeze_dried_with_model
	) REFERENCES procedure_template_asset_models(id, asset_model),
	-- We check that the `freeze_dried_container_model` is indeed a container that is compatible with the procedure template.
	FOREIGN KEY (
		procedure_template_freeze_dried_container_model,
		freeze_dried_container_model
	) REFERENCES procedure_template_asset_models(id, asset_model),
	-- We check that the `freeze_dried_container_model` is indeed a freeze drier that can hold the `freeze_dried_with_model`.
	FOREIGN KEY (
		freeze_dried_with_model,
		freeze_dried_container_model
	) REFERENCES asset_compatibility_rules(left_asset_model, right_asset_model),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_freeze_dried_with_model`
	-- for the current `procedure_template`.
	UNIQUE (
		procedure_template,
		procedure_template_freeze_dried_with_model
	),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_freeze_dried_container_model`
	-- for the current `procedure_template`.
	UNIQUE (
		procedure_template,
		procedure_template_freeze_dried_container_model
	)
);
CREATE TABLE IF NOT EXISTS freeze_drying_procedures (
	-- Identifier of the freeze drying procedure, which is also a foreign key to the general procedure.
	procedure UUID PRIMARY KEY REFERENCES procedures(procedure) ON DELETE CASCADE,
	-- The template of this procedure should be a freeze drying procedure template.
	procedure_template INTEGER NOT NULL REFERENCES freeze_drying_procedure_templates(procedure_template),
	-- The container that is being freeze dried, which must be a volumetric container.
	freeze_dried_container UUID NOT NULL REFERENCES volumetric_containers(id),
	-- The container model that is being freeze dried, which must be a volumetric container model.
	freeze_dried_container_model INTEGER NOT NULL REFERENCES volumetric_container_models(id),
	-- The procedure template asset model associated to the `freeze_dried_container`.
	procedure_template_freeze_dried_container_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The procedure asset associated to the `freeze_dried_container`.
	procedure_freeze_dried_container UUID NOT NULL REFERENCES procedure_assets(id) ON DELETE CASCADE,
	-- The freeze drier used for the freeze drying procedure. This field is optional, as the freeze drier might not necessarily be tracked.
	freeze_dried_with UUID REFERENCES freeze_dryers(id),
	-- The model of the freeze drier used, which must be a freeze drier model.
	freeze_dried_with_model INTEGER NOT NULL REFERENCES freeze_dryer_models(id),
	-- The procedure template asset model associated to the `freeze_dried_with`.
	procedure_template_freeze_dried_with_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The procedure asset associated to the `freeze_dried_with`.
	procedure_freeze_dried_with UUID NOT NULL REFERENCES procedure_assets(id) ON DELETE CASCADE,
	-- We enforce that the current `freeze_drying_procedures` has indeed the same `freeze_drying_procedures_template`.
	FOREIGN KEY (procedure, procedure_template) REFERENCES procedures(procedure, procedure_template),
	-- We enforce that the `freeze_dried_with` is indeed a weighing device of the correct model.
	FOREIGN KEY (freeze_dried_with, freeze_dried_with_model) REFERENCES assets(id, model),
	-- We enforce that the `procedure_template_freeze_dried_with_model` is the same as the one in the procedure template.
	FOREIGN KEY (
		procedure_template,
		procedure_template_freeze_dried_with_model
	) REFERENCES freeze_drying_procedure_templates(
		procedure_template,
		procedure_template_freeze_dried_with_model
	),
	-- We enforce that the `procedure_template_freeze_dried_container_model` is the same as the one in the procedure template.
	FOREIGN KEY (
		procedure_template,
		procedure_template_freeze_dried_container_model
	) REFERENCES freeze_drying_procedure_templates(
		procedure_template,
		procedure_template_freeze_dried_container_model
	),
	-- We enforce that the `procedure_freeze_dried_container` is associated with `procedure_template_freeze_dried_container_model`.
	FOREIGN KEY (
		procedure_freeze_dried_container,
		procedure_template_freeze_dried_container_model
	) REFERENCES procedure_assets(id, procedure_template_asset_model),
	-- We enforce that the `procedure_freeze_dried_with` is associated with `procedure_template_freeze_dried_with_model`.
	FOREIGN KEY (
		procedure_freeze_dried_with,
		procedure_template_freeze_dried_with_model
	) REFERENCES procedure_assets(id, procedure_template_asset_model),
	-- The compatibility rules between the freeze drier and the container being freeze dried must be respected.
	FOREIGN KEY (
		freeze_dried_with_model,
		freeze_dried_container_model
	) REFERENCES asset_compatibility_rules(left_asset_model, right_asset_model),
	-- We enforce that the `procedure_freeze_dried_container` is associated with the `freeze_dried_container_model`.
	FOREIGN KEY (
		procedure_freeze_dried_container,
		freeze_dried_container_model
	) REFERENCES procedure_assets(id, asset_model),
	-- We enforce that the `procedure_freeze_dried_with` is associated with the `freeze_dried_with_model`.
	FOREIGN KEY (
		procedure_freeze_dried_with,
		freeze_dried_with_model
	) REFERENCES procedure_assets(id, asset_model),
	-- We enforce that the `procedure_freeze_dried_container` is associated with the `freeze_dried_container`.
	FOREIGN KEY (
		procedure_freeze_dried_container,
		freeze_dried_container
	) REFERENCES procedure_assets(id, asset),
	-- We enforce that the `procedure_freeze_dried_with` is associated with the `freeze_dried_with`.
	FOREIGN KEY (procedure_freeze_dried_with, freeze_dried_with) REFERENCES procedure_assets(id, asset)
);