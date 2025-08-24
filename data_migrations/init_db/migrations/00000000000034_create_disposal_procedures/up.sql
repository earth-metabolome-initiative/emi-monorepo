CREATE TABLE IF NOT EXISTS procedure_models.disposal_procedure_models (
	procedure_model_id INTEGER PRIMARY KEY REFERENCES procedure_models.procedure_models(id),
	-- The disposed trackable is the one that is being disposed of.
	disposed_id INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- The disposal procedure model should always have a trackable that is compatible with it.
	FOREIGN KEY (disposed_id, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE
);