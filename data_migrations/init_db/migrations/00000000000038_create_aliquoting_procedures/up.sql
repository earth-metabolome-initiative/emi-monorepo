CREATE TABLE IF NOT EXISTS aliquoting_procedure_templates (
	procedure_template INTEGER PRIMARY KEY REFERENCES procedure_templates(procedure_template) ON DELETE CASCADE,
	-- The amount of liters that should be aliquoted.
	liters REAL NOT NULL CHECK (must_be_strictly_positive_f32(liters)),
	-- Source container from which the aliquot is taken.
	aliquoted_from_model INTEGER NOT NULL REFERENCES volumetric_container_models(id),
	foreign_procedure_template INTEGER NOT NULL REFERENCES procedure_templates(procedure_template),
	procedure_template_aliquoted_from_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- Destination container to which the aliquot is transferred.
	aliquoted_into_model INTEGER NOT NULL REFERENCES volumetric_container_models(id),
	procedure_template_aliquoted_into_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id) ON DELETE CASCADE,
	-- The device used for the aliquoting procedure.
	aliquoted_with_model INTEGER NOT NULL REFERENCES pipette_models(id),
	procedure_template_aliquoted_with_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id) ON DELETE CASCADE,
	-- The pipette tip to be mounted on the pipette.
	pipette_tip_model INTEGER NOT NULL REFERENCES pipette_tip_models(id),
	procedure_template_pipette_tip_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id) ON DELETE CASCADE,
	-- We check that the `procedure_template_aliquoted_with_model` is indeed a trackable that is compatible with the procedure template.
	FOREIGN KEY (
		procedure_template_aliquoted_with_model,
		procedure_template
	) REFERENCES procedure_template_asset_models(id, procedure_template),
	FOREIGN KEY (
		procedure_template_aliquoted_with_model,
		aliquoted_with_model
	) REFERENCES procedure_template_asset_models(id, asset_model),
	-- We check that the `procedure_template_pipette_tip_model` is indeed a trackable that is compatible with the procedure template.
	FOREIGN KEY (
		procedure_template_pipette_tip_model,
		procedure_template
	) REFERENCES procedure_template_asset_models(id, procedure_template),
	FOREIGN KEY (
		procedure_template_pipette_tip_model,
		pipette_tip_model
	) REFERENCES procedure_template_asset_models(id, asset_model),
	-- We check that the `aliquoted_with_model` is compatible with the `pipette_tip_model`.
	CONSTRAINT aliquoting_pm_compatibility_rules FOREIGN KEY (aliquoted_with_model, pipette_tip_model) REFERENCES asset_compatibility_rules(left_asset_model, right_asset_model),
	-- We check that the `source` is indeed a trackable that is compatible with the procedure template.
	FOREIGN KEY (
		procedure_template_aliquoted_from_model,
		foreign_procedure_template
	) REFERENCES procedure_template_asset_models(id, procedure_template),
	FOREIGN KEY (
		procedure_template_aliquoted_from_model,
		aliquoted_from_model
	) REFERENCES procedure_template_asset_models(id, asset_model),
	-- We check that the `destination` is indeed a trackable that is compatible with the procedure template.
	FOREIGN KEY (
		procedure_template_aliquoted_into_model,
		procedure_template
	) REFERENCES procedure_template_asset_models(id, procedure_template),
	FOREIGN KEY (
		procedure_template_aliquoted_into_model,
		aliquoted_into_model
	) REFERENCES procedure_template_asset_models(id, asset_model),
	-- We define the same-as index to allow for foreign key references to check wether a
	-- `aliquoting_procedure_template` is associated with a given
	-- `aliquoting_foreign_procedure_template`.
	UNIQUE (procedure_template, foreign_procedure_template)
);
CREATE TABLE IF NOT EXISTS aliquoting_procedures (
	procedure UUID PRIMARY KEY REFERENCES procedures(procedure) ON DELETE CASCADE,
	-- We enforce that the model of this procedure must be an aliquoting procedure template.
	procedure_template INTEGER NOT NULL REFERENCES aliquoting_procedure_templates(procedure_template),
	-- The procedure template associated with the foreign procedure template.
	foreign_procedure_template INTEGER NOT NULL REFERENCES procedure_templates(procedure_template) CHECK (
		must_be_distinct_i32(
			procedure_template,
			foreign_procedure_template
		)
	),
	-- The foreign procedure that has populated the container being aliquoted from (e.g., a sampling or fractioning procedure).
	foreign_procedure UUID NOT NULL REFERENCES procedures(procedure) CHECK (
		must_be_distinct_uuid(procedure, foreign_procedure)
	),
	-- The identifier of the instrument used for aliquoting.
	aliquoted_with UUID NOT NULL REFERENCES pipettes(id),
	-- The pipette tip used for aliquoting, which must be a pipette tip model.
	pipette_tip_model INTEGER NOT NULL REFERENCES pipette_tip_models(id),
	-- The container being aliquoted, which must be a volumetric container model.
	aliquoted_from UUID NOT NULL REFERENCES volumetric_containers(id),
	-- We enforce that the extended `procedure` has indeed the same `procedure_template`, making
	-- sure that the procedure is an aliquoting procedure without the possibility of a mistake.
	FOREIGN KEY (procedure, procedure_template) REFERENCES procedures(procedure, procedure_template),
	-- We enforce that there exists a `procedure_assets` entry for the instrument used in the procedure.
	FOREIGN KEY (procedure, aliquoted_with) REFERENCES procedure_assets(procedure, asset) ON DELETE CASCADE,
	-- We enforce that there exists a `procedure_assets` entry for the pipette tip used in the procedure.
	FOREIGN KEY (procedure, pipette_tip_model) REFERENCES procedure_assets(procedure, asset_model) ON DELETE CASCADE,
	-- We enforce that the `aliquoted_from` is indeed a procedure trackable.
	FOREIGN KEY (foreign_procedure, aliquoted_from) REFERENCES procedure_assets(procedure, asset) ON DELETE CASCADE,
	-- We enforce that the `foreign_procedure` has indeed the `foreign_procedure_template`.
	FOREIGN KEY (foreign_procedure, foreign_procedure_template) REFERENCES procedures(procedure, procedure_template),
	-- We enforce that the provided `foreign_procedure_template` is the one specified in the `aliquoting_procedure_templates` table.
	FOREIGN KEY (procedure_template, foreign_procedure_template) REFERENCES aliquoting_procedure_templates(procedure_template, foreign_procedure_template)
);