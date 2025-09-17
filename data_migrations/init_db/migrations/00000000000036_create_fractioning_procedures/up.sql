CREATE TABLE IF NOT EXISTS fractioning_procedure_templates (
	-- Identifier of the fractioning procedure template, which is also a foreign key to the general procedure template.
	procedure_template INTEGER PRIMARY KEY REFERENCES procedure_templates(procedure_template) ON DELETE CASCADE,
	-- Expected amount of the fraction to be collected in kilograms.
	kilograms REAL NOT NULL CHECK (must_be_strictly_positive_f32(kilograms)),
	-- The tolerance percentage of the fraction in kilograms.
	tolerance_percentage REAL NOT NULL CHECK (
		must_be_strictly_positive_f32(tolerance_percentage)
		AND must_be_smaller_than_f32(tolerance_percentage, 100.0)
	),
	-- The model of the scale used to measure the fraction in kilograms.
	weighed_with_model INTEGER NOT NULL REFERENCES weighing_device_models(id),
	-- The model of the instrument used for weighing should always be an asset model that is compatible with the procedure template.
	procedure_template_weighed_with_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id) ON DELETE CASCADE,
	-- Source container model from which the fraction is taken.
	fragment_container_model INTEGER NOT NULL REFERENCES volumetric_container_models(id),
	procedure_template_fragment_container_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- Destination container model to which the fraction is transferred.
	fragment_placed_into_model INTEGER NOT NULL REFERENCES volumetric_container_models(id),
	procedure_template_fragment_placed_into_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id) ON DELETE CASCADE,
	-- Foreign key to ensure that the `weighed_with_model` is indeed a weighing instrument.
	FOREIGN KEY (
		procedure_template_weighed_with_model,
		weighed_with_model
	) REFERENCES procedure_template_asset_models(id, asset_model),
	-- We enforce that the `fragment_container_model` is indeed a procedure asset model.
	FOREIGN KEY (
		procedure_template_fragment_container_model,
		fragment_container_model
	) REFERENCES procedure_template_asset_models(id, asset_model),
	FOREIGN KEY (
		procedure_template_fragment_placed_into_model,
		fragment_placed_into_model
	) REFERENCES procedure_template_asset_models(id, asset_model),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_weighed_with_model`
	-- for the current `procedure_template`.
	UNIQUE (
		procedure_template,
		procedure_template_weighed_with_model
	),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_fragment_container_model`
	-- for the current `procedure_template`.
	UNIQUE (
		procedure_template,
		procedure_template_fragment_container_model
	),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_fragment_placed_into_model`
	-- for the current `procedure_template`.
	UNIQUE (
		procedure_template,
		procedure_template_fragment_placed_into_model
	)
);
CREATE TABLE IF NOT EXISTS fractioning_procedures (
	-- Identifier of the fractioning procedure, which is also a foreign key to the general procedure.
	procedure UUID PRIMARY KEY REFERENCES procedures(procedure) ON DELETE CASCADE,
	-- The template of this procedure should be a fractioning procedure template.
	procedure_template INTEGER NOT NULL REFERENCES fractioning_procedure_templates(procedure_template),
	-- The source container from which the fraction is taken, which must be a volumetric container model.
	fragment_container UUID NOT NULL REFERENCES volumetric_containers(id),
	-- The procedure template asset model associated to the `fragment_container`.
	procedure_template_fragment_container_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The procedure asset associated to the `fragment_container`.
	procedure_fragment_container UUID NOT NULL REFERENCES procedure_assets(id) ON DELETE CASCADE,
	-- The destination container to which the fraction is transferred, which must be a volumetric container model.
	fragment_placed_into UUID NOT NULL REFERENCES volumetric_containers(id),
	-- The procedure template asset model associated to the `fragment_placed_into`.
	procedure_template_fragment_placed_into_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The procedure asset associated to the `fragment_placed_into`.
	procedure_fragment_placed_into UUID NOT NULL REFERENCES procedure_assets(id) ON DELETE CASCADE,
	-- The actual amount of the fraction collected in kilograms.
	kilograms REAL NOT NULL CHECK (must_be_strictly_positive_f32(kilograms)),
	-- The scale used to measure the fraction in kilograms. This field is optional, 
	-- as the weighing device might not necessarily be tracked.
	weighed_with UUID REFERENCES weighing_devices(id),
	-- The procedure template asset model associated to the `weighed_with_model`.
	procedure_template_weighed_with_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The procedure asset associated to the `weighed_with`.
	procedure_weighed_with UUID NOT NULL REFERENCES procedure_assets(id) ON DELETE CASCADE,
	-- We enforce that the current `fractioning_procedures` has indeed the same `fractioning_procedures_template`.
	FOREIGN KEY (procedure, procedure_template) REFERENCES procedures(procedure, procedure_template),
	-- We enforce that the `procedure_template_weighed_with_model` is associated with the `procedure_template`.
	FOREIGN KEY (
		procedure_template,
		procedure_template_weighed_with_model
	) REFERENCES fractioning_procedure_templates(
		procedure_template,
		procedure_template_weighed_with_model
	),
	-- We enforce that the `procedure_template_fragment_container_model` is associated with the `procedure_template`.
	FOREIGN KEY (
		procedure_template,
		procedure_template_fragment_container_model
	) REFERENCES fractioning_procedure_templates(
		procedure_template,
		procedure_template_fragment_container_model
	),
	-- We enforce that the `procedure_template_fragment_placed_into_model` is associated with the `procedure_template`.
	FOREIGN KEY (
		procedure_template,
		procedure_template_fragment_placed_into_model
	) REFERENCES fractioning_procedure_templates(
		procedure_template,
		procedure_template_fragment_placed_into_model
	),
	-- We enforce that the `procedure_fragment_container` is associated with the `fragment_container`.
	FOREIGN KEY (
		procedure_fragment_container,
		fragment_container
	) REFERENCES procedure_assets(id, asset),
	-- We enforce that the `procedure_fragment_placed_into` is associated with the `fragment_placed_into`.
	FOREIGN KEY (
		procedure_fragment_placed_into,
		fragment_placed_into
	) REFERENCES procedure_assets(id, asset),
	-- We enforce that the `procedure_weighed_with` is associated with the `weighed_with`.
	FOREIGN KEY (procedure_weighed_with, weighed_with) REFERENCES procedure_assets(id, asset),
	-- We enforce that the `procedure_fragment_container` is associated with `procedure_template_fragment_container_model`.
	FOREIGN KEY (
		procedure_fragment_container,
		procedure_template_fragment_container_model
	) REFERENCES procedure_assets(id, procedure_template_asset_model),
	-- We enforce that the `procedure_fragment_placed_into` is associated with `procedure_template_fragment_placed_into_model`.
	FOREIGN KEY (
		procedure_fragment_placed_into,
		procedure_template_fragment_placed_into_model
	) REFERENCES procedure_assets(id, procedure_template_asset_model),
	-- We enforce that the `procedure_weighed_with` is associated with `procedure_template_weighed_with_model`.
	FOREIGN KEY (
		procedure_weighed_with,
		procedure_template_weighed_with_model
	) REFERENCES procedure_assets(id, procedure_template_asset_model)
);