CREATE TABLE IF NOT EXISTS procedure_templates.disposal_procedure_templates (
	procedure_template INTEGER PRIMARY KEY REFERENCES procedure_templates.procedure_templates(procedure_template) ON DELETE CASCADE,
	-- The disposed asset asset model being disposed of.
	disposed_asset_model INTEGER NOT NULL REFERENCES physical_asset_models(id),
	-- The foreign procedure template which originated the disposed asset (e.g., a sampling or fractioning procedure).
	foreign_procedure_template INTEGER NOT NULL REFERENCES procedure_templates.procedure_templates(procedure_template) CHECK (
		must_be_distinct_i32(
			procedure_template,
			foreign_procedure_template
		)
	),
	-- The associated procedure asset model for the disposed asset.
	procedure_template_disposed_asset_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The disposal procedure template should always have an asset that is compatible with it.
	FOREIGN KEY (
		procedure_template_disposed_asset_model,
		foreign_procedure_template
	) REFERENCES procedure_template_asset_models(id, procedure_template),
	-- The disposed asset model must match the procedure template of the procedure.
	FOREIGN KEY (
		procedure_template_disposed_asset_model,
		disposed_asset_model
	) REFERENCES procedure_template_asset_models(id, asset_model),
	-- We create a same-as index to allow for foreign key references to check whether a `disposal_procedure_templates.procedure_template`
	-- is associated with a given `disposal_procedure_templates.foreign_procedure_template`.
	UNIQUE (procedure_template, foreign_procedure_template)
);
CREATE TABLE IF NOT EXISTS procedures.disposal_procedures (
	procedure UUID PRIMARY KEY REFERENCES procedures.procedures(procedure) ON DELETE CASCADE,
	-- The model of the procedure.
	procedure_template INTEGER NOT NULL REFERENCES procedure_templates.disposal_procedure_templates(procedure_template) ON DELETE CASCADE,
	-- The foreign procedure template which originated the disposed asset (e.g., a sampling or fractioning procedure).
	foreign_procedure_template INTEGER NOT NULL REFERENCES procedure_templates.procedure_templates(procedure_template) CHECK (
		must_be_distinct_i32(
			procedure_template,
			foreign_procedure_template
		)
	),
	-- The foreign procedure that has first defined the asset being disposed of (e.g., a sampling or fractioning procedure).
	foreign_procedure UUID NOT NULL REFERENCES procedures.procedures(procedure) CHECK (
		must_be_distinct_uuid(procedure, foreign_procedure)
	),
	-- The disposed asset is the one that is being disposed_asset of.
	disposed_asset UUID NOT NULL REFERENCES physical_assets(id),
	-- We ensure that the parent table's procedure_template is indeed a disposal_procedure_template.
	FOREIGN KEY (procedure, procedure_template) REFERENCES procedures.procedures(procedure, procedure_template),
	-- We enforce that the `foreign_procedure` has as `procedure_template` the specified `foreign_procedure_template`.
	FOREIGN KEY (foreign_procedure, foreign_procedure_template) REFERENCES procedures.procedures(procedure, procedure_template),
	-- The disposed asset must be a procedure asset of the correct model.
	FOREIGN KEY (foreign_procedure, disposed_asset) REFERENCES procedure_assets(procedure, asset),
	-- We enforce that the associated procedure template requires the provided foreign procedure template.
	FOREIGN KEY (procedure_template, foreign_procedure_template) REFERENCES procedure_templates.disposal_procedure_templates(procedure_template, foreign_procedure_template)
);