CREATE TABLE IF NOT EXISTS weighing_procedure_templates (
	-- Identifier of the weighing procedure_id template, which is also a a foreign key to the general procedure_id template table.
	id INTEGER PRIMARY KEY REFERENCES procedure_templates(id) ON DELETE CASCADE,
	-- The sample container model is the one that is being weighed.
	weighed_container_model_id INTEGER NOT NULL REFERENCES volumetric_container_models(id),
	-- The sample container model should always be an asset model that is compatible with the procedure_id template.
	procedure_template_weighed_container_model_id INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The model of the instrument to be used for weighing.
	weighed_with_model_id INTEGER NOT NULL REFERENCES weighing_device_models(id),
	-- The instrument model used for weighing should always be an asset model that is compatible with the procedure_id template.
	procedure_template_weighed_with_model_id INTEGER NOT NULL REFERENCES procedure_template_asset_models(id) ON DELETE CASCADE,
	-- We check that the `weighed_with` is indeed an asset model that is compatible with the procedure_id template.
	FOREIGN KEY (
		procedure_template_weighed_with_model_id,
		weighed_with_model_id
	) REFERENCES procedure_template_asset_models(id, asset_model_id),
	-- We enforce that the `weighed_container_model` is indeed a procedure_id asset model.
	FOREIGN KEY (
		procedure_template_weighed_container_model_id,
		weighed_container_model_id
	) REFERENCES procedure_template_asset_models(id, asset_model_id),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_weighed_container_model`
	-- for the current `procedure_template`.
	UNIQUE (
		id,
		procedure_template_weighed_container_model_id
	),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_weighed_with_model`
	-- for the current `procedure_template`.
	UNIQUE (
		id,
		procedure_template_weighed_with_model_id
	)
);
CREATE TABLE IF NOT EXISTS weighing_procedures(
	id UUID PRIMARY KEY REFERENCES procedures(id) ON DELETE CASCADE,
	-- We enforce that the model of this procedure_id must be a weighing procedure_id template.
	weighing_procedure_template_id INTEGER NOT NULL REFERENCES weighing_procedure_templates(id),
	-- The container being weighed, which must be a volumetric container model.
	weighed_container_id UUID NOT NULL REFERENCES volumetric_containers(id),
	-- The procedure_id template asset model associated to the `weighed_container`.
	procedure_template_weighed_container_model_id INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The procedure_id asset associated to the `weighed_container`.
	procedure_weighed_container_id UUID NOT NULL REFERENCES procedure_assets(id) ON DELETE CASCADE,
	-- Mass in kilograms. The measured weight, which must be strictly positive.
	mass REAL NOT NULL CHECK (mass > 0.0),
	-- The weighing device used for weighing. This field is optional as there
	-- are several situations where the weighing device is not tracked.
	weighed_with_id UUID REFERENCES weighing_devices(id),
	-- The procedure_id template asset model associated to the `weighed_with_model`.
	procedure_template_weighed_with_model_id INTEGER NOT NULL REFERENCES procedure_template_asset_models(id) ON DELETE CASCADE,
	-- The procedure_id asset associated to the `weighed_with_model`.
	procedure_weighed_with_id UUID NOT NULL REFERENCES procedure_assets(id) ON DELETE CASCADE,
	-- We enforce that the extended `procedure` has indeed the same `procedure_template`, making
	-- sure that the procedure_id is a weighing procedure_id without the possibility of a mistake.
	FOREIGN KEY (id, weighing_procedure_template_id) REFERENCES procedures(id, procedure_template_id),
	-- The `procedure_template_weighed_with_model` must be the same as in the `weighing_procedure_templates`.
	FOREIGN KEY (
		weighing_procedure_template_id,
		procedure_template_weighed_container_model_id
	) REFERENCES weighing_procedure_templates(
		id,
		procedure_template_weighed_container_model_id
	),
	-- The `procedure_template_weighed_container_model` must be the same as in the `weighing_procedure_templates`.
	FOREIGN KEY (
		weighing_procedure_template_id,
		procedure_template_weighed_with_model_id
	) REFERENCES weighing_procedure_templates(
		id,
		procedure_template_weighed_with_model_id
	),
	-- We check that the `procedure_weighed_container` is associated to the `weighed_container`.
	FOREIGN KEY (procedure_weighed_container_id, weighed_container_id) REFERENCES procedure_assets(id, asset_id),
	-- We check that the `procedure_weighed_with` is associated to the `procedure_template_weighed_container_model`.
	FOREIGN KEY (
		procedure_weighed_container_id,
		procedure_template_weighed_container_model_id
	) REFERENCES procedure_assets(id, procedure_template_asset_model_id),
	-- We check that the `procedure_weighed_with` is associated to the `procedure_template_weighed_with_model`.
	FOREIGN KEY (
		procedure_weighed_with_id,
		procedure_template_weighed_with_model_id
	) REFERENCES procedure_assets(id, procedure_template_asset_model_id),
	-- We check that the `procedure_weighed_with` is associated to the `weighed_with` asset (if any).
	FOREIGN KEY (procedure_weighed_with_id, weighed_with_id) REFERENCES procedure_assets(id, asset_id)
);