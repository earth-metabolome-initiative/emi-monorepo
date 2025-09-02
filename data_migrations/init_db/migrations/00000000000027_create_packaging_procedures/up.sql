CREATE TABLE IF NOT EXISTS packaging_procedure_templates (
	procedure_template INTEGER PRIMARY KEY REFERENCES procedure_templates(procedure_template) ON DELETE CASCADE,
	packaged_with_model INTEGER NOT NULL REFERENCES packaging_models(id),
	procedure_template_packaged_with_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id) ON DELETE CASCADE,
	procedure_template_sample_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id) ON DELETE CASCADE,
	-- We check that the `procedure_template_packaged_with_model` is indeed assigned to the `procedure_template`.
	FOREIGN KEY (
		procedure_template_packaged_with_model,
		procedure_template
	) REFERENCES procedure_template_asset_models(id, procedure_template),
	-- We check that the `packaged_with_model` is indeed an asset model that is compatible with the procedure template.
	FOREIGN KEY (
		procedure_template_packaged_with_model,
		packaged_with_model
	) REFERENCES procedure_template_asset_models(id, asset_model),
	-- We check that the `procedure_template_sample_model` is indeed assigned to the `procedure_template`.
	FOREIGN KEY (
		procedure_template_sample_model,
		procedure_template
	) REFERENCES procedure_template_asset_models(id, procedure_template)
);
CREATE TABLE IF NOT EXISTS packaging_procedures (
	-- The extended `procedure`.
	procedure UUID PRIMARY KEY REFERENCES procedures(procedure) ON DELETE CASCADE,
	-- The procedure template of the extended `procedure`.
	procedure_template INTEGER NOT NULL REFERENCES packaging_procedure_templates(procedure_template),
	-- The packaging used for packaging, which must be a packaging model.
	packaged_with_model INTEGER NOT NULL REFERENCES packaging_models(id),
	-- We enforce that the extended `procedure` has indeed the same `procedure_template`, making
	-- sure that the procedure is a packaging procedure.
	FOREIGN KEY (procedure, procedure_template) REFERENCES procedures(procedure, procedure_template),
	-- We enforce that the `packaged_with_model` is indeed a procedure asset model.
	FOREIGN KEY (procedure, packaged_with_model) REFERENCES procedure_assets(procedure, asset_model)
);