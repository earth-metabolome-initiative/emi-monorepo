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
	-- The procedure template which originated the container being freeze dryed (e.g., a sampling or fractioning procedure template).
	foreign_procedure_template INTEGER NOT NULL REFERENCES procedure_templates(procedure_template) CHECK (
		must_be_distinct_i32(
			procedure_template,
			foreign_procedure_template
		)
	),
	procedure_template_freeze_dried_container_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- We check that the `procedure_template_freeze_dried_with_model` is indeed a trackable that is compatible with the procedure template.
	FOREIGN KEY (
		procedure_template_freeze_dried_with_model,
		procedure_template
	) REFERENCES procedure_template_asset_models(id, procedure_template),
	-- We check that the `procedure_template_freeze_dried_with_model` is indeed a trackable that is compatible with the procedure template.
	FOREIGN KEY (
		procedure_template_freeze_dried_container_model,
		foreign_procedure_template
	) REFERENCES procedure_template_asset_models(id, procedure_template),
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
	CONSTRAINT freeze_drying_pm_compatibility_rules FOREIGN KEY (
		freeze_dried_with_model,
		freeze_dried_container_model
	) REFERENCES asset_compatibility_rules(left_asset_model, right_asset_model),
	-- We define a same-as index to allow for foreign key references to check whether a `freeze_drying_procedure_template`
	-- is associated with a given `freeze_drying_foreign_procedure_template`.
	UNIQUE (procedure_template, foreign_procedure_template)
);
CREATE TABLE IF NOT EXISTS freeze_drying_procedures (
	-- Identifier of the freeze drying procedure, which is also a foreign key to the general procedure.
	procedure UUID PRIMARY KEY REFERENCES procedures(procedure) ON DELETE CASCADE,
	-- The template of this procedure should be a freeze drying procedure template.
	procedure_template INTEGER NOT NULL REFERENCES freeze_drying_procedure_templates(procedure_template),
	-- The procedure template associated with the foreign procedure template.
	foreign_procedure_template INTEGER NOT NULL REFERENCES procedure_templates(procedure_template) CHECK (
		must_be_distinct_i32(
			procedure_template,
			foreign_procedure_template
		)
	),
	-- The procedure that has populated the source container (e.g., a sampling procedure).
	foreign_procedure UUID NOT NULL REFERENCES procedures(procedure) CHECK (
		must_be_distinct_uuid(procedure, foreign_procedure)
	),
	-- The container that is being freeze dryed, which must be a volumetric container.
	freeze_dryed_container UUID NOT NULL REFERENCES volumetric_containers(id),
	-- The freeze drier used for the freeze drying procedure. This field is optional, as the freeze drier might not necessarily be tracked.
	freeze_dryed_with UUID REFERENCES freeze_dryers(id),
	-- The model of the freeze drier used, which must be a freeze drier model.
	freeze_dryed_with_model INTEGER NOT NULL REFERENCES freeze_dryer_models(id),
	-- We enforce that the current `freeze_drying_procedures` has indeed the same `freeze_drying_procedures_template`.
	FOREIGN KEY (procedure, procedure_template) REFERENCES procedures(procedure, procedure_template),
	-- We enforce that the `foreign_procedure` has as `procedure_template` the specified `foreign_procedure_template`.
	FOREIGN KEY (foreign_procedure, foreign_procedure_template) REFERENCES procedures(procedure, procedure_template),
	-- Additionally, we enforce that the `freeze_dryed_container` is indeed a procedure asset of the correct model.
	FOREIGN KEY (foreign_procedure, freeze_dryed_container) REFERENCES procedure_assets(procedure, asset),
	-- We enforce that the `freeze_dryed_with_model` is indeed a procedure asset model.
	FOREIGN KEY (procedure, freeze_dryed_with_model) REFERENCES procedure_assets(procedure, asset_model),
	-- And that the `freeze_dryed_with` is indeed a procedure asset of the correct model.
	FOREIGN KEY (procedure, freeze_dryed_with) REFERENCES procedure_assets(procedure, asset),
	-- We enforce that the `freeze_dryed_with` is indeed a weighing device of the correct model.
	FOREIGN KEY (freeze_dryed_with, freeze_dryed_with_model) REFERENCES assets(id, model),
	-- We enforce that the associated procedure template requires the provided foreign procedure template.
	FOREIGN KEY (procedure_template, foreign_procedure_template) REFERENCES freeze_drying_procedure_templates(procedure_template, foreign_procedure_template)
);