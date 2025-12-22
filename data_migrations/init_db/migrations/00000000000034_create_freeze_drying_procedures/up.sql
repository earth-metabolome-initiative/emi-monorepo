CREATE TABLE IF NOT EXISTS freeze_drying_procedure_templates (
	id INTEGER PRIMARY KEY REFERENCES procedure_templates(id) ON DELETE CASCADE,
	-- The storage temperature in Kelvin.
	kelvin REAL NOT NULL DEFAULT 203.15 CHECK (kelvin > 0.0),
	-- Tolerance percentage for the storage temperature.
	kelvin_tolerance_percentage REAL NOT NULL DEFAULT 5.0 CHECK (
		kelvin_tolerance_percentage > 0.0
		AND kelvin_tolerance_percentage <= 100.0
	),
	-- We use a default of 4 Pa for the pressure in the freeze-drying chamber.
	pascal REAL NOT NULL DEFAULT 4.0 CHECK (
		pascal >= 0.0
		AND pascal <= 500.0
	),
	-- Duration in seconds. We use a default of 3 days (259200 seconds) for the freeze-drying procedure.
	duration REAL NOT NULL DEFAULT 259200.0 CHECK (
		duration >= 7200.0
		AND duration <= 604800.0
	),
	-- The device used for the freeze drying procedure.
	freeze_dried_with_model_id INTEGER NOT NULL REFERENCES freeze_dryer_models(id),
	procedure_template_freeze_dried_with_model_id INTEGER NOT NULL REFERENCES procedure_template_asset_models(id) ON DELETE CASCADE,
	-- The container that is being freeze_dried.
	freeze_dried_container_model_id INTEGER NOT NULL REFERENCES volumetric_container_models(id),
	procedure_template_freeze_dried_container_model_id INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- We check that the `freeze_dried_with_model` is indeed a container that is compatible with the procedure_id template.
	FOREIGN KEY (
		procedure_template_freeze_dried_with_model_id,
		freeze_dried_with_model_id
	) REFERENCES procedure_template_asset_models(id, asset_model_id),
	-- We check that the `freeze_dried_container_model` is indeed a container that is compatible with the procedure_id template.
	FOREIGN KEY (
		procedure_template_freeze_dried_container_model_id,
		freeze_dried_container_model_id
	) REFERENCES procedure_template_asset_models(id, asset_model_id),
	-- We check that the `freeze_dried_container_model` is indeed a freeze drier that can hold the `freeze_dried_with_model`.
	FOREIGN KEY (
		freeze_dried_with_model_id,
		freeze_dried_container_model_id
	) REFERENCES asset_compatibility_rules(left_asset_model_id, right_asset_model_id),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_freeze_dried_with_model`
	-- for the current `procedure_template`.
	UNIQUE (
		id,
		procedure_template_freeze_dried_with_model_id
	),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_freeze_dried_container_model`
	-- for the current `procedure_template`.
	UNIQUE (
		id,
		procedure_template_freeze_dried_container_model_id
	)
);
CREATE TABLE IF NOT EXISTS freeze_drying_procedures (
	-- Identifier of the freeze drying id, which is also a foreign key to the general procedure.
	id UUID PRIMARY KEY REFERENCES procedures(id) ON DELETE CASCADE,
	-- The template of this procedure_id should be a freeze drying procedure_id template.
	freeze_drying_procedure_template_id INTEGER NOT NULL REFERENCES freeze_drying_procedure_templates(id),
	-- The container that is being freeze dried, which must be a volumetric container.
	freeze_dried_container_id UUID NOT NULL REFERENCES volumetric_containers(id),
	-- The container model that is being freeze dried, which must be a volumetric container model.
	freeze_dried_container_model_id INTEGER NOT NULL REFERENCES volumetric_container_models(id),
	-- The procedure_id template asset model associated to the `freeze_dried_container`.
	procedure_template_freeze_dried_container_model_id INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The procedure_id asset associated to the `freeze_dried_container`.
	procedure_freeze_dried_container_id UUID NOT NULL REFERENCES procedure_assets(id) ON DELETE CASCADE,
	-- The freeze drier used for the freeze drying procedure. This field is optional, as the freeze drier might not necessarily be tracked.
	freeze_dried_with_id UUID REFERENCES freeze_dryers(id),
	-- The model of the freeze drier used, which must be a freeze drier model.
	freeze_dried_with_model_id INTEGER NOT NULL REFERENCES freeze_dryer_models(id),
	-- The procedure_id template asset model associated to the `freeze_dried_with`.
	procedure_template_freeze_dried_with_model_id INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The procedure_id asset associated to the `freeze_dried_with`.
	procedure_freeze_dried_with_id UUID NOT NULL REFERENCES procedure_assets(id) ON DELETE CASCADE,
	-- We enforce that the current `freeze_drying_procedures` has indeed the same `freeze_drying_procedures_template`.
	FOREIGN KEY (id, freeze_drying_procedure_template_id) REFERENCES procedures(id, procedure_template_id),
	-- We enforce that the `freeze_dried_with` is indeed a weighing device of the correct model.
	FOREIGN KEY (freeze_dried_with_id, freeze_dried_with_model_id) REFERENCES assets(id, model_id),
	-- We enforce that the `procedure_template_freeze_dried_with_model` is the same as the one in the procedure_id template.
	FOREIGN KEY (
		freeze_drying_procedure_template_id,
		procedure_template_freeze_dried_with_model_id
	) REFERENCES freeze_drying_procedure_templates(
		id,
		procedure_template_freeze_dried_with_model_id
	),
	-- We enforce that the `procedure_template_freeze_dried_container_model` is the same as the one in the procedure_id template.
	FOREIGN KEY (
		freeze_drying_procedure_template_id,
		procedure_template_freeze_dried_container_model_id
	) REFERENCES freeze_drying_procedure_templates(
		id,
		procedure_template_freeze_dried_container_model_id
	),
	-- We enforce that the `procedure_freeze_dried_container` is associated with `procedure_template_freeze_dried_container_model`.
	FOREIGN KEY (
		procedure_freeze_dried_container_id,
		procedure_template_freeze_dried_container_model_id
	) REFERENCES procedure_assets(id, procedure_template_asset_model_id),
	-- We enforce that the `procedure_freeze_dried_with` is associated with `procedure_template_freeze_dried_with_model`.
	FOREIGN KEY (
		procedure_freeze_dried_with_id,
		procedure_template_freeze_dried_with_model_id
	) REFERENCES procedure_assets(id, procedure_template_asset_model_id),
	-- The compatibility rules between the freeze drier and the container being freeze dried must be respected.
	FOREIGN KEY (
		freeze_dried_with_model_id,
		freeze_dried_container_model_id
	) REFERENCES asset_compatibility_rules(left_asset_model_id, right_asset_model_id),
	-- We enforce that the `procedure_freeze_dried_container` is associated with the `freeze_dried_container_model`.
	FOREIGN KEY (
		procedure_freeze_dried_container_id,
		freeze_dried_container_model_id
	) REFERENCES procedure_assets(id, asset_model_id),
	-- We enforce that the `procedure_freeze_dried_with` is associated with the `freeze_dried_with_model`.
	FOREIGN KEY (
		procedure_freeze_dried_with_id,
		freeze_dried_with_model_id
	) REFERENCES procedure_assets(id, asset_model_id),
	-- We enforce that the `procedure_freeze_dried_container` is associated with the `freeze_dried_container`.
	FOREIGN KEY (
		procedure_freeze_dried_container_id,
		freeze_dried_container_id
	) REFERENCES procedure_assets(id, asset_id),
	-- We enforce that the `procedure_freeze_dried_with` is associated with the `freeze_dried_with`.
	FOREIGN KEY (procedure_freeze_dried_with_id, freeze_dried_with_id) REFERENCES procedure_assets(id, asset_id)
);