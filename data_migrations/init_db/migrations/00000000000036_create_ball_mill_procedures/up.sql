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
	-- Foreign procedure template which originated the container being milled (e.g., a sampling or fractioning procedure template).
	foreign_procedure_template INTEGER NOT NULL REFERENCES procedure_templates(procedure_template) CHECK (
		must_be_distinct_i32(
			procedure_template,
			foreign_procedure_template
		)
	),
	procedure_template_milled_container_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- We check that the `bead_model` is indeed a beads model from the associated procedure template assets.
	FOREIGN KEY (
		procedure_template_bead_model,
		procedure_template
	) REFERENCES procedure_template_asset_models(id, procedure_template),
	FOREIGN KEY (procedure_template_bead_model, bead_model) REFERENCES procedure_template_asset_models(id, asset_model),
	-- We check that the `procedure_template_milled_with_model` is indeed an asset model that is compatible with the procedure template.
	FOREIGN KEY (
		procedure_template_milled_with_model,
		procedure_template
	) REFERENCES procedure_template_asset_models(id, procedure_template),
	-- We check that the `procedure_template_milled_container_model` is indeed an asset model that is compatible with the procedure template.
	FOREIGN KEY (
		procedure_template_milled_container_model,
		foreign_procedure_template
	) REFERENCES procedure_template_asset_models(id, procedure_template),
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
	CONSTRAINT ball_mill_pm_container_compatibility_rules FOREIGN KEY (milled_with_model, milled_container_model) REFERENCES asset_compatibility_rules(left_asset_model, right_asset_model),
	-- We check that the `milled_with` is indeed a ball mill machine that can use the `bead_model`.
	CONSTRAINT ball_mill_pm_beads_compatibility_rules FOREIGN KEY (milled_with_model, bead_model) REFERENCES asset_compatibility_rules(left_asset_model, right_asset_model),
	-- We check that the `bead_model` is indeed a beads model that can be used with the `milled_container_model`.
	CONSTRAINT ball_mill_pm_beads_container_compatibility_rules FOREIGN KEY (bead_model, milled_container_model) REFERENCES asset_compatibility_rules(left_asset_model, right_asset_model),
	-- We define the same-as index to allow for foreign key references to check wether a `ball_mill_procedure_template` is associated with a given `ball_mill_foreign_procedure_template`.
	UNIQUE (procedure_template, foreign_procedure_template)
);
CREATE TABLE IF NOT EXISTS ball_mill_procedures (
	procedure UUID PRIMARY KEY REFERENCES procedures(procedure) ON DELETE CASCADE,
	procedure_template INTEGER NOT NULL REFERENCES ball_mill_procedure_templates(procedure_template),
	-- The procedure template associated with the foreign procedure template.
	foreign_procedure_template INTEGER NOT NULL REFERENCES procedure_templates(procedure_template) CHECK (
		must_be_distinct_i32(
			procedure_template,
			foreign_procedure_template
		)
	),
	-- The foreign procedure that has populated the container being centrifuged (e.g., a sampling or fractioning procedure).
	foreign_procedure UUID NOT NULL REFERENCES procedures(procedure) CHECK (
		must_be_distinct_uuid(procedure, foreign_procedure)
	),
	-- The beads model used for the procedure.
	bead_model INTEGER NOT NULL REFERENCES bead_models(id),
	-- The device used for the ball mill procedure.
	milled_with_model INTEGER NOT NULL REFERENCES ball_mill_machine_models(id),
	milled_with UUID REFERENCES ball_mill_machines(id),
	-- The container that is being milled.
	milled_container UUID NOT NULL REFERENCES volumetric_containers(id),
	-- We enforce that the extended `procedure` has indeed the same `procedure_template`, making
	-- sure that the procedure is a ball mill procedure without the possibility of a mistake.
	FOREIGN KEY (procedure, procedure_template) REFERENCES procedures(procedure, procedure_template),
	-- We enforce that the `bead_model` is indeed a procedure asset of the correct model.
	FOREIGN KEY (procedure, bead_model) REFERENCES procedure_assets(procedure, asset_model),
	-- We enforce that the `milled_with_model` is indeed a procedure asset model.
	FOREIGN KEY (procedure, milled_with_model) REFERENCES procedure_assets(procedure, asset_model),
	-- And that the `milled_with` is indeed a procedure asset, if it is not NULL.
	FOREIGN KEY (procedure, milled_with) REFERENCES procedure_assets(procedure, asset),
	-- We enforce that the specified ball mill machine has as parent model the specified model.
	FOREIGN KEY (milled_with, milled_with_model) REFERENCES assets(id, model),
	-- Additionally, we enforce that the `milled_container` is indeed a procedure asset.
	FOREIGN KEY (foreign_procedure, milled_container) REFERENCES procedure_assets(procedure, asset),
	-- We enforce that the associated procedure template requires the provided foreign procedure template.
	FOREIGN KEY (procedure_template, foreign_procedure_template) REFERENCES ball_mill_procedure_templates(procedure_template, foreign_procedure_template)
);