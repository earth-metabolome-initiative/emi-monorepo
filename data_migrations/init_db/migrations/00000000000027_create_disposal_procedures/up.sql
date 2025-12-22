CREATE TABLE IF NOT EXISTS disposal_procedure_templates (
	id INTEGER PRIMARY KEY REFERENCES procedure_templates(id) ON DELETE CASCADE,
	-- The disposed asset asset model being disposed of.
	disposed_asset_model_id INTEGER NOT NULL REFERENCES physical_asset_models(id),
	-- The associated procedure_id asset model for the disposed asset.
	procedure_template_disposed_asset_model_id INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The disposed asset model must match the procedure_id template of the procedure.
	FOREIGN KEY (
		procedure_template_disposed_asset_model_id,
		disposed_asset_model_id
	) REFERENCES procedure_template_asset_models(id, asset_model_id),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_disposed_asset_model`
	-- for the current `procedure_template`.
	UNIQUE (
		id,
		procedure_template_disposed_asset_model_id
	)
);
CREATE TABLE IF NOT EXISTS disposal_procedures (
	id UUID PRIMARY KEY REFERENCES procedures(id) ON DELETE CASCADE,
	-- The model of the procedure.
	disposal_procedure_template_id INTEGER NOT NULL REFERENCES disposal_procedure_templates(id) ON DELETE CASCADE,
	-- The disposed asset is the one that is being disposed_asset_id of.
	disposed_asset_id UUID REFERENCES physical_assets(id),
	-- The procedure_id template asset model associated to the `disposed_asset`.
	procedure_template_disposed_asset_model_id INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The procedure_id asset associated to the `disposed_asset`.
	procedure_disposed_asset_id UUID NOT NULL REFERENCES procedure_assets(id) ON DELETE CASCADE,
	-- We ensure that the parent_id table's procedure_template_id is indeed a disposal_procedure_template.
	FOREIGN KEY (id, disposal_procedure_template_id) REFERENCES procedures(id, procedure_template_id),
	-- The procedure_id template asset model describing the `disposed_asset` must be the same one
	-- as the one in the procedure_id template.
	FOREIGN KEY (
		disposal_procedure_template_id,
		procedure_template_disposed_asset_model_id
	) REFERENCES disposal_procedure_templates(
		id,
		procedure_template_disposed_asset_model_id
	),
	-- We enforce that the `procedure_disposed_asset` is associated with the `procedure_template_disposed_asset_model`.
	FOREIGN KEY (
		procedure_disposed_asset_id,
		procedure_template_disposed_asset_model_id
	) REFERENCES procedure_assets(id, procedure_template_asset_model_id),
	-- We enforce that the `procedure_disposed_asset` is associated with the `disposed_asset`.
	FOREIGN KEY (procedure_disposed_asset_id, disposed_asset_id) REFERENCES procedure_assets(id, asset_id)
);