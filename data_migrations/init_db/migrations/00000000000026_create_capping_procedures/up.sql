CREATE TABLE IF NOT EXISTS capping_procedure_templates (
	-- Identifier of the capping procedure_id template, which is also a foreign key to the general procedure_id template.
	id INTEGER PRIMARY KEY REFERENCES procedure_templates(id) ON DELETE CASCADE,
	-- The container to be capped.
	capped_container_model_id INTEGER NOT NULL REFERENCES volumetric_container_models(id),
	-- The procedure_id template associated with the container model.
	procedure_template_capped_container_model_id INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The cap to be used for the container.
	capped_with_model_id INTEGER NOT NULL REFERENCES cap_models(id),
	-- The procedure_id template associated with the cap model.
	procedure_template_capped_with_model_id INTEGER NOT NULL REFERENCES procedure_template_asset_models(id) ON DELETE CASCADE,
	-- We check that the `container_model` is indeed the trackable that is compatible with the procedure_id template.
	FOREIGN KEY (
		procedure_template_capped_container_model_id,
		capped_container_model_id
	) REFERENCES procedure_template_asset_models(id, asset_model_id),
	-- We check that the `capped_with_model` is indeed a trackable that is compatible with the procedure_id template.
	FOREIGN KEY (
		procedure_template_capped_with_model_id,
		capped_with_model_id
	) REFERENCES procedure_template_asset_models(id, asset_model_id),
	-- We check that the `capped_with_model` is indeed a cap that can be used with the `capped_container_model`.
	FOREIGN KEY (capped_container_model_id, capped_with_model_id) REFERENCES asset_compatibility_rules(left_asset_model_id, right_asset_model_id),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_capped_container_model`
	-- for the current `procedure_template`.
	UNIQUE (
		id,
		procedure_template_capped_container_model_id
	),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_capped_with_model`
	-- for the current `procedure_template`.
	UNIQUE (
		id,
		procedure_template_capped_with_model_id
	)
);
CREATE TABLE IF NOT EXISTS capping_procedures (
	-- Identifier of the capping id, which is also a foreign key to the general procedure.
	id UUID PRIMARY KEY REFERENCES procedures(id) ON DELETE CASCADE,
	-- We enforce that the model of this procedure_id must be a capping procedure_id template.
	capping_procedure_template_id INTEGER NOT NULL REFERENCES capping_procedure_templates(id),
	-- The container being capped, which must be a volumetric container.
	capped_container_id UUID NOT NULL REFERENCES volumetric_containers(id),
	-- The model of the container being capped.
	capped_container_model_id INTEGER NOT NULL REFERENCES volumetric_container_models(id),
	-- The procedure_id template asset model describing the `capped_container`.
	procedure_template_capped_container_model_id INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The procedure_id asset describing the `capped_container`.
	procedure_capped_container_id UUID NOT NULL REFERENCES procedure_assets(id),
	-- The cap being used, which must be a cap model.
	capped_with_model_id INTEGER NOT NULL REFERENCES cap_models(id),
	-- The procedure_id template asset model describing the `capped_with_model`.
	procedure_template_capped_with_model_id INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The procedure_id asset describing the `capped_with_model`.
	procedure_capped_with_id UUID NOT NULL REFERENCES procedure_assets(id),
	-- The current procedure_id must be a capping procedure.
	FOREIGN KEY (id, capping_procedure_template_id) REFERENCES procedures(id, procedure_template_id),
	-- The procedure_id template asset model describing the `capped_container` must be the same one
	-- as the one in the procedure_id template.
	FOREIGN KEY (
		capping_procedure_template_id,
		procedure_template_capped_container_model_id
	) REFERENCES capping_procedure_templates(
		id,
		procedure_template_capped_container_model_id
	),
	-- The procedure_id template asset model describing the `capped_with_model` must be the same one
	-- as the one in the procedure_id template.
	FOREIGN KEY (
		capping_procedure_template_id,
		procedure_template_capped_with_model_id
	) REFERENCES capping_procedure_templates(
		id,
		procedure_template_capped_with_model_id
	),
	-- The procedure_id template asset model and the procedure_id asset describing the `capped_container`
	-- must be compatible.
	FOREIGN KEY (
		procedure_capped_container_id,
		procedure_template_capped_container_model_id
	) REFERENCES procedure_assets(id, procedure_template_asset_model_id),
	-- The procedure_id template asset model and the procedure_id asset describing the `capped_with_model`
	-- must be compatible.
	FOREIGN KEY (
		procedure_capped_with_id,
		procedure_template_capped_with_model_id
	) REFERENCES procedure_assets(id, procedure_template_asset_model_id),
	-- We ensure that the `procedure_capped_container` is associated with the `capped_container_model`.
	FOREIGN KEY (
		procedure_capped_container_id,
		capped_container_model_id
	) REFERENCES procedure_assets(id, asset_model_id),
	-- We ensure that the `procedure_capped_with` is associated with the `capped_with_model`.
	FOREIGN KEY (procedure_capped_with_id, capped_with_model_id) REFERENCES procedure_assets(id, asset_model_id),
	-- We ensure that the `procedure_capped_container` is associated with the `capped_container`.
	FOREIGN KEY (procedure_capped_container_id, capped_container_id) REFERENCES procedure_assets(id, asset_id),
	-- We check that the `capped_container_model` is compatible with the `capped_with_model`.
	FOREIGN KEY (capped_container_model_id, capped_with_model_id) REFERENCES asset_compatibility_rules(left_asset_model_id, right_asset_model_id)
);