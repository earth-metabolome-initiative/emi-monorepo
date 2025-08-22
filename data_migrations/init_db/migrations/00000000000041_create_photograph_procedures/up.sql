CREATE TABLE IF NOT EXISTS photograph_procedure_models (
	procedure_model_id INTEGER PRIMARY KEY REFERENCES procedure_models(id),
	-- The device used for photograph.
	photographed_with UUID NOT NULL REFERENCES camera_models(id) ON DELETE CASCADE,
	procedure_photographed_with INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	trackable_id INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- We check that the `photographed_with` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (procedure_photographed_with, photographed_with) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE,
	-- We check that the `procedure_photographed_with` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (procedure_photographed_with, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	-- We check that the `trackable_id` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (trackable_id, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE
);