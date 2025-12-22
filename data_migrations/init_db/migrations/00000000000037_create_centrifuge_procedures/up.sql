CREATE TABLE IF NOT EXISTS centrifuge_procedure_templates (
	id INTEGER PRIMARY KEY REFERENCES procedure_templates(id) ON DELETE CASCADE,
	-- The storage temperature in Kelvin.
	kelvin REAL NOT NULL DEFAULT 293.15 CHECK (kelvin > 0.0),
	-- Tolerance percentage for the storage temperature.
	kelvin_tolerance_percentage REAL NOT NULL DEFAULT 1.0 CHECK (
		kelvin_tolerance_percentage > 0.0
		AND kelvin_tolerance_percentage <= 100.0
	),
	-- Duration in seconds that the centrifuge should be used for the procedure.
	duration REAL NOT NULL DEFAULT 120.0 CHECK (
		duration >= 30.0
		AND duration <= 1800.0
	),
	-- The RPMs (rotations per minute) of the centrifuge.
	rotation_per_minute REAL NOT NULL DEFAULT 13000.0 CHECK (
		rotation_per_minute >= 5000.0
		AND rotation_per_minute <= 30000.0
	),
	-- The device used for the centrifuge procedure.
	centrifuged_with_model_id INTEGER NOT NULL REFERENCES centrifuge_models(id),
	procedure_template_centrifuged_with_model_id INTEGER NOT NULL REFERENCES procedure_template_asset_models(id) ON DELETE CASCADE,
	-- The container that is being centrifuged.
	centrifuged_container_model_id INTEGER NOT NULL REFERENCES volumetric_container_models(id),
	-- The centrifuged container model should allways be an asset model that is compatible with the procedure_id template.
	procedure_template_centrifuged_container_model_id INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- We check that the `centrifuged_with_model` is indeed a instrument that is compatible with the procedure_id template.
	FOREIGN KEY (
		procedure_template_centrifuged_with_model_id,
		centrifuged_with_model_id
	) REFERENCES procedure_template_asset_models(id, asset_model_id),
	-- We check that the `procedure_template_centrifuged_container_model` is indeed a container that is compatible with the procedure_id template.
	FOREIGN KEY (
		procedure_template_centrifuged_container_model_id,
		centrifuged_container_model_id
	) REFERENCES procedure_template_asset_models(id, asset_model_id),
	-- We check that the `centrifuged_with_model` is indeed a container that can hold the `centrifuged_with_model`.
	CONSTRAINT centrifuge_pm_compatibility_rules FOREIGN KEY (
		centrifuged_with_model_id,
		centrifuged_container_model_id
	) REFERENCES asset_compatibility_rules(left_asset_model_id, right_asset_model_id),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_centrifuged_with_model`
	-- for the current `procedure_template`.
	UNIQUE (
		id,
		procedure_template_centrifuged_with_model_id
	),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_centrifuged_container_model`
	-- for the current `procedure_template`.
	UNIQUE (
		id,
		procedure_template_centrifuged_container_model_id
	)
);
CREATE TABLE IF NOT EXISTS centrifuge_procedures (
	-- Identifier of the centrifuge id, which is also a foreign key to the general procedure.
	id UUID PRIMARY KEY REFERENCES procedures(id) ON DELETE CASCADE,
	-- We enforce that the model of this procedure_id must be a centrifuge procedure_id template.
	centrifuge_procedure_template_id INTEGER NOT NULL REFERENCES centrifuge_procedure_templates(id),
	-- The container that is being centrifuged, which must be a volumetric container.
	centrifuged_container_id UUID NOT NULL REFERENCES volumetric_containers(id),
	-- The model of the container that is being centrifuged.
	centrifuged_container_model_id INTEGER NOT NULL REFERENCES volumetric_container_models(id),
	-- The procedure_id template asset model associated to the `centrifuged_container`.
	procedure_template_centrifuged_container_model_id INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The procedure_id asset associated to the `centrifuged_container`.
	procedure_centrifuged_container_id UUID NOT NULL REFERENCES procedure_assets(id) ON DELETE CASCADE,
	-- The centrifuge model used for the centrifuge procedure.
	centrifuged_with_model_id INTEGER NOT NULL REFERENCES centrifuge_models(id),
	-- The centrifuge used for the procedure. This field is optional because the centrifuge
	-- might not have been recorded at the time of performing the procedure.
	centrifuged_with_id UUID REFERENCES centrifuges(id),
	-- The procedure_id template asset model associated to the `centrifuged_with_model`.
	procedure_template_centrifuged_with_model_id INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The procedure_id asset associated to the `centrifuged_with`.
	procedure_centrifuged_with_id UUID NOT NULL REFERENCES procedure_assets(id) ON DELETE CASCADE,
	-- We enforce that the extended `procedure` has indeed the same `procedure_template`, making
	-- sure that the procedure_id is a centrifugating procedure.
	FOREIGN KEY (id, centrifuge_procedure_template_id) REFERENCES procedures(id, procedure_template_id),
	-- We enforce that the specified `centrifuged_with` is of the specified `centrifuged_with_model`.
	FOREIGN KEY (centrifuged_with_id, centrifuged_with_model_id) REFERENCES assets(id, model_id),
	-- The procedure_id template asset model describing the `centrifuged_container` must be the same one
	-- as the one in the procedure_id template.
	FOREIGN KEY (
		centrifuge_procedure_template_id,
		procedure_template_centrifuged_container_model_id
	) REFERENCES centrifuge_procedure_templates(
		id,
		procedure_template_centrifuged_container_model_id
	),
	-- The procedure_id template asset model describing the `centrifuged_with_model` must be the same one
	-- as the one in the procedure_id template.
	FOREIGN KEY (
		centrifuge_procedure_template_id,
		procedure_template_centrifuged_with_model_id
	) REFERENCES centrifuge_procedure_templates(
		id,
		procedure_template_centrifuged_with_model_id
	),
	-- We enforce that the specified `procedure_centrifuged_container` is of the specified `procedure_template_centrifuged_container_model`.
	FOREIGN KEY (
		procedure_centrifuged_container_id,
		procedure_template_centrifuged_container_model_id
	) REFERENCES procedure_assets(id, procedure_template_asset_model_id),
	-- We enforce that the specified `procedure_centrifuged_with` is of the specified `procedure_template_centrifuged_with_model`.
	FOREIGN KEY (
		procedure_centrifuged_with_id,
		procedure_template_centrifuged_with_model_id
	) REFERENCES procedure_assets(id, procedure_template_asset_model_id),
	-- We check that the `centrifuged_with_model` is indeed a instrument that is compatible with the `centrifuged_container_model`.
	FOREIGN KEY (
		centrifuged_with_model_id,
		centrifuged_container_model_id
	) REFERENCES asset_compatibility_rules(left_asset_model_id, right_asset_model_id),
	-- We ensure that the `procedure_centrifuged_container` is associated with the `centrifuged_container_model`.
	FOREIGN KEY (
		procedure_centrifuged_container_id,
		centrifuged_container_model_id
	) REFERENCES procedure_assets(id, asset_model_id),
	-- We ensure that the `procedure_centrifuged_container` is associated with the `centrifuged_container`.
	FOREIGN KEY (
		procedure_centrifuged_container_id,
		centrifuged_container_id
	) REFERENCES procedure_assets(id, asset_id),
	-- We ensure that the `procedure_centrifuged_with` is associated with the `centrifuged_container`.
	FOREIGN KEY (
		procedure_centrifuged_with_id,
		centrifuged_with_model_id
	) REFERENCES procedure_assets(id, asset_model_id),
	-- We ensure that the `procedure_centrifuged_with` is associated with the `centrifuged_with`.
	FOREIGN KEY (procedure_centrifuged_with_id, centrifuged_with_id) REFERENCES procedure_assets(id, asset_id)
);