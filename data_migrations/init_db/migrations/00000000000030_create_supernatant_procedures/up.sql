CREATE TABLE IF NOT EXISTS procedure_models.supernatant_procedure_models (
	procedure_model_id INTEGER PRIMARY KEY REFERENCES procedure_models.procedure_models(id),
	-- The amount of liters that should be transferred
	liters REAL NOT NULL CHECK (must_be_strictly_positive_f32(liters)),
	-- The source container from which the supernatant is taken.
	stratified_source UUID NOT NULL REFERENCES volumetric_container_models(id),
	procedure_stratified_source INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- Destination container to which the supernatant is transferred.
	supernatant_destination UUID NOT NULL REFERENCES volumetric_container_models(id),
	procedure_supernatant_destination INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- The device used for the aliquoting procedure.
	transferred_with UUID NOT NULL REFERENCES pipette_models(id),
	procedure_transferred_with INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- The pipette tip to be mounted on the pipette.
	pipette_tip UUID NOT NULL REFERENCES pipette_tip_models(id),
	procedure_pipette_tip INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- We check that the `procedure_transferred_with` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (procedure_transferred_with, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	FOREIGN KEY (procedure_transferred_with, transferred_with) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE,
	-- We check that the `procedure_pipette_tip` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (procedure_pipette_tip, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	FOREIGN KEY (procedure_pipette_tip, pipette_tip) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE,
	-- We check that the `transferred_with` is compatible with the `pipette_tip`.
	CONSTRAINT supernatant_pm_compatibility_rules FOREIGN KEY (transferred_with, pipette_tip) REFERENCES compatibility_rules(left_trackable_id, right_trackable_id) ON DELETE CASCADE,
	-- We check that the `procedure_stratified_source` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (procedure_stratified_source, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	FOREIGN KEY (procedure_stratified_source, stratified_source) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE,
	-- We check that the `procedure_supernatant_destination` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (
		procedure_supernatant_destination,
		procedure_model_id
	) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	FOREIGN KEY (
		procedure_supernatant_destination,
		supernatant_destination
	) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS procedures.supernatant_procedures (
	procedure_id UUID PRIMARY KEY REFERENCES procedures.procedures(id),
	-- We enforce that the model of this procedure must be a supernatant procedure model.
	procedure_model_id INTEGER NOT NULL REFERENCES procedure_models.supernatant_procedure_models(procedure_model_id),
	-- The source container from which the supernatant is taken.
	stratified_source UUID NOT NULL REFERENCES volumetric_container_models(id),
	-- The destination container to which the supernatant is transferred.
	supernatant_destination UUID NOT NULL REFERENCES volumetric_container_models(id),
	-- The device used for the aliquoting procedure.
	transferred_with UUID NOT NULL REFERENCES pipette_models(id),
	-- The pipette tip to be mounted on the pipette.
	pipette_tip UUID NOT NULL REFERENCES pipette_tip_models(id),
	-- We enforce that the extended `procedure` has indeed the same `procedure_model_id`, making
	-- sure that the procedure is a supernatant procedure without the possibility of a mistake.
	FOREIGN KEY (procedure_id, procedure_model_id) REFERENCES procedures.procedures(id, procedure_model_id) ON DELETE CASCADE,
	-- We enforce that there exists a `procedure_trackables` entry for the stratified source in the procedure.
	FOREIGN KEY (procedure_id, stratified_source) REFERENCES procedure_trackables(procedure_id, trackable_id) ON DELETE CASCADE,
	-- We enforce that there exists a `procedure_trackables` entry for the supernatant destination in the procedure.
	FOREIGN KEY (procedure_id, supernatant_destination) REFERENCES procedure_trackables(procedure_id, trackable_id) ON DELETE CASCADE,
	-- We enforce that there exists a `procedure_trackables` entry for the pipette used in the procedure.
	FOREIGN KEY (procedure_id, transferred_with) REFERENCES procedure_trackables(procedure_id, trackable_id) ON DELETE CASCADE,
	-- We enforce that there exists a `procedure_trackables` entry for the pipette tip used in the procedure.
	FOREIGN KEY (procedure_id, pipette_tip) REFERENCES procedure_trackables(procedure_id, trackable_id) ON DELETE CASCADE
);