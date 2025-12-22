CREATE TABLE IF NOT EXISTS pouring_procedure_templates (
	-- Identifier of the pouring procedure_id template, which is also a foreign key to the general procedure_id template.
	id INTEGER PRIMARY KEY REFERENCES procedure_templates(id) ON DELETE CASCADE,
	-- The device model used to measure the liquid volume.
	measured_with_model_id INTEGER NOT NULL REFERENCES volume_measuring_device_models(id),
	-- The associated procedure_id asset model for the measuring device.
	procedure_template_measured_with_model_id INTEGER NOT NULL REFERENCES procedure_template_asset_models(id) ON DELETE CASCADE,
	-- The source container from which the liquid is poured.
	poured_from_model_id INTEGER NOT NULL REFERENCES volumetric_container_models(id),
	-- The associated procedure_id asset model for the source container. It may be associated
	-- to any type of other procedure_id templates (e.g., fractioning, packaging, etc.).
	procedure_template_poured_from_model_id INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The volumetric container into which the liquid is poured.
	poured_into_model_id INTEGER NOT NULL REFERENCES volumetric_container_models(id),
	-- The associated procedure_id asset model for the destination container. It is always associated
	-- to the same procedure_id template of this pouring procedure_id template.
	procedure_template_poured_into_model_id INTEGER NOT NULL REFERENCES procedure_template_asset_models(id) ON DELETE CASCADE,
	-- Volume in liters. The amount of liquid that is poured into the container.
	volume REAL NOT NULL CHECK (volume > 0.0),
	-- The measuring device must match the procedure_id template of the procedure.
	FOREIGN KEY (
		procedure_template_measured_with_model_id,
		measured_with_model_id
	) REFERENCES procedure_template_asset_models(id, asset_model_id),
	FOREIGN KEY (
		procedure_template_poured_into_model_id,
		poured_into_model_id
	) REFERENCES procedure_template_asset_models(id, asset_model_id),
	FOREIGN KEY (
		procedure_template_poured_from_model_id,
		poured_from_model_id
	) REFERENCES procedure_template_asset_models(id, asset_model_id),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_measured_with_model`
	-- for the current `procedure_template`.
	UNIQUE (
		id,
		procedure_template_measured_with_model_id
	),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_poured_from_model`
	-- for the current `procedure_template`.
	UNIQUE (
		id,
		procedure_template_poured_from_model_id
	),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_poured_into_model`
	-- for the current `procedure_template`.
	UNIQUE (
		id,
		procedure_template_poured_into_model_id
	)
);
CREATE TABLE IF NOT EXISTS pouring_procedures (
	-- The extended `procedure`.
	id UUID PRIMARY KEY REFERENCES procedures(id) ON DELETE CASCADE,
	-- The procedure_id template of the extended `procedure`.
	pouring_procedure_template_id INTEGER NOT NULL REFERENCES pouring_procedure_templates(id),
	-- The container from which the liquid is poured.
	poured_from_id UUID NOT NULL REFERENCES volumetric_containers(id),
	-- The procedure_id template asset model associated to the `poured_from`.
	procedure_template_poured_from_model_id INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The procedure_id asset associated to the `poured_from`.
	procedure_poured_from_id UUID NOT NULL REFERENCES procedure_assets(id) ON DELETE CASCADE,
	-- The actual measuring device (if known) used to measure the liquid volume.
	measured_with_id UUID REFERENCES volume_measuring_devices(id),
	-- The procedure_id template asset model associated to the `measured_with_model`.
	procedure_template_measured_with_model_id INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The procedure_id asset associated to the `measured_with`.
	procedure_measured_with_id UUID NOT NULL REFERENCES procedure_assets(id) ON DELETE CASCADE,
	-- The container into which the liquid is poured.
	poured_into_id UUID NOT NULL REFERENCES volumetric_containers(id),
	-- The procedure_id template asset model associated to the `poured_into`.
	procedure_template_poured_into_model_id INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The procedure_id asset associated to the `poured_into`.
	procedure_poured_into_id UUID NOT NULL REFERENCES procedure_assets(id) ON DELETE CASCADE,
	-- We enforce that the extended `procedure` has indeed the same `procedure_template`, making
	-- sure that the procedure_id is a packaging procedure.
	FOREIGN KEY (id, pouring_procedure_template_id) REFERENCES procedures(id, procedure_template_id),
	-- The `procedure_template_measured_with_model` must be the same as in the `pouring_procedure_templates`.
	FOREIGN KEY (
		pouring_procedure_template_id,
		procedure_template_measured_with_model_id
	) REFERENCES pouring_procedure_templates(
		id,
		procedure_template_measured_with_model_id
	),
	-- The `procedure_template_poured_from_model` must be the same as in the `pouring_procedure_templates`.
	FOREIGN KEY (
		pouring_procedure_template_id,
		procedure_template_poured_from_model_id
	) REFERENCES pouring_procedure_templates(
		id,
		procedure_template_poured_from_model_id
	),
	-- The `procedure_template_poured_into_model` must be the same as in the `pouring_procedure_templates`.
	FOREIGN KEY (
		pouring_procedure_template_id,
		procedure_template_poured_into_model_id
	) REFERENCES pouring_procedure_templates(
		id,
		procedure_template_poured_into_model_id
	),
	-- We check that the `procedure_poured_from` is associated to the `poured_from`.
	FOREIGN KEY (procedure_poured_from_id, poured_from_id) REFERENCES procedure_assets(id, asset_id),
	-- We check that the `procedure_poured_into` is associated to the `poured_into`.
	FOREIGN KEY (procedure_poured_into_id, poured_into_id) REFERENCES procedure_assets(id, asset_id),
	-- We check that the `procedure_poured_from` is indeed associated to the `procedure_template_poured_from_model`.
	FOREIGN KEY (
		procedure_poured_from_id,
		procedure_template_poured_from_model_id
	) REFERENCES procedure_assets(id, procedure_template_asset_model_id),
	-- We check that the `procedure_measured_with` is indeed associated to the `procedure_template_measured_with_model`.
	FOREIGN KEY (
		procedure_measured_with_id,
		procedure_template_measured_with_model_id
	) REFERENCES procedure_assets(id, procedure_template_asset_model_id),
	-- We check that the `procedure_poured_into` is indeed associated to the `procedure_template_poured_into_model`.
	FOREIGN KEY (
		procedure_poured_into_id,
		procedure_template_poured_into_model_id
	) REFERENCES procedure_assets(id, procedure_template_asset_model_id),
	-- We check that the `procedure_measured_with` is associated to the `measured_with` asset (if any).
	FOREIGN KEY (procedure_measured_with_id, measured_with_id) REFERENCES procedure_assets(id, asset_id)
);