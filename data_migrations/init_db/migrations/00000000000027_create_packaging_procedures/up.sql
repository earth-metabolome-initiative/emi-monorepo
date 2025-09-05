CREATE TABLE IF NOT EXISTS packaging_procedure_templates (
	procedure_template INTEGER PRIMARY KEY REFERENCES procedure_templates(procedure_template) ON DELETE CASCADE,
	packaged_with_model INTEGER NOT NULL REFERENCES packaging_models(id),
	procedure_template_packaged_with_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id) ON DELETE CASCADE,
	sample_model INTEGER NOT NULL REFERENCES physical_asset_models(id),
	procedure_template_sample_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id) ON DELETE CASCADE,
	-- We check that the `packaged_with_model` is indeed an asset model that is compatible with the procedure template asset model.
	FOREIGN KEY (
		procedure_template_packaged_with_model,
		packaged_with_model
	) REFERENCES procedure_template_asset_models(id, asset_model),
	-- We check that the `sample_model` is indeed an asset model that is compatible with the procedure template asset model.
	FOREIGN KEY (
		procedure_template_sample_model,
		sample_model
	) REFERENCES procedure_template_asset_models(id, asset_model),
	-- The `sample_model` must be compatible with the `packaged_with_model`.
	FOREIGN KEY (packaged_with_model, sample_model) REFERENCES asset_compatibility_rules(left_asset_model, right_asset_model),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_packaged_with_model`
	-- for the current `procedure_template`.
	UNIQUE (
		procedure_template,
		procedure_template_packaged_with_model
	),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_sample_model`
	-- for the current `procedure_template`.
	UNIQUE (
		procedure_template,
		procedure_template_sample_model
	)
);
CREATE TABLE IF NOT EXISTS packaging_procedures (
	-- The extended `procedure`.
	procedure UUID PRIMARY KEY REFERENCES procedures(procedure) ON DELETE CASCADE,
	-- The procedure template of the extended `procedure`.
	procedure_template INTEGER NOT NULL REFERENCES packaging_procedure_templates(procedure_template),
	-- The sample being packaged, which must be a physical asset.
	sample UUID NOT NULL REFERENCES physical_assets(id),
	-- The model of the sample being packaged, which must be a physical asset model.
	sample_model INTEGER NOT NULL REFERENCES physical_asset_models(id),
	-- The procedure template asset model associated to the `sample`.
	procedure_template_sample_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The procedure asset associated to the `sample`.
	procedure_sample UUID NOT NULL REFERENCES procedure_assets(id) ON DELETE CASCADE,
	-- The packaging used for packaging, which must be a packaging model.
	packaged_with_model INTEGER NOT NULL REFERENCES packaging_models(id),
	-- The procedure template asset model associated to the `packaged_with_model`.
	procedure_template_packaged_with_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The procedure asset associated to the `packaged_with_model`.
	procedure_packaged_with UUID NOT NULL REFERENCES procedure_assets(id) ON DELETE CASCADE,
	-- We enforce that the extended `procedure` has indeed the same `procedure_template`, making
	-- sure that the procedure is a packaging procedure.
	FOREIGN KEY (procedure, procedure_template) REFERENCES procedures(procedure, procedure_template),
	-- The `procedure_template_packaged_with_model` must be the same as in the `packaging_procedure_templates`.
	FOREIGN KEY (
		procedure_template,
		procedure_template_packaged_with_model
	) REFERENCES packaging_procedure_templates(
		procedure_template,
		procedure_template_packaged_with_model
	),
	-- The `procedure_template_sample_model` must be the same as in the `packaging_procedure_templates`.
	FOREIGN KEY (
		procedure_template,
		procedure_template_sample_model
	) REFERENCES packaging_procedure_templates(
		procedure_template,
		procedure_template_sample_model
	),
	-- We check that the `procedure_sample` is associated to the `sample`.
	FOREIGN KEY (procedure_sample, sample) REFERENCES procedure_assets(id, asset),
	-- We check that the `procedure_packaged_with` is associated to the `packaged_with_model`.
	FOREIGN KEY (procedure_packaged_with, packaged_with_model) REFERENCES procedure_assets(id, asset_model),
	-- We check that the `procedure_sample` is indeed associated to the `procedure_template_sample_model`.
	FOREIGN KEY (
		procedure_sample,
		procedure_template_sample_model
	) REFERENCES procedure_assets(id, procedure_template_asset_model),
	-- We check that the `procedure_packaged_with` is indeed associated to the `procedure_template_packaged_with_model`.
	FOREIGN KEY (
		procedure_packaged_with,
		procedure_template_packaged_with_model
	) REFERENCES procedure_assets(id, procedure_template_asset_model),
	-- We check that the `sample` is indeed compatible with the `packaged_with_model`.
	FOREIGN KEY (packaged_with_model, sample_model) REFERENCES asset_compatibility_rules(left_asset_model, right_asset_model),
	-- We check that the `procedure_sample` is associated to the `sample_model`.
	FOREIGN KEY (procedure_sample, sample_model) REFERENCES procedure_assets(id, asset_model)
);