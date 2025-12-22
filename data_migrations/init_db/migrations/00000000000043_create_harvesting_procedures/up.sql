CREATE TABLE IF NOT EXISTS harvesting_procedure_templates (
	-- Identifier of the harvesting procedure_id template, which is also a foreign key to the general procedure_id template.
	id INTEGER PRIMARY KEY REFERENCES procedure_templates(id) ON DELETE CASCADE,
	-- Sample source model from which the sample is taken.
	sample_source_model_id INTEGER NOT NULL REFERENCES sample_source_models(id),
	procedure_template_sample_source_model_id INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- Sample model harvested from the sample source model.
	sample_model_id INTEGER NOT NULL REFERENCES sample_models(id),
	procedure_template_sample_model_id INTEGER NOT NULL REFERENCES procedure_template_asset_models(id) ON DELETE CASCADE,
	-- We enforce that the `sample_source_model` is indeed a procedure_id asset model.
	FOREIGN KEY (
		procedure_template_sample_source_model_id,
		sample_source_model_id
	) REFERENCES procedure_template_asset_models(id, asset_model_id),
	FOREIGN KEY (
		procedure_template_sample_model_id,
		sample_model_id
	) REFERENCES procedure_template_asset_models(id, asset_model_id),
	-- We enforce that the `sample_model` is associated with the `sample_source_model`.
	FOREIGN KEY (sample_model_id, sample_source_model_id) REFERENCES sample_models(id, sample_source_model_id),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_sample_source_model`
	-- for the current `procedure_template`.
	UNIQUE (
		id,
		procedure_template_sample_source_model_id
	),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_sample_model`
	-- for the current `procedure_template`.
	UNIQUE (
		id,
		procedure_template_sample_model_id
	)
);

CREATE TABLE IF NOT EXISTS harvesting_procedures (
	-- Identifier of the harvesting id, which is also a foreign key to the general procedure.
	id UUID PRIMARY KEY REFERENCES procedures(id) ON DELETE CASCADE,
	-- The template of this procedure_id should be a harvesting procedure_id template.
	harvesting_procedure_template_id INTEGER NOT NULL REFERENCES harvesting_procedure_templates(id),
	-- The sample source from which the sample is harvested, which must be a sample source asset.
	sample_source_id UUID NOT NULL REFERENCES sample_sources(id),
	-- The procedure_id template asset model associated to the `sample_source`.
	procedure_template_sample_source_model_id INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The procedure_id asset associated to the `sample_source`.
	procedure_sample_source_id UUID NOT NULL REFERENCES procedure_assets(id) ON DELETE CASCADE,
	-- The sample harvetsed from the sample source, which must be a sample asset.
	sample_id UUID NOT NULL REFERENCES samples(id),
	-- The procedure_id template asset model associated to the `sample`.
	procedure_template_sample_model_id INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The procedure_id asset associated to the `sample`.
	procedure_sample_id UUID NOT NULL REFERENCES procedure_assets(id) ON DELETE CASCADE,
	FOREIGN KEY (id, harvesting_procedure_template_id) REFERENCES procedures(id, procedure_template_id),
	-- We enforce that the `procedure_template_sample_source_model` is associated with the `procedure_template`.
	FOREIGN KEY (
		harvesting_procedure_template_id,
		procedure_template_sample_source_model_id
	) REFERENCES harvesting_procedure_templates(
		id,
		procedure_template_sample_source_model_id
	),
	-- We enforce that the `procedure_template_sample_model` is associated with the `procedure_template`.
	FOREIGN KEY (
		harvesting_procedure_template_id,
		procedure_template_sample_model_id
	) REFERENCES harvesting_procedure_templates(
		id,
		procedure_template_sample_model_id
	),
	-- We enforce that the `procedure_sample_source` is associated with the `sample_source`.
	FOREIGN KEY (procedure_sample_source_id, sample_source_id) REFERENCES procedure_assets(id, asset_id),
	-- We enforce that the `procedure_sample` is associated with the `sample`.
	FOREIGN KEY (procedure_sample_id, sample_id) REFERENCES procedure_assets(id, asset_id),
	-- We enforce that the `procedure_sample_source` is associated with `procedure_template_sample_source_model`.
	FOREIGN KEY (
		procedure_sample_source_id,
		procedure_template_sample_source_model_id
	) REFERENCES procedure_assets(id, procedure_template_asset_model_id),
	-- We enforce that the `procedure_sample` is associated with `procedure_template_sample_model`.
	FOREIGN KEY (
		procedure_sample_id,
		procedure_template_sample_model_id
	) REFERENCES procedure_assets(id, procedure_template_asset_model_id)
);