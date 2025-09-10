CREATE TABLE IF NOT EXISTS harvesting_procedure_templates (
	-- Identifier of the harvesting procedure template, which is also a foreign key to the general procedure template.
	procedure_template INTEGER PRIMARY KEY REFERENCES procedure_templates(procedure_template) ON DELETE CASCADE,
	-- Sample source model from which the sample is taken.
	sample_source_model INTEGER NOT NULL REFERENCES sample_source_models(id),
	procedure_template_sample_source_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- Sample model harvested from the sample source model.
	sample_model INTEGER NOT NULL REFERENCES sample_models(id),
	procedure_template_sample_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id) ON DELETE CASCADE,
	-- We enforce that the `sample_source_model` is indeed a procedure asset model.
	FOREIGN KEY (
		procedure_template_sample_source_model,
		sample_source_model
	) REFERENCES procedure_template_asset_models(id, asset_model),
	FOREIGN KEY (
		procedure_template_sample_model,
		sample_model
	) REFERENCES procedure_template_asset_models(id, asset_model),
	-- We enforce that the `sample_model` is associated with the `sample_source_model`.
	FOREIGN KEY (sample_model, sample_source_model) REFERENCES sample_models(id, sample_source_model),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_sample_source_model`
	-- for the current `procedure_template`.
	UNIQUE (
		procedure_template,
		procedure_template_sample_source_model
	),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_sample_model`
	-- for the current `procedure_template`.
	UNIQUE (
		procedure_template,
		procedure_template_sample_model
	)
);

CREATE TABLE IF NOT EXISTS harvesting_procedures (
	-- Identifier of the harvesting procedure, which is also a foreign key to the general procedure.
	procedure UUID PRIMARY KEY REFERENCES procedures(procedure) ON DELETE CASCADE,
	-- The template of this procedure should be a harvesting procedure template.
	procedure_template INTEGER NOT NULL REFERENCES harvesting_procedure_templates(procedure_template),
	-- The sample source from which the sample is harvested, which must be a sample source asset.
	sample_source UUID NOT NULL REFERENCES sample_sources(id),
	-- The procedure template asset model associated to the `sample_source`.
	procedure_template_sample_source_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The procedure asset associated to the `sample_source`.
	procedure_sample_source UUID NOT NULL REFERENCES procedure_assets(id) ON DELETE CASCADE,
	-- The sample harvetsed from the sample source, which must be a sample asset.
	sample UUID NOT NULL REFERENCES samples(id),
	-- The procedure template asset model associated to the `sample`.
	procedure_template_sample_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The procedure asset associated to the `sample`.
	procedure_sample UUID NOT NULL REFERENCES procedure_assets(id) ON DELETE CASCADE,
	FOREIGN KEY (procedure, procedure_template) REFERENCES procedures(procedure, procedure_template),
	-- We enforce that the `procedure_template_sample_source_model` is associated with the `procedure_template`.
	FOREIGN KEY (
		procedure_template,
		procedure_template_sample_source_model
	) REFERENCES harvesting_procedure_templates(
		procedure_template,
		procedure_template_sample_source_model
	),
	-- We enforce that the `procedure_template_sample_model` is associated with the `procedure_template`.
	FOREIGN KEY (
		procedure_template,
		procedure_template_sample_model
	) REFERENCES harvesting_procedure_templates(
		procedure_template,
		procedure_template_sample_model
	),
	-- We enforce that the `procedure_sample_source` is associated with the `sample_source`.
	FOREIGN KEY (procedure_sample_source, sample_source) REFERENCES procedure_assets(id, asset),
	-- We enforce that the `procedure_sample` is associated with the `sample`.
	FOREIGN KEY (procedure_sample, sample) REFERENCES procedure_assets(id, asset),
	-- We enforce that the `procedure_sample_source` is associated with `procedure_template_sample_source_model`.
	FOREIGN KEY (
		procedure_sample_source,
		procedure_template_sample_source_model
	) REFERENCES procedure_assets(id, procedure_template_asset_model),
	-- We enforce that the `procedure_sample` is associated with `procedure_template_sample_model`.
	FOREIGN KEY (
		procedure_sample,
		procedure_template_sample_model
	) REFERENCES procedure_assets(id, procedure_template_asset_model)
);