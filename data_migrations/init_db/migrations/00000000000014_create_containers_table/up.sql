CREATE TABLE IF NOT EXISTS container_models (
    id INTEGER PRIMARY KEY REFERENCES physical_asset_models(id)
);
CREATE TABLE IF NOT EXISTS volumetric_container_models (
    id INTEGER PRIMARY KEY REFERENCES container_models(id),
    -- The maximum volume of the container in liters.
    liters REAL NOT NULL CHECK (must_be_strictly_positive_f32(liters))
);
CREATE TABLE IF NOT EXISTS container_compatibility_rules (
    container_model INTEGER NOT NULL REFERENCES container_models(id),
    contained_asset_model INTEGER NOT NULL REFERENCES physical_asset_models(id),
    -- The maximal quantity of the right trackable that can be associated with the left trackable.
    quantity SMALLINT CHECK (must_be_strictly_positive_i16(quantity)),
    created_by INTEGER NOT NULL REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (container_model, contained_asset_model),
    CHECK (
        must_be_distinct_i32(container_model, contained_asset_model)
    )
);
CREATE TABLE IF NOT EXISTS containers (
    id UUID PRIMARY KEY REFERENCES physical_assets(id) ON DELETE CASCADE,
    container_model INTEGER NOT NULL REFERENCES container_models(id),
    FOREIGN KEY (id, container_model) REFERENCES assets(id, model)
);
CREATE TABLE IF NOT EXISTS volumetric_containers (
    id UUID PRIMARY KEY REFERENCES containers(id) ON DELETE CASCADE,
    volumetric_container_model INTEGER NOT NULL REFERENCES volumetric_container_models(id),
    -- We ensure that the parent table's container_model is indeed a volumetric_container_model.
    FOREIGN KEY (id, volumetric_container_model) REFERENCES assets(id, model)
);