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
	frozen_container_model INTEGER NOT NULL REFERENCES volumetric_container_models(id),
	procedure_template_frozen_container_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
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
	FOREIGN KEY (frozen_with_model, frozen_container_model) REFERENCES asset_compatibility_rules(left_asset_model, right_asset_model),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_frozen_with_model`
	-- for the current `procedure_template`.
	UNIQUE (
		procedure_template,
		procedure_template_frozen_with_model
	),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_frozen_container_model`
	-- for the current `procedure_template`.
	UNIQUE (
		procedure_template,
		procedure_template_frozen_container_model
	)
);
CREATE TABLE IF NOT EXISTS freezing_procedures (
	-- Identifier of the freezing procedure, which is also a foreign key to the general procedure.
	procedure UUID PRIMARY KEY REFERENCES procedures(procedure) ON DELETE CASCADE,
	-- The template of this procedure should be a freezing procedure template.
	procedure_template INTEGER NOT NULL REFERENCES freezing_procedure_templates(procedure_template),
	-- The container that is being frozen, which must be a volumetric container.
	frozen_container UUID NOT NULL REFERENCES volumetric_containers(id),
	-- The model of the container being frozen, which must be a container model.
	frozen_container_model INTEGER NOT NULL REFERENCES volumetric_container_models(id),
	-- The procedure template asset model associated to the `frozen_container`.
	procedure_template_frozen_container_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The procedure asset associated to the `frozen_container`.
	procedure_frozen_container UUID NOT NULL REFERENCES procedure_assets(id) ON DELETE CASCADE,
	-- The freezer used for the freezing procedure. This field is optional, as the freezer might not necessarily be tracked.
	frozen_with UUID REFERENCES freezers(id),
	-- The model of the freezer used, which must be a freezer model.
	frozen_with_model INTEGER NOT NULL REFERENCES freezer_models(id),
	-- The procedure template asset model associated to the `frozen_with_model`.
	procedure_template_frozen_with_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The procedure asset associated to the `frozen_with`.
	procedure_frozen_with UUID NOT NULL REFERENCES procedure_assets(id) ON DELETE CASCADE,
	-- We enforce that the current `freezing_procedure_templates` has indeed the same `freezing_procedure_templates_template`.
	FOREIGN KEY (procedure, procedure_template) REFERENCES procedures(procedure, procedure_template),
	-- The `procedure_template_frozen_with_model` must be the same as in the `freezing_procedure_templates`.
	FOREIGN KEY (
		procedure_template,
		procedure_template_frozen_with_model
	) REFERENCES freezing_procedure_templates(
		procedure_template,
		procedure_template_frozen_with_model
	),
	-- The `procedure_template_frozen_container_model` must be the same as in the `freezing_procedure_templates`.
	FOREIGN KEY (
		procedure_template,
		procedure_template_frozen_container_model
	) REFERENCES freezing_procedure_templates(
		procedure_template,
		procedure_template_frozen_container_model
	),
	-- We check that the `frozen_with_model` is compatible with the `frozen_container_model`.
	FOREIGN KEY (frozen_with_model, frozen_container_model) REFERENCES asset_compatibility_rules(left_asset_model, right_asset_model),
	-- We ensure that the `procedure_frozen_container` is associated with the `procedure_template_frozen_container_model`.
	FOREIGN KEY (
		procedure_frozen_container,
		procedure_template_frozen_container_model
	) REFERENCES procedure_assets(id, procedure_template_asset_model),
	-- We ensure that the `procedure_frozen_with` is associated with the `procedure_template_frozen_with_model`.
	FOREIGN KEY (
		procedure_frozen_with,
		procedure_template_frozen_with_model
	) REFERENCES procedure_assets(id, procedure_template_asset_model),
	-- We ensure that the `procedure_frozen_container` is associated to the `frozen_container_model`.
	FOREIGN KEY (
		procedure_frozen_container,
		frozen_container_model
	) REFERENCES procedure_assets(id, asset_model),
	-- We ensure that the `procedure_frozen_with` is associated to the `frozen_with_model`.
	FOREIGN KEY (procedure_frozen_with, frozen_with_model) REFERENCES procedure_assets(id, asset_model),
	-- We ensure that the `procedure_frozen_container` is associated to the `frozen_container`.
	FOREIGN KEY (procedure_frozen_container, frozen_container) REFERENCES procedure_assets(id, asset),
	-- We ensure that the `procedure_frozen_with` is associated to the `frozen_with`.
	FOREIGN KEY (procedure_frozen_with, frozen_with) REFERENCES procedure_assets(id, asset)
);