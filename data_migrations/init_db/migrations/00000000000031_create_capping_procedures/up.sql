CREATE TABLE IF NOT EXISTS capping_procedure_models (
	procedure_model_id INTEGER PRIMARY KEY REFERENCES procedure_models(id),
	-- The container to be capped.
	container_id UUID NOT NULL REFERENCES volumetric_container_models(id) ON DELETE CASCADE,
	procedure_container_id INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- The cap to be used for the container.
	capped_with UUID NOT NULL REFERENCES trackables(id) ON DELETE CASCADE,
	procedure_capped_with INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- We check that the `procedure_container_id` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (procedure_container_id, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	-- We check that the `procedure_capped_with` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (procedure_capped_with, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	-- We check that the `container_id` is indeed the trackable that is compatible with the procedure model.
	FOREIGN KEY (procedure_container_id, container_id) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE,
	-- We check that the `capped_with` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (procedure_capped_with, capped_with) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE,
	-- We check that the `capped_with` is indeed a cap that can be used with the `container_id`.
	CONSTRAINT capping_pm_compatibility_rules FOREIGN KEY (container_id, capped_with) REFERENCES compatibility_rules(left_trackable_id, right_trackable_id) ON DELETE CASCADE
);