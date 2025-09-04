CREATE TABLE IF NOT EXISTS procedure_templates (
	procedure_template SERIAL PRIMARY KEY,
	most_concrete_table TEXT NOT NULL,
	name TEXT UNIQUE NOT NULL CHECK (must_be_paragraph(name)),
	description TEXT NOT NULL CHECK (must_be_paragraph(description)),
	deprecated BOOLEAN NOT NULL DEFAULT FALSE,
	-- photograph_id UUID REFERENCES documents(id),
	icon TEXT NOT NULL DEFAULT 'book' CHECK (must_be_font_awesome_class(icon)),
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_by INTEGER NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	CHECK (must_be_distinct(name, description)),
	CHECK (must_be_smaller_than_utc(created_at, updated_at))
);
CREATE TABLE IF NOT EXISTS parent_procedure_templates (
	parent_procedure_template INTEGER NOT NULL REFERENCES procedure_templates(procedure_template) ON DELETE CASCADE,
	child_procedure_template INTEGER NOT NULL REFERENCES procedure_templates(procedure_template) ON DELETE CASCADE,
	snoozable BOOLEAN NOT NULL DEFAULT FALSE,
	copiable BOOLEAN NOT NULL DEFAULT FALSE,
	repeatable BOOLEAN NOT NULL DEFAULT FALSE,
	skippable BOOLEAN NOT NULL DEFAULT FALSE,
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	CHECK (
		must_be_distinct_i32(
			parent_procedure_template,
			child_procedure_template
		)
	),
	PRIMARY KEY (
		parent_procedure_template,
		child_procedure_template
	)
);
CREATE TABLE IF NOT EXISTS next_procedure_templates (
	parent INTEGER NOT NULL REFERENCES procedure_templates(procedure_template) ON DELETE CASCADE,
	current INTEGER NOT NULL REFERENCES procedure_templates(procedure_template) ON DELETE CASCADE,
	successor_id INTEGER NOT NULL REFERENCES procedure_templates(procedure_template) ON DELETE CASCADE,
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	PRIMARY KEY (parent, current, successor_id),
	CHECK (must_be_distinct_i32(current, successor_id)),
	FOREIGN KEY (parent, current) REFERENCES parent_procedure_templates(
		parent_procedure_template,
		child_procedure_template
	) ON DELETE CASCADE,
	FOREIGN KEY (parent, successor_id) REFERENCES parent_procedure_templates(
		parent_procedure_template,
		child_procedure_template
	) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS procedure_template_asset_models (
	id SERIAL PRIMARY KEY,
	name TEXT NOT NULL CHECK (must_be_paragraph(name)),
	procedure_template INTEGER NOT NULL REFERENCES procedure_templates(procedure_template) ON DELETE CASCADE,
	asset_model INTEGER NOT NULL REFERENCES asset_models(id) ON DELETE CASCADE,
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_by INTEGER NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	UNIQUE (procedure_template, name),
	UNIQUE (id, procedure_template),
	UNIQUE (id, asset_model),
	CHECK (must_be_smaller_than_utc(created_at, updated_at))
);