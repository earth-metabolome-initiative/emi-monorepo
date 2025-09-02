//! Submodule providing an illustration of the asset models hierarchy using an
//! Entity-Relationship Diagram (ERD).

use core_structures::{AssetCompatibilityRule, AssetModel};
use diesel::PgConnection;
use mermaid::prelude::*;
use web_common_traits::database::BoundedRead;

use crate::Error;

/// Generates an Entity-Relationship Diagram (ERD) for the asset models
/// hierarchy.
pub fn asset_model_hierarchy(conn: &mut PgConnection) -> Result<ERDiagram, Error> {
    let asset_models: Vec<AssetModel> = AssetModel::bounded_read(0, u16::MAX, conn)?;
    let compatibility_rules: Vec<AssetCompatibilityRule> =
        AssetCompatibilityRule::bounded_read(0, u16::MAX, conn)?;

    let mut builder = ERDiagramBuilder::default()
        .configuration(ERDiagramConfigurationBuilder::default().title("Trackables Hierarchy")?)?;

    // First, we inser all of the asset_models as nodes.
    for asset_model in &asset_models {
        builder.node(ERNodeBuilder::default().label(
            asset_model.name.clone().unwrap_or_else(|| format!("asset model {}", asset_model.id)),
        )?)?;
    }

    // Next, we insert the asset_models' parent-child relationships as edges.
    for asset_model in &asset_models {
        let Some(parent) = asset_model.parent_model(conn)? else {
            continue;
        };

        let child_node = builder
            .get_node_by_label(
                asset_model
                    .name
                    .clone()
                    .unwrap_or_else(|| format!("asset_model {}", asset_model.id)),
            )
            .expect("Trackable node not found");
        let parent_node = builder
            .get_node_by_label(
                parent.name.clone().unwrap_or_else(|| format!("asset_model {}", parent.id)),
            )
            .expect("Parent asset_model node not found");

        builder.edge(EREdgeBuilder::one_to_one(child_node, parent_node).label("child of")?)?;
    }

    // Finally, we insert the compatibility rules as edges.
    for rule in &compatibility_rules {
        let left_asset_model = rule.left_asset_model(conn)?;
        let right_asset_model = rule.right_asset_model(conn)?;
        let left_node = builder
            .get_node_by_label(
                left_asset_model
                    .name
                    .clone()
                    .unwrap_or_else(|| format!("asset_model {}", left_asset_model.id)),
            )
            .expect("Left asset_model node not found");
        let right_node = builder
            .get_node_by_label(
                right_asset_model
                    .name
                    .clone()
                    .unwrap_or_else(|| format!("asset_model {}", right_asset_model.id)),
            )
            .expect("Right asset_model node not found");
        builder.edge(EREdgeBuilder::one_to_one(left_node, right_node).label("compatible with")?)?;
    }

    Ok(builder.into())
}
