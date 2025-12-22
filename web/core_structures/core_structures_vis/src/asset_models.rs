//! Submodule providing an illustration of the asset models hierarchy using an
//! Entity-Relationship Diagram (ERD).

use core_structures::{AssetCompatibilityRule, AssetModel};
use diesel::PgConnection;
use mermaid::prelude::*;
use web_common_traits::database::BoundedRead;

use crate::Error;

fn asset_model_hash(asset_model: &AssetModel) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    asset_model.hash(&mut hasher);
    hasher.finish()
}

/// Generates an Entity-Relationship Diagram (ERD) for the asset models
/// hierarchy.
///
/// # Arguments
///
/// * `conn` - A mutable reference to the `PostgreSQL` database connection.
///
/// # Errors
///
/// * Returns an `Error` if there is an issue generating the ERD.
pub fn asset_model_hierarchy(conn: &mut PgConnection) -> Result<ERDiagram, Error> {
    let asset_models: Vec<AssetModel> = AssetModel::bounded_read(0, u16::MAX, conn)?;
    let compatibility_rules: Vec<AssetCompatibilityRule> =
        AssetCompatibilityRule::bounded_read(0, u16::MAX, conn)?;

    let mut builder = ERDiagramBuilder::default()
        .configuration(ERDiagramConfigurationBuilder::default().title("Trackables Hierarchy")?)?;

    // First, we inser all of the asset_models as nodes.
    for asset_model_id in &asset_models {
        builder.node(
            ERNodeBuilder::default()
                .id(asset_model_hash(asset_model_id))
                .label(&asset_model.name)?,
        )?;
    }

    // Next, we insert the asset_models' parent-child relationships as edges.
    for asset_model_id in &asset_models {
        let Some(parent) = asset_model.parent_model(conn)? else {
            continue;
        };

        let parent_node_id = asset_model_hash(&parent);
        let child_node_id = asset_model_hash(asset_model_id);

        let child_node = builder.get_node_by_id(child_node_id).expect("Trackable node not found");
        let parent_node =
            builder.get_node_by_id(parent_node_id).expect("Parent asset_model_id node not found");

        builder.edge(EREdgeBuilder::one_to_one(child_node, parent_node).label("child of")?)?;
    }

    // Finally, we insert the compatibility rules as edges.
    for rule in &compatibility_rules {
        let left_asset_model_id = rule.left_asset_model(conn)?;
        let right_asset_model_id = rule.right_asset_model(conn)?;

        let left_node_id = asset_model_hash(&left_asset_model_id);
        let right_node_id = asset_model_hash(&right_asset_model_id);

        let left_node =
            builder.get_node_by_id(left_node_id).expect("Left asset_model_id node not found");
        let right_node =
            builder.get_node_by_id(right_node_id).expect("Right asset_model_id node not found");
        builder.edge(EREdgeBuilder::one_to_one(left_node, right_node).label("compatible with")?)?;
    }

    Ok(builder.into())
}
