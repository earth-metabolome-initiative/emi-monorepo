CREATE TABLE IF NOT EXISTS weighing_procedure_templates (
	-- Identifier of the weighing procedure template, which is also a a foreign key to the general procedure template table.
	procedure_template INTEGER PRIMARY KEY REFERENCES procedure_templates(procedure_template) ON DELETE CASCADE,
	-- The sample container model is the one that is being weighed.
	weighed_container_model INTEGER NOT NULL REFERENCES volumetric_container_models(id),
	-- The sample container model should always be an asset model that is compatible with the procedure template.
	procedure_template_weighed_container_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The model of the instrument to be used for weighing.
	weighed_with_model INTEGER NOT NULL REFERENCES weighing_device_models(id),
	-- The instrument model used for weighing should always be an asset model that is compatible with the procedure template.
	procedure_template_weighed_with_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id) ON DELETE CASCADE,
	-- We check that the `weighed_with` is indeed an asset model that is compatible with the procedure template.
	FOREIGN KEY (
		procedure_template_weighed_with_model,
		weighed_with_model
	) REFERENCES procedure_template_asset_models(id, asset_model),
	-- We enforce that the `weighed_container_model` is indeed a procedure asset model.
	FOREIGN KEY (
		procedure_template_weighed_container_model,
		weighed_container_model
	) REFERENCES procedure_template_asset_models(id, asset_model),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_weighed_container_model`
	-- for the current `procedure_template`.
	UNIQUE (
		procedure_template,
		procedure_template_weighed_container_model
	),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_weighed_with_model`
	-- for the current `procedure_template`.
	UNIQUE (
		procedure_template,
		procedure_template_weighed_with_model
	)
);
CREATE TABLE IF NOT EXISTS weighing_procedures(
	procedure UUID PRIMARY KEY REFERENCES procedures(procedure) ON DELETE CASCADE,
	-- We enforce that the model of this procedure must be a weighing procedure template.
	procedure_template INTEGER NOT NULL REFERENCES weighing_procedure_templates(procedure_template),
	-- The container being weighed, which must be a volumetric container model.
	weighed_container UUID NOT NULL REFERENCES volumetric_containers(id),
	-- The procedure template asset model associated to the `weighed_container`.
	procedure_template_weighed_container_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The procedure asset associated to the `weighed_container`.
	procedure_weighed_container UUID NOT NULL REFERENCES procedure_assets(id) ON DELETE CASCADE,
	-- The measured weight, which must be strictly positive.
	kilograms REAL NOT NULL CHECK (kilograms > 0.0),
	-- The weighing device used for weighing. This field is optional as there
	-- are several situations where the weighing device is not tracked.
	weighed_with UUID REFERENCES weighing_devices(id),
	-- The procedure template asset model associated to the `weighed_with_model`.
	procedure_template_weighed_with_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id) ON DELETE CASCADE,
	-- The procedure asset associated to the `weighed_with_model`.
	procedure_weighed_with UUID NOT NULL REFERENCES procedure_assets(id) ON DELETE CASCADE,
	-- We enforce that the extended `procedure` has indeed the same `procedure_template`, making
	-- sure that the procedure is a weighing procedure without the possibility of a mistake.
	FOREIGN KEY (procedure, procedure_template) REFERENCES procedures(procedure, procedure_template),
	-- The `procedure_template_weighed_with_model` must be the same as in the `weighing_procedure_templates`.
	FOREIGN KEY (
		procedure_template,
		procedure_template_weighed_container_model
	) REFERENCES weighing_procedure_templates(
		procedure_template,
		procedure_template_weighed_container_model
	),
	-- The `procedure_template_weighed_container_model` must be the same as in the `weighing_procedure_templates`.
	FOREIGN KEY (
		procedure_template,
		procedure_template_weighed_with_model
	) REFERENCES weighing_procedure_templates(
		procedure_template,
		procedure_template_weighed_with_model
	),
	-- We check that the `procedure_weighed_container` is associated to the `weighed_container`.
	FOREIGN KEY (procedure_weighed_container, weighed_container) REFERENCES procedure_assets(id, asset),
	-- We check that the `procedure_weighed_with` is associated to the `procedure_template_weighed_container_model`.
	FOREIGN KEY (
		procedure_weighed_container,
		procedure_template_weighed_container_model
	) REFERENCES procedure_assets(id, procedure_template_asset_model),
	-- We check that the `procedure_weighed_with` is associated to the `procedure_template_weighed_with_model`.
	FOREIGN KEY (
		procedure_weighed_with,
		procedure_template_weighed_with_model
	) REFERENCES procedure_assets(id, procedure_template_asset_model),
	-- We check that the `procedure_weighed_with` is associated to the `weighed_with` asset (if any).
	FOREIGN KEY (procedure_weighed_with, weighed_with) REFERENCES procedure_assets(id, asset)
);