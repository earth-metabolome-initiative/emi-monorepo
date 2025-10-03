CREATE TABLE IF NOT EXISTS ppe_reminder_procedure_templates (
	procedure_template INTEGER PRIMARY KEY REFERENCES procedure_templates(procedure_template) ON DELETE CASCADE,
	-- The ppe asset  model being reminded of.
	ppe_asset_model INTEGER NOT NULL REFERENCES personal_protective_equipment_models(id),
	-- The associated procedure asset model for the ppe asset.
	procedure_template_ppe_asset_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The ppe asset model must match the procedure template of the procedure.
	FOREIGN KEY (
		procedure_template_ppe_asset_model,
		ppe_asset_model
	) REFERENCES procedure_template_asset_models(id, asset_model),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_ppe_asset_model`
	-- for the current `procedure_template`.
	UNIQUE (
		procedure_template,
		procedure_template_ppe_asset_model
	)
);
CREATE TABLE IF NOT EXISTS ppe_reminder_procedures (
	procedure UUID PRIMARY KEY REFERENCES procedures(procedure) ON DELETE CASCADE,
	-- The model of the procedure.
	procedure_template INTEGER NOT NULL REFERENCES ppe_reminder_procedure_templates(procedure_template) ON DELETE CASCADE,
	-- The procedure template asset model associated to the `ppe_asset`.
	procedure_template_ppe_asset_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The procedure asset associated to the `ppe_asset`.
	procedure_ppe_asset UUID NOT NULL REFERENCES procedure_assets(id) ON DELETE CASCADE,
	-- We ensure that the parent table's procedure_template is indeed a ppe_reminder_procedure_template.
	FOREIGN KEY (procedure, procedure_template) REFERENCES procedures(procedure, procedure_template),
	-- The procedure template asset model describing the `ppe_asset` must be the same one
	-- as the one in the procedure template.
	FOREIGN KEY (
		procedure_template,
		procedure_template_ppe_asset_model
	) REFERENCES ppe_reminder_procedure_templates(
		procedure_template,
		procedure_template_ppe_asset_model
	),
	-- We enforce that the `procedure_ppe_asset` is associated with the `procedure_template_ppe_asset_model`.
	FOREIGN KEY (
		procedure_ppe_asset,
		procedure_template_ppe_asset_model
	) REFERENCES procedure_assets(id, procedure_template_asset_model)
);