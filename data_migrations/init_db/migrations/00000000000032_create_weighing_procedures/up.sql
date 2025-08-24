CREATE TABLE IF NOT EXISTS procedure_models.weighing_with_device_model_procedure_models (
	procedure_model_id INTEGER PRIMARY KEY REFERENCES procedure_models.procedure_models(id),
	-- The sample container is the one that is being weighed.
	sample_container_id UUID NOT NULL REFERENCES volumetric_container_models(id),
	procedure_sample_container INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- The instrument used for weighing.
	weighed_with UUID NOT NULL REFERENCES weighing_device_models(id),
	-- The instrument used for weighing should always be a trackable that is compatible with the procedure model.
	procedure_weighed_with INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- We check that the `weighed_with` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (procedure_weighed_with, weighed_with) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE,
	-- We ensure that the `procedure_weighed_with` is indeed associated with the parent procedure model.
	FOREIGN KEY (procedure_weighed_with, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	-- We check that the `procedure_sample_container` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (procedure_sample_container, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	FOREIGN KEY (procedure_sample_container, sample_container_id) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS procedure_models.weighing_with_device_procedure_models (
	procedure_model_id INTEGER PRIMARY KEY REFERENCES procedure_models.weighing_with_device_model_procedure_models(procedure_model_id)
);
CREATE TABLE IF NOT EXISTS procedures.weighing_with_device_models(
	procedure_id UUID PRIMARY KEY REFERENCES procedures.procedures(id),
	-- We enforce that the model of this procedure must be a weighing procedure model.
	procedure_model_id INTEGER NOT NULL REFERENCES procedure_models.weighing_with_device_model_procedure_models(procedure_model_id),
	-- The container being weighed, which must be a volumetric container model.
	weighted_container_id UUID NOT NULL REFERENCES volumetric_container_models(id),
	-- The measured weight, which must be strictly positive.
	kilograms REAL NOT NULL CHECK (must_be_strictly_positive_f32(kilograms)),
	-- The identifier of the instrument used for weighing.
	weighted_with_model UUID NOT NULL REFERENCES weighing_device_models(id),
	-- We enforce that the extended `procedure` has indeed the same `procedure_model_id`, making
	-- sure that the procedure is a weighing procedure without the possibility of a mistake.
	FOREIGN KEY (procedure_id, procedure_model_id) REFERENCES procedures.procedures(id, procedure_model_id) ON DELETE CASCADE,
	-- We enforce that the `weighted_container_id` is indeed a procedure trackable.
	FOREIGN KEY (procedure_id, weighted_container_id) REFERENCES procedure_trackables(procedure_id, trackable_id) ON DELETE CASCADE -- There us no need here to check that the `weighted_with` is a weighing instrument nor that it is
);
CREATE TABLE IF NOT EXISTS procedures.weighing_with_devices(
	procedure_id UUID PRIMARY KEY REFERENCES procedures.weighing_with_device_models(id),
	-- We enforce that the model of this procedure must be a weighing procedure model.
	procedure_model_id INTEGER NOT NULL REFERENCES procedure_models.weighing_with_device_procedure_models(procedure_model_id),
	-- The identifier of the instrument used for weighing.
	weighted_with UUID NOT NULL REFERENCES weighing_devices(id),
	-- We enforce that the extended `procedure` has indeed the same `procedure_model_id`, making
	-- sure that the procedure is a weighing procedure without the possibility of a mistake.
	FOREIGN KEY (procedure_id, procedure_model_id) REFERENCES procedures.procedures(id, procedure_model_id) ON DELETE CASCADE,
	-- We enforce that the `weighted_container_id` is indeed a procedure trackable.
	FOREIGN KEY (procedure_id, weighted_container_id) REFERENCES procedure_trackables(procedure_id, trackable_id) ON DELETE CASCADE -- There us no need here to check that the `weighted_with` is a weighing instrument nor that it is
);