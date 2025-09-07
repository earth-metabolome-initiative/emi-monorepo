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
	-- We enforce that the name and description are distinct to avoid lazy duplicates
	CHECK (must_be_distinct(name, description))
);
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
	FOREIGN KEY (parent, predecessor) REFERENCES parent_procedure_templates(parent, child) ON DELETE CASCADE,
	-- We enforce that the parent procedure is indeed a parent of the successor procedure
	FOREIGN KEY (parent, successor) REFERENCES parent_procedure_templates(parent, child) ON DELETE CASCADE
);
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