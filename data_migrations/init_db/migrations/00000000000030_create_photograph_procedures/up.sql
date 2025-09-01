CREATE TABLE IF NOT EXISTS procedure_templates.photograph_procedure_templates (
	procedure_template INTEGER PRIMARY KEY REFERENCES procedure_templates.procedure_templates(procedure_template) ON DELETE CASCADE,
	-- The device used for photograph.
	photographed_with_model INTEGER NOT NULL REFERENCES camera_models(id),
	procedure_template_photographed_with_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id) ON DELETE CASCADE,
	photographed_asset_model INTEGER NOT NULL REFERENCES physical_asset_models(id),
	foreign_procedure_template INTEGER NOT NULL REFERENCES procedure_templates.procedure_templates(procedure_template) ON DELETE CASCADE,
	procedure_template_photographed_asset_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- We check that the `photographed_with_model` is indeed a trackable that is compatible with the procedure template.
	FOREIGN KEY (
		procedure_template_photographed_with_model,
		photographed_with_model
	) REFERENCES procedure_template_asset_models(id, asset_model),
	-- We check that the `procedure_template_photographed_with_model` is indeed a trackable that is compatible with the procedure template.
	FOREIGN KEY (
		procedure_template_photographed_with_model,
		procedure_template
	) REFERENCES procedure_template_asset_models(id, procedure_template),
	FOREIGN KEY (
		procedure_template_photographed_asset_model,
		foreign_procedure_template
	) REFERENCES procedure_template_asset_models(id, procedure_template),
	FOREIGN KEY (
		procedure_template_photographed_asset_model,
		photographed_asset_model
	) REFERENCES procedure_template_asset_models(id, asset_model),
	-- We define a same-as index to allow for foreign key references to check whether a `photograph_procedure_templates.procedure_template`
	-- is associated with a given `photograph_procedure_templates.foreign_procedure_template`.
	UNIQUE (procedure_template, foreign_procedure_template)
);
CREATE TABLE IF NOT EXISTS procedures.photograph_procedures (
	-- Identifier of the photograph procedure, which is also a foreign key to the general procedure.
	procedure UUID PRIMARY KEY REFERENCES procedures.procedures(procedure) ON DELETE CASCADE,
	-- The template of this procedure should be a photograph procedure template.
	procedure_template INTEGER NOT NULL REFERENCES procedure_templates.photograph_procedure_templates(procedure_template),
	-- The procedure template associated with the foreign procedure template.
	foreign_procedure_template INTEGER NOT NULL REFERENCES procedure_templates.procedure_templates(procedure_template) CHECK (
		must_be_distinct_i32(
			procedure_template,
			foreign_procedure_template
		)
	),
	-- The foreign procedure that has first defined the asset being photographed (e.g., a sampling or fractioning procedure).
	foreign_procedure UUID NOT NULL REFERENCES procedures.procedures(procedure) CHECK (
		must_be_distinct_uuid(procedure, foreign_procedure)
	),
	-- The asset being photographed, which must be a physical asset.
	photographed_asset UUID NOT NULL REFERENCES physical_assets(id),
	-- The positioning device used for photograph. This field is optional, as the positioning device might not necessarily be tracked.
	photographed_with UUID REFERENCES cameras(id),
	-- The model of the positioning device used for photograph, which must be a positioning device model.
	photographed_with_model INTEGER NOT NULL REFERENCES camera_models(id),
	-- We enforce that the current `photograph` has indeed the same `photograph_template`.
	FOREIGN KEY (procedure, procedure_template) REFERENCES procedures.procedures(procedure, procedure_template),
	-- We enforce that the `foreign_procedure` has as `procedure_template` the specified `foreign_procedure_template`.
	FOREIGN KEY (foreign_procedure, foreign_procedure_template) REFERENCES procedures.procedures(procedure, procedure_template),
	-- Additionally, we enforce that the `photographed_asset` is indeed a procedure asset of the correct model.
	FOREIGN KEY (foreign_procedure, photographed_asset) REFERENCES procedure_assets(procedure, asset),
	-- We enforce that the `photographed_with_model` is indeed a procedure asset model.
	FOREIGN KEY (procedure, photographed_with_model) REFERENCES procedure_assets(procedure, asset_model),
	-- And that the `photographed_with` is indeed a procedure asset of the correct model.
	FOREIGN KEY (procedure, photographed_with) REFERENCES procedure_assets(procedure, asset),
	-- We enforce that the `photographed_with` is indeed a weighing device of the correct model.
	FOREIGN KEY (photographed_with, photographed_with_model) REFERENCES assets(id, model_id),
	-- We enforce that the associated procedure template requires the provided foreign procedure template.
	FOREIGN KEY (procedure_template, foreign_procedure_template) REFERENCES procedure_templates.photograph_procedure_templates(procedure_template, foreign_procedure_template)
);