CREATE TABLE IF NOT EXISTS procedure_models.mixing_procedure_models (
	procedure_model_id INTEGER PRIMARY KEY REFERENCES procedure_models.procedure_models(id),
	-- The weighting device used to measure the solid.
	measured_with UUID NOT NULL REFERENCES weighing_device_models(id) ON DELETE CASCADE,
	procedure_measured_with INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	source INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- The volumetric container into which the solid is mixed.
	mixed_with UUID NOT NULL REFERENCES volumetric_container_models(id) ON DELETE CASCADE,
	procedure_mixed_into INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- The amount of solid that is mixed into the container.
	kilograms REAL NOT NULL CHECK (must_be_strictly_positive_f32(kilograms)),
	FOREIGN KEY (procedure_measured_with, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	FOREIGN KEY (procedure_measured_with, measured_with) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE,
	FOREIGN KEY (source, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	FOREIGN KEY (procedure_mixed_into, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	FOREIGN KEY (procedure_mixed_into, mixed_with) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE
);