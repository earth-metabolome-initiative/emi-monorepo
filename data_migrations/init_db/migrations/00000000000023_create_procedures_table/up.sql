CREATE TABLE IF NOT EXISTS procedures (
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	procedure_model_id INTEGER NOT NULL REFERENCES procedure_models(id),
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_by INTEGER NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	CHECK (must_be_smaller_than_utc(created_at, updated_at)),
	UNIQUE (id, procedure_model_id)
);
CREATE TABLE IF NOT EXISTS procedure_trackables (
	procedure_id UUID NOT NULL REFERENCES procedures(id),
	procedure_model_id INTEGER NOT NULL REFERENCES procedure_models(id),
	trackable_id UUID NOT NULL REFERENCES trackables(id),
	-- We enforce that there must be a procedure model trackable for this trackable.
	procedure_model_trackable_id INTEGER NOT NULL REFERENCES procedure_model_trackables(id),
	-- We redound the parent trackable defined in the procedure model trackable, which will be used
	-- to both ensure that the `procedure_model_trackable_id` is indeed compatible, and also to check
	-- that the `trackable_id` is a descendant of the parent trackable.
	ancestor_trackable_id UUID NOT NULL REFERENCES trackables(id),
	parent_trackable_id UUID NOT NULL REFERENCES trackables(id),
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	PRIMARY KEY (procedure_id, trackable_id),
	-- The procedure model must match the procedure model of the procedure.
	FOREIGN KEY (procedure_id, procedure_model_id) REFERENCES procedures(id, procedure_model_id) ON DELETE CASCADE,
	-- The procedure model trackable must must be compatible with the procedure model of the procedure.
	FOREIGN KEY (procedure_model_trackable_id, procedure_model_id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE,
	-- We check that the ancestor trackable is indeed the one defined in the procedure model trackable.
	FOREIGN KEY (
		procedure_model_trackable_id,
		ancestor_trackable_id
	) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE,
	-- We check that the trackable is indeed a descendant of the ancestor trackable defined in the procedure model trackable.
	FOREIGN KEY (trackable_id, ancestor_trackable_id) REFERENCES trackable_ancestors(trackable_id, ancestor_id)
);