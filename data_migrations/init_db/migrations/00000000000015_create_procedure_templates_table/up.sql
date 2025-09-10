CREATE TABLE IF NOT EXISTS procedure_templates (
	-- Identifier of the procedure template
	procedure_template SERIAL PRIMARY KEY,
	-- The most concrete table variant descendant of this procedure template,
	-- which allows for rapidly determining the type of a procedure template
	-- without having to execute multiple queries.
	most_concrete_table TEXT NOT NULL,
	-- Human-readable name of the procedure template
	name TEXT UNIQUE NOT NULL CHECK (must_be_paragraph(name)),
	-- Human-readable description of the procedure template
	description TEXT NOT NULL CHECK (must_be_paragraph(description)),
	-- photograph_id UUID REFERENCES documents(id),
	icon TEXT NOT NULL DEFAULT 'book' CHECK (must_be_font_awesome_class(icon)),
	-- The user who created this procedure template
	created_by INTEGER NOT NULL REFERENCES users(id),
	-- The timestamp when this procedure template was created
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	-- The user who last updated this procedure template
	updated_by INTEGER NOT NULL REFERENCES users(id),
	-- The timestamp when this procedure template was last updated
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP CHECK (must_be_smaller_than_utc(created_at, updated_at)),
	-- Whether this procedure template is deprecated and should not be used for new procedures
	deprecated BOOLEAN NOT NULL DEFAULT FALSE,
	-- The number of subprocedures this procedure template has. This field
	-- is generally automatically updated via triggers on the parent_procedure_templates
	-- table, but it can also be manually updated if needed.
	number_of_subprocedure_templates SMALLINT NOT NULL DEFAULT 0 CHECK (
		must_be_strictly_positive_i16(number_of_subprocedure_templates)
	),
	-- We enforce that the name and description are distinct to avoid lazy duplicates
	CHECK (must_be_distinct(name, description))
);
CREATE TABLE IF NOT EXISTS parent_procedure_templates (
	PRIMARY KEY (parent, child),
	-- The parent procedure template
	parent INTEGER NOT NULL REFERENCES procedure_templates(procedure_template) ON DELETE CASCADE,
	-- The child procedure template
	child INTEGER NOT NULL REFERENCES procedure_templates(procedure_template) ON DELETE CASCADE CHECK (must_be_distinct_i32(parent, child)),
	-- The user who created this relationship
	created_by INTEGER NOT NULL REFERENCES users(id),
	-- The timestamp when this relationship was created
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Upon inserting a new parent-child relationship, we increment the number_of_subprocedure_templates
-- of the parent procedure template and update the `updated_at` and `updated_by` fields.
CREATE OR REPLACE FUNCTION increment_number_of_subprocedure_templates() RETURNS TRIGGER AS $$ BEGIN
UPDATE procedure_templates
SET number_of_subprocedure_templates = number_of_subprocedure_templates + 1,
	updated_at = CURRENT_TIMESTAMP,
	updated_by = NEW.created_by
WHERE procedure_template = NEW.parent;
RETURN NEW;
END;
$$ LANGUAGE plpgsql;
CREATE OR REPLACE TRIGGER trg_increment_number_of_subprocedure_templates
AFTER
INSERT ON parent_procedure_templates FOR EACH ROW EXECUTE FUNCTION increment_number_of_subprocedure_templates();

CREATE TABLE IF NOT EXISTS next_procedure_templates (
	PRIMARY KEY (parent, predecessor, successor),
	-- The parent procedure template
	parent INTEGER NOT NULL REFERENCES procedure_templates(procedure_template) ON DELETE CASCADE,
	-- The predecessor procedure template
	predecessor INTEGER NOT NULL REFERENCES procedure_templates(procedure_template) ON DELETE CASCADE,
	-- The successor procedure template
	successor INTEGER NOT NULL REFERENCES procedure_templates(procedure_template) ON DELETE CASCADE CHECK (must_be_distinct_i32(predecessor, successor)),
	-- The user who created this relationship
	created_by INTEGER NOT NULL REFERENCES users(id),
	-- The timestamp when this relationship was created
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	-- We enforce that the parent procedure is indeed a parent of the predecessor procedure
	FOREIGN KEY (parent, predecessor) REFERENCES parent_procedure_templates(parent, child),
	-- We enforce that the parent procedure is indeed a parent of the successor procedure
	FOREIGN KEY (parent, successor) REFERENCES parent_procedure_templates(parent, child)
);
-- Trigger function
CREATE OR REPLACE FUNCTION ensure_parent_procedure_templates() RETURNS TRIGGER LANGUAGE plpgsql AS $$ BEGIN -- Insert predecessor parent relationship if missing
INSERT INTO parent_procedure_templates (parent, child, created_by)
VALUES (NEW.parent, NEW.predecessor, NEW.created_by) ON CONFLICT (parent, child) DO NOTHING;
-- Insert successor parent relationship if missing
INSERT INTO parent_procedure_templates (parent, child, created_by)
VALUES (NEW.parent, NEW.successor, NEW.created_by) ON CONFLICT (parent, child) DO NOTHING;
RETURN NEW;
END;
$$;
-- Trigger
CREATE TRIGGER before_insert_next_procedure_templates BEFORE
INSERT ON next_procedure_templates FOR EACH ROW EXECUTE FUNCTION ensure_parent_procedure_templates();
CREATE TABLE IF NOT EXISTS procedure_template_asset_models (
	-- Identifier of the procedure template asset model
	id SERIAL PRIMARY KEY,
	-- The name of the procedure template asset model
	name TEXT NOT NULL CHECK (must_be_paragraph(name)),
	-- Procedure template this asset model is associated with
	procedure_template INTEGER NOT NULL REFERENCES procedure_templates(procedure_template) ON DELETE CASCADE,
	-- Optional reference to a procedure template asset model from another procedure template
	-- which this procedure template asset model is based on
	based_on INTEGER REFERENCES procedure_template_asset_models(id),
	-- The asset model this procedure template asset model is associated with
	asset_model INTEGER NOT NULL REFERENCES asset_models(id) ON DELETE CASCADE,
	-- The user who created this procedure template asset model
	created_by INTEGER NOT NULL REFERENCES users(id),
	-- The timestamp when this procedure template asset model was created
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	-- We enforce that, if based_on is specified, then the asset model must be the same as the one
	-- of the procedure template asset model it is based on.
	FOREIGN KEY (based_on, asset_model) REFERENCES procedure_template_asset_models(id, asset_model),
	-- The name of the procedure template asset model must be unique for a given procedure template
	-- (i.e., you cannot have two asset models with the same name for the same procedure template)
	UNIQUE (procedure_template, name),
	-- We create an index on (procedure_template, asset_model) to allow for foreign
	-- keys from the concrete procedures to check that the asset model is correctly aligned.
	UNIQUE (id, procedure_template),
	-- We create an index on (procedure_template, asset_model) to allow for foreign
	-- keys from the concrete procedures to check that the asset model is correctly aligned.
	UNIQUE (id, asset_model)
);
CREATE OR REPLACE FUNCTION inherit_procedure_template_asset_models() RETURNS TRIGGER AS $$ BEGIN
INSERT INTO procedure_template_asset_models (
		name,
		procedure_template,
		based_on,
		asset_model,
		created_by,
		created_at
	)
SELECT pam.name,
	NEW.parent,
	pam.id,
	pam.asset_model,
	NEW.created_by,
	NEW.created_at
FROM procedure_template_asset_models pam
WHERE pam.procedure_template = NEW.child;
RETURN NEW;
END;
$$ LANGUAGE plpgsql;
CREATE OR REPLACE TRIGGER trg_inherit_procedure_template_asset_models
AFTER
INSERT ON parent_procedure_templates FOR EACH ROW EXECUTE FUNCTION inherit_procedure_template_asset_models();