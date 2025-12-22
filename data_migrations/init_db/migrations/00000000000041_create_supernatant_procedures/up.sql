CREATE TABLE IF NOT EXISTS supernatant_procedure_templates (
	id INTEGER PRIMARY KEY REFERENCES procedure_templates(id) ON DELETE CASCADE,
	-- Volume in liters. The amount that should be transferred.
	volume REAL NOT NULL CHECK (volume > 0.0),
	-- The source container from which the supernatant is taken.
	stratified_source_model_id INTEGER NOT NULL REFERENCES volumetric_container_models(id),
	procedure_template_stratified_source_model_id INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- Destination container to which the supernatant is transferred.
	supernatant_destination_model_id INTEGER NOT NULL REFERENCES volumetric_container_models(id),
	procedure_template_supernatant_destination_model_id INTEGER NOT NULL REFERENCES procedure_template_asset_models(id) ON DELETE CASCADE,
	-- The device used for the aliquoting procedure.
	transferred_with_model_id INTEGER NOT NULL REFERENCES pipette_models(id),
	procedure_template_transferred_with_model_id INTEGER NOT NULL REFERENCES procedure_template_asset_models(id) ON DELETE CASCADE,
	-- The pipette tip to be mounted on the pipette.
	pipette_tip_model_id INTEGER NOT NULL REFERENCES pipette_tip_models(id),
	procedure_template_pipette_tip_model_id INTEGER NOT NULL REFERENCES procedure_template_asset_models(id) ON DELETE CASCADE,
	FOREIGN KEY (
		procedure_template_transferred_with_model_id,
		transferred_with_model_id
	) REFERENCES procedure_template_asset_models(id, asset_model_id),
	FOREIGN KEY (
		procedure_template_pipette_tip_model_id,
		pipette_tip_model_id
	) REFERENCES procedure_template_asset_models(id, asset_model_id),
	-- We check that the `transferred_with_model` is compatible with the `pipette_tip_model_id`.
	CONSTRAINT supernatant_pm_compatibility_rules FOREIGN KEY (transferred_with_model_id, pipette_tip_model_id) REFERENCES asset_compatibility_rules(left_asset_model_id, right_asset_model_id) ON DELETE CASCADE,
	FOREIGN KEY (
		procedure_template_stratified_source_model_id,
		stratified_source_model_id
	) REFERENCES procedure_template_asset_models(id, asset_model_id),
	FOREIGN KEY (
		procedure_template_supernatant_destination_model_id,
		supernatant_destination_model_id
	) REFERENCES procedure_template_asset_models(id, asset_model_id),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_stratified_source_model`
	-- for the current `procedure_template`.
	UNIQUE (
		id,
		procedure_template_stratified_source_model_id
	),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_supernatant_destination_model`
	-- for the current `procedure_template`.
	UNIQUE (
		id,
		procedure_template_supernatant_destination_model_id
	),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_transferred_with_model`
	-- for the current `procedure_template`.
	UNIQUE (
		id,
		procedure_template_transferred_with_model_id
	),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_pipette_tip_model_id`
	-- for the current `procedure_template`.
	UNIQUE (
		id,
		procedure_template_pipette_tip_model_id
	)
);
CREATE TABLE IF NOT EXISTS supernatant_procedures (
	id UUID PRIMARY KEY REFERENCES procedures(id) ON DELETE CASCADE,
	-- We enforce that the model of this procedure_id must be a supernatant procedure_id template.
	supernatant_procedure_template_id INTEGER NOT NULL REFERENCES supernatant_procedure_templates(id),
	-- The source container from which the supernatant is taken.
	stratified_source_id UUID NOT NULL REFERENCES volumetric_containers(id),
	-- The procedure_id template asset model associated to the `stratified_source`.
	procedure_template_stratified_source_model_id INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The procedure_id asset associated to the `stratified_source`.
	procedure_stratified_source_id UUID NOT NULL REFERENCES procedure_assets(id) ON DELETE CASCADE,
	-- The destination container to which the supernatant is transferred.
	supernatant_destination_id UUID NOT NULL REFERENCES volumetric_containers(id),
	-- The procedure_id template asset model associated to the `supernatant_destination`.
	procedure_template_supernatant_destination_model_id INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The procedure_id asset associated to the `supernatant_destination`.
	procedure_supernatant_destination_id UUID NOT NULL REFERENCES procedure_assets(id) ON DELETE CASCADE,
	-- The device used for the aliquoting procedure.
	transferred_with_id UUID NOT NULL REFERENCES pipettes(id),
	-- The model of the device used for the aliquoting procedure.
	transferred_with_model_id INTEGER NOT NULL REFERENCES pipette_models(id),
	-- The procedure_id template asset model associated to the `transferred_with`.
	procedure_template_transferred_with_model_id INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The procedure_id asset associated to the `transferred_with`.
	procedure_transferred_with_id UUID NOT NULL REFERENCES procedure_assets(id) ON DELETE CASCADE,
	-- The pipette tip to be mounted on the pipette.
	pipette_tip_model_id INTEGER NOT NULL REFERENCES pipette_tip_models(id),
	-- The procedure_id template asset model associated to the `pipette_tip_model_id`.
	procedure_template_pipette_tip_model_id INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The procedure_id asset associated to the `pipette_tip_model_id`.
	procedure_pipette_tip_id UUID NOT NULL REFERENCES procedure_assets(id) ON DELETE CASCADE,
	-- We enforce that the extended `procedure` has indeed the same `procedure_template`, making
	-- sure that the procedure_id is a supernatant procedure_id without the possibility of a mistake.
	FOREIGN KEY (id, supernatant_procedure_template_id) REFERENCES procedures(id, procedure_template_id),
	-- The `procedure_template_stratified_source_model` must be the same as in the `supernatant_procedure_templates`.
	FOREIGN KEY (
		supernatant_procedure_template_id,
		procedure_template_stratified_source_model_id
	) REFERENCES supernatant_procedure_templates(
		id,
		procedure_template_stratified_source_model_id
	),
	-- The `procedure_template_supernatant_destination_model` must be the same as in the `supernatant_procedure_templates`.
	FOREIGN KEY (
		supernatant_procedure_template_id,
		procedure_template_supernatant_destination_model_id
	) REFERENCES supernatant_procedure_templates(
		id,
		procedure_template_supernatant_destination_model_id
	),
	-- The `procedure_template_transferred_with_model` must be the same as in the `supernatant_procedure_templates`.
	FOREIGN KEY (
		supernatant_procedure_template_id,
		procedure_template_transferred_with_model_id
	) REFERENCES supernatant_procedure_templates(
		id,
		procedure_template_transferred_with_model_id
	),
	-- The `procedure_template_pipette_tip_model_id` must be the same as in the `supernatant_procedure_templates`.
	FOREIGN KEY (
		supernatant_procedure_template_id,
		procedure_template_pipette_tip_model_id
	) REFERENCES supernatant_procedure_templates(
		id,
		procedure_template_pipette_tip_model_id
	),
	-- We check that the `procedure_stratified_source` is associated to the `stratified_source`.
	FOREIGN KEY (procedure_stratified_source_id, stratified_source_id) REFERENCES procedure_assets(id, asset_id),
	-- We check that the `procedure_supernatant_destination` is associated to the `supernatant_destination`.
	FOREIGN KEY (
		procedure_supernatant_destination_id,
		supernatant_destination_id
	) REFERENCES procedure_assets(id, asset_id),
	-- We check that the `procedure_transferred_with` is associated to the `transferred_with_model`.
	FOREIGN KEY (procedure_transferred_with_id, transferred_with_id) REFERENCES procedure_assets(id, asset_id),
	-- We check that the `procedure_pipette_tip_id` is associated to the `pipette_tip_model_id`.
	FOREIGN KEY (procedure_pipette_tip_id, pipette_tip_model_id) REFERENCES procedure_assets(id, asset_model_id),
	-- We check that the `procedure_stratified_source` is indeed associated to the `procedure_template_stratified_source_model`.
	FOREIGN KEY (
		procedure_stratified_source_id,
		procedure_template_stratified_source_model_id
	) REFERENCES procedure_assets(id, procedure_template_asset_model_id),
	-- We check that the `procedure_supernatant_destination` is indeed associated to the `procedure_template_supernatant_destination_model`.
	FOREIGN KEY (
		procedure_supernatant_destination_id,
		procedure_template_supernatant_destination_model_id
	) REFERENCES procedure_assets(id, procedure_template_asset_model_id),
	-- We check that the `procedure_transferred_with` is indeed associated to the `procedure_template_transferred_with_model`.
	FOREIGN KEY (
		procedure_transferred_with_id,
		procedure_template_transferred_with_model_id
	) REFERENCES procedure_assets(id, procedure_template_asset_model_id),
	-- We check that the `procedure_pipette_tip_id` is indeed associated to the `procedure_template_pipette_tip_model_id`.
	FOREIGN KEY (
		procedure_pipette_tip_id,
		procedure_template_pipette_tip_model_id
	) REFERENCES procedure_assets(id, procedure_template_asset_model_id),
	-- We check that the `transferred_with_model` is compatible with the `pipette_tip_model_id`.
	FOREIGN KEY (transferred_with_model_id, pipette_tip_model_id) REFERENCES asset_compatibility_rules(left_asset_model_id, right_asset_model_id),
	-- We check that the `procedure_transferred_with` is associated to the `transferred_with_model`.
	FOREIGN KEY (
		procedure_transferred_with_id,
		transferred_with_model_id
	) REFERENCES procedure_assets(id, asset_model_id)
);