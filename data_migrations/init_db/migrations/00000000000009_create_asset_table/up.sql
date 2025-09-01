CREATE TABLE IF NOT EXISTS asset_models (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) UNIQUE CHECK (must_be_paragraph(description)),
    description TEXT CHECK (must_be_paragraph(description)),
    parent_model_id INTEGER REFERENCES asset_models(id) ON DELETE CASCADE,
    created_by INTEGER NOT NULL REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by INTEGER NOT NULL REFERENCES users(id),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CHECK (must_be_distinct(name, description)),
    CHECK (must_be_distinct_i32(id, parent_model_id)),
    CHECK (must_be_smaller_than_utc(created_at, updated_at)),
    UNIQUE (id, parent_model_id)
);
CREATE TABLE IF NOT EXISTS assets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) UNIQUE CHECK (must_be_paragraph(description)),
    description TEXT CHECK (must_be_paragraph(description)),
    model_id INTEGER NOT NULL REFERENCES asset_models(id) ON DELETE CASCADE,
    created_by INTEGER NOT NULL REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by INTEGER NOT NULL REFERENCES users(id),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CHECK (must_be_distinct(name, description)),
    CHECK (must_be_smaller_than_utc(created_at, updated_at)),
    UNIQUE (id, model_id)
);
CREATE TABLE IF NOT EXISTS physical_asset_models (
    id INTEGER PRIMARY KEY REFERENCES asset_models(id) ON DELETE CASCADE,
    parent_model_id INTEGER REFERENCES physical_asset_models(id) ON DELETE CASCADE,
    FOREIGN KEY (id, parent_model_id) REFERENCES asset_models(id, parent_model_id)
);
CREATE TABLE IF NOT EXISTS physical_assets (
    id UUID PRIMARY KEY REFERENCES assets(id) ON DELETE CASCADE,
    model_id INTEGER NOT NULL REFERENCES physical_asset_models(id) ON DELETE CASCADE,
    FOREIGN KEY (id, model_id) REFERENCES assets(id, model_id)
);
CREATE TABLE IF NOT EXISTS digital_asset_models (
    id INTEGER PRIMARY KEY REFERENCES asset_models(id) ON DELETE CASCADE,
    parent_model_id INTEGER REFERENCES digital_asset_models(id) ON DELETE CASCADE,
    FOREIGN KEY (id, parent_model_id) REFERENCES asset_models(id, parent_model_id)
);
CREATE TABLE IF NOT EXISTS digital_assets (
    id UUID PRIMARY KEY REFERENCES assets(id) ON DELETE CASCADE,
    model_id INTEGER NOT NULL REFERENCES digital_asset_models(id) ON DELETE CASCADE,
    FOREIGN KEY (id, model_id) REFERENCES assets(id, model_id)
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
CREATE TABLE IF NOT EXISTS asset_model_ancestors (
    descendant_model INTEGER NOT NULL REFERENCES asset_models(id) ON DELETE CASCADE,
    ancestor_model INTEGER NOT NULL REFERENCES asset_models(id) ON DELETE CASCADE,
    PRIMARY KEY (descendant_model, ancestor_model)
);