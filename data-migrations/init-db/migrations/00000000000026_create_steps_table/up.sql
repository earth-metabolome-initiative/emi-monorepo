CREATE TABLE IF NOT EXISTS steps (
	id UUID PRIMARY KEY,
	procedure_id INTEGER NOT NULL REFERENCES procedures(id),
	step_model_id INTEGER NOT NULL REFERENCES step_models(id),
	begun_at TIMESTAMP WITH TIME ZONE NOT NULL,
	finished_at TIMESTAMP WITH TIME ZONE NOT NULL,
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Table providing the instruments necessary for a given step
CREATE TABLE IF NOT EXISTS step_instruments (
	id UUID PRIMARY KEY,
	step_id UUID NOT NULL REFERENCES steps(id),
	instrument_id INTEGER NOT NULL REFERENCES instruments(id),
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Table providing the nameplate models used within a certain step
CREATE TABLE IF NOT EXISTS step_nameplate_models (
	id UUID PRIMARY KEY,
	step_id UUID NOT NULL REFERENCES steps(id),
	nameplate_model_id INTEGER NOT NULL REFERENCES nameplate_models(id),
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS step_container_models (
	id UUID PRIMARY KEY,
	step_id UUID NOT NULL REFERENCES steps(id),
	container_model_id INTEGER NOT NULL REFERENCES container_models(id),
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS step_storage_containers (
	id UUID PRIMARY KEY,
	step_id UUID NOT NULL REFERENCES steps(id),
	storage_container_id UUID NOT NULL REFERENCES storage_containers(id),
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS step_tool_models (
	id UUID PRIMARY KEY,
	step_id UUID NOT NULL REFERENCES steps(id),
	tool_model_id INTEGER NOT NULL REFERENCES tool_models(id),
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

--------------------------------------------
-- HERE BEGIN THE SPECIALIZED STEP TABLES --
--------------------------------------------

CREATE TABLE IF NOT EXISTS sampling_steps (
	id UUID PRIMARY KEY REFERENCES steps(id),
	processable_id UUID NOT NULL REFERENCES processables(id),
	trackable_id UUID NOT NULL REFERENCES trackables(id),
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS processing_steps (
	id UUID PRIMARY KEY REFERENCES steps(id),
	processable_id UUID NOT NULL REFERENCES processables(id),
	instrument_id INTEGER NOT NULL REFERENCES instruments(id),
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS weighing_steps (
	id UUID PRIMARY KEY REFERENCES steps(id),
	processable_id UUID NOT NULL REFERENCES processables(id),
	weighing_step_model_id INTEGER NOT NULL REFERENCES weighing_step_models(id),
	instrument_id INTEGER NOT NULL REFERENCES instruments(id),
	kilograms REAL NOT NULL CHECK (must_be_strictly_positive_f32(kilograms)),
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS fractioning_steps (
	id UUID PRIMARY KEY REFERENCES steps(id),
	source_processable_id UUID NOT NULL REFERENCES processables(id),
	destination_processable_id UUID NOT NULL REFERENCES processables(id),
	instrument_id INTEGER NOT NULL REFERENCES instruments(id),
	kilograms REAL NOT NULL CHECK (must_be_strictly_positive_f32(kilograms)),
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS shaking_steps (
	id UUID PRIMARY KEY REFERENCES steps(id),
	processable_id UUID NOT NULL REFERENCES processables(id),
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS ball_mill_steps (
	id UUID PRIMARY KEY REFERENCES steps(id),
	processable_id UUID NOT NULL REFERENCES processables(id),
	instrument_id INTEGER NOT NULL REFERENCES instruments(id),
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS centrifuge_steps (
	id UUID PRIMARY KEY REFERENCES steps(id),
	processable_id UUID NOT NULL REFERENCES processables(id),
	instrument_id INTEGER NOT NULL REFERENCES instruments(id),
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS disposal_steps (
	id UUID PRIMARY KEY REFERENCES steps(id),
	processable_id UUID NOT NULL REFERENCES processables(id),
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS aliquoting_steps (
	id UUID PRIMARY KEY REFERENCES steps(id),
	source_processable_id UUID NOT NULL REFERENCES volumetric_processables(id),
	destination_processable_id UUID NOT NULL REFERENCES volumetric_processables(id),
	instrument_id INTEGER NOT NULL REFERENCES instruments(id),
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);