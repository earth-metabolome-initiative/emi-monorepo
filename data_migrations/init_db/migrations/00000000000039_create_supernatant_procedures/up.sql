CREATE TABLE IF NOT EXISTS supernatant_procedure_templates (
	procedure_template INTEGER PRIMARY KEY REFERENCES procedure_templates(procedure_template) ON DELETE CASCADE,
	-- The amount of liters that should be transferred
	liters REAL NOT NULL CHECK (must_be_strictly_positive_f32(liters)),
	-- The source container from which the supernatant is taken.
	stratified_source_model INTEGER NOT NULL REFERENCES volumetric_container_models(id),
	foreign_procedure_template INTEGER NOT NULL REFERENCES procedure_templates(procedure_template),
	procedure_template_stratified_source_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- Destination container to which the supernatant is transferred.
	supernatant_destination_model INTEGER NOT NULL REFERENCES volumetric_container_models(id),
	procedure_template_supernatant_destination_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id) ON DELETE CASCADE,
	-- The device used for the aliquoting procedure.
	transferred_with_model INTEGER NOT NULL REFERENCES pipette_models(id),
	procedure_template_transferred_with_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id) ON DELETE CASCADE,
	-- The pipette tip to be mounted on the pipette.
	pipette_tip_model INTEGER NOT NULL REFERENCES pipette_tip_models(id),
	procedure_template_pipette_tip_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id) ON DELETE CASCADE,
	-- We check that the `procedure_template_transferred_with_model` is indeed a trackable that is compatible with the procedure template.
	FOREIGN KEY (
		procedure_template_transferred_with_model,
		procedure_template
	) REFERENCES procedure_template_asset_models(id, procedure_template),
	FOREIGN KEY (
		procedure_template_transferred_with_model,
		transferred_with_model
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
	-- We check that the `transferred_with_model` is compatible with the `pipette_tip_model`.
	CONSTRAINT supernatant_pm_compatibility_rules FOREIGN KEY (transferred_with_model, pipette_tip_model) REFERENCES asset_compatibility_rules(left_asset_model, right_asset_model) ON DELETE CASCADE,
	-- We check that the `procedure_template_stratified_source_model` is indeed a trackable that is compatible with the procedure template.
	FOREIGN KEY (
		procedure_template_stratified_source_model,
		foreign_procedure_template
	) REFERENCES procedure_template_asset_models(id, procedure_template),
	FOREIGN KEY (
		procedure_template_stratified_source_model,
		stratified_source_model
	) REFERENCES procedure_template_asset_models(id, asset_model),
	-- We check that the `procedure_template_supernatant_destination_model` is indeed a trackable that is compatible with the procedure template.
	FOREIGN KEY (
		procedure_template_supernatant_destination_model,
		procedure_template
	) REFERENCES procedure_template_asset_models(id, procedure_template),
	FOREIGN KEY (
		procedure_template_supernatant_destination_model,
		supernatant_destination_model
	) REFERENCES procedure_template_asset_models(id, asset_model),
	-- We define the same-as index to allow for foreign key references to check wether a
	-- `supernatant_procedure_template` is associated with a given
	-- `supernatant_foreign_procedure_template`.
	UNIQUE (procedure_template, foreign_procedure_template)
);
CREATE TABLE IF NOT EXISTS supernatant_procedures (
	procedure UUID PRIMARY KEY REFERENCES procedures(procedure) ON DELETE CASCADE,
	-- We enforce that the model of this procedure must be a supernatant procedure template.
	procedure_template INTEGER NOT NULL REFERENCES supernatant_procedure_templates(procedure_template),
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
	-- The source container from which the supernatant is taken.
	stratified_source UUID NOT NULL REFERENCES volumetric_containers(id),
	-- The destination container to which the supernatant is transferred.
	supernatant_destination UUID NOT NULL REFERENCES volumetric_containers(id),
	-- The device used for the aliquoting procedure.
	transferred_with UUID NOT NULL REFERENCES pipettes(id),
	-- The pipette tip to be mounted on the pipette.
	pipette_tip_model INTEGER NOT NULL REFERENCES pipette_tip_models(id),
	-- We enforce that the extended `procedure` has indeed the same `procedure_template`, making
	-- sure that the procedure is a supernatant procedure without the possibility of a mistake.
	FOREIGN KEY (procedure, procedure_template) REFERENCES procedures(procedure, procedure_template),
	-- We enforce that there exists a `procedure_assets` entry for the stratified source in the procedure.
	FOREIGN KEY (foreign_procedure, stratified_source) REFERENCES procedure_assets(procedure, asset) ON DELETE CASCADE,
	-- We enforce that there exists a `procedure_assets` entry for the supernatant destination in the procedure.
	FOREIGN KEY (procedure, supernatant_destination) REFERENCES procedure_assets(procedure, asset) ON DELETE CASCADE,
	-- We enforce that there exists a `procedure_assets` entry for the pipette used in the procedure.
	FOREIGN KEY (procedure, transferred_with) REFERENCES procedure_assets(procedure, asset) ON DELETE CASCADE,
	-- We enforce that there exists a `procedure_assets` entry for the pipette tip used in the procedure.
	FOREIGN KEY (procedure, pipette_tip_model) REFERENCES procedure_assets(procedure, asset_model) ON DELETE CASCADE,
	-- We enforce that the `foreign_procedure` has as `procedure_template` the specified `foreign_procedure_template`.
	FOREIGN KEY (foreign_procedure, foreign_procedure_template) REFERENCES procedures(procedure, procedure_template),
	-- We enforce that the associated procedure template requires the provided foreign procedure template.
	FOREIGN KEY (procedure_template, foreign_procedure_template) REFERENCES supernatant_procedure_templates(procedure_template, foreign_procedure_template)
);