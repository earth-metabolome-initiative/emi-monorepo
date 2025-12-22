CREATE TABLE IF NOT EXISTS ball_mill_procedure_templates (
	id INTEGER PRIMARY KEY REFERENCES procedure_templates(id) ON DELETE CASCADE,
	-- The storage temperature in Kelvin.
	kelvin REAL NOT NULL DEFAULT 293.15 CHECK (kelvin > 0.0),
	-- Tolerance percentage for the storage temperature.
	kelvin_tolerance_percentage REAL NOT NULL DEFAULT 1.0 CHECK (
		kelvin_tolerance_percentage > 0.0
		AND kelvin_tolerance_percentage <= 100.0
	),
	-- Duration in seconds. By default, we set it to 150 seconds (2.5 minutes).
	duration REAL NOT NULL DEFAULT 150.0 CHECK (
		duration <= 900.0
		AND duration >= 30.0
	),
	-- The frequency in hertz at which the ball mill should operate during the procedure.
	hertz REAL NOT NULL DEFAULT 25.0 CHECK (
		hertz <= 50.0
		AND hertz >= 15.0
	),
	-- The beads model used for the procedure.
	bead_model_id INTEGER NOT NULL REFERENCES bead_models(id),
	procedure_template_bead_model_id INTEGER NOT NULL REFERENCES procedure_template_asset_models(id) ON DELETE CASCADE,
	--- The count of beads used in the procedure.
	bead_count SMALLINT NOT NULL DEFAULT 3 CHECK (bead_count > 0),
	-- The device used for the ball mill procedure.
	milled_with_model_id INTEGER NOT NULL REFERENCES ball_mill_machine_models(id),
	procedure_template_milled_with_model_id INTEGER NOT NULL REFERENCES procedure_template_asset_models(id) ON DELETE CASCADE,
	-- The container that is being milled.
	milled_container_model_id INTEGER NOT NULL REFERENCES volumetric_container_models(id),
	procedure_template_milled_container_model_id INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	FOREIGN KEY (procedure_template_bead_model_id, bead_model_id) REFERENCES procedure_template_asset_models(id, asset_model_id),
	-- We check that the `milled_with` is indeed a container that is compatible with the procedure_id template.
	FOREIGN KEY (
		procedure_template_milled_with_model_id,
		milled_with_model_id
	) REFERENCES procedure_template_asset_models(id, asset_model_id),
	-- We check that the `milled_container_model` is indeed a container that is compatible with the procedure_id template.
	FOREIGN KEY (
		procedure_template_milled_container_model_id,
		milled_container_model_id
	) REFERENCES procedure_template_asset_models(id, asset_model_id),
	-- We check that the `milled_with` is indeed a ball mill machine that can hold the `milled_container_model`.
	FOREIGN KEY (milled_with_model_id, milled_container_model_id) REFERENCES asset_compatibility_rules(left_asset_model_id, right_asset_model_id),
	-- We check that the `milled_with` is indeed a ball mill machine that can use the `bead_model`.
	FOREIGN KEY (milled_with_model_id, bead_model_id) REFERENCES asset_compatibility_rules(left_asset_model_id, right_asset_model_id),
	-- We check that the `bead_model` is indeed a beads model that can be used with the `milled_container_model`.
	FOREIGN KEY (bead_model_id, milled_container_model_id) REFERENCES asset_compatibility_rules(left_asset_model_id, right_asset_model_id),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_bead_model`
	-- for the current `procedure_template`.
	UNIQUE (
		id,
		procedure_template_bead_model_id
	),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_milled_with_model`
	-- for the current `procedure_template`.
	UNIQUE (
		id,
		procedure_template_milled_with_model_id
	),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_milled_container_model`
	-- for the current `procedure_template`.
	UNIQUE (
		id,
		procedure_template_milled_container_model_id
	)
);
CREATE TABLE IF NOT EXISTS ball_mill_procedures (
	id UUID PRIMARY KEY REFERENCES procedures(id) ON DELETE CASCADE,
	ball_mill_procedure_template_id INTEGER NOT NULL REFERENCES ball_mill_procedure_templates(id),
	-- The beads model used for the procedure.
	bead_model_id INTEGER NOT NULL REFERENCES bead_models(id),
	-- The procedure_id template asset model associated to the `bead_model`.
	procedure_template_bead_model_id INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The procedure_id asset associated to the `bead_model`.
	procedure_bead_id UUID NOT NULL REFERENCES procedure_assets(id) ON DELETE CASCADE,
	-- The device used for the ball mill procedure.
	milled_with_model_id INTEGER NOT NULL REFERENCES ball_mill_machine_models(id),
	-- The procedure_id template asset model associated to the `milled_with_model`.
	procedure_template_milled_with_model_id INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The procedure_id asset associated to the `milled_with_model`.
	procedure_milled_with_id UUID NOT NULL REFERENCES procedure_assets(id) ON DELETE CASCADE,
	-- The ball mill machine used for the procedure. This field is optional because the ball mill
	-- machine might not have been recorded at the time of performing the procedure.
	milled_with_id UUID REFERENCES ball_mill_machines(id),
	-- The container that is being milled.
	milled_container_id UUID NOT NULL REFERENCES volumetric_containers(id),
	-- The container model that is being milled.
	milled_container_model_id INTEGER NOT NULL REFERENCES volumetric_container_models(id),
	-- The procedure_id template asset model associated to the `milled_container`.
	procedure_template_milled_container_model_id INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The procedure_id asset associated to the `milled_container`.
	procedure_milled_container_id UUID NOT NULL REFERENCES procedure_assets(id) ON DELETE CASCADE,
	-- We enforce that the extended `procedure` has indeed the same `procedure_template`, making
	-- sure that the procedure_id is a ball mill procedure_id without the possibility of a mistake.
	FOREIGN KEY (id, ball_mill_procedure_template_id) REFERENCES procedures(id, procedure_template_id),
	-- The procedure_id template asset model describing the `bead_model` must be the same one
	-- as the one in the procedure_id template.
	FOREIGN KEY (
		ball_mill_procedure_template_id,
		procedure_template_bead_model_id
	) REFERENCES ball_mill_procedure_templates(
		id,
		procedure_template_bead_model_id
	),
	-- The procedure_id template asset model describing the `milled_with_model` must be the same one
	-- as the one in the procedure_id template.
	FOREIGN KEY (
		ball_mill_procedure_template_id,
		procedure_template_milled_with_model_id
	) REFERENCES ball_mill_procedure_templates(
		id,
		procedure_template_milled_with_model_id
	),
	-- The procedure_id template asset model describing the `milled_container_model` must be the same one
	-- as the one in the procedure_id template.
	FOREIGN KEY (
		ball_mill_procedure_template_id,
		procedure_template_milled_container_model_id
	) REFERENCES ball_mill_procedure_templates(
		id,
		procedure_template_milled_container_model_id
	),
	-- We enforce that the procedure_id template asset model reported in the procedure_id is indeed
	-- the same one associated to the procedure_id asset for the asset model `bead_model`.
	FOREIGN KEY (procedure_bead_id, procedure_template_bead_model_id) REFERENCES procedure_assets(id, procedure_template_asset_model_id),
	-- We enforce that the procedure_id template asset model reported in the procedure_id is indeed
	-- the same one associated to the procedure_id asset for the asset model `milled_with_model`.
	FOREIGN KEY (
		procedure_milled_with_id,
		procedure_template_milled_with_model_id
	) REFERENCES procedure_assets(id, procedure_template_asset_model_id),
	-- We enforce that the procedure_id template asset model reported in the procedure_id is indeed
	-- the same one associated to the procedure_id asset for the asset model `milled_container_model`.
	FOREIGN KEY (
		procedure_milled_container_id,
		procedure_template_milled_container_model_id
	) REFERENCES procedure_assets(id, procedure_template_asset_model_id),
	-- We enforce that the `procedure_milled_container` procedure_id asset is indeed associated to the `milled_container_model`.
	FOREIGN KEY (
		procedure_milled_container_id,
		milled_container_model_id
	) REFERENCES procedure_assets(id, asset_model_id),
	-- We enforce that the `procedure_milled_with` procedure_id asset is indeed associated to the `milled_with_model`.
	FOREIGN KEY (procedure_milled_with_id, milled_with_model_id) REFERENCES procedure_assets(id, asset_model_id),
	-- We enforce that the `procedure_milled_with` procedure_id asset is indeed associated to the `milled_with`.
	FOREIGN KEY (procedure_milled_with_id, milled_with_id) REFERENCES procedure_assets(id, asset_id),
	-- We enforce that the `procedure_bead_id` procedure_id asset is indeed associated to the `bead_model`.
	FOREIGN KEY (procedure_bead_id, bead_model_id) REFERENCES procedure_assets(id, asset_model_id),
	-- We enforce that the `procedure_milled_container` procedure_id asset is indeed associated to the `milled_container`.
	FOREIGN KEY (procedure_milled_container_id, milled_container_id) REFERENCES procedure_assets(id, asset_id),
	-- We check that the `milled_with` is indeed a ball mill machine that can hold the `milled_container_model`.
	FOREIGN KEY (milled_with_model_id, milled_container_model_id) REFERENCES asset_compatibility_rules(left_asset_model_id, right_asset_model_id),
	-- We check that the `milled_with` is indeed a ball mill machine that can use the `bead_model`.
	FOREIGN KEY (milled_with_model_id, bead_model_id) REFERENCES asset_compatibility_rules(left_asset_model_id, right_asset_model_id),
	-- We check that the `bead_model` is indeed a beads model that can be used with the `milled_container_model`.
	FOREIGN KEY (bead_model_id, milled_container_model_id) REFERENCES asset_compatibility_rules(left_asset_model_id, right_asset_model_id)
);