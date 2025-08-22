CREATE TABLE IF NOT EXISTS geolocation_procedure_models (
	procedure_model_id INTEGER PRIMARY KEY REFERENCES procedure_models(id),
	-- The device used for geolocation.
	geolocated_with UUID NOT NULL REFERENCES positioning_device_models(id) ON DELETE CASCADE,
	procedure_geolocated_with INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	trackable_id INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- We check that the `geolocated_with` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (procedure_geolocated_with, geolocated_with) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE,
	-- We check that the `procedure_geolocated_with` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (procedure_geolocated_with, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	-- We check that the `trackable_id` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (trackable_id, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE
);