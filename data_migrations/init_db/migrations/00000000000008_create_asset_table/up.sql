CREATE TABLE IF NOT EXISTS asset_models (
    id SERIAL PRIMARY KEY,
    most_concrete_table TEXT NOT NULL,
    name VARCHAR(255) NOT NULL UNIQUE CHECK (must_be_paragraph(name)),
    description TEXT NOT NULL CHECK (must_be_paragraph(description)),
    parent_model_id INTEGER REFERENCES asset_models(id) ON DELETE CASCADE,
    created_by_id INTEGER NOT NULL REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by_id INTEGER NOT NULL REFERENCES users(id),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CHECK (name <> description),
    CHECK (id <> parent_model_id),
    CHECK (created_at <= updated_at),
    UNIQUE (id, parent_model_id)
);
CREATE TABLE IF NOT EXISTS assets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    most_concrete_table TEXT NOT NULL,
    name VARCHAR(255) CHECK (must_be_paragraph(name)),
    description TEXT CHECK (must_be_paragraph(description)),
    model_id INTEGER NOT NULL REFERENCES asset_models(id) ON DELETE CASCADE,
    created_by_id INTEGER NOT NULL REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by_id INTEGER NOT NULL REFERENCES users(id),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CHECK (name <> description),
    CHECK (created_at <= updated_at),
    UNIQUE (id, model_id),
    -- Assets of different models can have the same name, but not assets of the same model.
    UNIQUE (name, model_id)
);
CREATE TABLE IF NOT EXISTS physical_asset_models (
    id INTEGER PRIMARY KEY REFERENCES asset_models(id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS physical_assets (
    id UUID PRIMARY KEY REFERENCES assets(id) ON DELETE CASCADE,
    physical_asset_model_id INTEGER NOT NULL REFERENCES physical_asset_models(id) ON DELETE CASCADE,
    FOREIGN KEY (id, physical_asset_model_id) REFERENCES assets(id, model_id)
);
CREATE TABLE IF NOT EXISTS digital_asset_models (
    id INTEGER PRIMARY KEY REFERENCES asset_models(id) ON DELETE CASCADE,
    mime_type MediaType NOT NULL
);
CREATE TABLE IF NOT EXISTS digital_assets (
    id UUID PRIMARY KEY REFERENCES assets(id) ON DELETE CASCADE,
    digital_asset_model_id INTEGER NOT NULL REFERENCES digital_asset_models(id) ON DELETE CASCADE,
    FOREIGN KEY (id, digital_asset_model_id) REFERENCES assets(id, model_id)
);
CREATE TABLE IF NOT EXISTS asset_compatibility_rules (
    left_asset_model_id INTEGER NOT NULL REFERENCES asset_models(id) ON DELETE CASCADE,
    right_asset_model_id INTEGER NOT NULL REFERENCES asset_models(id) ON DELETE CASCADE,
    created_by_id INTEGER NOT NULL REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (left_asset_model_id, right_asset_model_id),
    CHECK (
        left_asset_model_id <> right_asset_model_id
    )
);
CREATE UNIQUE INDEX unique_asset_compatibility_pair ON asset_compatibility_rules (
    LEAST(left_asset_model_id, right_asset_model_id),
    GREATEST(left_asset_model_id, right_asset_model_id)
);
CREATE TABLE IF NOT EXISTS asset_model_ancestors (
    descendant_model_id INTEGER NOT NULL REFERENCES asset_models(id) ON DELETE CASCADE,
    ancestor_model_id INTEGER NOT NULL REFERENCES asset_models(id) ON DELETE CASCADE,
    PRIMARY KEY (descendant_model_id, ancestor_model_id)
);
-- When a new `asset_models` row is inserted, we also populate the `asset_model_ancestors` table
-- with the tautological relationship (a row is an ancestor of itself) and all the ancestors of its parent_id model.
CREATE OR REPLACE FUNCTION populate_asset_model_ancestors() RETURNS TRIGGER AS $$ BEGIN -- Insert the tautological relationship
INSERT INTO asset_model_ancestors (descendant_model_id, ancestor_model_id)
VALUES (NEW.id, NEW.id);
-- Insert all ancestors of the parent_id model
INSERT INTO asset_model_ancestors (descendant_model_id, ancestor_model_id)
SELECT NEW.id,
    ancestor_model_id
FROM asset_model_ancestors
WHERE descendant_model_id = NEW.parent_model_id AND NEW.parent_model_id IS NOT NULL;
RETURN NEW;
END;
$$ LANGUAGE plpgsql;
CREATE OR REPLACE TRIGGER after_insert_asset_models
AFTER
INSERT ON asset_models FOR EACH ROW EXECUTE FUNCTION populate_asset_model_ancestors();
-- When an `asset_models` row is deleted, we also delete all its descendants.
CREATE OR REPLACE FUNCTION delete_descendant_asset_models() RETURNS TRIGGER AS $$ BEGIN
DELETE FROM asset_models
WHERE id IN (
        SELECT descendant_model_id
        FROM asset_model_ancestors
        WHERE ancestor_model_id = OLD.id
            AND descendant_model_id != OLD.id
    );
RETURN OLD;
END;
$$ LANGUAGE plpgsql;
CREATE OR REPLACE TRIGGER after_delete_asset_models
AFTER DELETE ON asset_models FOR EACH ROW EXECUTE FUNCTION delete_descendant_asset_models();