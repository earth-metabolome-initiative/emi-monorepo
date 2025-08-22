CREATE TABLE IF NOT EXISTS procedure_models (
	id SERIAL PRIMARY KEY,
	name TEXT UNIQUE NOT NULL CHECK (must_be_paragraph(name)),
	description TEXT NOT NULL CHECK (must_be_paragraph(description)),
	deprecated BOOLEAN NOT NULL DEFAULT FALSE,
	photograph_id UUID REFERENCES documents(id),
	icon TEXT NOT NULL DEFAULT 'book' CHECK (must_be_font_awesome_class(icon)),
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_by INTEGER NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	CHECK (must_be_distinct(name, description)),
	CHECK (must_be_smaller_than_utc(created_at, updated_at))
);
CREATE TABLE IF NOT EXISTS parent_procedure_models (
	parent_procedure_model_id INTEGER NOT NULL REFERENCES procedure_models(id) ON DELETE CASCADE,
	child_procedure_model_id INTEGER NOT NULL REFERENCES procedure_models(id) ON DELETE CASCADE,
	snoozable BOOLEAN NOT NULL DEFAULT FALSE,
	copiable BOOLEAN NOT NULL DEFAULT FALSE,
	repeatable BOOLEAN NOT NULL DEFAULT FALSE,
	skippable BOOLEAN NOT NULL DEFAULT FALSE,
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	CHECK (
		must_be_distinct_i32(
			parent_procedure_model_id,
			child_procedure_model_id
		)
	),
	PRIMARY KEY (
		parent_procedure_model_id,
		child_procedure_model_id
	)
);
CREATE TABLE IF NOT EXISTS next_procedure_models (
	parent_id INTEGER NOT NULL REFERENCES procedure_models(id) ON DELETE CASCADE,
	current_id INTEGER NOT NULL REFERENCES procedure_models(id) ON DELETE CASCADE,
	successor_id INTEGER NOT NULL REFERENCES procedure_models(id) ON DELETE CASCADE,
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	PRIMARY KEY (parent_id, current_id, successor_id),
	CHECK (must_be_distinct_i32(current_id, successor_id)),
	FOREIGN KEY (parent_id, current_id) REFERENCES parent_procedure_models(
		parent_procedure_model_id,
		child_procedure_model_id
	) ON DELETE CASCADE,
	FOREIGN KEY (parent_id, successor_id) REFERENCES parent_procedure_models(
		parent_procedure_model_id,
		child_procedure_model_id
	) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS procedure_model_trackables (
	id SERIAL PRIMARY KEY,
	name TEXT NOT NULL CHECK (must_be_paragraph(name)),
	procedure_model_id INTEGER NOT NULL REFERENCES procedure_models(id) ON DELETE CASCADE,
	trackable_id UUID NOT NULL REFERENCES trackables(id) ON DELETE CASCADE,
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_by INTEGER NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	UNIQUE (procedure_model_id, name),
	UNIQUE (id, procedure_model_id),
	UNIQUE (id, trackable_id),
	CHECK (must_be_smaller_than_utc(created_at, updated_at))
);
CREATE TABLE IF NOT EXISTS shared_procedure_model_trackables (
	parent_id INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	parent_trackable_id UUID NOT NULL REFERENCES trackables(id) ON DELETE CASCADE,
	parent_procedure_model_id INTEGER NOT NULL REFERENCES procedure_models(id) ON DELETE CASCADE,
	child_id INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	child_trackable_id UUID NOT NULL REFERENCES trackables(id) ON DELETE CASCADE,
	child_procedure_model_id INTEGER NOT NULL REFERENCES procedure_models(id) ON DELETE CASCADE,
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	PRIMARY KEY (parent_id, child_id),
	-- This foreign key ensures that the the `trackable_id` existing in the `parent_trackable_id` row
	-- is indeed the same as the one in the `procedure_model_trackables` table.
	FOREIGN KEY (parent_id, parent_trackable_id) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE,
	-- And this one is the analogous one for the `child_trackable_id`.
	FOREIGN KEY (child_id, child_trackable_id) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE,
	-- This foreign key ensures that the `parent_procedure_model_id` is indeed the same as the one in the `procedure_models` table.
	FOREIGN KEY (parent_id, parent_procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	-- And this one is the analogous one for the `child_procedure_model_id`.
	FOREIGN KEY (child_id, child_procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	-- Furthermore, we want to check that `parent_procedure_model_id` is indeed the parent of `child_procedure_model_id`
	-- as defined by the `parent_procedure_models` table.
	FOREIGN KEY (
		parent_procedure_model_id,
		child_procedure_model_id
	) REFERENCES parent_procedure_models(
		parent_procedure_model_id,
		child_procedure_model_id
	) ON DELETE CASCADE,
	CHECK (must_be_distinct_i32(parent_id, child_id)) -- TODO: add a check that ensures that the `parent_trackable_id` is compatible with the `child_trackable_id` as
	-- defined by the `trackables` table hierarchy. Specifically, the `parent_trackable_id` should be a descendant
	-- of the `child_trackable_id` as the child procedures are generally meant to be **less** specific than the parent ones.
);