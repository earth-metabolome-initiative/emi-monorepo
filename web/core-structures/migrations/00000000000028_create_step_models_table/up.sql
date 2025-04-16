CREATE TABLE IF NOT EXISTS step_models (
	id SERIAL PRIMARY KEY,
	name TEXT NOT NULL CHECK (must_not_be_empty(name)),
	description TEXT NOT NULL CHECK (must_not_be_empty(description)),
	-- How long a step can be snoozed for. When None, the step is not snoozable.
	-- Examples of snoozable steps are: "put on gloves before collecting".
	snoozable_for INTERVAL,
	-- Whether this step can be copied to a new procedure of the same type,
	-- such as the case when you are collecting multiple samples from the same
	-- organism. In such a case, you would specify that the step defining the
	-- organism is copiable and the user interface would show a button at the
	-- end of the procedure called `New {}`.
	copiable BOOLEAN DEFAULT FALSE,
	-- Image illustrating what the step looks like
	photograph_id INT NOT NULL REFERENCES photographs(id),
	step_model_category_id SMALLINT NOT NULL REFERENCES step_model_categories(id),
	created_by INT NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
	updated_by INT NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Table providing the instrument types necessary for a given abstract step model
CREATE TABLE IF NOT EXISTS step_model_instrument_categories (
	id SERIAL PRIMARY KEY,
	step_model_id INT NOT NULL REFERENCES step_models(id),
	instrument_category_id SMALLINT NOT NULL REFERENCES instrument_categories(id),
	created_by INT NOT NULL REFERENCES users(id),
	created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	updated_by INT NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Table providing the instrument types necessary for a given abstract procedure model
CREATE TABLE IF NOT EXISTS step_model_instrument_models (
	id INTEGER PRIMARY KEY REFERENCES step_model_instrument_categories(id),
	instrument_model_id SMALLINT NOT NULL REFERENCES instrument_models(id),
	created_by INT NOT NULL REFERENCES users(id),
	created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	updated_by INT NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Table providing the instrument types necessary for a given abstract procedure model
CREATE TABLE IF NOT EXISTS step_model_instruments (
	id INTEGER PRIMARY KEY REFERENCES step_model_instrument_models(id),
	instrument_id SMALLINT NOT NULL REFERENCES instruments(id),
	created_by INT NOT NULL REFERENCES users(id),
	created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	updated_by INT NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Table providing the nameplate categories used within a certain procedure model
CREATE TABLE IF NOT EXISTS step_model_nameplate_categories (
	id SERIAL PRIMARY KEY,
	step_model_id INT NOT NULL REFERENCES step_models(id),
	nameplate_category_id SMALLINT NOT NULL REFERENCES nameplate_categories(id),
	created_by INT NOT NULL REFERENCES users(id),
	created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	updated_by INT NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS step_model_container_categories (
	id SERIAL PRIMARY KEY,
	step_model_id INT NOT NULL REFERENCES step_models(id),
	container_category_id SMALLINT NOT NULL REFERENCES container_categories(id),
	expected_kelvin REAL NOT NULL CHECK (must_be_strictly_positive_f32(expected_kelvin)),
	tolerance_kelvin REAL NOT NULL CHECK (must_be_strictly_positive_f32(tolerance_kelvin)),
	created_by INT NOT NULL REFERENCES users(id),
	created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	updated_by INT NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	CONSTRAINT check_kelvin CHECK (must_be_strictly_smaller_than_f32(tolerance_kelvin, expected_kelvin))
);

CREATE TABLE IF NOT EXISTS step_model_tool_categories (
	id SERIAL PRIMARY KEY,
	step_model_id INT NOT NULL REFERENCES step_models(id),
	tool_category_id SMALLINT NOT NULL REFERENCES tool_categories(id),
	created_by INT NOT NULL REFERENCES users(id),
	created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	updated_by INT NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

---------------------------------------------------
-- HERE BEGIN THE SPECIALIZED STEP MODELS TABLES --
---------------------------------------------------

CREATE TABLE IF NOT EXISTS freeze_drying_step_models (
	id INTEGER PRIMARY KEY REFERENCES step_models(id),
	step_model_instrument_category_id INT NOT NULL REFERENCES step_model_instrument_categories(id),
	expected_kelvin REAL NOT NULL CHECK (must_be_strictly_positive_f32(expected_kelvin)),
	expected_pascal REAL NOT NULL CHECK (must_be_strictly_positive_f32(expected_pascal)),
	created_by INT NOT NULL REFERENCES users(id),
	created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	updated_by INT NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	CONSTRAINT check_kelvin CHECK (must_be_strictly_smaller_than_f32(expected_kelvin, 250.0)),
	CONSTRAINT check_pascal CHECK (must_be_strictly_smaller_than_f32(expected_pascal, 100.0))
);

CREATE TABLE IF NOT EXISTS weighing_step_models (
	id INTEGER PRIMARY KEY REFERENCES step_models(id),
	step_model_instrument_category_id INT NOT NULL REFERENCES step_model_instrument_categories(id),
	created_by INT NOT NULL REFERENCES users(id),
	created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	updated_by INT NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS fractioning_step_models (
	id INTEGER PRIMARY KEY REFERENCES step_models(id),
	step_model_instrument_category_id INT NOT NULL REFERENCES step_model_instrument_categories(id),
	expected_kilograms REAL NOT NULL CHECK (must_be_strictly_positive_f32(expected_kilograms)),
	tolerance_kilograms REAL NOT NULL CHECK (must_be_strictly_positive_f32(tolerance_kilograms)),
	created_by INT NOT NULL REFERENCES users(id),
	created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	updated_by INT NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	CONSTRAINT check_tolerance CHECK (must_be_strictly_smaller_than_f32(tolerance_kilograms, expected_kilograms))
);

CREATE TABLE IF NOT EXISTS grinding_step_models (
	id INTEGER PRIMARY KEY REFERENCES step_models(id),
	step_model_instrument_category_id INT NOT NULL REFERENCES step_model_instrument_categories(id),
	seconds REAL NOT NULL CHECK (must_be_strictly_positive_f32(seconds)),
	created_by INT NOT NULL REFERENCES users(id),
	created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	updated_by INT NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS aliquoting_step_models (
	id INTEGER PRIMARY KEY REFERENCES step_models(id),
	step_model_instrument_category_id INT NOT NULL REFERENCES step_model_instrument_categories(id),
	expected_liters REAL NOT NULL CHECK (must_be_strictly_positive_f32(expected_liters)),
	tolerance_liters REAL NOT NULL CHECK (must_be_strictly_positive_f32(tolerance_liters)),
	created_by INT NOT NULL REFERENCES users(id),
	created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	updated_by INT NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	CONSTRAINT check_tolerance CHECK (must_be_strictly_smaller_than_f32(tolerance_liters, expected_liters))
);
