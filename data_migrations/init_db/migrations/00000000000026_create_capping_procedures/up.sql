CREATE TABLE IF NOT EXISTS capping_procedure_templates (
	-- Identifier of the capping procedure template, which is also a foreign key to the general procedure template.
	id INTEGER PRIMARY KEY REFERENCES procedure_templates(id) ON DELETE CASCADE,
	-- The container to be capped.
	capped_container_model INTEGER NOT NULL REFERENCES volumetric_container_models(id),
	-- The procedure template associated with the container model.
	procedure_template_capped_container_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The cap to be used for the container.
	capped_with_model INTEGER NOT NULL REFERENCES cap_models(id),
	-- The procedure template associated with the cap model.
	procedure_template_capped_with_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id) ON DELETE CASCADE,
	-- We check that the `container_model` is indeed the trackable that is compatible with the procedure template.
	FOREIGN KEY (
		procedure_template_capped_container_model,
		capped_container_model
	) REFERENCES procedure_template_asset_models(id, asset_model),
	-- We check that the `capped_with_model` is indeed a trackable that is compatible with the procedure template.
	FOREIGN KEY (
		procedure_template_capped_with_model,
		capped_with_model
	) REFERENCES procedure_template_asset_models(id, asset_model),
	-- We check that the `capped_with_model` is indeed a cap that can be used with the `capped_container_model`.
	FOREIGN KEY (capped_container_model, capped_with_model) REFERENCES asset_compatibility_rules(left_asset_model, right_asset_model),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_capped_container_model`
	-- for the current `procedure_template`.
	UNIQUE (
		id,
		procedure_template_capped_container_model
	),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_capped_with_model`
	-- for the current `procedure_template`.
	UNIQUE (
		id,
		procedure_template_capped_with_model
	)
);
CREATE TABLE IF NOT EXISTS capping_procedures (
	-- Identifier of the capping id, which is also a foreign key to the general procedure.
	id UUID PRIMARY KEY REFERENCES procedures(id) ON DELETE CASCADE,
	-- We enforce that the model of this procedure must be a capping procedure template.
	capping_procedure_template INTEGER NOT NULL REFERENCES capping_procedure_templates(id),
	-- The container being capped, which must be a volumetric container.
	capped_container UUID NOT NULL REFERENCES volumetric_containers(id),
	-- The model of the container being capped.
	capped_container_model INTEGER NOT NULL REFERENCES volumetric_container_models(id),
	-- The procedure template asset model describing the `capped_container`.
	procedure_template_capped_container_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The procedure asset describing the `capped_container`.
	procedure_capped_container UUID NOT NULL REFERENCES procedure_assets(id),
	-- The cap being used, which must be a cap model.
	capped_with_model INTEGER NOT NULL REFERENCES cap_models(id),
	-- The procedure template asset model describing the `capped_with_model`.
	procedure_template_capped_with_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The procedure asset describing the `capped_with_model`.
	procedure_capped_with UUID NOT NULL REFERENCES procedure_assets(id),
	-- The current procedure must be a capping procedure.
	FOREIGN KEY (id, capping_procedure_template) REFERENCES procedures(id, procedure_template),
	-- The procedure template asset model describing the `capped_container` must be the same one
	-- as the one in the procedure template.
	FOREIGN KEY (
		capping_procedure_template,
		procedure_template_capped_container_model
	) REFERENCES capping_procedure_templates(
		id,
		procedure_template_capped_container_model
	),
	-- The procedure template asset model describing the `capped_with_model` must be the same one
	-- as the one in the procedure template.
	FOREIGN KEY (
		capping_procedure_template,
		procedure_template_capped_with_model
	) REFERENCES capping_procedure_templates(
		id,
		procedure_template_capped_with_model
	),
	-- The procedure template asset model and the procedure asset describing the `capped_container`
	-- must be compatible.
	FOREIGN KEY (
		procedure_capped_container,
		procedure_template_capped_container_model
	) REFERENCES procedure_assets(id, procedure_template_asset_model),
	-- The procedure template asset model and the procedure asset describing the `capped_with_model`
	-- must be compatible.
	FOREIGN KEY (
		procedure_capped_with,
		procedure_template_capped_with_model
	) REFERENCES procedure_assets(id, procedure_template_asset_model),
	-- We ensure that the `procedure_capped_container` is associated with the `capped_container_model`.
	FOREIGN KEY (
		procedure_capped_container,
		capped_container_model
	) REFERENCES procedure_assets(id, asset_model),
	-- We ensure that the `procedure_capped_with` is associated with the `capped_with_model`.
	FOREIGN KEY (procedure_capped_with, capped_with_model) REFERENCES procedure_assets(id, asset_model),
	-- We ensure that the `procedure_capped_container` is associated with the `capped_container`.
	FOREIGN KEY (procedure_capped_container, capped_container) REFERENCES procedure_assets(id, asset),
	-- We check that the `capped_container_model` is compatible with the `capped_with_model`.
	FOREIGN KEY (capped_container_model, capped_with_model) REFERENCES asset_compatibility_rules(left_asset_model, right_asset_model)
);