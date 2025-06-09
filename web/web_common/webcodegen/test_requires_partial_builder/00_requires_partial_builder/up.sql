CREATE TABLE IF NOT EXISTS trackables (
	id SERIAL PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS procedure_models (
	id SERIAL PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS specific_procedure_models (
	id INTEGER PRIMARY KEY REFERENCES procedure_models(id)
);

CREATE TABLE IF NOT EXISTS procedure_model_trackables (
	id SERIAL PRIMARY KEY,
	procedure_model_id INTEGER NOT NULL REFERENCES procedure_models(id) ON DELETE CASCADE,
	trackable_id INTEGER NOT NULL REFERENCES trackables(id) ON DELETE CASCADE,
	UNIQUE (id, procedure_model_id)
);

CREATE TABLE IF NOT EXISTS instrumented_procedure_models (
	id INTEGER PRIMARY KEY REFERENCES procedure_models(id),
	instrument_id INTEGER NOT NULL REFERENCES procedure_model_trackables(id) ON DELETE CASCADE,
	FOREIGN KEY (instrument_id, id) REFERENCES procedure_model_trackables(id, procedure_model_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS procedures (
	id SERIAL PRIMARY KEY,
	procedure_model_id INTEGER NOT NULL REFERENCES procedure_models(id) ON DELETE CASCADE,
	UNIQUE (id, procedure_model_id)
);

CREATE TABLE IF NOT EXISTS specific_procedures (
	id INTEGER PRIMARY KEY REFERENCES procedures(id),
	procedure_model_id INTEGER NOT NULL REFERENCES specific_procedure_models(id) ON DELETE CASCADE,
	FOREIGN KEY (id, procedure_model_id) REFERENCES procedures(id, procedure_model_id) ON DELETE CASCADE
);