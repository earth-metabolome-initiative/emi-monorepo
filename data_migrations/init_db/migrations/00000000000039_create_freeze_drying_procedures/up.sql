CREATE TABLE IF NOT EXISTS freeze_drying_procedure_models (
	procedure_model_id INTEGER PRIMARY KEY REFERENCES procedure_models(id),
	-- The storage temperature in Kelvin.
	kelvin REAL NOT NULL DEFAULT 203.15 CHECK (must_be_strictly_positive_f32(kelvin)),
	-- Tolerance percentage for the storage temperature.
	kelvin_tolerance_percentage REAL NOT NULL DEFAULT 5.0 CHECK (
		must_be_strictly_positive_f32(kelvin_tolerance_percentage)
		AND must_be_smaller_than_f32(kelvin_tolerance_percentage, 100.0)
	),
	-- We use a default of 4 Pa for the pressure in the freeze-drying chamber.
	pascal REAL NOT NULL DEFAULT 4.0 CHECK (
		must_be_strictly_positive_f32(pascal)
		AND must_be_smaller_than_f32(pascal, 500.0)
	),
	-- We use a default of 3 days (259200 seconds) for the freeze-drying procedure.
	seconds REAL NOT NULL DEFAULT 259200.0 CHECK (
		must_be_strictly_greater_than_f32(seconds, 7200.0)
		AND must_be_strictly_smaller_than_f32(seconds, 604800.0)
	),
	-- The device used for the freeze drying procedure.
	freeze_dried_with UUID NOT NULL REFERENCES freeze_drier_models(id),
	procedure_freeze_dried_with INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- The container that is being freeze_dried.
	freeze_dried_container_id UUID NOT NULL REFERENCES volumetric_container_models(id),
	procedure_freeze_dried_container_id INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- We check that the `procedure_freeze_dried_with` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (procedure_freeze_dried_with, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	-- We check that the `procedure_freeze_dried_with` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (
		procedure_freeze_dried_container_id,
		procedure_model_id
	) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	-- We check that the `freeze_dried_with` is indeed a container that is compatible with the procedure model.
	FOREIGN KEY (procedure_freeze_dried_with, freeze_dried_with) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE,
	-- We check that the `freeze_dried_container_id` is indeed a container that is compatible with the procedure model.
	FOREIGN KEY (
		procedure_freeze_dried_container_id,
		freeze_dried_container_id
	) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE,
	-- We check that the `freeze_dried_container_id` is indeed a freeze drier that can hold the `freeze_dried_with`.
	CONSTRAINT freeze_drying_pm_compatibility_rules FOREIGN KEY (freeze_dried_with, freeze_dried_container_id) REFERENCES compatibility_rules(left_trackable_id, right_trackable_id)
);