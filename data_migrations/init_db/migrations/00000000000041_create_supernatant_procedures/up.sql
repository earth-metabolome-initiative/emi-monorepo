CREATE TABLE IF NOT EXISTS supernatant_procedure_templates (
	id INTEGER PRIMARY KEY REFERENCES procedure_templates(id) ON DELETE CASCADE,
	-- Volume in liters. The amount that should be transferred.
	volume REAL NOT NULL CHECK (volume > 0.0),
	-- The source container from which the supernatant is taken.
	stratified_source_model INTEGER NOT NULL REFERENCES volumetric_container_models(id),
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
	FOREIGN KEY (
		procedure_template_transferred_with_model,
		transferred_with_model
	) REFERENCES procedure_template_asset_models(id, asset_model),
	FOREIGN KEY (
		procedure_template_pipette_tip_model,
		pipette_tip_model
	) REFERENCES procedure_template_asset_models(id, asset_model),
	-- We check that the `transferred_with_model` is compatible with the `pipette_tip_model`.
	CONSTRAINT supernatant_pm_compatibility_rules FOREIGN KEY (transferred_with_model, pipette_tip_model) REFERENCES asset_compatibility_rules(left_asset_model, right_asset_model) ON DELETE CASCADE,
	FOREIGN KEY (
		procedure_template_stratified_source_model,
		stratified_source_model
	) REFERENCES procedure_template_asset_models(id, asset_model),
	FOREIGN KEY (
		procedure_template_supernatant_destination_model,
		supernatant_destination_model
	) REFERENCES procedure_template_asset_models(id, asset_model),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_stratified_source_model`
	-- for the current `procedure_template`.
	UNIQUE (
		id,
		procedure_template_stratified_source_model
	),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_supernatant_destination_model`
	-- for the current `procedure_template`.
	UNIQUE (
		id,
		procedure_template_supernatant_destination_model
	),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_transferred_with_model`
	-- for the current `procedure_template`.
	UNIQUE (
		id,
		procedure_template_transferred_with_model
	),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_pipette_tip_model`
	-- for the current `procedure_template`.
	UNIQUE (
		id,
		procedure_template_pipette_tip_model
	)
);
CREATE TABLE IF NOT EXISTS supernatant_procedures (
	id UUID PRIMARY KEY REFERENCES procedures(id) ON DELETE CASCADE,
	-- We enforce that the model of this procedure must be a supernatant procedure template.
	supernatant_procedure_template INTEGER NOT NULL REFERENCES supernatant_procedure_templates(id),
	-- The source container from which the supernatant is taken.
	stratified_source UUID NOT NULL REFERENCES volumetric_containers(id),
	-- The procedure template asset model associated to the `stratified_source`.
	procedure_template_stratified_source_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The procedure asset associated to the `stratified_source`.
	procedure_stratified_source UUID NOT NULL REFERENCES procedure_assets(id) ON DELETE CASCADE,
	-- The destination container to which the supernatant is transferred.
	supernatant_destination UUID NOT NULL REFERENCES volumetric_containers(id),
	-- The procedure template asset model associated to the `supernatant_destination`.
	procedure_template_supernatant_destination_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The procedure asset associated to the `supernatant_destination`.
	procedure_supernatant_destination UUID NOT NULL REFERENCES procedure_assets(id) ON DELETE CASCADE,
	-- The device used for the aliquoting procedure.
	transferred_with UUID NOT NULL REFERENCES pipettes(id),
	-- The model of the device used for the aliquoting procedure.
	transferred_with_model INTEGER NOT NULL REFERENCES pipette_models(id),
	-- The procedure template asset model associated to the `transferred_with`.
	procedure_template_transferred_with_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The procedure asset associated to the `transferred_with`.
	procedure_transferred_with UUID NOT NULL REFERENCES procedure_assets(id) ON DELETE CASCADE,
	-- The pipette tip to be mounted on the pipette.
	pipette_tip_model INTEGER NOT NULL REFERENCES pipette_tip_models(id),
	-- The procedure template asset model associated to the `pipette_tip_model`.
	procedure_template_pipette_tip_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The procedure asset associated to the `pipette_tip_model`.
	procedure_pipette_tip UUID NOT NULL REFERENCES procedure_assets(id) ON DELETE CASCADE,
	-- We enforce that the extended `procedure` has indeed the same `procedure_template`, making
	-- sure that the procedure is a supernatant procedure without the possibility of a mistake.
	FOREIGN KEY (id, supernatant_procedure_template) REFERENCES procedures(id, procedure_template),
	-- The `procedure_template_stratified_source_model` must be the same as in the `supernatant_procedure_templates`.
	FOREIGN KEY (
		supernatant_procedure_template,
		procedure_template_stratified_source_model
	) REFERENCES supernatant_procedure_templates(
		id,
		procedure_template_stratified_source_model
	),
	-- The `procedure_template_supernatant_destination_model` must be the same as in the `supernatant_procedure_templates`.
	FOREIGN KEY (
		supernatant_procedure_template,
		procedure_template_supernatant_destination_model
	) REFERENCES supernatant_procedure_templates(
		id,
		procedure_template_supernatant_destination_model
	),
	-- The `procedure_template_transferred_with_model` must be the same as in the `supernatant_procedure_templates`.
	FOREIGN KEY (
		supernatant_procedure_template,
		procedure_template_transferred_with_model
	) REFERENCES supernatant_procedure_templates(
		id,
		procedure_template_transferred_with_model
	),
	-- The `procedure_template_pipette_tip_model` must be the same as in the `supernatant_procedure_templates`.
	FOREIGN KEY (
		supernatant_procedure_template,
		procedure_template_pipette_tip_model
	) REFERENCES supernatant_procedure_templates(
		id,
		procedure_template_pipette_tip_model
	),
	-- We check that the `procedure_stratified_source` is associated to the `stratified_source`.
	FOREIGN KEY (procedure_stratified_source, stratified_source) REFERENCES procedure_assets(id, asset),
	-- We check that the `procedure_supernatant_destination` is associated to the `supernatant_destination`.
	FOREIGN KEY (
		procedure_supernatant_destination,
		supernatant_destination
	) REFERENCES procedure_assets(id, asset),
	-- We check that the `procedure_transferred_with` is associated to the `transferred_with_model`.
	FOREIGN KEY (procedure_transferred_with, transferred_with) REFERENCES procedure_assets(id, asset),
	-- We check that the `procedure_pipette_tip` is associated to the `pipette_tip_model`.
	FOREIGN KEY (procedure_pipette_tip, pipette_tip_model) REFERENCES procedure_assets(id, asset_model),
	-- We check that the `procedure_stratified_source` is indeed associated to the `procedure_template_stratified_source_model`.
	FOREIGN KEY (
		procedure_stratified_source,
		procedure_template_stratified_source_model
	) REFERENCES procedure_assets(id, procedure_template_asset_model),
	-- We check that the `procedure_supernatant_destination` is indeed associated to the `procedure_template_supernatant_destination_model`.
	FOREIGN KEY (
		procedure_supernatant_destination,
		procedure_template_supernatant_destination_model
	) REFERENCES procedure_assets(id, procedure_template_asset_model),
	-- We check that the `procedure_transferred_with` is indeed associated to the `procedure_template_transferred_with_model`.
	FOREIGN KEY (
		procedure_transferred_with,
		procedure_template_transferred_with_model
	) REFERENCES procedure_assets(id, procedure_template_asset_model),
	-- We check that the `procedure_pipette_tip` is indeed associated to the `procedure_template_pipette_tip_model`.
	FOREIGN KEY (
		procedure_pipette_tip,
		procedure_template_pipette_tip_model
	) REFERENCES procedure_assets(id, procedure_template_asset_model),
	-- We check that the `transferred_with_model` is compatible with the `pipette_tip_model`.
	FOREIGN KEY (transferred_with_model, pipette_tip_model) REFERENCES asset_compatibility_rules(left_asset_model, right_asset_model),
	-- We check that the `procedure_transferred_with` is associated to the `transferred_with_model`.
	FOREIGN KEY (
		procedure_transferred_with,
		transferred_with_model
	) REFERENCES procedure_assets(id, asset_model)
);