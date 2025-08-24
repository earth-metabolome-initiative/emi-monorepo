CREATE TABLE IF NOT EXISTS procedure_models.pouring_procedure_models (
	procedure_model_id INTEGER PRIMARY KEY REFERENCES procedure_models.procedure_models(id),
	measured_with INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	source INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- The volumetric container into which the liquid is poured.
	poured_into UUID NOT NULL REFERENCES volumetric_container_models(id) ON DELETE CASCADE,
	procedure_poured_into INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- The amount of liquid that is poured into the container.
	liters REAL NOT NULL CHECK (must_be_strictly_positive_f32(liters)),
	FOREIGN KEY (measured_with, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	FOREIGN KEY (source, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	FOREIGN KEY (procedure_poured_into, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	FOREIGN KEY (procedure_poured_into, poured_into) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE
);

