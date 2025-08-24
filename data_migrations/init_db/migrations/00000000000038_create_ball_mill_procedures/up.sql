CREATE TABLE IF NOT EXISTS procedure_models.ball_mill_procedure_models (
	procedure_model_id INTEGER PRIMARY KEY REFERENCES procedure_models.procedure_models(id),
	-- The storage temperature in Kelvin.
	kelvin REAL NOT NULL DEFAULT 293.15 CHECK (must_be_strictly_positive_f32(kelvin)),
	-- Tolerance percentage for the storage temperature.
	kelvin_tolerance_percentage REAL NOT NULL DEFAULT 1.0 CHECK (
		must_be_strictly_positive_f32(kelvin_tolerance_percentage)
		AND must_be_smaller_than_f32(kelvin_tolerance_percentage, 100.0)
	),
	-- By default, we set it to 150 seconds (2.5 minutes).
	seconds REAL NOT NULL DEFAULT 150.0 CHECK (
		must_be_strictly_smaller_than_f32(seconds, 900.0)
		AND must_be_strictly_greater_than_f32(seconds, 30.0)
	),
	-- The time in seconds that the ball mill should be used for the procedure.
	hertz REAL NOT NULL DEFAULT 25.0 CHECK (
		must_be_strictly_smaller_than_f32(hertz, 50.0)
		AND must_be_strictly_greater_than_f32(hertz, 15.0)
	),
	-- The device used for the ball mill procedure.
	milled_with UUID NOT NULL REFERENCES ball_mill_machine_models(id) ON DELETE CASCADE,
	procedure_milled_with INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- The container that is being milled.
	milled_container_id UUID NOT NULL REFERENCES volumetric_container_models(id) ON DELETE CASCADE,
	procedure_milled_container_id INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- We check that the `procedure_milled_with` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (procedure_milled_with, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	-- We check that the `procedure_milled_container_id` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (
		procedure_milled_container_id,
		procedure_model_id
	) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	-- We check that the `milled_with` is indeed a container that is compatible with the procedure model.
	FOREIGN KEY (procedure_milled_with, milled_with) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE,
	-- We check that the `milled_container_id` is indeed a container that is compatible with the procedure model.
	FOREIGN KEY (
		procedure_milled_container_id,
		milled_container_id
	) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE,
	-- We check that the `milled_with` is indeed a ball mill machine that can hold the `milled_container_id`.
	CONSTRAINT ball_mill_pm_compatibility_rules FOREIGN KEY (milled_with, milled_container_id) REFERENCES compatibility_rules(left_trackable_id, right_trackable_id) ON DELETE CASCADE
);