CREATE TABLE IF NOT EXISTS storage_procedure_templates (
	id INTEGER PRIMARY KEY REFERENCES procedure_templates(id) ON DELETE CASCADE,
	-- The storage temperature in Kelvin.
	kelvin REAL NOT NULL DEFAULT 293.15 CHECK (kelvin > 0.0),
	-- Tolerance percentage for the storage temperature.
	kelvin_tolerance_percentage REAL NOT NULL DEFAULT 1.0 CHECK (
		kelvin_tolerance_percentage > 0.0
		AND kelvin_tolerance_percentage <= 100.0
	),
	-- The container that will be used for storage.
	stored_into_model_id INTEGER NOT NULL REFERENCES container_models(id),
	procedure_template_stored_into_model_id INTEGER NOT NULL REFERENCES procedure_template_asset_models(id) ON DELETE CASCADE,
	-- The asset that is being stored.
	stored_asset_model_id INTEGER NOT NULL REFERENCES physical_asset_models(id),
	procedure_template_stored_asset_model_id INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- We check that the `stored_into_model` is indeed a container that is compatible with the procedure_id template.
	FOREIGN KEY (
		procedure_template_stored_into_model_id,
		stored_into_model_id
	) REFERENCES procedure_template_asset_models(id, asset_model_id),
	-- We check that the `stored_asset_model` is indeed a container that is compatible with the procedure_id template.
	FOREIGN KEY (
		procedure_template_stored_asset_model_id,
		stored_asset_model_id
	) REFERENCES procedure_template_asset_models(id, asset_model_id),
	-- We check that the `stored_into_model` is indeed a container that can hold the `stored_asset_model`.
	FOREIGN KEY (stored_into_model_id, stored_asset_model_id) REFERENCES container_compatibility_rules(container_model_id, contained_asset_model_id),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_stored_into_model`
	-- for the current `procedure_template`.
	UNIQUE (
		id,
		procedure_template_stored_into_model_id
	),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_stored_asset_model`
	-- for the current `procedure_template`.
	UNIQUE (
		id,
		procedure_template_stored_asset_model_id
	)
);
CREATE TABLE IF NOT EXISTS storage_procedures (
	-- Identifier of the storage id, which is also a foreign key to the general procedure.
	id UUID PRIMARY KEY REFERENCES procedures(id) ON DELETE CASCADE,
	-- The template of this procedure_id should be a storage procedure_id template.
	storage_procedure_template_id INTEGER NOT NULL REFERENCES storage_procedure_templates(id),
	-- The asset being stored, which must be a physical asset.
	stored_asset_id UUID NOT NULL REFERENCES physical_assets(id),
	-- The model of the asset being stored, which must be a physical asset model.
	stored_asset_model_id INTEGER NOT NULL REFERENCES physical_asset_models(id),
	-- The procedure_id template asset model describing the `stored_asset`.
	procedure_template_stored_asset_model_id INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The procedure_id asset describing the `stored_asset`.
	procedure_stored_asset_id UUID NOT NULL REFERENCES procedure_assets(id),
	-- The container into which the asset is being stored.
	stored_into_id UUID NOT NULL REFERENCES containers(id),
	-- The model of the container into which the asset is being stored.
	stored_into_model_id INTEGER NOT NULL REFERENCES container_models(id),
	-- The procedure_id template asset model describing the `stored_into`.
	procedure_template_stored_into_model_id INTEGER NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The procedure_id asset describing the `stored_into`.
	procedure_stored_into_id UUID NOT NULL REFERENCES procedure_assets(id),
	-- The current procedure_id must be a storage procedure.
	FOREIGN KEY (id, storage_procedure_template_id) REFERENCES procedures(id, procedure_template_id),
	-- The procedure_id template asset model describing the `stored_asset` must be the same one
	-- as the one in the procedure_id template.
	FOREIGN KEY (
		storage_procedure_template_id,
		procedure_template_stored_asset_model_id
	) REFERENCES storage_procedure_templates(
		id,
		procedure_template_stored_asset_model_id
	),
	-- The procedure_id template asset model describing the `stored_into` must be the same one
	-- as the one in the procedure_id template.
	FOREIGN KEY (
		storage_procedure_template_id,
		procedure_template_stored_into_model_id
	) REFERENCES storage_procedure_templates(
		id,
		procedure_template_stored_into_model_id
	),
	-- The procedure_id template asset model and the procedure_id asset describing the `stored_asset`
	-- must be compatible.
	FOREIGN KEY (
		procedure_stored_asset_id,
		procedure_template_stored_asset_model_id
	) REFERENCES procedure_assets(id, procedure_template_asset_model_id),
	-- The procedure_id template asset model and the procedure_id asset describing the `stored_into`
	-- must be compatible.
	FOREIGN KEY (
		procedure_stored_into_id,
		procedure_template_stored_into_model_id
	) REFERENCES procedure_assets(id, procedure_template_asset_model_id),
	-- Check the compatibility between the `stored_asset` and the `stored_into_model`.
	FOREIGN KEY (stored_into_model_id, stored_asset_model_id) REFERENCES container_compatibility_rules(container_model_id, contained_asset_model_id),
	-- We check that the `procedure_stored_asset` is associated to the `stored_asset_model`.
	FOREIGN KEY (procedure_stored_asset_id, stored_asset_model_id) REFERENCES procedure_assets(id, asset_model_id),
	-- We check that the `procedure_stored_into` is associated to the `stored_into_model`.
	FOREIGN KEY (procedure_stored_into_id, stored_into_model_id) REFERENCES procedure_assets(id, asset_model_id),
	-- We check that the `procedure_stored_asset` is associated to the `stored_asset`.
	FOREIGN KEY (procedure_stored_asset_id, stored_asset_id) REFERENCES procedure_assets(id, asset_id),
	-- We check that the `procedure_stored_into` is associated to the `stored_into`.
	FOREIGN KEY (procedure_stored_into_id, stored_into_id) REFERENCES procedure_assets(id, asset_id)
);