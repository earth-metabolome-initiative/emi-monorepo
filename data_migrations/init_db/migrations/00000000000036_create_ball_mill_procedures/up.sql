CREATE TABLE IF NOT EXISTS ball_mill_procedure_templates (
	procedure_template INTEGER PRIMARY KEY REFERENCES procedure_templates(procedure_template) ON DELETE CASCADE,
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
	-- The beads model used for the procedure.
	bead_model INTEGER NOT NULL REFERENCES bead_models(id),
	procedure_template_bead_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id) ON DELETE CASCADE,
	--- The number of beads used in the procedure.
	number_of_beads SMALLINT NOT NULL DEFAULT 3 CHECK (must_be_strictly_positive_i16(number_of_beads)),
	-- The device used for the ball mill procedure.
	milled_with_model INTEGER NOT NULL REFERENCES ball_mill_machine_models(id),
	procedure_template_milled_with_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id) ON DELETE CASCADE,
	-- The container that is being milled.
	milled_container_model INTEGER NOT NULL REFERENCES volumetric_container_models(id),
	procedure_template_milled_container_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	FOREIGN KEY (procedure_template_bead_model, bead_model) REFERENCES procedure_template_asset_models(id, asset_model),
	-- We check that the `milled_with` is indeed a container that is compatible with the procedure template.
	FOREIGN KEY (
		procedure_template_milled_with_model,
		milled_with_model
	) REFERENCES procedure_template_asset_models(id, asset_model),
	-- We check that the `milled_container_model` is indeed a container that is compatible with the procedure template.
	FOREIGN KEY (
		procedure_template_milled_container_model,
		milled_container_model
	) REFERENCES procedure_template_asset_models(id, asset_model),
	-- We check that the `milled_with` is indeed a ball mill machine that can hold the `milled_container_model`.
	FOREIGN KEY (milled_with_model, milled_container_model) REFERENCES asset_compatibility_rules(left_asset_model, right_asset_model),
	-- We check that the `milled_with` is indeed a ball mill machine that can use the `bead_model`.
	FOREIGN KEY (milled_with_model, bead_model) REFERENCES asset_compatibility_rules(left_asset_model, right_asset_model),
	-- We check that the `bead_model` is indeed a beads model that can be used with the `milled_container_model`.
	FOREIGN KEY (bead_model, milled_container_model) REFERENCES asset_compatibility_rules(left_asset_model, right_asset_model),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_bead_model`
	-- for the current `procedure_template`.
	UNIQUE (
		procedure_template,
		procedure_template_bead_model
	),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_milled_with_model`
	-- for the current `procedure_template`.
	UNIQUE (
		procedure_template,
		procedure_template_milled_with_model
	),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_milled_container_model`
	-- for the current `procedure_template`.
	UNIQUE (
		procedure_template,
		procedure_template_milled_container_model
	)
);
CREATE TABLE IF NOT EXISTS ball_mill_procedures (
	procedure UUID PRIMARY KEY REFERENCES procedures(procedure) ON DELETE CASCADE,
	procedure_template INTEGER NOT NULL REFERENCES ball_mill_procedure_templates(procedure_template),
	-- The beads model used for the procedure.
	bead_model INTEGER NOT NULL REFERENCES bead_models(id),
	-- The procedure template asset model associated to the `bead_model`.
	procedure_template_bead_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The procedure asset associated to the `bead_model`.
	procedure_bead UUID NOT NULL REFERENCES procedure_assets(id) ON DELETE CASCADE,
	-- The device used for the ball mill procedure.
	milled_with_model INTEGER NOT NULL REFERENCES ball_mill_machine_models(id),
	-- The procedure template asset model associated to the `milled_with_model`.
	procedure_template_milled_with_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The procedure asset associated to the `milled_with_model`.
	procedure_milled_with UUID NOT NULL REFERENCES procedure_assets(id) ON DELETE CASCADE,
	-- The ball mill machine used for the procedure. This field is optional because the ball mill
	-- machine might not have been recorded at the time of performing the procedure.
	milled_with UUID REFERENCES ball_mill_machines(id),
	-- The container that is being milled.
	milled_container UUID NOT NULL REFERENCES volumetric_containers(id),
	-- The container model that is being milled.
	milled_container_model INTEGER NOT NULL REFERENCES volumetric_container_models(id),
	-- The procedure template asset model associated to the `milled_container`.
	procedure_template_milled_container_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The procedure asset associated to the `milled_container`.
	procedure_milled_container UUID NOT NULL REFERENCES procedure_assets(id) ON DELETE CASCADE,
	-- We enforce that the extended `procedure` has indeed the same `procedure_template`, making
	-- sure that the procedure is a ball mill procedure without the possibility of a mistake.
	FOREIGN KEY (procedure, procedure_template) REFERENCES procedures(procedure, procedure_template),
	-- The procedure template asset model describing the `bead_model` must be the same one
	-- as the one in the procedure template.
	FOREIGN KEY (
		procedure_template,
		procedure_template_bead_model
	) REFERENCES ball_mill_procedure_templates(
		procedure_template,
		procedure_template_bead_model
	),
	-- The procedure template asset model describing the `milled_with_model` must be the same one
	-- as the one in the procedure template.
	FOREIGN KEY (
		procedure_template,
		procedure_template_milled_with_model
	) REFERENCES ball_mill_procedure_templates(
		procedure_template,
		procedure_template_milled_with_model
	),
	-- The procedure template asset model describing the `milled_container_model` must be the same one
	-- as the one in the procedure template.
	FOREIGN KEY (
		procedure_template,
		procedure_template_milled_container_model
	) REFERENCES ball_mill_procedure_templates(
		procedure_template,
		procedure_template_milled_container_model
	),
	-- We enforce that the procedure template asset model reported in the procedure is indeed
	-- the same one associated to the procedure asset for the asset model `bead_model`.
	FOREIGN KEY (procedure_bead, procedure_template_bead_model) REFERENCES procedure_assets(id, procedure_template_asset_model),
	-- We enforce that the procedure template asset model reported in the procedure is indeed
	-- the same one associated to the procedure asset for the asset model `milled_with_model`.
	FOREIGN KEY (
		procedure_milled_with,
		procedure_template_milled_with_model
	) REFERENCES procedure_assets(id, procedure_template_asset_model),
	-- We enforce that the procedure template asset model reported in the procedure is indeed
	-- the same one associated to the procedure asset for the asset model `milled_container_model`.
	FOREIGN KEY (
		procedure_milled_container,
		procedure_template_milled_container_model
	) REFERENCES procedure_assets(id, procedure_template_asset_model),
	-- We enforce that the `procedure_milled_container` procedure asset is indeed associated to the `milled_container_model`.
	FOREIGN KEY (
		procedure_milled_container,
		milled_container_model
	) REFERENCES procedure_assets(id, asset_model),
	-- We enforce that the `procedure_milled_with` procedure asset is indeed associated to the `milled_with_model`.
	FOREIGN KEY (procedure_milled_with, milled_with_model) REFERENCES procedure_assets(id, asset_model),
	-- We enforce that the `procedure_milled_with` procedure asset is indeed associated to the `milled_with`.
	FOREIGN KEY (procedure_milled_with, milled_with) REFERENCES procedure_assets(id, asset),
	-- We enforce that the `procedure_bead` procedure asset is indeed associated to the `bead_model`.
	FOREIGN KEY (procedure_bead, bead_model) REFERENCES procedure_assets(id, asset_model),
	-- We enforce that the `procedure_milled_container` procedure asset is indeed associated to the `milled_container`.
	FOREIGN KEY (procedure_milled_container, milled_container) REFERENCES procedure_assets(id, asset),
	-- We check that the `milled_with` is indeed a ball mill machine that can hold the `milled_container_model`.
	FOREIGN KEY (milled_with_model, milled_container_model) REFERENCES asset_compatibility_rules(left_asset_model, right_asset_model),
	-- We check that the `milled_with` is indeed a ball mill machine that can use the `bead_model`.
	FOREIGN KEY (milled_with_model, bead_model) REFERENCES asset_compatibility_rules(left_asset_model, right_asset_model),
	-- We check that the `bead_model` is indeed a beads model that can be used with the `milled_container_model`.
	FOREIGN KEY (bead_model, milled_container_model) REFERENCES asset_compatibility_rules(left_asset_model, right_asset_model)
);