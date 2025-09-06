CREATE TABLE IF NOT EXISTS asset_models (
    id SERIAL PRIMARY KEY,
    most_concrete_table TEXT NOT NULL,
    name VARCHAR(255) NOT NULL UNIQUE CHECK (must_be_paragraph(name)),
    description TEXT NOT NULL CHECK (must_be_paragraph(description)),
    parent_model INTEGER REFERENCES asset_models(id) ON DELETE CASCADE,
    created_by INTEGER NOT NULL REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by INTEGER NOT NULL REFERENCES users(id),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CHECK (must_be_distinct(name, description)),
    CHECK (must_be_distinct_i32(id, parent_model)),
    CHECK (must_be_smaller_than_utc(created_at, updated_at)),
    UNIQUE (id, parent_model)
);
CREATE TABLE IF NOT EXISTS assets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    most_concrete_table TEXT NOT NULL,
    name VARCHAR(255) CHECK (must_be_paragraph(name)),
    description TEXT CHECK (must_be_paragraph(description)),
    model INTEGER NOT NULL REFERENCES asset_models(id) ON DELETE CASCADE,
    created_by INTEGER NOT NULL REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by INTEGER NOT NULL REFERENCES users(id),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CHECK (must_be_distinct(name, description)),
    CHECK (must_be_smaller_than_utc(created_at, updated_at)),
    UNIQUE (id, model),
    -- Assets of different models can have the same name, but not assets of the same model.
    UNIQUE (name, model)
);
CREATE TABLE IF NOT EXISTS physical_asset_models (
    id INTEGER PRIMARY KEY REFERENCES asset_models(id) ON DELETE CASCADE,
    parent_model INTEGER REFERENCES physical_asset_models(id) ON DELETE CASCADE,
    FOREIGN KEY (id, parent_model) REFERENCES asset_models(id, parent_model)
);
CREATE TABLE IF NOT EXISTS physical_assets (
    id UUID PRIMARY KEY REFERENCES assets(id) ON DELETE CASCADE,
    model INTEGER NOT NULL REFERENCES physical_asset_models(id) ON DELETE CASCADE,
    FOREIGN KEY (id, model) REFERENCES assets(id, model)
);
CREATE TABLE IF NOT EXISTS digital_asset_models (
    id INTEGER PRIMARY KEY REFERENCES asset_models(id) ON DELETE CASCADE,
    parent_model INTEGER REFERENCES digital_asset_models(id) ON DELETE CASCADE,
    FOREIGN KEY (id, parent_model) REFERENCES asset_models(id, parent_model)
);
CREATE TABLE IF NOT EXISTS digital_assets (
    id UUID PRIMARY KEY REFERENCES assets(id) ON DELETE CASCADE,
    model INTEGER NOT NULL REFERENCES digital_asset_models(id) ON DELETE CASCADE,
    FOREIGN KEY (id, model) REFERENCES assets(id, model)
);
CREATE TABLE IF NOT EXISTS asset_compatibility_rules (
    left_asset_model INTEGER NOT NULL REFERENCES asset_models(id) ON DELETE CASCADE,
    right_asset_model INTEGER NOT NULL REFERENCES asset_models(id) ON DELETE CASCADE,
    created_by INTEGER NOT NULL REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (left_asset_model, right_asset_model),
    CHECK (
        must_be_distinct_i32(left_asset_model, right_asset_model)
    )
);
CREATE UNIQUE INDEX unique_asset_compatibility_pair ON asset_compatibility_rules (
    LEAST(left_asset_model, right_asset_model),
    GREATEST(left_asset_model, right_asset_model)
);
CREATE TABLE IF NOT EXISTS asset_model_ancestors (
    descendant_model INTEGER NOT NULL REFERENCES asset_models(id) ON DELETE CASCADE,
    ancestor_model INTEGER NOT NULL REFERENCES asset_models(id) ON DELETE CASCADE,
    PRIMARY KEY (descendant_model, ancestor_model)
);