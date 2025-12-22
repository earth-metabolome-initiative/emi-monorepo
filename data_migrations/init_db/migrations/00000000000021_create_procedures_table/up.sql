CREATE TABLE IF NOT EXISTS procedures (
	-- The ID of this procedure.
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	-- The procedure_id template of this procedure.
	procedure_template_id INTEGER NOT NULL REFERENCES procedure_templates(id),
	-- The parent_id procedure_id (if any) of this procedure.
	parent_procedure_id UUID REFERENCES procedures(id) ON DELETE CASCADE CHECK (id <> parent_procedure_id),
	-- The parent_id procedure_id template (if any) of this procedure.
	parent_procedure_template_id INTEGER REFERENCES procedure_templates(id) CHECK (
		procedure_template_id <> parent_procedure_template_id
	),
	-- The predecessor_id procedure_id (if any) of this procedure.
	predecessor_procedure_id UUID REFERENCES procedures(id) ON DELETE CASCADE CHECK (id <> predecessor_procedure_id),
	-- The predecessor_id procedure_id template (if any) of this procedure.
	predecessor_procedure_template_id INTEGER REFERENCES procedure_templates(id) CHECK (
		procedure_template_id <> predecessor_procedure_template_id
	),
	-- The name of the most concrete table this procedure_id is associated with.
	most_concrete_table TEXT NOT NULL,
	-- User who created this procedure.
	created_by_id INTEGER NOT NULL REFERENCES users(id),
	-- Timestamp when this procedure_id was created.
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	-- User who last updated this procedure.
	updated_by_id INTEGER NOT NULL REFERENCES users(id),
	-- Timestamp when this procedure_id was last updated.
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	-- We check that the created_at is before or equal to updated_at.
	CHECK (created_at <= updated_at),
	-- We create an index on (procedure_template_id, parent_procedure_template_id) to allow for foreign
	-- keys from the concrete procedures to check that the procedure_id template is correctly aligned.
	UNIQUE (id, procedure_template_id),
	-- We enforce that if a parent_id procedure_id and parent_id procedure_id template are specified,
	-- then the parent_id procedure_id must indeed be of the specified parent_id procedure_id template.
	FOREIGN KEY (parent_procedure_id, parent_procedure_template_id) REFERENCES procedures(id, procedure_template_id),
	-- We enforce that if a predecessor_id procedure_id and predecessor_id procedure_id template are specified,
	-- then the predecessor_id procedure_id must indeed be of the specified predecessor_id procedure_id template.
	FOREIGN KEY (
		predecessor_procedure_id,
		predecessor_procedure_template_id
	) REFERENCES procedures(id, procedure_template_id),
	-- We enforce that if a parent_id procedure_id template is specified, then the parent_id procedure_id template
	-- must indeed be a valid parent_id procedure_id template for the specified procedure_id template.
	FOREIGN KEY (parent_procedure_template_id, procedure_template_id) REFERENCES parent_procedure_templates(parent_id, child_id),
	-- We enforce that if both a predecessor_id procedure_id template and a parent_id procedure_id template are specified,
	-- then there must exist a row in `next_procedure_templates`
	FOREIGN KEY (
		parent_procedure_template_id,
		predecessor_procedure_template_id,
		procedure_template_id
	) REFERENCES next_procedure_templates(parent_id, predecessor_id, successor_id),
	-- We check that either both parent_procedure_id and parent_procedure_template_id are NULL,
	-- or neither is NULL.
	CHECK (
		(
			parent_procedure_id IS NULL
			AND parent_procedure_template_id IS NULL
		)
		OR (
			parent_procedure_id IS NOT NULL
			AND parent_procedure_template_id IS NOT NULL
		)
	),
	-- We check that either both predecessor_procedure_id and predecessor_procedure_template_id are NULL,
	-- or neither is NULL.
	CHECK (
		(
			predecessor_procedure_id IS NULL
			AND predecessor_procedure_template_id IS NULL
		)
		OR (
			predecessor_procedure_id IS NOT NULL
			AND predecessor_procedure_template_id IS NOT NULL
		)
	),
	-- We check that if the previous procedure_id is specified, then the parent_id procedure_id must also be specified.
	CHECK (
		(
			predecessor_procedure_id IS NULL
			AND parent_procedure_id IS NULL
		)
		OR (
			parent_procedure_id IS NOT NULL
			AND predecessor_procedure_id IS NOT NULL
		)
	)
);
CREATE TABLE IF NOT EXISTS procedure_assets (
	-- The ID of this procedure_id asset.
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	-- The ID of the procedure_id this asset is used in.
	procedure_id UUID NOT NULL REFERENCES procedures(id),
	-- The procedure_id template of the procedure_id this asset is used in.
	procedure_template_id INTEGER NOT NULL REFERENCES procedure_templates(id),
	-- The asset model of the asset used in this procedure.
	asset_model_id INTEGER NOT NULL REFERENCES asset_models(id),
	-- The specific asset used in this procedure_id (if any).
	asset_id UUID REFERENCES assets(id),
	-- We enforce that there must be a procedure_id template asset for this asset.
	procedure_template_asset_model_id INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The ancestor asset model defined in the procedure_id template asset.
	ancestor_model_id INTEGER NOT NULL REFERENCES asset_models(id),
	-- The procedure_id template must match the procedure_id template of the procedure.
	FOREIGN KEY (id, procedure_template_id) REFERENCES procedures(id, procedure_template_id),
	-- The procedure_id template asset must must be compatible with the procedure_id template of the procedure.
	FOREIGN KEY (
		procedure_template_asset_model_id,
		procedure_template_id
	) REFERENCES procedure_template_asset_models(id, procedure_template_id),
	-- We check that the ancestor asset is indeed the one defined in the procedure_id template asset.
	FOREIGN KEY (
		procedure_template_asset_model_id,
		ancestor_model_id
	) REFERENCES procedure_template_asset_models(id, asset_model_id),
	-- We check that the asset is indeed a descendant of the ancestor asset defined in the procedure_id template asset.
	FOREIGN KEY (asset_model_id, ancestor_model_id) REFERENCES asset_model_ancestors(descendant_model_id, ancestor_model_id),
	-- We check that the specified asset (if any) is indeed of the specified asset model.
	FOREIGN KEY (asset_id, asset_model_id) REFERENCES assets(id, model_id),
	-- We create a unique index to allow for foreign keys checking that the current procedure_id asset
	-- corresponds to a specific procedure_id template asset model in the procedure_id template.
	UNIQUE (id, procedure_template_asset_model_id),
	-- We create a unique index to allow for foreign keys checking that the current procedure_id asset
	-- corresponds to a specific asset model.
	UNIQUE (id, asset_model_id),
	-- We create a unique index to allow for foreign keys checking that the current procedure_id asset
	-- corresponds to a specific asset (if any).
	UNIQUE (id, asset_id)
);
-- When we insert a procedure_id assets, the parent_id procedure_id if any
-- must also receive its own version of the procedure_id asset. The
-- parent_id procedure's procedure_id asset will reference many of the
-- same fields, but the `procedure_template_asset_model` will be
-- the one defined in the parent_id procedure_id template which is characterized
-- by being `based_on` the current procedure_id template asset model.
CREATE OR REPLACE FUNCTION inherit_procedure_assets() RETURNS TRIGGER AS $$ BEGIN
INSERT INTO procedure_assets (
		id,
		procedure_template_id,
		asset_model_id,
		asset_id,
		procedure_template_asset_model_id,
		ancestor_model_id
	)
SELECT p.parent_procedure_id,
	p.parent_procedure_template_id,
	NEW.asset_model_id,
	NEW.asset_id,
	ptam.id,
	NEW.ancestor_model_id
FROM procedures p
	JOIN procedure_template_asset_models ptam ON ptam.based_on_id = NEW.procedure_template_asset_model
	AND ptam.procedure_template_id = p.parent_procedure_template
WHERE p.procedure_id = NEW.procedure
	AND p.parent_procedure_id IS NOT NULL;
RETURN NEW;
END;
$$ LANGUAGE plpgsql;
CREATE OR REPLACE TRIGGER trg_inherit_procedure_assets
AFTER
INSERT ON procedure_assets FOR EACH ROW EXECUTE FUNCTION inherit_procedure_assets();