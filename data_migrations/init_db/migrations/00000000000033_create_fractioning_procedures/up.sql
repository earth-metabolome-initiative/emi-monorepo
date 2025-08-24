CREATE TABLE IF NOT EXISTS procedure_models.fractioning_procedure_models (
	procedure_model_id INTEGER PRIMARY KEY REFERENCES procedure_models.procedure_models(id),
	-- Expected amount of the fraction to be collected in kilograms.
	kilograms REAL NOT NULL CHECK (must_be_strictly_positive_f32(kilograms)),
	-- The tolerance percentage of the fraction in kilograms.
	tolerance_percentage REAL NOT NULL CHECK (
		must_be_strictly_positive_f32(tolerance_percentage)
		AND must_be_smaller_than_f32(tolerance_percentage, 100.0)
	),
	-- The scale used to measure the fraction in kilograms.
	weighed_with UUID NOT NULL REFERENCES weighing_device_models(id) ON DELETE CASCADE,
	procedure_weighed_with INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- Source container from which the fraction is taken.
	procedure_fragment_source INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- Destination container to which the fraction is transferred.
	fragment_placed_into UUID NOT NULL REFERENCES volumetric_container_models(id) ON DELETE CASCADE,
	procedure_fragment_placed_into INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- Foreign key to ensure that the `weighed_with` is indeed a weighing instrument.
	FOREIGN KEY (procedure_weighed_with, weighed_with) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE,
	-- Foreign key to ensure that the `procedure_weighed_with` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (procedure_weighed_with, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	-- Foreign key to ensure that the `procedure_fragment_source` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (procedure_fragment_source, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	-- Foreign key to ensure that the `procedure_fragment_placed_into` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (
		procedure_fragment_placed_into,
		procedure_model_id
	) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	FOREIGN KEY (
		procedure_fragment_placed_into,
		fragment_placed_into
	) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE
);