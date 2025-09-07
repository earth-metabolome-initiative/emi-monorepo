CREATE TABLE IF NOT EXISTS procedures (
	-- The ID of this procedure.
	procedure UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	-- The procedure template of this procedure.
	procedure_template INTEGER NOT NULL REFERENCES procedure_templates(procedure_template),
	-- The parent procedure (if any) of this procedure.
	parent_procedure UUID REFERENCES procedures(procedure) ON DELETE CASCADE CHECK (
		must_be_distinct_uuid(procedure, parent_procedure)
	),
	-- The parent procedure template (if any) of this procedure.
	parent_procedure_template INTEGER REFERENCES procedure_templates(procedure_template) CHECK (
		must_be_distinct_i32(procedure_template, parent_procedure_template)
	),
	-- The name of the most concrete table this procedure is associated with.
	most_concrete_table TEXT NOT NULL,
	-- User who created this procedure.
	created_by INTEGER NOT NULL REFERENCES users(id),
	-- Timestamp when this procedure was created.
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	-- User who last updated this procedure.
	updated_by INTEGER NOT NULL REFERENCES users(id),
	-- Timestamp when this procedure was last updated.
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	-- We check that the created_at is before or equal to updated_at.
	CHECK (must_be_smaller_than_utc(created_at, updated_at)),
	-- We create an index on (procedure_template, parent_procedure_template) to allow for foreign
	-- keys from the concrete procedures to check that the procedure template is correctly aligned.
	UNIQUE (procedure, procedure_template),
	-- We enforce that if a parent procedure and parent procedure template are specified,
	-- then the parent procedure must indeed be of the specified parent procedure template.
	FOREIGN KEY (parent_procedure, parent_procedure_template) REFERENCES procedures(procedure, procedure_template),
	-- We enforce that if a parent procedure template is specified, then the parent procedure template
	-- must indeed be a valid parent procedure template for the specified procedure template.
	FOREIGN KEY (parent_procedure_template, procedure_template) REFERENCES parent_procedure_templates(
		parent_procedure_template,
		child_procedure_template
	),
	-- We check that either both parent_procedure and parent_procedure_template are NULL,
	-- or neither is NULL.
	CHECK (
		(
			parent_procedure IS NULL
			AND parent_procedure_template IS NULL
		)
		OR (
			parent_procedure IS NOT NULL
			AND parent_procedure_template IS NOT NULL
		)
	)
);
CREATE TABLE IF NOT EXISTS procedure_assets (
	-- The ID of this procedure asset.
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	-- The ID of the procedure this asset is used in.
	procedure UUID NOT NULL REFERENCES procedures(procedure),
	-- The procedure template of the procedure this asset is used in.
	procedure_template INTEGER NOT NULL REFERENCES procedure_templates(procedure_template),
	-- The asset model of the asset used in this procedure.
	asset_model INTEGER NOT NULL REFERENCES asset_models(id),
	-- The specific asset used in this procedure (if any).
	asset UUID REFERENCES assets(id),
	-- We enforce that there must be a procedure template asset for this asset.
	procedure_template_asset_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The ancestor asset model defined in the procedure template asset.
	ancestor_model INTEGER NOT NULL REFERENCES asset_models(id),
	-- User who created this procedure asset.
	created_by INTEGER NOT NULL REFERENCES users(id),
	-- Timestamp when this procedure asset was created.
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	-- The procedure template must match the procedure template of the procedure.
	FOREIGN KEY (procedure, procedure_template) REFERENCES procedures(procedure, procedure_template),
	-- The procedure template asset must must be compatible with the procedure template of the procedure.
	FOREIGN KEY (
		procedure_template_asset_model,
		procedure_template
	) REFERENCES procedure_template_asset_models(id, procedure_template),
	-- We check that the ancestor asset is indeed the one defined in the procedure template asset.
	FOREIGN KEY (
		procedure_template_asset_model,
		ancestor_model
	) REFERENCES procedure_template_asset_models(id, asset_model),
	-- We check that the asset is indeed a descendant of the ancestor asset defined in the procedure template asset.
	FOREIGN KEY (asset_model, ancestor_model) REFERENCES asset_model_ancestors(descendant_model, ancestor_model),
	-- We check that the specified asset (if any) is indeed of the specified asset model.
	FOREIGN KEY (asset, asset_model) REFERENCES assets(id, model),
	-- We create a unique index to allow for foreign keys checking that the current procedure asset
	-- corresponds to a specific procedure template asset model in the procedure template.
	UNIQUE (id, procedure_template_asset_model),
	-- We create a unique index to allow for foreign keys checking that the current procedure asset
	-- corresponds to a specific asset model.
	UNIQUE (id, asset_model),
	-- We create a unique index to allow for foreign keys checking that the current procedure asset
	-- corresponds to a specific asset (if any).
	UNIQUE (id, asset)
);