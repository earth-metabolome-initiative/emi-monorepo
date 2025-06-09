CREATE TABLE IF NOT EXISTS procedure_models (
	id SERIAL PRIMARY KEY,
	name TEXT UNIQUE NOT NULL CHECK (must_be_paragraph(name)),
	description TEXT NOT NULL CHECK (must_be_paragraph(description)),
	deprecated BOOLEAN NOT NULL DEFAULT FALSE,
	photograph_id UUID REFERENCES documents(id),
	icon TEXT NOT NULL DEFAULT 'book' CHECK (must_be_font_awesome_class(icon)),
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_by INTEGER NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	CHECK (must_be_distinct(name, description)),
	CHECK (must_be_smaller_than_utc(created_at, updated_at))
);

CREATE TABLE IF NOT EXISTS parent_procedure_models (
	parent_procedure_model_id INTEGER NOT NULL REFERENCES procedure_models(id) ON DELETE CASCADE,
	child_procedure_model_id INTEGER NOT NULL REFERENCES procedure_models(id) ON DELETE CASCADE,
	snoozable BOOLEAN NOT NULL DEFAULT FALSE,
	copiable BOOLEAN NOT NULL DEFAULT FALSE,
	repeatable BOOLEAN NOT NULL DEFAULT FALSE,
	skippable BOOLEAN NOT NULL DEFAULT FALSE,
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	CHECK (must_be_distinct_i32(parent_procedure_model_id, child_procedure_model_id)),
	PRIMARY KEY (parent_procedure_model_id, child_procedure_model_id)
);

CREATE TABLE IF NOT EXISTS next_procedure_models (
	parent_id INTEGER NOT NULL REFERENCES procedure_models(id) ON DELETE CASCADE,
	current_id INTEGER NOT NULL REFERENCES procedure_models(id) ON DELETE CASCADE,
	successor_id INTEGER NOT NULL REFERENCES procedure_models(id) ON DELETE CASCADE,
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	PRIMARY KEY (parent_id, current_id, successor_id),
	CHECK (must_be_distinct_i32(current_id, successor_id)),
	FOREIGN KEY (parent_id, current_id) REFERENCES parent_procedure_models(parent_procedure_model_id, child_procedure_model_id) ON DELETE CASCADE,
	FOREIGN KEY (parent_id, successor_id) REFERENCES parent_procedure_models(parent_procedure_model_id, child_procedure_model_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS procedure_model_trackables (
	id SERIAL PRIMARY KEY,
	name TEXT NOT NULL CHECK (must_be_paragraph(name)),
	procedure_model_id INTEGER NOT NULL REFERENCES procedure_models(id) ON DELETE CASCADE,
	trackable_id UUID NOT NULL REFERENCES trackables(id) ON DELETE CASCADE,
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_by INTEGER NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	UNIQUE (procedure_model_id, name),
	UNIQUE (id, procedure_model_id),
	UNIQUE (id, trackable_id),
	CHECK (must_be_smaller_than_utc(created_at, updated_at))
);

CREATE TABLE IF NOT EXISTS shared_procedure_model_trackables (
	parent_id INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	parent_trackable_id UUID NOT NULL REFERENCES trackables(id) ON DELETE CASCADE,
	parent_procedure_model_id INTEGER NOT NULL REFERENCES procedure_models(id) ON DELETE CASCADE,
	child_id INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	child_trackable_id UUID NOT NULL REFERENCES trackables(id) ON DELETE CASCADE,
	child_procedure_model_id INTEGER NOT NULL REFERENCES procedure_models(id) ON DELETE CASCADE,
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	PRIMARY KEY (parent_id, child_id),
	-- This foreign key ensures that the the `trackable_id` existing in the `parent_trackable_id` row
	-- is indeed the same as the one in the `procedure_model_trackables` table.
	FOREIGN KEY (parent_id, parent_trackable_id) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE,
	-- And this one is the analogous one for the `child_trackable_id`.
	FOREIGN KEY (child_id, child_trackable_id) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE,
	-- This foreign key ensures that the `parent_procedure_model_id` is indeed the same as the one in the `procedure_models` table.
	FOREIGN KEY (parent_id, parent_procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	-- And this one is the analogous one for the `child_procedure_model_id`.
	FOREIGN KEY (child_id, child_procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	-- Furthermore, we want to check that `parent_procedure_model_id` is indeed the parent of `child_procedure_model_id`
	-- as defined by the `parent_procedure_models` table.
	FOREIGN KEY (parent_procedure_model_id, child_procedure_model_id) REFERENCES parent_procedure_models(parent_procedure_model_id, child_procedure_model_id) ON DELETE CASCADE,
	CHECK (must_be_distinct_i32(parent_id, child_id))
	-- TODO: add a check that ensures that the `parent_trackable_id` is compatible with the `child_trackable_id` as
	-- defined by the `trackables` table hierarchy. Specifically, the `parent_trackable_id` should be a descendant
	-- of the `child_trackable_id` as the child procedures are generally meant to be **less** specific than the parent ones.
);

--------------------------------------------------------
-- HERE BEGIN THE SPECIALIZED PROCEDURE MODELS TABLES --
--------------------------------------------------------

CREATE TABLE IF NOT EXISTS sampling_procedure_models (
	id INTEGER PRIMARY KEY REFERENCES procedure_models(id)
);

CREATE TABLE IF NOT EXISTS packaging_procedure_models (
	id INTEGER PRIMARY KEY REFERENCES procedure_models(id),
	packaging_model_id UUID NOT NULL REFERENCES packaging_models(id)
);

CREATE TABLE IF NOT EXISTS pouring_procedure_models (
	id INTEGER PRIMARY KEY REFERENCES procedure_models(id),
	measured_with INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	source INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	destination INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	liters REAL NOT NULL CHECK (must_be_strictly_positive_f32(liters)),
	FOREIGN KEY (measured_with, id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	FOREIGN KEY (source, id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	FOREIGN KEY (destination, id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS mix_solid_procedure_models (
	id INTEGER PRIMARY KEY REFERENCES procedure_models(id),
	measured_with INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	source INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	destination INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	kilograms REAL NOT NULL CHECK (must_be_strictly_positive_f32(kilograms)),
	FOREIGN KEY (measured_with, id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	FOREIGN KEY (source, id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	FOREIGN KEY (destination, id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS mix_countable_procedure_models (
	id INTEGER PRIMARY KEY REFERENCES procedure_models(id),
	source INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	destination INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	quantity SMALLINT NOT NULL CHECK (must_be_strictly_positive_i16(quantity)),
	FOREIGN KEY (source, id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	FOREIGN KEY (destination, id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS aliquoting_procedure_models (
	id INTEGER PRIMARY KEY REFERENCES procedure_models(id),
	liters REAL NOT NULL CHECK (must_be_strictly_positive_f32(liters)),
	error REAL NOT NULL CHECK (must_be_strictly_positive_f32(error)),
	CHECK (must_be_smaller_than_f32(error, liters))
);

CREATE TABLE IF NOT EXISTS freeze_drying_procedure_models (
	id INTEGER PRIMARY KEY REFERENCES procedure_models(id),
	kelvin REAL NOT NULL CHECK (must_be_strictly_positive_f32(kelvin)),
	pascal REAL NOT NULL CHECK (must_be_strictly_positive_f32(pascal)),
	seconds REAL NOT NULL CHECK (must_be_strictly_positive_f32(seconds)),
	CHECK (must_be_strictly_smaller_than_f32(kelvin, 250.0)),
	CHECK (must_be_strictly_smaller_than_f32(pascal, 100.0)),
	-- Should always be greater than 2 hours
	CHECK (must_be_strictly_greater_than_f32(seconds, 7200.0)),
	-- Should always be less than 7 days
	CHECK (must_be_strictly_smaller_than_f32(seconds, 604800.0))
);

CREATE TABLE IF NOT EXISTS weighing_procedure_models (
	id INTEGER PRIMARY KEY REFERENCES procedure_models(id),
	instrument_id INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- We check that the `instrument_id` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (instrument_id, id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE
	-- TODO: find an appropriate check to ensure that the `instrument_id` is indeed a weighing instrument.
);

CREATE TABLE IF NOT EXISTS fractioning_procedure_models (
	id INTEGER PRIMARY KEY REFERENCES procedure_models(id),
	kilograms REAL NOT NULL CHECK (must_be_strictly_positive_f32(kilograms))
);

CREATE TABLE IF NOT EXISTS shaking_procedure_models (
	id INTEGER PRIMARY KEY REFERENCES procedure_models(id),
	seconds REAL NOT NULL CHECK (must_be_strictly_positive_f32(seconds))
);

CREATE TABLE IF NOT EXISTS disposal_procedure_models (
	id INTEGER PRIMARY KEY REFERENCES procedure_models(id)
);

CREATE TABLE IF NOT EXISTS ball_mill_procedure_models (
	id INTEGER PRIMARY KEY REFERENCES procedure_models(id),
	seconds REAL NOT NULL CHECK (must_be_strictly_positive_f32(seconds)),
	hertz REAL NOT NULL CHECK (must_be_strictly_positive_f32(hertz)),
	CHECK (must_be_strictly_smaller_than_f32(seconds, 1800.0)),
	CHECK (must_be_strictly_greater_than_f32(seconds, 10.0)),
	CHECK (must_be_strictly_smaller_than_f32(hertz, 100.0)),
	CHECK (must_be_strictly_greater_than_f32(hertz, 0.0))
);

CREATE TABLE IF NOT EXISTS centrifuge_procedure_models (
	id INTEGER PRIMARY KEY REFERENCES procedure_models(id),
	seconds REAL NOT NULL CHECK (must_be_strictly_positive_f32(seconds)),
	rotation_per_minute REAL NOT NULL CHECK (must_be_strictly_positive_f32(rotation_per_minute)),
	CHECK (must_be_strictly_smaller_than_f32(seconds, 3600.0)),
	CHECK (must_be_greater_than_f32(seconds, 30.0)),
	CHECK (must_be_smaller_than_f32(rotation_per_minute, 30000.0)),
	CHECK (must_be_greater_than_f32(rotation_per_minute, 5000.0))
);
