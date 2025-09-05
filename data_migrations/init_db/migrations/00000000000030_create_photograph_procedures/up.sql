CREATE TABLE IF NOT EXISTS photograph_procedure_templates (
	procedure_template INTEGER PRIMARY KEY REFERENCES procedure_templates(procedure_template) ON DELETE CASCADE,
	-- The device used for photograph.
	photographed_with_model INTEGER NOT NULL REFERENCES camera_models(id),
	procedure_template_photographed_with_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id) ON DELETE CASCADE,
	photographed_asset_model INTEGER NOT NULL REFERENCES physical_asset_models(id),
	procedure_template_photographed_asset_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- We check that the `photographed_with_model` is indeed a trackable that is compatible with the procedure template.
	FOREIGN KEY (
		procedure_template_photographed_with_model,
		photographed_with_model
	) REFERENCES procedure_template_asset_models(id, asset_model),
	FOREIGN KEY (
		procedure_template_photographed_asset_model,
		photographed_asset_model
	) REFERENCES procedure_template_asset_models(id, asset_model),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_photographed_with_model`
	-- for the current `procedure_template`.
	UNIQUE (
		procedure_template,
		procedure_template_photographed_with_model
	),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_photographed_asset_model`
	-- for the current `procedure_template`.
	UNIQUE (
		procedure_template,
		procedure_template_photographed_asset_model
	)
);
CREATE TABLE IF NOT EXISTS photograph_procedures (
	-- Identifier of the photograph procedure, which is also a foreign key to the general procedure.
	procedure UUID PRIMARY KEY REFERENCES procedures(procedure) ON DELETE CASCADE,
	-- The template of this procedure should be a photograph procedure template.
	procedure_template INTEGER NOT NULL REFERENCES photograph_procedure_templates(procedure_template),
	-- The asset being photographed, which must be a physical asset.
	photographed_asset UUID NOT NULL REFERENCES physical_assets(id),
	-- The procedure template asset model associated to the `photographed_asset`.
	procedure_template_photographed_asset_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The procedure asset associated to the `photographed_asset`.
	procedure_photographed_asset UUID NOT NULL REFERENCES procedure_assets(id) ON DELETE CASCADE,
	-- The positioning device used for photograph. This field is optional, as the positioning device might not necessarily be tracked.
	photographed_with UUID REFERENCES cameras(id),
	-- The procedure template asset model associated to the `photographed_with_model`.
	procedure_template_photographed_with_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The procedure asset associated to the `photographed_with`.
	procedure_photographed_with UUID NOT NULL REFERENCES procedure_assets(id) ON DELETE CASCADE,
	-- We enforce that the current `photograph` has indeed the same `photograph_template`.
	FOREIGN KEY (procedure, procedure_template) REFERENCES procedures(procedure, procedure_template),
	-- The `procedure_template_photographed_with_model` must be the same as in the `photograph_procedure_templates`.
	FOREIGN KEY (
		procedure_template,
		procedure_template_photographed_with_model
	) REFERENCES photograph_procedure_templates(
		procedure_template,
		procedure_template_photographed_with_model
	),
	-- The `procedure_template_photographed_asset_model` must be the same as in the `photograph_procedure_templates`.
	FOREIGN KEY (
		procedure_template,
		procedure_template_photographed_asset_model
	) REFERENCES photograph_procedure_templates(
		procedure_template,
		procedure_template_photographed_asset_model
	),
	-- We check that the `procedure_photographed_with` is associated to the same `procedure_template_photographed_with_model`.
	FOREIGN KEY (
		procedure_photographed_with,
		procedure_template_photographed_with_model
	) REFERENCES procedure_assets(id, procedure_template_asset_model),
	-- We check that the `procedure_photographed_asset` is associated to the same `procedure_template_photographed_asset_model`.
	FOREIGN KEY (
		procedure_photographed_asset,
		procedure_template_photographed_asset_model
	) REFERENCES procedure_assets(id, procedure_template_asset_model),
	-- We check that the `procedure_photographed_asset` is associated to the `photographed_asset`.
	FOREIGN KEY (procedure_photographed_asset, photographed_asset) REFERENCES procedure_assets(id, asset),
	-- We check that the `procedure_photographed_with` is associated to the `photographed_with`.
	FOREIGN KEY (procedure_photographed_with, photographed_with) REFERENCES procedure_assets(id, asset)
);