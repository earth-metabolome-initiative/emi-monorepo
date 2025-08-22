CREATE TABLE IF NOT EXISTS aliquoting_procedure_models (
	procedure_model_id INTEGER PRIMARY KEY REFERENCES procedure_models(id),
	-- The amount of liters that should be aliquoted.
	liters REAL NOT NULL CHECK (must_be_strictly_positive_f32(liters)),
	-- Source container from which the aliquot is taken.
	aliquoted_from UUID NOT NULL REFERENCES volumetric_container_models(id) ON DELETE CASCADE,
	procedure_aliquoted_from INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- Destination container to which the aliquot is transferred.
	aliquoted_into UUID NOT NULL REFERENCES volumetric_container_models(id) ON DELETE CASCADE,
	procedure_aliquoted_into INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- The device used for the aliquoting procedure.
	aliquoted_with UUID NOT NULL REFERENCES pipette_models(id) ON DELETE CASCADE,
	procedure_aliquoted_with INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- The pipette tip to be mounted on the pipette.
	pipette_tip UUID NOT NULL REFERENCES pipette_tip_models(id) ON DELETE CASCADE,
	procedure_pipette_tip INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- We check that the `procedure_aliquoted_with` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (procedure_aliquoted_with, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	FOREIGN KEY (procedure_aliquoted_with, aliquoted_with) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE,
	-- We check that the `procedure_pipette_tip` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (procedure_pipette_tip, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	FOREIGN KEY (procedure_pipette_tip, pipette_tip) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE,
	-- We check that the `aliquoted_with` is compatible with the `pipette_tip`.
	CONSTRAINT aliquoting_pm_compatibility_rules FOREIGN KEY (aliquoted_with, pipette_tip) REFERENCES compatibility_rules(left_trackable_id, right_trackable_id),
	-- We check that the `source` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (procedure_aliquoted_from, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	FOREIGN KEY (procedure_aliquoted_from, aliquoted_from) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE,
	-- We check that the `destination` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (procedure_aliquoted_into, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	FOREIGN KEY (procedure_aliquoted_into, aliquoted_into) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS aliquoting_procedures (
	procedure_id UUID PRIMARY KEY REFERENCES procedures(id),
	-- We enforce that the model of this procedure must be an aliquoting procedure model.
	procedure_model_id INTEGER NOT NULL REFERENCES aliquoting_procedure_models(procedure_model_id),
	-- The identifier of the instrument used for aliquoting.
	aliquoted_with UUID NOT NULL REFERENCES pipette_models(id),
	-- The pipette tip used for aliquoting, which must be a pipette tip model.
	pipette_tip UUID NOT NULL REFERENCES pipette_tip_models(id),
	-- The container being aliquoted, which must be a volumetric container model.
	aliquoted_container_id UUID NOT NULL REFERENCES volumetric_container_models(id),
	-- We enforce that the extended `procedure` has indeed the same `procedure_model_id`, making
	-- sure that the procedure is an aliquoting procedure without the possibility of a mistake.
	FOREIGN KEY (procedure_id, procedure_model_id) REFERENCES procedures(id, procedure_model_id) ON DELETE CASCADE,
	-- We enforce that there exists a `procedure_trackables` entry for the instrument used in the procedure.
	FOREIGN KEY (procedure_id, aliquoted_with) REFERENCES procedure_trackables(procedure_id, trackable_id) ON DELETE CASCADE,
	-- We enforce that there exists a `procedure_trackables` entry for the pipette tip used in the procedure.
	FOREIGN KEY (procedure_id, pipette_tip) REFERENCES procedure_trackables(procedure_id, trackable_id) ON DELETE CASCADE,
	-- We enforce that the `aliquoted_container_id` is indeed a procedure trackable.
	FOREIGN KEY (procedure_id, aliquoted_container_id) REFERENCES procedure_trackables(procedure_id, trackable_id) ON DELETE CASCADE
);