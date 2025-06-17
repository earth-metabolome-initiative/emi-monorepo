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
	FOREIGN KEY (procedure_model_trackable_id, ancestor_trackable_id) REFERENCES procedure_model_trackables(id, trackable_id) ON DELETE CASCADE
	-- TODO: add a check using the trackables closure table to ensure that the `trackable_id` is a descendant of the `ancestor_trackable_id`.
);

--------------------------------------------
-- HERE BEGIN THE SPECIALIZED STEP TABLES --
--------------------------------------------

CREATE TABLE IF NOT EXISTS weighing_procedures (
	procedure_id UUID PRIMARY KEY REFERENCES procedures(id),
	-- We enforce that the model of this procedure must be a weighing procedure model.
	procedure_model_id INTEGER NOT NULL REFERENCES weighing_procedure_models(procedure_model_id),
	-- The identifier of the instrument used for weighing.
	instrument_id UUID NOT NULL REFERENCES trackables(id),
	-- The measured weight, which must be strictly positive.
	kilograms REAL NOT NULL CHECK (must_be_strictly_positive_f32(kilograms)),
	-- We enforce that the extended `procedure` has indeed the same `procedure_model_id`, making
	-- sure that the procedure is a weighing procedure without the possibility of a mistake.
	FOREIGN KEY (procedure_id, procedure_model_id) REFERENCES procedures(id, procedure_model_id) ON DELETE CASCADE,
	-- We enforce that there exists a `procedure_trackables` entry for the instrument used in the procedure.
	FOREIGN KEY (procedure_id, instrument_id) REFERENCES procedure_trackables(procedure_id, trackable_id) ON DELETE CASCADE
	-- There us no need here to check that the `instrument_id` is a weighing instrument nor that it is
	-- a descendant of the ancestor trackable defined in the procedure model trackable, as both checks are
	-- already enforced in the `procedure_trackables` table and are therefore fulfilled by definition when
	-- there exists the foreign key reference to `procedure_trackables` above.
);