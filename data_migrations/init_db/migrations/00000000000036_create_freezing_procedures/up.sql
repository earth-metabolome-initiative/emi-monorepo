CREATE TABLE IF NOT EXISTS procedure_models.freezing_procedure_models (
	procedure_model_id INTEGER PRIMARY KEY REFERENCES procedure_models.procedure_models(id),
	-- The storage temperature in Kelvin.
	kelvin REAL NOT NULL DEFAULT 203.15 CHECK (must_be_strictly_positive_f32(kelvin)),
	-- Tolerance percentage for the storage temperature.
	kelvin_tolerance_percentage REAL NOT NULL DEFAULT 5.0 CHECK (
		must_be_strictly_positive_f32(kelvin_tolerance_percentage)
		AND must_be_smaller_than_f32(kelvin_tolerance_percentage, 100.0)
	),
	-- We use a default of 43200 seconds (12 hours) for the freezing procedure.
	seconds REAL DEFAULT 43200.0 CHECK (
		must_be_strictly_positive_f32(seconds)
		AND must_be_strictly_greater_than_f32(seconds, 1800.0)
	),
	-- The device used for freezing.
	frozen_with UUID NOT NULL REFERENCES freezer_models(id) ON DELETE CASCADE,
	procedure_frozen_with INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- The container that is being stored in the freezer.
	frozen_container_id UUID NOT NULL REFERENCES container_models(id) ON DELETE CASCADE,
	procedure_frozen_container_id INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- We check that the `procedure_frozen_with` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (procedure_frozen_with, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	-- We check that the `procedure_frozen_container_id` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (
		procedure_frozen_container_id,
		procedure_model_id
	) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	-- We check that the `frozen_with` is indeed a container that is compatible with the procedure model.
	FOREIGN KEY (procedure_frozen_with, frozen_with) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE,
	-- We check that the `frozen_container_id` is indeed a container that is compatible with the procedure model.
	FOREIGN KEY (
		procedure_frozen_container_id,
		frozen_container_id
	) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE,
	-- We check that the `frozen_with` is indeed a container that can hold the `frozen_container_id`.
	CONSTRAINT freezing_pm_compatibility_rules FOREIGN KEY (frozen_with, frozen_container_id) REFERENCES compatibility_rules(left_trackable_id, right_trackable_id) ON DELETE CASCADE
);