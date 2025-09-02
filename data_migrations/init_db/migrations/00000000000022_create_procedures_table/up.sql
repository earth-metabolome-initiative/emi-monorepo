CREATE TABLE IF NOT EXISTS procedures (
	procedure UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	procedure_template INTEGER NOT NULL REFERENCES procedure_templates(procedure_template),
	most_concrete_table TEXT NOT NULL,
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_by INTEGER NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	CHECK (must_be_smaller_than_utc(created_at, updated_at)),
	UNIQUE (procedure, procedure_template)
);
CREATE TABLE IF NOT EXISTS procedure_assets (
	procedure UUID NOT NULL REFERENCES procedures(procedure),
	procedure_template INTEGER NOT NULL REFERENCES procedure_templates(procedure_template),
	asset_model INTEGER NOT NULL REFERENCES asset_models(id),
	asset UUID REFERENCES assets(id),
	-- We enforce that there must be a procedure template trackable for this trackable.
	procedure_template_asset_model INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	ancestor_model INTEGER NOT NULL REFERENCES asset_models(id),
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	-- The procedure template must match the procedure template of the procedure.
	FOREIGN KEY (procedure, procedure_template) REFERENCES procedures(procedure, procedure_template),
	-- The procedure template trackable must must be compatible with the procedure template of the procedure.
	FOREIGN KEY (
		procedure_template_asset_model,
		procedure_template
	) REFERENCES procedure_template_asset_models(id, procedure_template),
	-- We check that the ancestor trackable is indeed the one defined in the procedure template trackable.
	FOREIGN KEY (
		procedure_template_asset_model,
		ancestor_model
	) REFERENCES procedure_template_asset_models(id, asset_model),
	-- We check that the trackable is indeed a descendant of the ancestor trackable defined in the procedure template trackable.
	FOREIGN KEY (asset_model, ancestor_model) REFERENCES asset_model_ancestors(descendant_model, ancestor_model),
	-- We check that the specified asset (if any) is indeed of the specified asset model.
	FOREIGN KEY (asset, asset_model) REFERENCES assets(id, model_id),
	-- We check that the specified asset (if any) has as parent the specified asset model.
	PRIMARY KEY (procedure, asset_model),
	UNIQUE (procedure, asset)
);