CREATE TABLE IF NOT EXISTS freezing_procedure_templates (
	procedure_template INTEGER PRIMARY KEY REFERENCES procedure_templates(procedure_template) ON DELETE CASCADE,
	-- The storage temperature in Kelvin.
	kelvin REAL NOT NULL DEFAULT 203.15 CHECK (must_be_strictly_positive_f32(kelvin)),
	-- Tolerance percentage for the storage temperature.
	kelvin_tolerance_percentage REAL NOT NULL DEFAULT 5.0 CHECK (
		must_be_strictly_positive_f32(kelvin_tolerance_percentage)
		AND must_be_smaller_than_f32(kelvin_tolerance_percentage, 100.0)
	),
	-- We use a default of 43200 seconds (12 hours) for the freezing procedure.
	seconds REAL DEFAULT 43200.0 CHECK (
		must_be_strictly_positive_f32(seconds)
		AND must_be_strictly_greater_than_f32(seconds, 1800.0)
	),
	-- The device used for freezing.
	frozen_with_model INTEGER NOT NULL REFERENCES freezer_models(id),
	procedure_template_frozen_with_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id) ON DELETE CASCADE,
	-- The container that is being stored in the freezer.
	frozen_container_model INTEGER NOT NULL REFERENCES container_models(id),
	-- The procedure template which originated the container being frozen (e.g., a sampling or fractioning procedure template).
	foreign_procedure_template INTEGER NOT NULL REFERENCES procedure_templates(procedure_template) CHECK (
		must_be_distinct_i32(
			procedure_template,
			foreign_procedure_template
		)
	),
	procedure_template_frozen_container_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- We check that the `procedure_template_frozen_with_model` is indeed a trackable that is compatible with the procedure template.
	FOREIGN KEY (
		procedure_template_frozen_with_model,
		procedure_template
	) REFERENCES procedure_template_asset_models(id, procedure_template),
	-- We check that the `procedure_template_frozen_container_model` is indeed a trackable that is compatible with the procedure template.
	FOREIGN KEY (
		procedure_template_frozen_container_model,
		foreign_procedure_template
	) REFERENCES procedure_template_asset_models(id, procedure_template),
	-- We check that the `frozen_with_model` is indeed a container that is compatible with the procedure template.
	FOREIGN KEY (
		procedure_template_frozen_with_model,
		frozen_with_model
	) REFERENCES procedure_template_asset_models(id, asset_model),
	-- We check that the `frozen_container_model` is indeed a container that is compatible with the procedure template.
	FOREIGN KEY (
		procedure_template_frozen_container_model,
		frozen_container_model
	) REFERENCES procedure_template_asset_models(id, asset_model),
	-- We check that the `frozen_with_model` is indeed a container that can hold the `frozen_container_model`.
	CONSTRAINT freezing_pm_compatibility_rules FOREIGN KEY (frozen_with_model, frozen_container_model) REFERENCES asset_compatibility_rules(left_asset_model, right_asset_model),
	-- We define a same-as index to allow for foreign key references to check whether a `freezing_procedure_template`
	-- is associated with a given `freezing_foreign_procedure_template`.
	UNIQUE (procedure_template, foreign_procedure_template)
);
CREATE TABLE IF NOT EXISTS freezing_procedures (
	-- Identifier of the freezing procedure, which is also a foreign key to the general procedure.
	procedure UUID PRIMARY KEY REFERENCES procedures(procedure) ON DELETE CASCADE,
	-- The template of this procedure should be a freezing procedure template.
	procedure_template INTEGER NOT NULL REFERENCES freezing_procedure_templates(procedure_template),
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
	-- The container that is being frozen, which must be a volumetric container.
	frozen_container UUID NOT NULL REFERENCES volumetric_containers(id),
	-- The freezer used for the freezing procedure. This field is optional, as the freezer might not necessarily be tracked.
	frozen_with UUID REFERENCES freezers(id),
	-- The model of the freezer used, which must be a freezer model.
	frozen_with_model INTEGER NOT NULL REFERENCES freezer_models(id),
	-- We enforce that the current `freezing_procedure_templates` has indeed the same `freezing_procedure_templates_template`.
	FOREIGN KEY (procedure, procedure_template) REFERENCES procedures(procedure, procedure_template),
	-- We enforce that the `foreign_procedure` has as `procedure_template` the specified `foreign_procedure_template`.
	FOREIGN KEY (foreign_procedure, foreign_procedure_template) REFERENCES procedures(procedure, procedure_template),
	-- Additionally, we enforce that the `frozen_container` is indeed a procedure asset of the correct model.
	FOREIGN KEY (foreign_procedure, frozen_container) REFERENCES procedure_assets(procedure, asset),
	-- We enforce that the `frozen_with_model` is indeed a procedure asset model.
	FOREIGN KEY (procedure, frozen_with_model) REFERENCES procedure_assets(procedure, asset_model),
	-- And that the `frozen_with` is indeed a procedure asset of the correct model.
	FOREIGN KEY (procedure, frozen_with) REFERENCES procedure_assets(procedure, asset),
	-- We enforce that the `frozen_with` is indeed a weighing device of the correct model.
	FOREIGN KEY (frozen_with, frozen_with_model) REFERENCES assets(id, model),
	-- We enforce that the associated procedure template requires the provided foreign procedure template.
	FOREIGN KEY (procedure_template, foreign_procedure_template) REFERENCES freezing_procedure_templates(procedure_template, foreign_procedure_template)
);