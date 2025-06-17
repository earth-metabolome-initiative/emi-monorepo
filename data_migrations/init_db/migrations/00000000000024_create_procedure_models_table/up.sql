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

CREATE TABLE IF NOT EXISTS packaging_procedure_models (
	id INTEGER PRIMARY KEY REFERENCES procedure_models(id),
	packaging_model_id UUID NOT NULL REFERENCES packaging_models(trackable_id)
);

CREATE TABLE IF NOT EXISTS pouring_procedure_models (
	procedure_model_id INTEGER PRIMARY KEY REFERENCES procedure_models(id),
	measured_with INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	source INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	destination INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	liters REAL NOT NULL CHECK (must_be_strictly_positive_f32(liters)),
	FOREIGN KEY (measured_with, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	FOREIGN KEY (source, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	FOREIGN KEY (destination, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS mix_solid_procedure_models (
	procedure_model_id INTEGER PRIMARY KEY REFERENCES procedure_models(id),
	measured_with INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	source INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	destination INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	kilograms REAL NOT NULL CHECK (must_be_strictly_positive_f32(kilograms)),
	FOREIGN KEY (measured_with, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	FOREIGN KEY (source, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	FOREIGN KEY (destination, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS mix_countable_procedure_models (
	procedure_model_id INTEGER PRIMARY KEY REFERENCES procedure_models(id),
	source INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	destination INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	quantity SMALLINT NOT NULL CHECK (must_be_strictly_positive_i16(quantity)),
	FOREIGN KEY (source, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	FOREIGN KEY (destination, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS aliquoting_procedure_models (
	procedure_model_id INTEGER PRIMARY KEY REFERENCES procedure_models(id),
	-- The amount of liters that should be aliquoted.
	liters REAL NOT NULL CHECK (must_be_strictly_positive_f32(liters)),
	-- Source container from which the aliquot is taken.
	source INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- Destination container to which the aliquot is transferred.
	destination INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- The device used for the aliquoting procedure.
	aliquoted_with INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- We check that the `aliquoted_with` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (aliquoted_with, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	-- We check that the `source` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (source, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	-- We check that the `destination` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (destination, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS supernatant_procedure_models (
	procedure_model_id INTEGER PRIMARY KEY REFERENCES procedure_models(id),
	-- The amount of liters that should be transferred
	liters REAL NOT NULL CHECK (must_be_strictly_positive_f32(liters)),
	-- The source container from which the supernatant is taken.
	stratified_source INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- Destination container to which the supernatant is transferred.
	supernatant_destination INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- The device used for the aliquoting procedure.
	transferred_with INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- We check that the `transferred_with` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (transferred_with, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	-- We check that the `stratified_source` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (stratified_source, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	-- We check that the `supernatant_destination` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (supernatant_destination, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS mount_tip_procedure_models (
	procedure_model_id INTEGER PRIMARY KEY REFERENCES procedure_models(id),
	-- The pipette to be mounted.
	pipette INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- The pipette tip to be mounted on the pipette.
	pipette_tip INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- We check that the `pipette` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (pipette, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	-- We check that the `pipette_tip` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (pipette_tip, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS capping_procedure_models (
	procedure_model_id INTEGER PRIMARY KEY REFERENCES procedure_models(id),
	-- The container to be capped.
	container_id UUID NOT NULL REFERENCES container_models(id) ON DELETE CASCADE,
	procedure_container_id INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- The cap to be used for the container.
	capped_with UUID NOT NULL REFERENCES trackables(id) ON DELETE CASCADE,
	procedure_capped_with INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- We check that the `procedure_container_id` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (procedure_container_id, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	-- We check that the `procedure_capped_with` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (procedure_capped_with, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	-- We check that the `container_id` is indeed the trackable that is compatible with the procedure model.
	FOREIGN KEY (procedure_container_id, container_id) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE,
	-- We check that the `capped_with` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (procedure_capped_with, capped_with) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE,
	-- We check that the `capped_with` is indeed a cap that can be used with the `container_id`.
	FOREIGN KEY (container_id, capped_with) REFERENCES capping_rules(container_id, cap_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS freezing_procedure_models (
	procedure_model_id INTEGER PRIMARY KEY REFERENCES procedure_models(id),
	-- We use a default of 203.15 K (-70 °C) for the temperature in the freezing chamber.
	kelvin REAL NOT NULL DEFAULT 203.15 CHECK (must_be_strictly_positive_f32(kelvin)),
	-- We use a default of 43200 seconds (12 hours) for the freezing procedure.
	seconds REAL NOT NULL DEFAULT 43200.0 CHECK (must_be_strictly_positive_f32(seconds)),
	-- The device used for freezing.
	frozen_with UUID NOT NULL REFERENCES freezer_models(id) ON DELETE CASCADE,
	-- The procedure model trackable that is used for freezing.
	procedure_frozen_with INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- The container that is frozen. If this container is a container of containers, then all
	-- of the sub-containers are considered to be frozen as well.
	source_container INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	CHECK (must_be_strictly_smaller_than_f32(kelvin, 223.15)),
	-- Should always be greater than 30 minutes
	CHECK (must_be_strictly_greater_than_f32(seconds, 1800.0)),
	-- We check that the `frozen_with` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (procedure_frozen_with, frozen_with) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE,
	-- We check that the `procedure_frozen_with` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (procedure_frozen_with, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	-- We check that the `source_container` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (source_container, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS freeze_drying_procedure_models (
	procedure_model_id INTEGER PRIMARY KEY REFERENCES procedure_models(id),
	-- We use a default of 213.15 K (-60 °C) for the temperature in the freeze-drying chamber.
	kelvin REAL NOT NULL DEFAULT 213.15 CHECK (must_be_strictly_positive_f32(kelvin)),
	-- We use a default of 4 Pa for the pressure in the freeze-drying chamber.
	pascal REAL NOT NULL DEFAULT 4.0 CHECK (must_be_strictly_positive_f32(pascal)),
	-- We use a default of 3 days (259200 seconds) for the freeze-drying procedure.
	seconds REAL NOT NULL DEFAULT 259200.0 CHECK (must_be_strictly_positive_f32(seconds)),
	freeze_dried_with UUID NOT NULL REFERENCES freeze_drier_models(id) ON DELETE CASCADE,
	procedure_freeze_dried_with INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- The container that is freeze-dried. If this container is a container of containers, then all
	-- of the sub-containers are considered to be freeze-dried as well.
	source_container INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- A freeze dryer should always be used at a temperature lower than 250 K (-23 °C).
	CHECK (must_be_strictly_smaller_than_f32(kelvin, 250.0)),
	-- A freeze trier should always be used at a pressure lower than 500 Pa.
	CHECK (must_be_strictly_smaller_than_f32(pascal, 500.0)),
	-- Should always be greater than 2 hours
	CHECK (must_be_strictly_greater_than_f32(seconds, 7200.0)),
	-- Should always be less than 7 days
	CHECK (must_be_strictly_smaller_than_f32(seconds, 604800.0)),
	-- We check that the `freeze_dried_with` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (procedure_freeze_dried_with, freeze_dried_with) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE,
	-- We check that the `procedure_freeze_dried_with` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (procedure_freeze_dried_with, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	-- We check that the `source_container` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (source_container, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS weighing_procedure_models (
	procedure_model_id INTEGER PRIMARY KEY REFERENCES procedure_models(id),
	-- The instrument used for weighing.
	weighed_with UUID NOT NULL REFERENCES weighing_instrument_models(id) ON DELETE CASCADE,
	-- The instrument used for weighing should always be a trackable that is compatible with the procedure model.
	procedure_weighed_with INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- The sample container is the one that is being weighed.
	sample_container INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- We check that the `weighed_with` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (procedure_weighed_with, weighed_with) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE,
	-- We ensure that the `procedure_weighed_with` is indeed associated with the parent procedure model.
	FOREIGN KEY (procedure_weighed_with, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	-- We check that the `sample_container` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (sample_container, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS fractioning_procedure_models (
	procedure_model_id INTEGER PRIMARY KEY REFERENCES procedure_models(id),
	-- Expected amount of the fraction to be collected in kilograms.
	kilograms REAL NOT NULL CHECK (must_be_strictly_positive_f32(kilograms)),
	-- The tolerance percentage of the fraction in kilograms.
	tolerance_percentage REAL NOT NULL CHECK (must_be_strictly_positive_f32(tolerance_percentage)),
	-- The scale used to measure the fraction in kilograms.
	weighed_with UUID NOT NULL REFERENCES weighing_instrument_models(id) ON DELETE CASCADE,
	procedure_weighed_with INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- Source container from which the fraction is taken.
	source INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- Destination container to which the fraction is transferred.
	destination INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- We check that the tolerance percentage is a value between 0 and 100.
	CHECK (must_be_smaller_than_f32(tolerance_percentage, 100.0)),
	-- Foreign key to ensure that the `weighed_with` is indeed a weighing instrument.
	FOREIGN KEY (procedure_weighed_with, weighed_with) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE,
	-- Foreign key to ensure that the `procedure_weighed_with` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (procedure_weighed_with, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	-- Foreign key to ensure that the `source` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (source, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	-- Foreign key to ensure that the `destination` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (destination, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS ball_mill_procedure_models (
	procedure_model_id INTEGER PRIMARY KEY REFERENCES procedure_models(id),
	-- The time in seconds that the ball mill should be used for the procedure.
	-- By default, we set it to 150 seconds (2.5 minutes).
	seconds REAL NOT NULL DEFAULT 150.0,
	-- The time in seconds that the ball mill should be used for the procedure.
	hertz REAL NOT NULL DEFAULT 25.0,
	-- The device used for the ball mill procedure.
	milled_with UUID NOT NULL REFERENCES ball_mill_machine_models(id) ON DELETE CASCADE,
	-- The ball mill used for the procedure.
	procedure_milled_with INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- The container that is being milled.
	container_id UUID NOT NULL REFERENCES volumetric_container_models(id) ON DELETE CASCADE,
	-- The procedure container that is being milled.
	procedure_container_id INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- We check that we are not milling for more than 15 minutes.
	CHECK (must_be_strictly_smaller_than_f32(seconds, 900.0)),
	-- We check that we are milling for at least 30 seconds.
	CHECK (must_be_strictly_greater_than_f32(seconds, 30.0)),
	-- We check that the frequency is not greater than 50 Hz.
	CHECK (must_be_strictly_smaller_than_f32(hertz, 50.0)),
	-- We check that the frequency is not less than 15 Hz.
	CHECK (must_be_strictly_greater_than_f32(hertz, 15.0)),
	-- We check that the `milled_with` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (procedure_milled_with, milled_with) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE,
	-- We check that the `procedure_milled_with` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (procedure_milled_with, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	-- We check that the `container_id` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (procedure_container_id, container_id) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE,
	-- We check that the `container_id` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (procedure_container_id, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	-- We check that the `container_id` is indeed a container that can be milled with the `milled_with` device.
	FOREIGN KEY (milled_with, container_id) REFERENCES ball_mill_container_models(milled_with, container_model_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS centrifuge_procedure_models (
	procedure_model_id INTEGER PRIMARY KEY REFERENCES procedure_models(id),
	-- The time in seconds that the centrifuge should be used for the procedure.
	seconds REAL NOT NULL DEFAULT 120.0,
	-- The RPMs (rotations per minute) of the centrifuge.
	rotation_per_minute REAL NOT NULL DEFAULT 13000.0,
	-- The device used for the centrifuge procedure.
	centrifuged_with UUID NOT NULL REFERENCES centrifuge_models(id) ON DELETE CASCADE,
	procedure_centrifuged_with INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- The container that is being centrifuged.
	container_id UUID NOT NULL REFERENCES volumetric_container_models(id) ON DELETE CASCADE,
	-- The procedure container that is being centrifuged.
	procedure_container_id INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- We check that the centrifuge is not used for more than 30 minutes.
	CHECK (must_be_strictly_smaller_than_f32(seconds, 1800.0)),
	-- We check that the centrifuge is used for at least 30 seconds.
	CHECK (must_be_greater_than_f32(seconds, 30.0)),
	-- We check that the rotation per minute is not greater than 30000 RPM.
	CHECK (must_be_smaller_than_f32(rotation_per_minute, 30000.0)),
	-- We check that the rotation per minute is not less than 5000 RPM.
	CHECK (must_be_greater_than_f32(rotation_per_minute, 5000.0)),
	-- We check that the `centrifuged_with` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (procedure_centrifuged_with, centrifuged_with) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE,
	-- We check that the `procedure_centrifuged_with` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (procedure_centrifuged_with, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	-- We check that the `container_id` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (procedure_container_id, container_id) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE,
	-- We check that the `procedure_centrifuged_with` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (procedure_container_id, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	-- We check that the `container_id` is indeed a container that can be centrifuged with the `centrifuged_with` device.
	FOREIGN KEY (centrifuged_with, container_id) REFERENCES centrifugable_container_models(centrifuged_with, container_model_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS disposal_procedure_models (
	procedure_model_id INTEGER PRIMARY KEY REFERENCES procedure_models(id),
	-- The disposed trackable is the one that is being disposed of.
	disposed_id INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- The disposal procedure model should always have a trackable that is compatible with it.
	FOREIGN KEY (disposed_id, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS storage_procedure_models (
	procedure_model_id INTEGER PRIMARY KEY REFERENCES procedure_models(id),
	-- The container that is being stored.
	child_container_id UUID NOT NULL REFERENCES container_models(id) ON DELETE CASCADE,
	procedure_child_container_id INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- The container that will be used for storage.
	parent_container_id UUID NOT NULL REFERENCES container_models(id) ON DELETE CASCADE,
	procedure_parent_container_id INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- We check that the `procedure_parent_container_id` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (procedure_parent_container_id, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	-- We check that the `procedure_child_container_id` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (procedure_child_container_id, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	-- We check that the `parent_container_id` is indeed a container that is compatible with the procedure model.
	FOREIGN KEY (procedure_parent_container_id, parent_container_id) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE,
	-- We check that the `child_container_id` is indeed a container that is compatible with the procedure model.
	FOREIGN KEY (procedure_child_container_id, child_container_id) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE,
	-- We check that the `parent_container_id` is indeed a container that can hold the `child_container_id`.
	FOREIGN KEY (parent_container_id, child_container_id) REFERENCES storage_rules(parent_container_id, child_container_id) ON DELETE CASCADE
);
