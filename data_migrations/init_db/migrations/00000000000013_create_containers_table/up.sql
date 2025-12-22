CREATE TABLE IF NOT EXISTS container_models (
    id INTEGER PRIMARY KEY REFERENCES physical_asset_models(id)
);
CREATE TABLE IF NOT EXISTS volumetric_container_models (
    id INTEGER PRIMARY KEY REFERENCES container_models(id),
    -- Volume in liters. The maximum volume of the container.
    volume REAL NOT NULL CHECK (volume > 0.0)
);
CREATE TABLE IF NOT EXISTS container_compatibility_rules (
    container_model_id INTEGER NOT NULL REFERENCES container_models(id),
    contained_asset_model_id INTEGER NOT NULL REFERENCES physical_asset_models(id),
    -- The maximal quantity of the right trackable that can be associated with the left trackable.
    quantity SMALLINT CHECK (quantity > 0),
    created_by_id INTEGER NOT NULL REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (container_model_id, contained_asset_model_id),
    CHECK (
        container_model_id <> contained_asset_model
    )
);
CREATE TABLE IF NOT EXISTS containers (
    id UUID PRIMARY KEY REFERENCES physical_assets(id) ON DELETE CASCADE,
    container_model_id INTEGER NOT NULL REFERENCES container_models(id),
    FOREIGN KEY (id, container_model_id) REFERENCES assets(id, model_id)
);
CREATE TABLE IF NOT EXISTS volumetric_containers (
    id UUID PRIMARY KEY REFERENCES containers(id) ON DELETE CASCADE,
    volumetric_container_model_id INTEGER NOT NULL REFERENCES volumetric_container_models(id),
    -- We ensure that the parent_id table's container_model_id is indeed a volumetric_container_model.
    FOREIGN KEY (id, volumetric_container_model_id) REFERENCES assets(id, model_id)
);