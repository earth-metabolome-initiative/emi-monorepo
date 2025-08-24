CREATE TABLE IF NOT EXISTS packaging_procedure_models (
	procedure_model_id INTEGER PRIMARY KEY REFERENCES procedure_models.procedure_models(id),
	packaged_with UUID NOT NULL REFERENCES container_models(id) ON DELETE CASCADE,
	procedure_packaged_with INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	procedure_sample_id INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- We check that the `procedure_packaged_with` is indeed assigned to the `procedure_model_id`.
	FOREIGN KEY (procedure_packaged_with, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	-- We check that the `packaged_with` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (procedure_packaged_with, packaged_with) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE,
	-- We check that the `procedure_sample_id` is indeed assigned to the `procedure_model_id`.
	FOREIGN KEY (procedure_sample_id, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE
);