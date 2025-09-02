CREATE TABLE IF NOT EXISTS weighing_procedure_templates (
	-- Identifier of the weighing procedure template, which is also a a foreign key to the general procedure template table.
	procedure_template INTEGER PRIMARY KEY REFERENCES procedure_templates(procedure_template) ON DELETE CASCADE,
	-- The sample container model is the one that is being weighed.
	weighed_container_model INTEGER NOT NULL REFERENCES volumetric_container_models(id),
	-- Foreign procedure template which originated the sample container (e.g., a sampling procedure).
	foreign_procedure_template INTEGER NOT NULL REFERENCES procedure_templates(procedure_template) CHECK (
		must_be_distinct_i32(
			procedure_template,
			foreign_procedure_template
		)
	),
	-- The sample container model should always be an asset model that is compatible with the procedure template.
	procedure_template_weighed_container_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The model of the instrument to be used for weighing.
	weighed_with_model INTEGER NOT NULL REFERENCES weighing_device_models(id),
	-- The instrument model used for weighing should always be an asset model that is compatible with the procedure template.
	procedure_template_weighed_with_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id) ON DELETE CASCADE,
	-- We check that the `weighed_with` is indeed an asset model that is compatible with the procedure template.
	FOREIGN KEY (
		procedure_template_weighed_with_model,
		weighed_with_model
	) REFERENCES procedure_template_asset_models(id, asset_model),
	-- We ensure that the `procedure_template_weighed_with_model` is indeed associated with the parent procedure template.
	FOREIGN KEY (
		procedure_template_weighed_with_model,
		procedure_template
	) REFERENCES procedure_template_asset_models(id, procedure_template),
	-- We check that the `procedure_template_weighed_container_model` is indeed an asset model that exists within the foreign procedure template.
	FOREIGN KEY (
		procedure_template_weighed_container_model,
		foreign_procedure_template
	) REFERENCES procedure_template_asset_models(id, procedure_template),
	-- We enforce that the `weighed_container_model` is indeed a procedure asset model.
	FOREIGN KEY (
		procedure_template_weighed_container_model,
		weighed_container_model
	) REFERENCES procedure_template_asset_models(id, asset_model),
	-- We define a same-as index to allow for foreign key references to check whether a `weighing_procedure_template`
	-- is associated with a given `weighing_foreign_procedure_template`.
	UNIQUE (procedure_template, foreign_procedure_template)
);
CREATE TABLE IF NOT EXISTS weighing_procedures(
	procedure UUID PRIMARY KEY REFERENCES procedures(procedure) ON DELETE CASCADE,
	-- We enforce that the model of this procedure must be a weighing procedure template.
	procedure_template INTEGER NOT NULL REFERENCES weighing_procedure_templates(procedure_template),
	-- The procedure template associated with the foreign procedure template.
	foreign_procedure_template INTEGER NOT NULL REFERENCES procedure_templates(procedure_template) CHECK (
		must_be_distinct_i32(
			procedure_template,
			foreign_procedure_template
		)
	),
	-- The foreign procedure that has populated the container being weighed (e.g., a sampling or fractioning procedure).
	foreign_procedure UUID NOT NULL REFERENCES procedures(procedure) CHECK (
		must_be_distinct_uuid(procedure, foreign_procedure)
	),
	-- The container being weighed, which must be a volumetric container model.
	weighed_container UUID NOT NULL REFERENCES volumetric_containers(id),
	-- The measured weight, which must be strictly positive.
	kilograms REAL NOT NULL CHECK (must_be_strictly_positive_f32(kilograms)),
	-- The weighing device model used for weighing.
	weighed_with_model INTEGER NOT NULL REFERENCES weighing_device_models(id),
	-- The weighing device used for weighing. This field is optional as there
	-- are several situations where the weighing device is not tracked.
	weighed_with UUID REFERENCES weighing_devices(id),
	-- We enforce that the extended `procedure` has indeed the same `procedure_template`, making
	-- sure that the procedure is a weighing procedure without the possibility of a mistake.
	FOREIGN KEY (procedure, procedure_template) REFERENCES procedures(procedure, procedure_template),
	-- We enforce that the `weighed_with_model` is indeed a procedure asset model.
	FOREIGN KEY (procedure, weighed_with_model) REFERENCES procedure_assets(procedure, asset_model),
	-- And that the `weighed_with` is indeed a procedure asset, if it is not NULL.
	FOREIGN KEY (procedure, weighed_with) REFERENCES procedure_assets(procedure, asset),
	-- We enforce that the specified weighing device has as parent model the specified model.
	FOREIGN KEY (weighed_with, weighed_with_model) REFERENCES assets(id, model_id),
	-- Additionally, we enforce that the `weighed_container` is indeed a procedure asset of the correct model.
	FOREIGN KEY (foreign_procedure, weighed_container) REFERENCES procedure_assets(procedure, asset),
	-- We enforce that the `foreign_procedure` has as `procedure_template` the specified `foreign_procedure_template`.
	FOREIGN KEY (foreign_procedure, foreign_procedure_template) REFERENCES procedures(procedure, procedure_template),
	-- We enforce that the associated procedure template requires the provided foreign procedure template.
	FOREIGN KEY (procedure_template, foreign_procedure_template) REFERENCES weighing_procedure_templates(procedure_template, foreign_procedure_template)
);