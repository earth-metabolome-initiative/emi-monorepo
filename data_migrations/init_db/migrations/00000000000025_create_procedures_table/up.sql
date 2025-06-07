CREATE TABLE IF NOT EXISTS procedures (
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	procedure_model_id INTEGER NOT NULL REFERENCES procedure_models(id),
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_by INTEGER NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	CHECK (must_be_smaller_than_utc(created_at, updated_at))
);

CREATE TABLE IF NOT EXISTS procedure_trackables (
	procedure_id UUID NOT NULL REFERENCES procedures(id),
	trackable_id UUID NOT NULL REFERENCES trackables(id),
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	PRIMARY KEY (procedure_id, trackable_id)
);

--------------------------------------------
-- HERE BEGIN THE SPECIALIZED STEP TABLES --
--------------------------------------------

CREATE TABLE IF NOT EXISTS sampling_procedures (
	procedure_id UUID PRIMARY KEY REFERENCES procedures(id),
	processable_id UUID NOT NULL REFERENCES processables(id),
	trackable_id UUID NOT NULL REFERENCES trackables(id),
	FOREIGN KEY (procedure_id, processable_id) REFERENCES procedure_trackables(procedure_id, trackable_id) ON DELETE CASCADE,
	FOREIGN KEY (procedure_id, trackable_id) REFERENCES procedure_trackables(procedure_id, trackable_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS processing_procedures (
	procedure_id UUID PRIMARY KEY REFERENCES procedures(id),
	processable_id UUID NOT NULL REFERENCES processables(id),
	instrument_id UUID NOT NULL REFERENCES instruments(id),
	FOREIGN KEY (procedure_id, processable_id) REFERENCES procedure_trackables(procedure_id, trackable_id) ON DELETE CASCADE,
	FOREIGN KEY (procedure_id, instrument_id) REFERENCES procedure_trackables(procedure_id, trackable_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS fractioning_procedures (
	procedure_id UUID PRIMARY KEY REFERENCES procedures(id),
	source_processable_id UUID NOT NULL REFERENCES processables(id),
	destination_processable_id UUID NOT NULL REFERENCES processables(id),
	instrument_id UUID NOT NULL REFERENCES trackables(id),
	kilograms REAL NOT NULL CHECK (must_be_strictly_positive_f32(kilograms)),
	FOREIGN KEY (procedure_id, source_processable_id) REFERENCES procedure_trackables(procedure_id, trackable_id) ON DELETE CASCADE,
	FOREIGN KEY (procedure_id, destination_processable_id) REFERENCES procedure_trackables(procedure_id, trackable_id) ON DELETE CASCADE,
	FOREIGN KEY (procedure_id, instrument_id) REFERENCES procedure_trackables(procedure_id, trackable_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS aliquoting_procedures (
	procedure_id UUID PRIMARY KEY REFERENCES procedures(id),
	source_processable_id UUID NOT NULL REFERENCES volumetric_processables(id),
	destination_processable_id UUID NOT NULL REFERENCES volumetric_processables(id),
	instrument_id UUID NOT NULL REFERENCES trackables(id),
	FOREIGN KEY (procedure_id, source_processable_id) REFERENCES procedure_trackables(procedure_id, trackable_id) ON DELETE CASCADE,
	FOREIGN KEY (procedure_id, destination_processable_id) REFERENCES procedure_trackables(procedure_id, trackable_id) ON DELETE CASCADE,
	FOREIGN KEY (procedure_id, instrument_id) REFERENCES procedure_trackables(procedure_id, trackable_id) ON DELETE CASCADE
);