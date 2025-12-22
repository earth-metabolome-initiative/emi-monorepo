CREATE TABLE IF NOT EXISTS fractioning_procedure_templates (
	-- Identifier of the fractioning procedure_id template, which is also a foreign key to the general procedure_id template.
	id INTEGER PRIMARY KEY REFERENCES procedure_templates(id) ON DELETE CASCADE,
	-- Mass in kilograms. Expected amount of the fraction to be collected.
	mass REAL NOT NULL CHECK (mass > 0.0),
	-- The tolerance percentage of the fraction mass.
	tolerance_percentage REAL NOT NULL CHECK (
		tolerance_percentage > 0.0
		AND tolerance_percentage <= 100.0
	),
	-- The model of the scale used to measure the fraction mass.
	weighed_with_model_id INTEGER NOT NULL REFERENCES weighing_device_models(id),
	-- The model of the instrument used for weighing should always be an asset model that is compatible with the procedure_id template.
	procedure_template_weighed_with_model_id INTEGER NOT NULL REFERENCES procedure_template_asset_models(id) ON DELETE CASCADE,
	-- Source container model from which the fraction is taken.
	fragment_container_model_id INTEGER NOT NULL REFERENCES volumetric_container_models(id),
	procedure_template_fragment_container_model_id INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- Destination container model to which the fraction is transferred.
	fragment_placed_into_model_id INTEGER NOT NULL REFERENCES volumetric_container_models(id),
	procedure_template_fragment_placed_into_model_id INTEGER NOT NULL REFERENCES procedure_template_asset_models(id) ON DELETE CASCADE,
	-- Foreign key to ensure that the `weighed_with_model` is indeed a weighing instrument.
	FOREIGN KEY (
		procedure_template_weighed_with_model_id,
		weighed_with_model_id
	) REFERENCES procedure_template_asset_models(id, asset_model_id),
	-- We enforce that the `fragment_container_model` is indeed a procedure_id asset model.
	FOREIGN KEY (
		procedure_template_fragment_container_model_id,
		fragment_container_model_id
	) REFERENCES procedure_template_asset_models(id, asset_model_id),
	FOREIGN KEY (
		procedure_template_fragment_placed_into_model_id,
		fragment_placed_into_model_id
	) REFERENCES procedure_template_asset_models(id, asset_model_id),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_weighed_with_model`
	-- for the current `procedure_template`.
	UNIQUE (
		id,
		procedure_template_weighed_with_model_id
	),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_fragment_container_model`
	-- for the current `procedure_template`.
	UNIQUE (
		id,
		procedure_template_fragment_container_model_id
	),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_fragment_placed_into_model`
	-- for the current `procedure_template`.
	UNIQUE (
		id,
		procedure_template_fragment_placed_into_model_id
	)
);
CREATE TABLE IF NOT EXISTS fractioning_procedures (
	-- Identifier of the fractioning id, which is also a foreign key to the general procedure.
	id UUID PRIMARY KEY REFERENCES procedures(id) ON DELETE CASCADE,
	-- The template of this procedure_id should be a fractioning procedure_id template.
	fractioning_procedure_template_id INTEGER NOT NULL REFERENCES fractioning_procedure_templates(id),
	-- The source container from which the fraction is taken, which must be a volumetric container model.
	fragment_container_id UUID NOT NULL REFERENCES volumetric_containers(id),
	-- The procedure_id template asset model associated to the `fragment_container`.
	procedure_template_fragment_container_model_id INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The procedure_id asset associated to the `fragment_container`.
	procedure_fragment_container_id UUID NOT NULL REFERENCES procedure_assets(id) ON DELETE CASCADE,
	-- The destination container to which the fraction is transferred, which must be a volumetric container model.
	fragment_placed_into_id UUID NOT NULL REFERENCES volumetric_containers(id),
	-- The procedure_id template asset model associated to the `fragment_placed_into`.
	procedure_template_fragment_placed_into_model_id INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The procedure_id asset associated to the `fragment_placed_into`.
	procedure_fragment_placed_into_id UUID NOT NULL REFERENCES procedure_assets(id) ON DELETE CASCADE,
	-- Mass in kilograms. The actual amount of the fraction collected.
	mass REAL NOT NULL CHECK (mass > 0.0),
	-- The scale used to measure the fraction mass. This field is optional, 
	-- as the weighing device might not necessarily be tracked.
	weighed_with_id UUID REFERENCES weighing_devices(id),
	-- The procedure_id template asset model associated to the `weighed_with_model`.
	procedure_template_weighed_with_model_id INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The procedure_id asset associated to the `weighed_with`.
	procedure_weighed_with_id UUID NOT NULL REFERENCES procedure_assets(id) ON DELETE CASCADE,
	-- We enforce that the current `fractioning_procedures` has indeed the same `fractioning_procedures_template`.
	FOREIGN KEY (id, fractioning_procedure_template_id) REFERENCES procedures(id, procedure_template_id),
	-- We enforce that the `procedure_template_weighed_with_model` is associated with the `procedure_template`.
	FOREIGN KEY (
		fractioning_procedure_template_id,
		procedure_template_weighed_with_model_id
	) REFERENCES fractioning_procedure_templates(
		id,
		procedure_template_weighed_with_model_id
	),
	-- We enforce that the `procedure_template_fragment_container_model` is associated with the `procedure_template`.
	FOREIGN KEY (
		fractioning_procedure_template_id,
		procedure_template_fragment_container_model_id
	) REFERENCES fractioning_procedure_templates(
		id,
		procedure_template_fragment_container_model_id
	),
	-- We enforce that the `procedure_template_fragment_placed_into_model` is associated with the `procedure_template`.
	FOREIGN KEY (
		fractioning_procedure_template_id,
		procedure_template_fragment_placed_into_model_id
	) REFERENCES fractioning_procedure_templates(
		id,
		procedure_template_fragment_placed_into_model_id
	),
	-- We enforce that the `procedure_fragment_container` is associated with the `fragment_container`.
	FOREIGN KEY (
		procedure_fragment_container_id,
		fragment_container_id
	) REFERENCES procedure_assets(id, asset_id),
	-- We enforce that the `procedure_fragment_placed_into` is associated with the `fragment_placed_into`.
	FOREIGN KEY (
		procedure_fragment_placed_into_id,
		fragment_placed_into_id
	) REFERENCES procedure_assets(id, asset_id),
	-- We enforce that the `procedure_weighed_with` is associated with the `weighed_with`.
	FOREIGN KEY (procedure_weighed_with_id, weighed_with_id) REFERENCES procedure_assets(id, asset_id),
	-- We enforce that the `procedure_fragment_container` is associated with `procedure_template_fragment_container_model`.
	FOREIGN KEY (
		procedure_fragment_container_id,
		procedure_template_fragment_container_model_id
	) REFERENCES procedure_assets(id, procedure_template_asset_model_id),
	-- We enforce that the `procedure_fragment_placed_into` is associated with `procedure_template_fragment_placed_into_model`.
	FOREIGN KEY (
		procedure_fragment_placed_into_id,
		procedure_template_fragment_placed_into_model_id
	) REFERENCES procedure_assets(id, procedure_template_asset_model_id),
	-- We enforce that the `procedure_weighed_with` is associated with `procedure_template_weighed_with_model`.
	FOREIGN KEY (
		procedure_weighed_with_id,
		procedure_template_weighed_with_model_id
	) REFERENCES procedure_assets(id, procedure_template_asset_model_id)
);