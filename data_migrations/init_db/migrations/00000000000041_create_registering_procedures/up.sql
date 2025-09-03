CREATE TABLE IF NOT EXISTS registering_procedure_templates (
	-- Identifier of the registering procedure template, which is also a a foreign key to the general procedure template table.
	procedure_template INTEGER PRIMARY KEY REFERENCES procedure_templates(procedure_template) ON DELETE CASCADE,
	-- The model of the instrument to be used for registering.
	registered_asset_model INTEGER NOT NULL REFERENCES asset_models(id),
	-- The instrument model used for registering should always be an asset model that is compatible with the procedure template.
	procedure_template_registered_asset_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id) ON DELETE CASCADE,
	-- We ensure that the `procedure_template_registered_asset_model` is indeed associated with the parent procedure template.
	FOREIGN KEY (
		procedure_template_registered_asset_model,
		procedure_template
	) REFERENCES procedure_template_asset_models(id, procedure_template),
	-- We ensure that the `registered_asset_model` effectively comes from `procedure_template_registered_asset_model`.
	FOREIGN KEY (
		procedure_template_registered_asset_model,
		registered_asset_model
	) REFERENCES procedure_template_asset_models(id, asset_model)
);
CREATE TABLE IF NOT EXISTS registering_procedures(
	procedure UUID PRIMARY KEY REFERENCES procedures(procedure) ON DELETE CASCADE,
	-- We enforce that the model of this procedure must be a registering procedure template.
	procedure_template INTEGER NOT NULL REFERENCES registering_procedure_templates(procedure_template),
	-- The newly registered asset.
	registered_asset UUID NOT NULL REFERENCES assets(id),
	-- We enforce that the extended `procedure` has indeed the same `procedure_template`, making
	-- sure that the procedure is a registering procedure without the possibility of a mistake.
	FOREIGN KEY (procedure, procedure_template) REFERENCES procedures(procedure, procedure_template),
	-- We enforce that the `registered_asset` is indeed a procedure asset.
	FOREIGN KEY (procedure, registered_asset) REFERENCES procedure_assets(procedure, asset)
);