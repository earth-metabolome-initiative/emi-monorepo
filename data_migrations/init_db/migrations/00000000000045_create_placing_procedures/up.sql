CREATE TABLE IF NOT EXISTS placing_procedure_templates (
	-- Identifier of the placing procedure template, which is also a foreign key to the general procedure template.
	procedure_template INTEGER PRIMARY KEY REFERENCES geolocation_procedure_templates(procedure_template) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS placing_procedures (
	-- Identifier of the placing procedure, which is also a foreign key to the general procedure.
	procedure UUID PRIMARY KEY REFERENCES geolocation_procedures(procedure) ON DELETE CASCADE,
	-- The template of this procedure should be a placing procedure template.
	procedure_template INTEGER NOT NULL REFERENCES placing_procedure_templates(procedure_template),
	-- We enforce that the current `placing` has indeed the same `placing_template`.
	FOREIGN KEY (procedure, procedure_template) REFERENCES procedures(procedure, procedure_template)
);