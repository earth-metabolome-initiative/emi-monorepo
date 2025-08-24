CREATE TABLE IF NOT EXISTS procedure_models.storage_procedure_models (
	procedure_model_id INTEGER PRIMARY KEY REFERENCES procedure_models.procedure_models(id),
	-- The storage temperature in Kelvin.
	kelvin REAL NOT NULL DEFAULT 293.15 CHECK (must_be_strictly_positive_f32(kelvin)),
	-- Tolerance percentage for the storage temperature.
	kelvin_tolerance_percentage REAL NOT NULL DEFAULT 1.0 CHECK (
		must_be_strictly_positive_f32(kelvin_tolerance_percentage)
		AND must_be_smaller_than_f32(kelvin_tolerance_percentage, 100.0)
	),
	-- The container that will be used for storage.
	parent_container_id UUID NOT NULL REFERENCES container_models(id) ON DELETE CASCADE,
	procedure_parent_container_id INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- The container that is being stored.
	child_container_id UUID NOT NULL REFERENCES container_models(id) ON DELETE CASCADE,
	procedure_child_container_id INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- We check that the `procedure_parent_container_id` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (
		procedure_parent_container_id,
		procedure_model_id
	) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	-- We check that the `procedure_child_container_id` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (procedure_child_container_id, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	-- We check that the `parent_container_id` is indeed a container that is compatible with the procedure model.
	FOREIGN KEY (
		procedure_parent_container_id,
		parent_container_id
	) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE,
	-- We check that the `child_container_id` is indeed a container that is compatible with the procedure model.
	FOREIGN KEY (procedure_child_container_id, child_container_id) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE,
	-- We check that the `parent_container_id` is indeed a container that can hold the `child_container_id`.
	CONSTRAINT storage_pm_compatibility_rules FOREIGN KEY (parent_container_id, child_container_id) REFERENCES compatibility_rules(left_trackable_id, right_trackable_id)
);