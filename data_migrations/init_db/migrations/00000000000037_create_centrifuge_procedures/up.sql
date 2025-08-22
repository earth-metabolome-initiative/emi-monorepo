CREATE TABLE IF NOT EXISTS centrifuge_procedure_models (
	procedure_model_id INTEGER PRIMARY KEY REFERENCES procedure_models(id),
	-- The storage temperature in Kelvin.
	kelvin REAL NOT NULL DEFAULT 293.15 CHECK (must_be_strictly_positive_f32(kelvin)),
	-- Tolerance percentage for the storage temperature.
	kelvin_tolerance_percentage REAL NOT NULL DEFAULT 1.0 CHECK (
		must_be_strictly_positive_f32(kelvin_tolerance_percentage)
		AND must_be_smaller_than_f32(kelvin_tolerance_percentage, 100.0)
	),
	-- The time in seconds that the centrifuge should be used for the procedure.
	seconds REAL NOT NULL DEFAULT 120.0 CHECK (
		must_be_greater_than_f32(seconds, 30.0)
		AND must_be_smaller_than_f32(seconds, 1800.0)
	),
	-- The RPMs (rotations per minute) of the centrifuge.
	rotation_per_minute REAL NOT NULL DEFAULT 13000.0 CHECK (
		must_be_greater_than_f32(rotation_per_minute, 5000.0)
		AND must_be_smaller_than_f32(rotation_per_minute, 30000.0)
	),
	-- The device used for the centrifuge procedure.
	centrifuged_with UUID NOT NULL REFERENCES centrifuge_models(id) ON DELETE CASCADE,
	procedure_centrifuged_with INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- The container that is being centrifuged.
	centrifuged_container_id UUID NOT NULL REFERENCES volumetric_container_models(id) ON DELETE CASCADE,
	procedure_centrifuged_container_id INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- We check that the `procedure_centrifuged_with` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (procedure_centrifuged_with, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	-- We check that the `procedure_centrifuged_container_id` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (
		procedure_centrifuged_container_id,
		procedure_model_id
	) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	-- We check that the `centrifuged_with` is indeed a container that is compatible with the procedure model.
	FOREIGN KEY (procedure_centrifuged_with, centrifuged_with) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE,
	-- We check that the `procedure_centrifuged_container_id` is indeed a container that is compatible with the procedure model.
	FOREIGN KEY (
		procedure_centrifuged_container_id,
		centrifuged_container_id
	) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE,
	-- We check that the `centrifuged_with` is indeed a container that can hold the `centrifuged_with`.
	CONSTRAINT centrifuge_pm_compatibility_rules FOREIGN KEY (centrifuged_with, centrifuged_container_id) REFERENCES compatibility_rules(left_trackable_id, right_trackable_id) ON DELETE CASCADE
);