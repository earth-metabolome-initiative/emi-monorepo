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
	CHECK (
		must_be_distinct_i32(
			parent_procedure_model_id,
			child_procedure_model_id
		)
	),
	PRIMARY KEY (
		parent_procedure_model_id,
		child_procedure_model_id
	)
);
CREATE TABLE IF NOT EXISTS next_procedure_models (
	parent_id INTEGER NOT NULL REFERENCES procedure_models(id) ON DELETE CASCADE,
	current_id INTEGER NOT NULL REFERENCES procedure_models(id) ON DELETE CASCADE,
	successor_id INTEGER NOT NULL REFERENCES procedure_models(id) ON DELETE CASCADE,
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	PRIMARY KEY (parent_id, current_id, successor_id),
	CHECK (must_be_distinct_i32(current_id, successor_id)),
	FOREIGN KEY (parent_id, current_id) REFERENCES parent_procedure_models(
		parent_procedure_model_id,
		child_procedure_model_id
	) ON DELETE CASCADE,
	FOREIGN KEY (parent_id, successor_id) REFERENCES parent_procedure_models(
		parent_procedure_model_id,
		child_procedure_model_id
	) ON DELETE CASCADE
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
	FOREIGN KEY (
		parent_procedure_model_id,
		child_procedure_model_id
	) REFERENCES parent_procedure_models(
		parent_procedure_model_id,
		child_procedure_model_id
	) ON DELETE CASCADE,
	CHECK (must_be_distinct_i32(parent_id, child_id)) -- TODO: add a check that ensures that the `parent_trackable_id` is compatible with the `child_trackable_id` as
	-- defined by the `trackables` table hierarchy. Specifically, the `parent_trackable_id` should be a descendant
	-- of the `child_trackable_id` as the child procedures are generally meant to be **less** specific than the parent ones.
);
--------------------------------------------------------
-- HERE BEGIN THE SPECIALIZED PROCEDURE MODELS TABLES --
--------------------------------------------------------
CREATE TABLE IF NOT EXISTS packaging_procedure_models (
	procedure_model_id INTEGER PRIMARY KEY REFERENCES procedure_models(id),
	packaged_with UUID NOT NULL REFERENCES container_models(id) ON DELETE CASCADE,
	procedure_packaged_with INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	procedure_sample_id INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- We check that the `procedure_packaged_with` is indeed assigned to the `procedure_model_id`.
	FOREIGN KEY (procedure_packaged_with, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	-- We check that the `packaged_with` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (procedure_packaged_with, packaged_with) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE,
	-- We check that the `procedure_sample_id` is indeed assigned to the `procedure_model_id`.
	FOREIGN KEY (procedure_sample_id, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS pouring_procedure_models (
	procedure_model_id INTEGER PRIMARY KEY REFERENCES procedure_models(id),
	measured_with INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	source INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- The volumetric container into which the liquid is poured.
	poured_into UUID NOT NULL REFERENCES volumetric_container_models(id) ON DELETE CASCADE,
	procedure_poured_into INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- The amount of liquid that is poured into the container.
	liters REAL NOT NULL CHECK (must_be_strictly_positive_f32(liters)),
	FOREIGN KEY (measured_with, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	FOREIGN KEY (source, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	FOREIGN KEY (procedure_poured_into, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	FOREIGN KEY (procedure_poured_into, poured_into) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS mixing_procedure_models (
	procedure_model_id INTEGER PRIMARY KEY REFERENCES procedure_models(id),
	-- The weighting device used to measure the solid.
	measured_with UUID NOT NULL REFERENCES weighing_device_models(id) ON DELETE CASCADE,
	procedure_measured_with INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	source INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- The volumetric container into which the solid is mixed.
	mixed_with UUID NOT NULL REFERENCES volumetric_container_models(id) ON DELETE CASCADE,
	procedure_mixed_into INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- The amount of solid that is mixed into the container.
	kilograms REAL NOT NULL CHECK (must_be_strictly_positive_f32(kilograms)),
	FOREIGN KEY (procedure_measured_with, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	FOREIGN KEY (procedure_measured_with, measured_with) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE,
	FOREIGN KEY (source, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	FOREIGN KEY (procedure_mixed_into, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	FOREIGN KEY (procedure_mixed_into, mixed_with) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS placing_procedure_models (
	procedure_model_id INTEGER PRIMARY KEY REFERENCES procedure_models(id),
	source INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- The volumetric container into which the trackable is placed.
	placed_into UUID NOT NULL REFERENCES volumetric_container_models(id) ON DELETE CASCADE,
	procedure_placed_into INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- The quantity of the trackable that is placed into the container.
	quantity SMALLINT NOT NULL CHECK (must_be_strictly_positive_i16(quantity)),
	FOREIGN KEY (source, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	FOREIGN KEY (procedure_placed_into, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	FOREIGN KEY (procedure_placed_into, placed_into) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS aliquoting_procedure_models (
	procedure_model_id INTEGER PRIMARY KEY REFERENCES procedure_models(id),
	-- The amount of liters that should be aliquoted.
	liters REAL NOT NULL CHECK (must_be_strictly_positive_f32(liters)),
	-- Source container from which the aliquot is taken.
	aliquoted_from UUID NOT NULL REFERENCES volumetric_container_models(id) ON DELETE CASCADE,
	procedure_aliquoted_from INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- Destination container to which the aliquot is transferred.
	aliquoted_into UUID NOT NULL REFERENCES volumetric_container_models(id) ON DELETE CASCADE,
	procedure_aliquoted_into INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- The device used for the aliquoting procedure.
	aliquoted_with UUID NOT NULL REFERENCES pipette_models(id) ON DELETE CASCADE,
	procedure_aliquoted_with INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- The pipette tip to be mounted on the pipette.
	pipette_tip UUID NOT NULL REFERENCES pipette_tip_models(id) ON DELETE CASCADE,
	procedure_pipette_tip INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- We check that the `procedure_aliquoted_with` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (procedure_aliquoted_with, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	FOREIGN KEY (procedure_aliquoted_with, aliquoted_with) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE,
	-- We check that the `procedure_pipette_tip` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (procedure_pipette_tip, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	FOREIGN KEY (procedure_pipette_tip, pipette_tip) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE,
	-- We check that the `aliquoted_with` is compatible with the `pipette_tip`.
	FOREIGN KEY (aliquoted_with, pipette_tip) REFERENCES compatibility_rules(left_trackable_id, right_trackable_id) ON DELETE CASCADE,
	-- We check that the `source` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (procedure_aliquoted_from, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	FOREIGN KEY (procedure_aliquoted_from, aliquoted_from) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE,
	-- We check that the `destination` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (procedure_aliquoted_into, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	FOREIGN KEY (procedure_aliquoted_into, aliquoted_into) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS supernatant_procedure_models (
	procedure_model_id INTEGER PRIMARY KEY REFERENCES procedure_models(id),
	-- The amount of liters that should be transferred
	liters REAL NOT NULL CHECK (must_be_strictly_positive_f32(liters)),
	-- The source container from which the supernatant is taken.
	stratified_source UUID NOT NULL REFERENCES volumetric_container_models(id),
	procedure_stratified_source INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- Destination container to which the supernatant is transferred.
	supernatant_destination UUID NOT NULL REFERENCES volumetric_container_models(id),
	procedure_supernatant_destination INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- The device used for the aliquoting procedure.
	transferred_with UUID NOT NULL REFERENCES pipette_models(id),
	procedure_transferred_with INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- The pipette tip to be mounted on the pipette.
	pipette_tip UUID NOT NULL REFERENCES pipette_tip_models(id),
	procedure_pipette_tip INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- We check that the `procedure_transferred_with` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (procedure_transferred_with, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	FOREIGN KEY (procedure_transferred_with, transferred_with) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE,
	-- We check that the `procedure_pipette_tip` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (procedure_pipette_tip, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	FOREIGN KEY (procedure_pipette_tip, pipette_tip) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE,
	-- We check that the `transferred_with` is compatible with the `pipette_tip`.
	FOREIGN KEY (transferred_with, pipette_tip) REFERENCES compatibility_rules(left_trackable_id, right_trackable_id) ON DELETE CASCADE,
	-- We check that the `procedure_stratified_source` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (procedure_stratified_source, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	FOREIGN KEY (procedure_stratified_source, stratified_source) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE,
	-- We check that the `procedure_supernatant_destination` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (
		procedure_supernatant_destination,
		procedure_model_id
	) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	FOREIGN KEY (
		procedure_supernatant_destination,
		supernatant_destination
	) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS capping_procedure_models (
	procedure_model_id INTEGER PRIMARY KEY REFERENCES procedure_models(id),
	-- The container to be capped.
	container_id UUID NOT NULL REFERENCES volumetric_container_models(id) ON DELETE CASCADE,
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
	FOREIGN KEY (container_id, capped_with) REFERENCES compatibility_rules(left_trackable_id, right_trackable_id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS weighing_procedure_models (
	procedure_model_id INTEGER PRIMARY KEY REFERENCES procedure_models(id),
	-- The instrument used for weighing.
	weighed_with UUID NOT NULL REFERENCES weighing_device_models(id) ON DELETE CASCADE,
	-- The instrument used for weighing should always be a trackable that is compatible with the procedure model.
	procedure_weighed_with INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- The sample container is the one that is being weighed.
	sample_container_id UUID NOT NULL REFERENCES volumetric_container_models(id) ON DELETE CASCADE,
	procedure_sample_container INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- We check that the `weighed_with` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (procedure_weighed_with, weighed_with) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE,
	-- We ensure that the `procedure_weighed_with` is indeed associated with the parent procedure model.
	FOREIGN KEY (procedure_weighed_with, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	-- We check that the `procedure_sample_container` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (procedure_sample_container, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	FOREIGN KEY (procedure_sample_container, sample_container_id) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS fractioning_procedure_models (
	procedure_model_id INTEGER PRIMARY KEY REFERENCES procedure_models(id),
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
CREATE TABLE IF NOT EXISTS disposal_procedure_models (
	procedure_model_id INTEGER PRIMARY KEY REFERENCES procedure_models(id),
	-- The disposed trackable is the one that is being disposed of.
	disposed_id INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- The disposal procedure model should always have a trackable that is compatible with it.
	FOREIGN KEY (disposed_id, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS storage_procedure_models (
	procedure_model_id INTEGER PRIMARY KEY REFERENCES procedure_models(id),
	-- The storage temperature in Kelvin.
	kelvin REAL NOT NULL DEFAULT 293.15 CHECK (must_be_strictly_positive_f32(kelvin)),
	-- Tolerance percentage for the storage temperature.
	kelvin_tolerance_percentage REAL NOT NULL DEFAULT 1.0 CHECK (
		must_be_strictly_positive_f32(kelvin_tolerance_percentage)
		AND must_be_smaller_than_f32(kelvin_tolerance_percentage, 100.0)
	),
	-- The container that will be used for storage.
	parent_container_id UUID NOT NULL REFERENCES container_models(id) ON DELETE CASCADE,
	procedure_parent_container_id INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- The container that is being stored.
	child_container_id UUID NOT NULL REFERENCES container_models(id) ON DELETE CASCADE,
	procedure_child_container_id INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- We check that the `procedure_parent_container_id` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (
		procedure_parent_container_id,
		procedure_model_id
	) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	-- We check that the `procedure_child_container_id` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (procedure_child_container_id, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	-- We check that the `parent_container_id` is indeed a container that is compatible with the procedure model.
	FOREIGN KEY (
		procedure_parent_container_id,
		parent_container_id
	) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE,
	-- We check that the `child_container_id` is indeed a container that is compatible with the procedure model.
	FOREIGN KEY (procedure_child_container_id, child_container_id) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE,
	-- We check that the `parent_container_id` is indeed a container that can hold the `child_container_id`.
	CONSTRAINT storage_pm_compatibility_rule FOREIGN KEY (parent_container_id, child_container_id) REFERENCES compatibility_rules(left_trackable_id, right_trackable_id)
);
CREATE TABLE IF NOT EXISTS freezing_procedure_models (
	procedure_model_id INTEGER PRIMARY KEY REFERENCES procedure_models(id),
	-- The storage temperature in Kelvin.
	kelvin REAL NOT NULL DEFAULT 203.15 CHECK (must_be_strictly_positive_f32(kelvin)),
	-- Tolerance percentage for the storage temperature.
	kelvin_tolerance_percentage REAL NOT NULL DEFAULT 5.0 CHECK (
		must_be_strictly_positive_f32(kelvin_tolerance_percentage)
		AND must_be_smaller_than_f32(kelvin_tolerance_percentage, 100.0)
	),
	-- We use a default of 43200 seconds (12 hours) for the freezing procedure.
	seconds REAL DEFAULT 43200.0 CHECK (
		must_be_strictly_positive_f32(seconds)
		AND must_be_strictly_greater_than_f32(seconds, 1800.0)
	),
	-- The device used for freezing.
	frozen_with UUID NOT NULL REFERENCES freezer_models(id) ON DELETE CASCADE,
	procedure_frozen_with INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- The container that is being stored in the freezer.
	frozen_container_id UUID NOT NULL REFERENCES container_models(id) ON DELETE CASCADE,
	procedure_frozen_container_id INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- We check that the `procedure_frozen_with` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (procedure_frozen_with, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	-- We check that the `procedure_frozen_container_id` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (
		procedure_frozen_container_id,
		procedure_model_id
	) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	-- We check that the `frozen_with` is indeed a container that is compatible with the procedure model.
	FOREIGN KEY (procedure_frozen_with, frozen_with) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE,
	-- We check that the `frozen_container_id` is indeed a container that is compatible with the procedure model.
	FOREIGN KEY (
		procedure_frozen_container_id,
		frozen_container_id
	) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE,
	-- We check that the `frozen_with` is indeed a container that can hold the `frozen_container_id`.
	CONSTRAINT freezing_pm_compatibility_rule FOREIGN KEY (frozen_with, frozen_container_id) REFERENCES compatibility_rules(left_trackable_id, right_trackable_id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS centrifuge_procedure_models (
	procedure_model_id INTEGER PRIMARY KEY REFERENCES procedure_models(id),
	-- The storage temperature in Kelvin.
	kelvin REAL NOT NULL DEFAULT 293.15 CHECK (must_be_strictly_positive_f32(kelvin)),
	-- Tolerance percentage for the storage temperature.
	kelvin_tolerance_percentage REAL NOT NULL DEFAULT 1.0 CHECK (
		must_be_strictly_positive_f32(kelvin_tolerance_percentage)
		AND must_be_smaller_than_f32(kelvin_tolerance_percentage, 100.0)
	),
	-- The time in seconds that the centrifuge should be used for the procedure.
	seconds REAL NOT NULL DEFAULT 120.0 CHECK (
		must_be_greater_than_f32(seconds, 30.0)
		AND must_be_smaller_than_f32(seconds, 1800.0)
	),
	-- The RPMs (rotations per minute) of the centrifuge.
	rotation_per_minute REAL NOT NULL DEFAULT 13000.0 CHECK (
		must_be_greater_than_f32(rotation_per_minute, 5000.0)
		AND must_be_smaller_than_f32(rotation_per_minute, 30000.0)
	),
	-- The device used for the centrifuge procedure.
	centrifuged_with UUID NOT NULL REFERENCES centrifuge_models(id) ON DELETE CASCADE,
	procedure_centrifuged_with INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- The container that is being centrifuged.
	centrifuged_container_id UUID NOT NULL REFERENCES volumetric_container_models(id) ON DELETE CASCADE,
	procedure_centrifuged_container_id INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- We check that the `procedure_centrifuged_with` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (procedure_centrifuged_with, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	-- We check that the `procedure_centrifuged_container_id` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (procedure_centrifuged_container_id, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	-- We check that the `centrifuged_with` is indeed a container that is compatible with the procedure model.
	FOREIGN KEY (procedure_centrifuged_with, centrifuged_with) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE,
	-- We check that the `procedure_centrifuged_container_id` is indeed a container that is compatible with the procedure model.
	FOREIGN KEY (procedure_centrifuged_container_id, centrifuged_container_id) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE,
	-- We check that the `centrifuged_with` is indeed a container that can hold the `centrifuged_with`.
	CONSTRAINT centrifuge_pm_compatibility_rule FOREIGN KEY (centrifuged_with, centrifuged_container_id) REFERENCES compatibility_rules(left_trackable_id, right_trackable_id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS ball_mill_procedure_models (
	procedure_model_id INTEGER PRIMARY KEY REFERENCES procedure_models(id),
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
	FOREIGN KEY (procedure_milled_container_id, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	-- We check that the `milled_with` is indeed a container that is compatible with the procedure model.
	FOREIGN KEY (procedure_milled_with, milled_with) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE,
	-- We check that the `milled_container_id` is indeed a container that is compatible with the procedure model.
	FOREIGN KEY (procedure_milled_container_id, milled_container_id) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE,
	-- We check that the `milled_with` is indeed a ball mill machine that can hold the `milled_container_id`.
	CONSTRAINT ball_mill_pm_compatibility_rule FOREIGN KEY (milled_with, milled_container_id) REFERENCES compatibility_rules(left_trackable_id, right_trackable_id) ON DELETE CASCADE
);
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
	FOREIGN KEY (procedure_freeze_dried_container_id, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	-- We check that the `freeze_dried_with` is indeed a container that is compatible with the procedure model.
	FOREIGN KEY (procedure_freeze_dried_with, freeze_dried_with) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE,
	-- We check that the `freeze_dried_container_id` is indeed a container that is compatible with the procedure model.
	FOREIGN KEY (procedure_freeze_dried_container_id, freeze_dried_container_id) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE,
	-- We check that the `freeze_dried_container_id` is indeed a freeze drier that can hold the `freeze_dried_with`.
	CONSTRAINT freeze_drying_pm_compatibility_rule FOREIGN KEY (freeze_dried_with, freeze_dried_container_id) REFERENCES compatibility_rules(left_trackable_id, right_trackable_id)
);
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
CREATE TABLE IF NOT EXISTS photograph_procedure_models (
	procedure_model_id INTEGER PRIMARY KEY REFERENCES procedure_models(id),
	-- The device used for photograph.
	photographed_with UUID NOT NULL REFERENCES camera_models(id) ON DELETE CASCADE,
	procedure_photographed_with INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	trackable_id INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	-- We check that the `photographed_with` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (procedure_photographed_with, photographed_with) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE,
	-- We check that the `procedure_photographed_with` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (procedure_photographed_with, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	-- We check that the `trackable_id` is indeed a trackable that is compatible with the procedure model.
	FOREIGN KEY (trackable_id, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS binary_question_procedure_models (
	procedure_model_id INTEGER PRIMARY KEY REFERENCES procedure_models(id),
	-- The trackable that is associated to the question.
	trackable_id INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE
);