//! Submodule providing an illustration of the trackables hierarchy using an
//! Entity-Relationship Diagram (ERD).

use core_structures::{CompatibilityRule, Trackable};
use diesel::PgConnection;
use mermaid::prelude::*;
use web_common_traits::database::BoundedRead;

use crate::Error;

/// Generates an Entity-Relationship Diagram (ERD) for the trackables hierarchy.
pub fn trackables_hierarchy(conn: &mut PgConnection) -> Result<ERDiagram, Error> {
    let trackables: Vec<Trackable> = Trackable::bounded_read(0, u16::MAX, conn)?;
    let compatibility_rules: Vec<CompatibilityRule> =
        CompatibilityRule::bounded_read(0, u16::MAX, conn)?;

    let mut builder = ERDiagramBuilder::default()
        .configuration(ERDiagramConfigurationBuilder::default().title("Trackables Hierarchy")?)?;

    // First, we inser all of the trackables as nodes.
    for trackable in &trackables {
        builder.node(ERNodeBuilder::default().label(
            trackable.name.clone().unwrap_or_else(|| format!("trackable {}", trackable.id)),
        )?)?;
    }

    // Next, we insert the trackables' parent-child relationships as edges.
    for trackable in &trackables {
        let Some(parent) = trackable.parent(conn)? else {
            continue;
        };

        let child_node = builder
            .get_node_by_label(
                trackable.name.clone().unwrap_or_else(|| format!("trackable {}", trackable.id)),
            )
            .expect("Trackable node not found");
        let parent_node = builder
            .get_node_by_label(
                parent.name.clone().unwrap_or_else(|| format!("trackable {}", parent.id)),
            )
            .expect("Parent trackable node not found");

        builder.edge(EREdgeBuilder::one_to_one(child_node, parent_node).label("child of")?)?;
    }

    // Finally, we insert the compatibility rules as edges.
    for rule in &compatibility_rules {
        let left_trackable = rule.left_trackable(conn)?;
        let right_trackable = rule.right_trackable(conn)?;
        let left_node = builder
            .get_node_by_label(
                left_trackable
                    .name
                    .clone()
                    .unwrap_or_else(|| format!("trackable {}", left_trackable.id)),
            )
            .expect("Left trackable node not found");
        let right_node = builder
            .get_node_by_label(
                right_trackable
                    .name
                    .clone()
                    .unwrap_or_else(|| format!("trackable {}", right_trackable.id)),
            )
            .expect("Right trackable node not found");
        builder.edge(EREdgeBuilder::one_to_one(left_node, right_node).label("compatible with")?)?;
    }

    Ok(builder.into())
}
