CREATE TABLE IF NOT EXISTS photograph_procedure_templates (
	id INTEGER PRIMARY KEY REFERENCES procedure_templates(id) ON DELETE CASCADE,
	-- The device used for photograph.
	photographed_with_model_id INTEGER NOT NULL REFERENCES camera_models(id),
	procedure_template_photographed_with_model_id INTEGER NOT NULL REFERENCES procedure_template_asset_models(id) ON DELETE CASCADE,
	photographed_asset_model_id INTEGER NOT NULL REFERENCES physical_asset_models(id),
	procedure_template_photographed_asset_model_id INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	photograph_model_id INTEGER NOT NULL REFERENCES digital_asset_models(id),
	procedure_template_photograph_model_id INTEGER NOT NULL REFERENCES procedure_template_asset_models(id) ON DELETE CASCADE,
	-- We check that the `photographed_with_model` is indeed a trackable that is compatible with the procedure_id template.
	FOREIGN KEY (
		procedure_template_photographed_with_model_id,
		photographed_with_model_id
	) REFERENCES procedure_template_asset_models(id, asset_model_id),
	FOREIGN KEY (
		procedure_template_photographed_asset_model_id,
		photographed_asset_model_id
	) REFERENCES procedure_template_asset_models(id, asset_model_id),
	FOREIGN KEY (
		procedure_template_photograph_model_id,
		photograph_model_id
	) REFERENCES procedure_template_asset_models(id, asset_model_id),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_photographed_with_model`
	-- for the current `procedure_template`.
	UNIQUE (
		id,
		procedure_template_photographed_with_model_id
	),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_photographed_asset_model`
	-- for the current `procedure_template`.
	UNIQUE (
		id,
		procedure_template_photographed_asset_model_id
	),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_photograph_model`
	-- for the current `procedure_template`.
	UNIQUE (
		id,
		procedure_template_photograph_model_id
	)
);
CREATE TABLE IF NOT EXISTS photograph_procedures (
	-- Identifier of the photograph_id id, which is also a foreign key to the general procedure.
	id UUID PRIMARY KEY REFERENCES procedures(id) ON DELETE CASCADE,
	-- The template of this procedure_id should be a photograph_id procedure_id template.
	photograph_procedure_template_id INTEGER NOT NULL REFERENCES photograph_procedure_templates(id),
	-- The asset being photographed, which must be a physical asset.
	photographed_asset_id UUID REFERENCES physical_assets(id),
	-- The procedure_id template asset model associated to the `photographed_asset`.
	procedure_template_photographed_asset_model_id INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The procedure_id asset associated to the `photographed_asset`.
	procedure_photographed_asset_id UUID NOT NULL REFERENCES procedure_assets(id) ON DELETE CASCADE,
	-- The positioning device used for photograph. This field is optional, as the positioning device might not necessarily be tracked.
	photographed_with_id UUID REFERENCES cameras(id),
	-- The procedure_id template asset model associated to the `photographed_with_model`.
	procedure_template_photographed_with_model_id INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The procedure_id asset associated to the `photographed_with`.
	procedure_photographed_with_id UUID NOT NULL REFERENCES procedure_assets(id) ON DELETE CASCADE,
	-- The resulting photograph.
	photograph_id UUID NOT NULL REFERENCES photographs(id) ON DELETE CASCADE,
	-- The procedure_id template asset model associated to the `photograph_model`.
	procedure_template_photograph_model_id INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The procedure_id asset associated to the `photograph`.
	procedure_photograph_id UUID NOT NULL REFERENCES procedure_assets(id) ON DELETE CASCADE,
	-- We enforce that the current `photograph` has indeed the same `photograph_template`.
	FOREIGN KEY (id, photograph_procedure_template_id) REFERENCES procedures(id, procedure_template_id),
	-- The `procedure_template_photographed_with_model` must be the same as in the `photograph_procedure_templates`.
	FOREIGN KEY (
		photograph_procedure_template_id,
		procedure_template_photographed_with_model_id
	) REFERENCES photograph_procedure_templates(
		id,
		procedure_template_photographed_with_model_id
	),
	-- The `procedure_template_photographed_asset_model` must be the same as in the `photograph_procedure_templates`.
	FOREIGN KEY (
		photograph_procedure_template_id,
		procedure_template_photographed_asset_model_id
	) REFERENCES photograph_procedure_templates(
		id,
		procedure_template_photographed_asset_model_id
	),
	-- The `procedure_template_photograph_model` must be the same as in the `photograph_procedure_templates`.
	FOREIGN KEY (
		photograph_procedure_template_id,
		procedure_template_photograph_model_id
	) REFERENCES photograph_procedure_templates(
		id,
		procedure_template_photograph_model_id
	),
	-- We check that the `procedure_photographed_with` is associated to the same `procedure_template_photographed_with_model`.
	FOREIGN KEY (
		procedure_photographed_with_id,
		procedure_template_photographed_with_model_id
	) REFERENCES procedure_assets(id, procedure_template_asset_model_id),
	-- We check that the `procedure_photographed_asset` is associated to the same `procedure_template_photographed_asset_model`.
	FOREIGN KEY (
		procedure_photographed_asset_id,
		procedure_template_photographed_asset_model_id
	) REFERENCES procedure_assets(id, procedure_template_asset_model_id),
	-- We check that the `procedure_photograph` is associated to the same `procedure_template_photograph_model`.
	FOREIGN KEY (
		procedure_photograph_id,
		procedure_template_photograph_model_id
	) REFERENCES procedure_assets(id, procedure_template_asset_model_id),
	-- We check that the `procedure_photographed_asset` is associated to the `photographed_asset`.
	FOREIGN KEY (procedure_photographed_asset_id, photographed_asset_id) REFERENCES procedure_assets(id, asset_id),
	-- We check that the `procedure_photographed_with` is associated to the `photographed_with`.
	FOREIGN KEY (procedure_photographed_with_id, photographed_with_id) REFERENCES procedure_assets(id, asset_id),
	-- We check that the `procedure_photograph` is associated to the `photograph`.
	FOREIGN KEY (procedure_photograph_id, photograph_id) REFERENCES procedure_assets(id, asset_id)
);