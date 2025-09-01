CREATE TABLE IF NOT EXISTS procedure_templates.storage_procedure_templates (
	procedure_template INTEGER PRIMARY KEY REFERENCES procedure_templates.procedure_templates(procedure_template) ON DELETE CASCADE,
	-- The storage temperature in Kelvin.
	kelvin REAL NOT NULL DEFAULT 293.15 CHECK (must_be_strictly_positive_f32(kelvin)),
	-- Tolerance percentage for the storage temperature.
	kelvin_tolerance_percentage REAL NOT NULL DEFAULT 1.0 CHECK (
		must_be_strictly_positive_f32(kelvin_tolerance_percentage)
		AND must_be_smaller_than_f32(kelvin_tolerance_percentage, 100.0)
	),
	-- The container that will be used for storage.
	stored_into_model INTEGER NOT NULL REFERENCES container_models(id),
	procedure_template_stored_into_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id) ON DELETE CASCADE,
	-- The asset that is being stored.
	stored_asset_model INTEGER NOT NULL REFERENCES physical_asset_models(id),
	-- The procedure template which originated the container being stored (e.g., a sampling or fractioning procedure template).
	foreign_procedure_template INTEGER NOT NULL REFERENCES procedure_templates.procedure_templates(procedure_template) CHECK (
		must_be_distinct_i32(
			procedure_template,
			foreign_procedure_template
		)
	),
	procedure_template_stored_asset_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- We check that the `procedure_template_stored_into_model` is indeed an asset model that is compatible with the procedure template.
	FOREIGN KEY (
		procedure_template_stored_into_model,
		procedure_template
	) REFERENCES procedure_template_asset_models(id, procedure_template),
	-- We check that the `procedure_template_stored_asset_model` is indeed an asset model that is compatible with the procedure template.
	FOREIGN KEY (
		procedure_template_stored_asset_model,
		foreign_procedure_template
	) REFERENCES procedure_template_asset_models(id, procedure_template),
	-- We check that the `stored_into_model` is indeed a container that is compatible with the procedure template.
	FOREIGN KEY (
		procedure_template_stored_into_model,
		stored_into_model
	) REFERENCES procedure_template_asset_models(id, asset_model),
	-- We check that the `stored_asset_model` is indeed a container that is compatible with the procedure template.
	FOREIGN KEY (
		procedure_template_stored_asset_model,
		stored_asset_model
	) REFERENCES procedure_template_asset_models(id, asset_model),
	-- We check that the `stored_into_model` is indeed a container that can hold the `stored_asset_model`.
	CONSTRAINT storage_pm_compatibility_rules FOREIGN KEY (stored_into_model, stored_asset_model) REFERENCES container_compatibility_rules(container_model_id, contained_asset_model),
	-- We create a same-as index to allow for foreign key references to check whether a `storage_procedure_templates.procedure_template`
	-- is associated with a given `storage_procedure_templates.foreign_procedure_template`.
	UNIQUE (procedure_template, foreign_procedure_template)
);

CREATE TABLE IF NOT EXISTS procedures.storage_procedures (
	-- Identifier of the storage procedure, which is also a foreign key to the general procedure.
	procedure UUID PRIMARY KEY REFERENCES procedures.procedures(procedure) ON DELETE CASCADE,
	-- The template of this procedure should be a storage procedure template.
	procedure_template INTEGER NOT NULL REFERENCES procedure_templates.storage_procedure_templates(procedure_template),
	-- The procedure template associated with the foreign procedure template.
	foreign_procedure_template INTEGER NOT NULL REFERENCES procedure_templates.procedure_templates(procedure_template) CHECK (
		must_be_distinct_i32(
			procedure_template,
			foreign_procedure_template
		)
	),
	-- The foreign procedure that has first defined the asset being stored (e.g., a sampling or fractioning procedure).
	foreign_procedure UUID NOT NULL REFERENCES procedures.procedures(procedure) CHECK (
		must_be_distinct_uuid(procedure, foreign_procedure)
	),
	-- The asset being stored, which must be a physical asset.
	stored_asset UUID NOT NULL REFERENCES physical_assets(id),
	-- The positioning device used for storage. This field is optional, as the positioning device might not necessarily be tracked.
	stored_with UUID NOT NULL REFERENCES containers(id),
	-- We enforce that the current `storage` has indeed the same `photograph_template`.
	FOREIGN KEY (procedure, procedure_template) REFERENCES procedures.procedures(procedure, procedure_template),
	-- We enforce that the `foreign_procedure` has as `procedure_template` the specified `foreign_procedure_template`.
	FOREIGN KEY (foreign_procedure, foreign_procedure_template) REFERENCES procedures.procedures(procedure, procedure_template),
	-- Additionally, we enforce that the `stored_asset` is indeed a procedure asset of the correct model.
	FOREIGN KEY (foreign_procedure, stored_asset) REFERENCES procedure_assets(procedure, asset),
	-- And that the `stored_with` is indeed a procedure asset of the correct model.
	FOREIGN KEY (procedure, stored_with) REFERENCES procedure_assets(procedure, asset),
	-- We enforce that the associated procedure template requires the provided foreign procedure template.
	FOREIGN KEY (procedure_template, foreign_procedure_template) REFERENCES procedure_templates.storage_procedure_templates(procedure_template, foreign_procedure_template)
);