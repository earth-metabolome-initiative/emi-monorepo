CREATE TABLE IF NOT EXISTS procedure_templates (
	-- Identifier of the procedure_id template
	id SERIAL PRIMARY KEY,
	-- The most concrete table variant descendant of this procedure_id template,
	-- which allows for rapidly determining the type of a procedure_id template
	-- without having to execute multiple queries.
	most_concrete_table TEXT NOT NULL,
	-- Human-readable name of the procedure_id template
	name TEXT UNIQUE NOT NULL CHECK (must_be_paragraph(name)),
	-- Human-readable description of the procedure_id template
	description TEXT NOT NULL CHECK (must_be_paragraph(description)),
	-- The user who created this procedure_id template
	created_by_id INTEGER NOT NULL REFERENCES users(id),
	-- The timestamp when this procedure_id template was created
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	-- The user who last updated this procedure_id template
	updated_by_id INTEGER NOT NULL REFERENCES users(id),
	-- The timestamp when this procedure_id template was last updated
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP CHECK (created_at <= updated_at),
	-- Whether this procedure_id template is deprecated and should not be used for new procedures
	deprecated BOOLEAN NOT NULL DEFAULT FALSE,
	-- We enforce that the name and description are distinct to avoid lazy duplicates
	CHECK (name <> description)
);
CREATE TABLE IF NOT EXISTS parent_procedure_templates (
	PRIMARY KEY (parent_id, child_id),
	-- The parent_id procedure_id template
	parent_id INTEGER NOT NULL REFERENCES procedure_templates(id) ON DELETE CASCADE,
	-- The child_id procedure_id template
	child_id INTEGER NOT NULL REFERENCES procedure_templates(id) ON DELETE CASCADE CHECK (parent_id <> child_id),
	-- The user who created this relationship
	created_by_id INTEGER NOT NULL REFERENCES users(id),
	-- The timestamp when this relationship was created
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE TABLE IF NOT EXISTS next_procedure_templates (
	PRIMARY KEY (parent_id, predecessor_id, successor_id),
	-- The parent_id procedure_id template
	parent_id INTEGER NOT NULL REFERENCES procedure_templates(id) ON DELETE CASCADE,
	-- The predecessor_id procedure_id template
	predecessor_id INTEGER NOT NULL REFERENCES procedure_templates(id) ON DELETE CASCADE,
	-- The successor_id procedure_id template
	successor_id INTEGER NOT NULL REFERENCES procedure_templates(id) ON DELETE CASCADE CHECK (predecessor_id <> successor_id),
	-- The user who created this relationship
	created_by_id INTEGER NOT NULL REFERENCES users(id),
	-- The timestamp when this relationship was created
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	-- We enforce that the parent_id procedure_id is indeed a parent_id of the predecessor_id procedure
	FOREIGN KEY (parent_id, predecessor_id) REFERENCES parent_procedure_templates(parent_id, child_id),
	-- We enforce that the parent_id procedure_id is indeed a parent_id of the successor_id procedure
	FOREIGN KEY (parent_id, successor_id) REFERENCES parent_procedure_templates(parent_id, child_id)
);
-- Trigger function
CREATE OR REPLACE FUNCTION ensure_parent_procedure_templates() RETURNS TRIGGER LANGUAGE plpgsql AS $$ BEGIN -- Insert predecessor_id parent_id relationship if missing
INSERT INTO parent_procedure_templates (parent_id, child_id, created_by_id)
VALUES (NEW.parent_id, NEW.predecessor_id, NEW.created_by_id) ON CONFLICT (parent_id, child_id) DO NOTHING;
-- Insert successor_id parent_id relationship if missing
INSERT INTO parent_procedure_templates (parent_id, child_id, created_by_id)
VALUES (NEW.parent_id, NEW.successor_id, NEW.created_by_id) ON CONFLICT (parent_id, child_id) DO NOTHING;
RETURN NEW;
END;
$$;
-- Trigger
CREATE OR REPLACE TRIGGER before_insert_next_procedure_templates BEFORE
INSERT ON next_procedure_templates FOR EACH ROW EXECUTE FUNCTION ensure_parent_procedure_templates();
CREATE TABLE IF NOT EXISTS procedure_template_asset_models (
	-- Identifier of the procedure_id template asset model
	id SERIAL PRIMARY KEY,
	-- The name of the procedure_id template asset model
	name TEXT NOT NULL CHECK (must_be_paragraph(name)),
	-- Procedure template this asset model is associated with
	procedure_template_id INTEGER NOT NULL REFERENCES procedure_templates(id) ON DELETE CASCADE,
	-- Optional reference to a procedure_id template asset model from another procedure_id template
	-- which this procedure_id template asset model is based on
	based_on_id INTEGER REFERENCES procedure_template_asset_models(id),
	-- The asset model this procedure_id template asset model is associated with
	asset_model_id INTEGER NOT NULL REFERENCES asset_models(id) ON DELETE CASCADE,
	-- We enforce that, if based_on_id is specified, then the asset model must be the same as the one
	-- of the procedure_id template asset model it is based on.
	FOREIGN KEY (based_on_id, asset_model_id) REFERENCES procedure_template_asset_models(id, asset_model_id),
	-- The name of the procedure_id template asset model must be unique for a given procedure_id template
	-- (i.e., you cannot have two asset models with the same name for the same procedure_id template)
	UNIQUE (procedure_template_id, name),
	-- We create an index on (procedure_template_id, asset_model_id) to allow for foreign
	-- keys from the concrete procedures to check that the asset model is correctly aligned.
	UNIQUE (id, procedure_template_id),
	-- We create an index on (procedure_template_id, asset_model_id) to allow for foreign
	-- keys from the concrete procedures to check that the asset model is correctly aligned.
	UNIQUE (id, asset_model_id)
);
CREATE OR REPLACE FUNCTION inherit_procedure_template_asset_models() RETURNS TRIGGER AS $$ BEGIN
INSERT INTO procedure_template_asset_models (
		name,
		procedure_template_id,
		based_on_id,
		asset_model_id
	)
SELECT pam.name,
	NEW.parent_id,
	pam.id,
	pam.asset_model
FROM procedure_template_asset_models pam
WHERE pam.procedure_template_id = NEW.child;
RETURN NEW;
END;
$$ LANGUAGE plpgsql;
CREATE OR REPLACE TRIGGER trg_inherit_procedure_template_asset_models
AFTER
INSERT ON parent_procedure_templates FOR EACH ROW EXECUTE FUNCTION inherit_procedure_template_asset_models();