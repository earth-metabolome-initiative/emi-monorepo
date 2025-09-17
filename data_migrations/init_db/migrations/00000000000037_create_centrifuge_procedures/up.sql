CREATE TABLE IF NOT EXISTS centrifuge_procedure_templates (
	procedure_template INTEGER PRIMARY KEY REFERENCES procedure_templates(procedure_template) ON DELETE CASCADE,
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
	centrifuged_with_model INTEGER NOT NULL REFERENCES centrifuge_models(id),
	procedure_template_centrifuged_with_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id) ON DELETE CASCADE,
	-- The container that is being centrifuged.
	centrifuged_container_model INTEGER NOT NULL REFERENCES volumetric_container_models(id),
	-- The centrifuged container model should allways be an asset model that is compatible with the procedure template.
	procedure_template_centrifuged_container_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- We check that the `centrifuged_with_model` is indeed a instrument that is compatible with the procedure template.
	FOREIGN KEY (
		procedure_template_centrifuged_with_model,
		centrifuged_with_model
	) REFERENCES procedure_template_asset_models(id, asset_model),
	-- We check that the `procedure_template_centrifuged_container_model` is indeed a container that is compatible with the procedure template.
	FOREIGN KEY (
		procedure_template_centrifuged_container_model,
		centrifuged_container_model
	) REFERENCES procedure_template_asset_models(id, asset_model),
	-- We check that the `centrifuged_with_model` is indeed a container that can hold the `centrifuged_with_model`.
	CONSTRAINT centrifuge_pm_compatibility_rules FOREIGN KEY (
		centrifuged_with_model,
		centrifuged_container_model
	) REFERENCES asset_compatibility_rules(left_asset_model, right_asset_model),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_centrifuged_with_model`
	-- for the current `procedure_template`.
	UNIQUE (
		procedure_template,
		procedure_template_centrifuged_with_model
	),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_centrifuged_container_model`
	-- for the current `procedure_template`.
	UNIQUE (
		procedure_template,
		procedure_template_centrifuged_container_model
	)
);
CREATE TABLE IF NOT EXISTS centrifuge_procedures (
	-- Identifier of the centrifuge procedure, which is also a foreign key to the general procedure.
	procedure UUID PRIMARY KEY REFERENCES procedures(procedure) ON DELETE CASCADE,
	-- We enforce that the model of this procedure must be a centrifuge procedure template.
	procedure_template INTEGER NOT NULL REFERENCES centrifuge_procedure_templates(procedure_template),
	-- The container that is being centrifuged, which must be a volumetric container.
	centrifuged_container UUID NOT NULL REFERENCES volumetric_containers(id),
	-- The model of the container that is being centrifuged.
	centrifuged_container_model INTEGER NOT NULL REFERENCES volumetric_container_models(id),
	-- The procedure template asset model associated to the `centrifuged_container`.
	procedure_template_centrifuged_container_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The procedure asset associated to the `centrifuged_container`.
	procedure_centrifuged_container UUID NOT NULL REFERENCES procedure_assets(id) ON DELETE CASCADE,
	-- The centrifuge model used for the centrifuge procedure.
	centrifuged_with_model INTEGER NOT NULL REFERENCES centrifuge_models(id),
	-- The centrifuge used for the procedure. This field is optional because the centrifuge
	-- might not have been recorded at the time of performing the procedure.
	centrifuged_with UUID REFERENCES centrifuges(id),
	-- The procedure template asset model associated to the `centrifuged_with_model`.
	procedure_template_centrifuged_with_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The procedure asset associated to the `centrifuged_with`.
	procedure_centrifuged_with UUID NOT NULL REFERENCES procedure_assets(id) ON DELETE CASCADE,
	-- We enforce that the extended `procedure` has indeed the same `procedure_template`, making
	-- sure that the procedure is a centrifugating procedure.
	FOREIGN KEY (procedure, procedure_template) REFERENCES procedures(procedure, procedure_template),
	-- We enforce that the specified `centrifuged_with` is of the specified `centrifuged_with_model`.
	FOREIGN KEY (centrifuged_with, centrifuged_with_model) REFERENCES assets(id, model),
	-- The procedure template asset model describing the `centrifuged_container` must be the same one
	-- as the one in the procedure template.
	FOREIGN KEY (
		procedure_template,
		procedure_template_centrifuged_container_model
	) REFERENCES centrifuge_procedure_templates(
		procedure_template,
		procedure_template_centrifuged_container_model
	),
	-- The procedure template asset model describing the `centrifuged_with_model` must be the same one
	-- as the one in the procedure template.
	FOREIGN KEY (
		procedure_template,
		procedure_template_centrifuged_with_model
	) REFERENCES centrifuge_procedure_templates(
		procedure_template,
		procedure_template_centrifuged_with_model
	),
	-- We enforce that the specified `procedure_centrifuged_container` is of the specified `procedure_template_centrifuged_container_model`.
	FOREIGN KEY (
		procedure_centrifuged_container,
		procedure_template_centrifuged_container_model
	) REFERENCES procedure_assets(id, procedure_template_asset_model),
	-- We enforce that the specified `procedure_centrifuged_with` is of the specified `procedure_template_centrifuged_with_model`.
	FOREIGN KEY (
		procedure_centrifuged_with,
		procedure_template_centrifuged_with_model
	) REFERENCES procedure_assets(id, procedure_template_asset_model),
	-- We check that the `centrifuged_with_model` is indeed a instrument that is compatible with the `centrifuged_container_model`.
	FOREIGN KEY (
		centrifuged_with_model,
		centrifuged_container_model
	) REFERENCES asset_compatibility_rules(left_asset_model, right_asset_model),
	-- We ensure that the `procedure_centrifuged_container` is associated with the `centrifuged_container_model`.
	FOREIGN KEY (
		procedure_centrifuged_container,
		centrifuged_container_model
	) REFERENCES procedure_assets(id, asset_model),
	-- We ensure that the `procedure_centrifuged_container` is associated with the `centrifuged_container`.
	FOREIGN KEY (
		procedure_centrifuged_container,
		centrifuged_container
	) REFERENCES procedure_assets(id, asset),
	-- We ensure that the `procedure_centrifuged_with` is associated with the `centrifuged_container`.
	FOREIGN KEY (
		procedure_centrifuged_with,
		centrifuged_with_model
	) REFERENCES procedure_assets(id, asset_model),
	-- We ensure that the `procedure_centrifuged_with` is associated with the `centrifuged_with`.
	FOREIGN KEY (procedure_centrifuged_with, centrifuged_with) REFERENCES procedure_assets(id, asset)
);