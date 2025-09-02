CREATE TABLE IF NOT EXISTS capping_procedure_templates (
	procedure_template INTEGER PRIMARY KEY REFERENCES procedure_templates(procedure_template) ON DELETE CASCADE,
	-- The container to be capped.
	container_model INTEGER NOT NULL REFERENCES volumetric_container_models(id),
	-- The procedure template which has populated the container to be capped (e.g., a sampling or fractioning procedure template).
	foreign_procedure_template INTEGER NOT NULL REFERENCES procedure_templates(procedure_template) CHECK (
		must_be_distinct_i32(
			procedure_template,
			foreign_procedure_template
		)
	),
	procedure_template_container_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The cap to be used for the container.
	capped_with_model INTEGER NOT NULL REFERENCES caps_models(id),
	procedure_template_capped_with_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id) ON DELETE CASCADE,
	-- We check that the `procedure_template_container_model` is indeed a trackable that is compatible with the procedure template.
	FOREIGN KEY (
		procedure_template_container_model,
		foreign_procedure_template
	) REFERENCES procedure_template_asset_models(id, procedure_template),
	-- We check that the `procedure_template_capped_with_model` is indeed a trackable that is compatible with the procedure template.
	FOREIGN KEY (
		procedure_template_capped_with_model,
		procedure_template
	) REFERENCES procedure_template_asset_models(id, procedure_template),
	-- We check that the `container_model` is indeed the trackable that is compatible with the procedure template.
	FOREIGN KEY (
		procedure_template_container_model,
		container_model
	) REFERENCES procedure_template_asset_models(id, asset_model),
	-- We check that the `capped_with_model` is indeed a trackable that is compatible with the procedure template.
	FOREIGN KEY (
		procedure_template_capped_with_model,
		capped_with_model
	) REFERENCES procedure_template_asset_models(id, asset_model),
	-- We check that the `capped_with_model` is indeed a cap that can be used with the `container_model`.
	CONSTRAINT capping_pm_compatibility_rules FOREIGN KEY (container_model, capped_with_model) REFERENCES asset_compatibility_rules(left_asset_model, right_asset_model),
	-- We define a same-as index to allow for foreign key references to check whether a `capping_procedure_template`
	-- is associated with a given `capping_foreign_procedure_template`.
	UNIQUE (procedure_template, foreign_procedure_template)
);
CREATE TABLE IF NOT EXISTS capping_procedures (
	-- Identifier of the capping procedure, which is also a foreign key to the general procedure.
	procedure UUID PRIMARY KEY REFERENCES procedures(procedure) ON DELETE CASCADE,
	-- We enforce that the model of this procedure must be a capping procedure template.
	procedure_template INTEGER NOT NULL REFERENCES capping_procedure_templates(procedure_template),
	-- The procedure template associated with the foreign procedure template.
	foreign_procedure_template INTEGER NOT NULL REFERENCES procedure_templates(procedure_template) CHECK (
		must_be_distinct_i32(
			procedure_template,
			foreign_procedure_template
		)
	),
	-- The foreign procedure that has populated the container being capped (e.g., a sampling or fractioning procedure).
	foreign_procedure UUID NOT NULL REFERENCES procedures(procedure) CHECK (
		must_be_distinct_uuid(procedure, foreign_procedure)
	),
	-- The container being capped, which must be a volumetric container.
	capped_container UUID NOT NULL REFERENCES volumetric_containers(id),
	-- The cap being used, which must be a cap model.
	capped_with_model INTEGER NOT NULL REFERENCES caps_models(id),
	-- We ensure that the parent table's procedure_template is indeed a capping_
	FOREIGN KEY (procedure, procedure_template) REFERENCES procedures(procedure, procedure_template),
	-- We enforce that the `foreign_procedure` has as `procedure_template` the specified `foreign_procedure_template`.
	FOREIGN KEY (foreign_procedure, foreign_procedure_template) REFERENCES procedures(procedure, procedure_template),
	-- The disposed asset must be a procedure asset of the correct model.
	FOREIGN KEY (foreign_procedure, capped_container) REFERENCES procedure_assets(procedure, asset),
	-- We enforce that the `capped_with_model` is an asset model associated with the current procedure.
	FOREIGN KEY (procedure, capped_with_model) REFERENCES procedure_assets(procedure, asset_model),
	-- We enforce that the associated procedure template requires the provided foreign procedure template.
	FOREIGN KEY (procedure_template, foreign_procedure_template) REFERENCES capping_procedure_templates(procedure_template, foreign_procedure_template)
);