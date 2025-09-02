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
	-- Foreign procedure template which originated the source container (e.g., a sampling procedure).
	foreign_procedure_template INTEGER NOT NULL REFERENCES procedure_templates(procedure_template) CHECK (
		must_be_distinct_i32(
			procedure_template,
			foreign_procedure_template
		)
	),
	procedure_template_fragment_container_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- Destination container model to which the fraction is transferred.
	fragment_placed_into_model INTEGER NOT NULL REFERENCES volumetric_container_models(id),
	procedure_template_fragment_placed_into_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id) ON DELETE CASCADE,
	-- Foreign key to ensure that the `weighed_with_model` is indeed a weighing instrument.
	FOREIGN KEY (
		procedure_template_weighed_with_model,
		weighed_with_model
	) REFERENCES procedure_template_asset_models(id, asset_model),
	-- Foreign key to ensure that the `procedure_template_weighed_with_model` is indeed a trackable that is compatible with the procedure template.
	FOREIGN KEY (
		procedure_template_weighed_with_model,
		procedure_template
	) REFERENCES procedure_template_asset_models(id, procedure_template),
	-- We enforce that the `fragment_container_model` is indeed a procedure asset model.
	FOREIGN KEY (
		procedure_template_fragment_container_model,
		fragment_container_model
	) REFERENCES procedure_template_asset_models(id, asset_model),
	-- Foreign key to ensure that the `procedure_template_fragment_container_model` is indeed a trackable that is compatible with the procedure template.
	FOREIGN KEY (
		procedure_template_fragment_container_model,
		foreign_procedure_template
	) REFERENCES procedure_template_asset_models(id, procedure_template),
	-- Foreign key to ensure that the `procedure_template_fragment_placed_into_model` is indeed a trackable that is compatible with the procedure template.
	FOREIGN KEY (
		procedure_template_fragment_placed_into_model,
		procedure_template
	) REFERENCES procedure_template_asset_models(id, procedure_template),
	FOREIGN KEY (
		procedure_template_fragment_placed_into_model,
		fragment_placed_into_model
	) REFERENCES procedure_template_asset_models(id, asset_model),
	-- We define a same-as index to allow for foreign key references to check whether a `fractioning_procedure_template`
	-- is associated with a given `fractioning_foreign_procedure_template`.
	UNIQUE (procedure_template, foreign_procedure_template)
);
CREATE TABLE IF NOT EXISTS fractioning_procedures (
	-- Identifier of the fractioning procedure, which is also a foreign key to the general procedure.
	procedure UUID PRIMARY KEY REFERENCES procedures(procedure) ON DELETE CASCADE,
	-- The template of this procedure should be a fractioning procedure template.
	procedure_template INTEGER NOT NULL REFERENCES fractioning_procedure_templates(procedure_template),
	-- The procedure template associated with the foreign procedure template.
	foreign_procedure_template INTEGER NOT NULL REFERENCES procedure_templates(procedure_template) CHECK (
		must_be_distinct_i32(
			procedure_template,
			foreign_procedure_template
		)
	),
	-- The procedure that has populated the source container (e.g., a sampling procedure).
	foreign_procedure UUID NOT NULL REFERENCES procedures(procedure) CHECK (
		must_be_distinct_uuid(procedure, foreign_procedure)
	),
	-- The source container from which the fraction is taken, which must be a volumetric container model.
	fragment_container UUID NOT NULL REFERENCES volumetric_containers(id),
	-- The destination container to which the fraction is transferred, which must be a volumetric container model.
	fragment_placed_into UUID NOT NULL REFERENCES volumetric_containers(id),
	-- The actual amount of the fraction collected in kilograms.
	kilograms REAL NOT NULL CHECK (must_be_strictly_positive_f32(kilograms)),
	-- The scale used to measure the fraction in kilograms. This field is optional, 
	-- as the weighing device might not necessarily be tracked.
	weighed_with UUID REFERENCES weighing_devices(id),
	-- The model of the weighing device used, which must be a weighing device model.
	weighed_with_model INTEGER NOT NULL REFERENCES weighing_device_models(id),
	-- We enforce that the current `fractioning_procedures` has indeed the same `fractioning_procedures_template`.
	FOREIGN KEY (procedure, procedure_template) REFERENCES procedures(procedure, procedure_template),
	-- We enforce that the `foreign_procedure` has as `procedure_template` the specified `foreign_procedure_template`.
	FOREIGN KEY (foreign_procedure, foreign_procedure_template) REFERENCES procedures(procedure, procedure_template),
	-- Additionally, we enforce that the `fragment_container` is indeed a procedure asset of the correct model.
	FOREIGN KEY (foreign_procedure, fragment_container) REFERENCES procedure_assets(procedure, asset),
	-- Additionally, we enforce that the `fragment_placed_into` is indeed a procedure asset of the correct model.
	FOREIGN KEY (procedure, fragment_placed_into) REFERENCES procedure_assets(procedure, asset),
	-- We enforce that the `weighed_with_model` is indeed a procedure asset model.
	FOREIGN KEY (procedure, weighed_with_model) REFERENCES procedure_assets(procedure, asset_model),
	-- And that the `weighed_with` is indeed a procedure asset of the correct model.
	FOREIGN KEY (procedure, weighed_with) REFERENCES procedure_assets(procedure, asset),
	-- We enforce that the `weighed_with` is indeed a weighing device of the correct model.
	FOREIGN KEY (weighed_with, weighed_with_model) REFERENCES assets(id, model),
	-- We enforce that the associated procedure template requires the provided foreign procedure template.
	FOREIGN KEY (procedure_template, foreign_procedure_template) REFERENCES fractioning_procedure_templates(procedure_template, foreign_procedure_template)
);