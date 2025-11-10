CREATE TABLE IF NOT EXISTS pouring_procedure_templates (
	-- Identifier of the pouring procedure template, which is also a foreign key to the general procedure template.
	procedure_template INTEGER PRIMARY KEY REFERENCES procedure_templates(procedure_template) ON DELETE CASCADE,
	-- The device model used to measure the liquid volume.
	measured_with_model INTEGER NOT NULL REFERENCES volume_measuring_device_models(id),
	-- The associated procedure asset model for the measuring device.
	procedure_template_measured_with_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id) ON DELETE CASCADE,
	-- The source container from which the liquid is poured.
	poured_from_model INTEGER NOT NULL REFERENCES volumetric_container_models(id),
	-- The associated procedure asset model for the source container. It may be associated
	-- to any type of other procedure templates (e.g., fractioning, packaging, etc.).
	procedure_template_poured_from_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The volumetric container into which the liquid is poured.
	poured_into_model INTEGER NOT NULL REFERENCES volumetric_container_models(id),
	-- The associated procedure asset model for the destination container. It is always associated
	-- to the same procedure template of this pouring procedure template.
	procedure_template_poured_into_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id) ON DELETE CASCADE,
	-- The amount of liquid that is poured into the container.
	liters REAL NOT NULL CHECK (liters > 0.0),
	-- The measuring device must match the procedure template of the procedure.
	FOREIGN KEY (
		procedure_template_measured_with_model,
		measured_with_model
	) REFERENCES procedure_template_asset_models(id, asset_model),
	FOREIGN KEY (
		procedure_template_poured_into_model,
		poured_into_model
	) REFERENCES procedure_template_asset_models(id, asset_model),
	FOREIGN KEY (
		procedure_template_poured_from_model,
		poured_from_model
	) REFERENCES procedure_template_asset_models(id, asset_model),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_measured_with_model`
	-- for the current `procedure_template`.
	UNIQUE (
		procedure_template,
		procedure_template_measured_with_model
	),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_poured_from_model`
	-- for the current `procedure_template`.
	UNIQUE (
		procedure_template,
		procedure_template_poured_from_model
	),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_poured_into_model`
	-- for the current `procedure_template`.
	UNIQUE (
		procedure_template,
		procedure_template_poured_into_model
	)
);
CREATE TABLE IF NOT EXISTS pouring_procedures (
	-- The extended `procedure`.
	procedure UUID PRIMARY KEY REFERENCES procedures(procedure) ON DELETE CASCADE,
	-- The procedure template of the extended `procedure`.
	procedure_template INTEGER NOT NULL REFERENCES pouring_procedure_templates(procedure_template),
	-- The container from which the liquid is poured.
	poured_from UUID NOT NULL REFERENCES volumetric_containers(id),
	-- The procedure template asset model associated to the `poured_from`.
	procedure_template_poured_from_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The procedure asset associated to the `poured_from`.
	procedure_poured_from UUID NOT NULL REFERENCES procedure_assets(id) ON DELETE CASCADE,
	-- The actual measuring device (if known) used to measure the liquid volume.
	measured_with UUID REFERENCES volume_measuring_devices(id),
	-- The procedure template asset model associated to the `measured_with_model`.
	procedure_template_measured_with_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The procedure asset associated to the `measured_with`.
	procedure_measured_with UUID NOT NULL REFERENCES procedure_assets(id) ON DELETE CASCADE,
	-- The container into which the liquid is poured.
	poured_into UUID NOT NULL REFERENCES volumetric_containers(id),
	-- The procedure template asset model associated to the `poured_into`.
	procedure_template_poured_into_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The procedure asset associated to the `poured_into`.
	procedure_poured_into UUID NOT NULL REFERENCES procedure_assets(id) ON DELETE CASCADE,
	-- We enforce that the extended `procedure` has indeed the same `procedure_template`, making
	-- sure that the procedure is a packaging procedure.
	FOREIGN KEY (procedure, procedure_template) REFERENCES procedures(procedure, procedure_template),
	-- The `procedure_template_measured_with_model` must be the same as in the `pouring_procedure_templates`.
	FOREIGN KEY (
		procedure_template,
		procedure_template_measured_with_model
	) REFERENCES pouring_procedure_templates(
		procedure_template,
		procedure_template_measured_with_model
	),
	-- The `procedure_template_poured_from_model` must be the same as in the `pouring_procedure_templates`.
	FOREIGN KEY (
		procedure_template,
		procedure_template_poured_from_model
	) REFERENCES pouring_procedure_templates(
		procedure_template,
		procedure_template_poured_from_model
	),
	-- The `procedure_template_poured_into_model` must be the same as in the `pouring_procedure_templates`.
	FOREIGN KEY (
		procedure_template,
		procedure_template_poured_into_model
	) REFERENCES pouring_procedure_templates(
		procedure_template,
		procedure_template_poured_into_model
	),
	-- We check that the `procedure_poured_from` is associated to the `poured_from`.
	FOREIGN KEY (procedure_poured_from, poured_from) REFERENCES procedure_assets(id, asset),
	-- We check that the `procedure_poured_into` is associated to the `poured_into`.
	FOREIGN KEY (procedure_poured_into, poured_into) REFERENCES procedure_assets(id, asset),
	-- We check that the `procedure_poured_from` is indeed associated to the `procedure_template_poured_from_model`.
	FOREIGN KEY (
		procedure_poured_from,
		procedure_template_poured_from_model
	) REFERENCES procedure_assets(id, procedure_template_asset_model),
	-- We check that the `procedure_measured_with` is indeed associated to the `procedure_template_measured_with_model`.
	FOREIGN KEY (
		procedure_measured_with,
		procedure_template_measured_with_model
	) REFERENCES procedure_assets(id, procedure_template_asset_model),
	-- We check that the `procedure_poured_into` is indeed associated to the `procedure_template_poured_into_model`.
	FOREIGN KEY (
		procedure_poured_into,
		procedure_template_poured_into_model
	) REFERENCES procedure_assets(id, procedure_template_asset_model),
	-- We check that the `procedure_measured_with` is associated to the `measured_with` asset (if any).
	FOREIGN KEY (procedure_measured_with, measured_with) REFERENCES procedure_assets(id, asset)
);