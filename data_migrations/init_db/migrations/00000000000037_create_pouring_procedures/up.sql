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
	foreign_procedure_template INTEGER NOT NULL REFERENCES procedure_templates(procedure_template) ON DELETE CASCADE,
	-- The associated procedure asset model for the source container. It may be associated
	-- to any type of other procedure templates (e.g., fractioning, packaging, etc.).
	procedure_template_poured_from_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The volumetric container into which the liquid is poured.
	poured_into_model INTEGER NOT NULL REFERENCES volumetric_container_models(id),
	-- The associated procedure asset model for the destination container. It is always associated
	-- to the same procedure template of this pouring procedure template.
	procedure_template_poured_into_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id) ON DELETE CASCADE,
	-- The amount of liquid that is poured into the container.
	liters REAL NOT NULL CHECK (must_be_strictly_positive_f32(liters)),
	FOREIGN KEY (
		procedure_template_measured_with_model,
		procedure_template
	) REFERENCES procedure_template_asset_models(id, procedure_template),
	-- The measuring device must match the procedure template of the procedure.
	FOREIGN KEY (
		procedure_template_measured_with_model,
		measured_with_model
	) REFERENCES procedure_template_asset_models(id, asset_model),
	FOREIGN KEY (
		procedure_template_poured_into_model,
		procedure_template
	) REFERENCES procedure_template_asset_models(id, procedure_template),
	FOREIGN KEY (
		procedure_template_poured_into_model,
		poured_into_model
	) REFERENCES procedure_template_asset_models(id, asset_model),
	FOREIGN KEY (
		procedure_template_poured_from_model,
		foreign_procedure_template
	) REFERENCES procedure_template_asset_models(id, procedure_template),
	-- We define the same-as index to allow for foreign key references to check wether a
	-- `pouring_procedure_template` is associated with a given
	-- `pouring_foreign_procedure_template`.
	UNIQUE (procedure_template, foreign_procedure_template)
);
CREATE TABLE IF NOT EXISTS pouring_procedures (
	-- The extended `procedure`.
	procedure UUID PRIMARY KEY REFERENCES procedures(procedure) ON DELETE CASCADE,
	-- The container from which the liquid is poured.
	poured_from UUID NOT NULL REFERENCES volumetric_containers(id),
	-- The procedure template of the extended `procedure`.
	procedure_template INTEGER NOT NULL REFERENCES pouring_procedure_templates(procedure_template),
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
	-- The device model used to measure the liquid volume.
	measured_with_model INTEGER NOT NULL REFERENCES volume_measuring_device_models(id),
	-- The actual measuring device (if known) used to measure the liquid volume.
	measured_with UUID REFERENCES volume_measuring_devices(id),
	-- The container into which the liquid is poured.
	poured_into UUID NOT NULL REFERENCES volumetric_containers(id),
	-- We enforce that the extended `procedure` has indeed the same `procedure_template`, making
	-- sure that the procedure is a packaging procedure.
	FOREIGN KEY (procedure, procedure_template) REFERENCES procedures(procedure, procedure_template),
	-- We enforce that the `poured_into` is indeed a procedure assets.
	FOREIGN KEY (procedure, poured_into) REFERENCES procedure_assets(procedure, asset),
	-- We enforce that the `poured_from` is indeed a procedure assets.
	FOREIGN KEY (foreign_procedure, poured_from) REFERENCES procedure_assets(procedure, asset),
	-- We enforce that the `measured_with_model` is indeed a procedure asset model.
	FOREIGN KEY (procedure, measured_with_model) REFERENCES procedure_assets(procedure, asset_model),
	-- We enforce that the `measured_with` is indeed a procedure asset.
	FOREIGN KEY (procedure, measured_with) REFERENCES procedure_assets(procedure, asset),
	-- We enforce that the model of the `measured_with` matches the `measured_with_model`.
	FOREIGN KEY (measured_with, measured_with_model) REFERENCES assets(id, model),
	-- We enforce that the foreign procedure is of the foreign procedure template.
	FOREIGN KEY (foreign_procedure, foreign_procedure_template) REFERENCES procedures(procedure, procedure_template),
	-- We enforce that the associated procedure template requires the provided foreign procedure template.
	FOREIGN KEY (procedure_template, foreign_procedure_template) REFERENCES pouring_procedure_templates(procedure_template, foreign_procedure_template)
);