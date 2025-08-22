CREATE TABLE IF NOT EXISTS placing_procedure_models (
	procedure_model_id INTEGER PRIMARY KEY REFERENCES procedure_models(id),
	source INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- The volumetric container into which the trackable is placed.
	placed_into UUID NOT NULL REFERENCES volumetric_container_models(id) ON DELETE CASCADE,
	procedure_placed_into INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- The quantity of the trackable that is placed into the container.
	quantity SMALLINT NOT NULL CHECK (must_be_strictly_positive_i16(quantity)),
	FOREIGN KEY (source, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	FOREIGN KEY (procedure_placed_into, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	FOREIGN KEY (procedure_placed_into, placed_into) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE
);