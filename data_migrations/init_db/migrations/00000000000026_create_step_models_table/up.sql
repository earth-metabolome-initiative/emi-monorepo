CREATE TABLE IF NOT EXISTS step_models (
	id SERIAL PRIMARY KEY,
	procedure_model_id INTEGER NOT NULL REFERENCES procedure_models(id),
	name TEXT NOT NULL CHECK (must_be_paragraph(name)),
	description TEXT NOT NULL CHECK (must_be_paragraph(description)),
	-- How long a step can be snoozed for. When None, the step is not snoozable.
	-- Examples of snoozable steps are: "put on gloves before collecting".
	snoozable BOOLEAN NOT NULL DEFAULT FALSE,
	-- Whether this step can be copied to a new procedure of the same type,
	-- such as the case when you are collecting multiple samples from the same
	-- organism. In such a case, you would specify that the step defining the
	-- organism is copiable and the user interface would show a button at the
	-- end of the procedure called `New {}`.
	copiable BOOLEAN NOT NULL DEFAULT FALSE,
	-- Image illustrating what the step looks like
	photograph_id UUID NOT NULL REFERENCES documents(id),
	-- Icon used to represent the step in the user interface
	icon TEXT NOT NULL CHECK (must_be_font_awesome_class(icon)),
	step_model_category StepModelCategory NOT NULL,
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_by INTEGER NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	UNIQUE (procedure_model_id, name)
);

-- Table providing the instrument types necessary for a given abstract step model
CREATE TABLE IF NOT EXISTS step_model_instrument_categories (
	id SERIAL PRIMARY KEY,
	step_model_id INTEGER NOT NULL REFERENCES step_models(id),
	instrument_category InstrumentCategory NOT NULL,
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_by INTEGER NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Table providing the instrument types necessary for a given abstract procedure model
CREATE TABLE IF NOT EXISTS step_model_instrument_models (
	id INTEGER PRIMARY KEY REFERENCES step_model_instrument_categories(id),
	instrument_model_id INTEGER NOT NULL REFERENCES instrument_models(id),
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_by INTEGER NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Table providing the instrument types necessary for a given abstract procedure model
CREATE TABLE IF NOT EXISTS step_model_instruments (
	id INTEGER PRIMARY KEY REFERENCES step_model_instrument_models(id),
	instrument_id INTEGER NOT NULL REFERENCES instruments(id),
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_by INTEGER NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Table providing the nameplate categories used within a certain procedure model
CREATE TABLE IF NOT EXISTS step_model_nameplate_categories (
	id SERIAL PRIMARY KEY,
	step_model_id INTEGER NOT NULL REFERENCES step_models(id),
	procedure_model_nameplate_category_id INTEGER NOT NULL REFERENCES procedure_model_nameplate_categories(id),
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_by INTEGER NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS step_model_trackable_categories (
	id INTEGER PRIMARY KEY REFERENCES step_models(id) ON DELETE CASCADE,
	trackable_category_id INTEGER NOT NULL REFERENCES trackable_categories(id) ON DELETE CASCADE,
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_by INTEGER NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS step_model_container_categories (
	id SERIAL PRIMARY KEY,
	step_model_id INTEGER NOT NULL REFERENCES step_models(id),
	procedure_model_container_category_id INTEGER NOT NULL REFERENCES procedure_model_container_categories(id),
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_by INTEGER NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS step_model_tool_categories (
	id SERIAL PRIMARY KEY,
	step_model_id INTEGER NOT NULL REFERENCES step_models(id) ON DELETE CASCADE,
	procedure_model_tool_category_id INTEGER NOT NULL REFERENCES procedure_model_tool_categories(id) ON DELETE CASCADE,
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_by INTEGER NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

---------------------------------------------------
-- HERE BEGIN THE SPECIALIZED STEP MODELS TABLES --
---------------------------------------------------

CREATE TABLE IF NOT EXISTS sampling_step_models (
	id INTEGER PRIMARY KEY REFERENCES step_models(id),
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_by INTEGER NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS organism_sampling_step_models (
	id INTEGER PRIMARY KEY REFERENCES sampling_step_models(id),
	organism_id UUID NOT NULL REFERENCES organisms(id)
);

CREATE TABLE IF NOT EXISTS packaging_step_models (
	id SERIAL PRIMARY KEY,
	packaging_model_id INTEGER NOT NULL REFERENCES packaging_models(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	created_by INTEGER NOT NULL REFERENCES users(id),
	updated_by INTEGER NOT NULL REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS aliquoting_step_models (
	id INTEGER PRIMARY KEY REFERENCES sampling_step_models(id),
	liters REAL NOT NULL CHECK (must_be_strictly_positive_f32(liters)),
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_by INTEGER NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS freeze_drying_step_models (
	id INTEGER PRIMARY KEY REFERENCES step_models(id),
	expected_kelvin REAL NOT NULL CHECK (must_be_strictly_positive_f32(expected_kelvin)),
	expected_pascal REAL NOT NULL CHECK (must_be_strictly_positive_f32(expected_pascal)),
	expected_seconds REAL NOT NULL CHECK (must_be_strictly_positive_f32(expected_seconds)),
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_by INTEGER NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	CHECK (must_be_strictly_smaller_than_f32(expected_kelvin, 250.0)),
	CHECK (must_be_strictly_smaller_than_f32(expected_pascal, 100.0)),
	-- Should always be greater than 2 hours
	CHECK (must_be_strictly_greater_than_f32(expected_seconds, 7200.0)),
	-- Should always be less than 7 days
	CHECK (must_be_strictly_smaller_than_f32(expected_seconds, 604800.0))
);

CREATE TABLE IF NOT EXISTS weighing_step_models (
	id INTEGER PRIMARY KEY REFERENCES step_models(id),
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_by INTEGER NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS fractioning_step_models (
	id INTEGER PRIMARY KEY REFERENCES step_models(id),
	expected_kilograms REAL NOT NULL CHECK (must_be_strictly_positive_f32(expected_kilograms)),
	tolerance_kilograms REAL NOT NULL CHECK (must_be_strictly_positive_f32(tolerance_kilograms)),
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_by INTEGER NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	CHECK (must_be_strictly_smaller_than_f32(tolerance_kilograms, expected_kilograms))
);

CREATE TABLE IF NOT EXISTS shaking_step_models (
	id INTEGER PRIMARY KEY REFERENCES step_models(id),
	seconds REAL NOT NULL CHECK (must_be_strictly_positive_f32(seconds)),
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_by INTEGER NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS disposal_step_models (
	id INTEGER PRIMARY KEY REFERENCES step_models(id),
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_by INTEGER NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS ball_mill_step_models (
	id INTEGER PRIMARY KEY REFERENCES step_models(id),
	seconds REAL NOT NULL CHECK (must_be_strictly_positive_f32(seconds)),
	hertz REAL NOT NULL CHECK (must_be_strictly_positive_f32(hertz)),
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_by INTEGER NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	CHECK (must_be_strictly_smaller_than_f32(seconds, 1800.0)),
	CHECK (must_be_strictly_greater_than_f32(seconds, 10.0)),
	CHECK (must_be_strictly_smaller_than_f32(hertz, 100.0)),
	CHECK (must_be_strictly_greater_than_f32(hertz, 0.0))
);

CREATE TABLE IF NOT EXISTS centrifuge_step_models (
	id INTEGER PRIMARY KEY REFERENCES step_models(id),
	seconds REAL NOT NULL CHECK (must_be_strictly_positive_f32(seconds)),
	rotation_per_minute REAL NOT NULL CHECK (must_be_strictly_positive_f32(rotation_per_minute)),
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_by INTEGER NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	CHECK (must_be_strictly_smaller_than_f32(seconds, 3600.0)),
	CHECK (must_be_greater_than_f32(seconds, 30.0)),
	CHECK (must_be_smaller_than_f32(rotation_per_minute, 30000.0)),
	CHECK (must_be_greater_than_f32(rotation_per_minute, 5000.0))
);