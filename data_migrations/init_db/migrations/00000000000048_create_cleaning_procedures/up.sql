CREATE TABLE IF NOT EXISTS cleaning_procedure_templates (
	-- Identifier of the cleaning procedure template, which is also a foreign key to the general procedure template.
	procedure_template INTEGER PRIMARY KEY REFERENCES procedure_templates(procedure_template) ON DELETE CASCADE,
	-- The source container from which the liquid is poured.
	cleaned_with_model INTEGER NOT NULL REFERENCES volumetric_container_models(id),
	-- The associated procedure asset model for the source container. It may be associated
	-- to any type of other procedure templates (e.g., fractioning, packaging, etc.).
	procedure_template_cleaned_with_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The physical asset model which is being cleaned.
	cleaned_model INTEGER NOT NULL REFERENCES physical_asset_models(id),
	-- The associated procedure asset model for the cleaned physical asset.
	procedure_template_cleaned_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id) ON DELETE CASCADE,
	-- The amount of liquid that is poured into the container.
	liters REAL NOT NULL CHECK (must_be_strictly_positive_f32(liters)),
	FOREIGN KEY (
		procedure_template_cleaned_model,
		cleaned_model
	) REFERENCES procedure_template_asset_models(id, asset_model),
	FOREIGN KEY (
		procedure_template_cleaned_with_model,
		cleaned_with_model
	) REFERENCES procedure_template_asset_models(id, asset_model),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_cleaned_with_model`
	-- for the current `procedure_template`.
	UNIQUE (
		procedure_template,
		procedure_template_cleaned_with_model
	),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_cleaned_model`
	-- for the current `procedure_template`.
	UNIQUE (
		procedure_template,
		procedure_template_cleaned_model
	)
);

CREATE TABLE IF NOT EXISTS cleaning_procedures (
	-- The extended `procedure`.
	procedure UUID PRIMARY KEY REFERENCES procedures(procedure) ON DELETE CASCADE,
	-- The procedure template of the extended `procedure`.
	procedure_template INTEGER NOT NULL REFERENCES cleaning_procedure_templates(procedure_template),
	-- The procedure template asset model associated to the `cleaned_with`.
	procedure_template_cleaned_with_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The procedure asset associated to the `cleaned_with`.
	procedure_cleaned_with UUID NOT NULL REFERENCES procedure_assets(id) ON DELETE CASCADE,
	-- The procedure template asset model associated to the `cleaned`.
	procedure_template_cleaned_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The procedure asset associated to the `cleaned`.
	procedure_cleaned UUID NOT NULL REFERENCES procedure_assets(id) ON DELETE CASCADE,
	-- We enforce that the extended `procedure` has indeed the same `procedure_template`, making
	-- sure that the procedure is a packaging procedure.
	FOREIGN KEY (procedure, procedure_template) REFERENCES procedures(procedure, procedure_template),
	-- The `procedure_template_cleaned_with_model` must be the same as in the `cleaning_procedure_templates`.
	FOREIGN KEY (
		procedure_template,
		procedure_template_cleaned_with_model
	) REFERENCES cleaning_procedure_templates(
		procedure_template,
		procedure_template_cleaned_with_model
	),
	-- The `procedure_template_cleaned_model` must be the same as in the `cleaning_procedure_templates`.
	FOREIGN KEY (
		procedure_template,
		procedure_template_cleaned_model
	) REFERENCES cleaning_procedure_templates(
		procedure_template,
		procedure_template_cleaned_model
	),
	-- We check that the `procedure_cleaned_with` is indeed associated to the `procedure_template_cleaned_with_model`.
	FOREIGN KEY (
		procedure_cleaned_with,
		procedure_template_cleaned_with_model
	) REFERENCES procedure_assets(id, procedure_template_asset_model),
	-- We check that the `procedure_cleaned` is indeed associated to the `procedure_template_cleaned_model`.
	FOREIGN KEY (
		procedure_cleaned,
		procedure_template_cleaned_model
	) REFERENCES procedure_assets(id, procedure_template_asset_model)
);