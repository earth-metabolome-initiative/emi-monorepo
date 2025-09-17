CREATE TABLE IF NOT EXISTS trackables (
	id SERIAL PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS procedure_templates (
	id SERIAL PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS specific_procedure_templates (
	id INTEGER PRIMARY KEY REFERENCES procedure_templates(id)
);

CREATE TABLE IF NOT EXISTS procedure_template_trackables (
	id SERIAL PRIMARY KEY,
	procedure_template INTEGER NOT NULL REFERENCES procedure_templates(id) ON DELETE CASCADE,
	trackable_id INTEGER NOT NULL REFERENCES trackables(id) ON DELETE CASCADE,
	UNIQUE (id, procedure_template)
);

CREATE TABLE IF NOT EXISTS instrumented_procedure_templates (
	id INTEGER PRIMARY KEY REFERENCES procedure_templates(id),
	instrument_id INTEGER NOT NULL REFERENCES procedure_template_trackables(id) ON DELETE CASCADE,
	FOREIGN KEY (instrument_id, id) REFERENCES procedure_template_trackables(id, procedure_template) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS procedures (
	id SERIAL PRIMARY KEY,
	procedure_template INTEGER NOT NULL REFERENCES procedure_templates(id) ON DELETE CASCADE,
	UNIQUE (id, procedure_template)
);

CREATE TABLE IF NOT EXISTS specific_procedures (
	id INTEGER PRIMARY KEY REFERENCES procedures(id),
	procedure_template INTEGER NOT NULL REFERENCES specific_procedure_templates(id) ON DELETE CASCADE,
	FOREIGN KEY (id, procedure_template) REFERENCES procedures(id, procedure_template) ON DELETE CASCADE
);