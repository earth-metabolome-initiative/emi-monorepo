//! Submodule providing a function to create a mermaid-based illustration of the
//! relationships between the provided columns.

use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use diesel::PgConnection;
use mermaid::{
    prelude::{
        ArrowShape, Color, Flowchart, FlowchartBuilder, FlowchartEdgeBuilder, FlowchartNode,
        FlowchartNodeBuilder, FlowchartNodeShape, LineStyle, StyleClassBuilder, StyleProperty,
    },
    traits::{DiagramBuilder, EdgeBuilder, NodeBuilder},
};

use crate::{Column, Table};

/// Generates a mermaid-based illustration of the relationships between the
/// provided columns.
///
/// # Arguments
///
/// * `columns` - The columns to illustrate.
/// * `column_of_interest` - The column of interest to highlight.
/// * `conn` - A mutable reference to a `PgConnection`.
///
/// # Errors
///
/// * If the database connection fails.
pub(super) fn columns_to_mermaid_illustration(
    columns: &[Column],
    column_of_interest: &Column,
    conn: &mut PgConnection,
) -> Result<Flowchart, crate::errors::WebCodeGenError> {
    assert!(
        columns.contains(column_of_interest),
        "Column of interest {column_of_interest} must be in the provided columns"
    );

    assert_eq!(
        columns.iter().collect::<HashSet<_>>().len(),
        columns.len(),
        "Provided columns must be unique"
    );

    let mut inferred_columns = columns
        .iter()
        .flat_map(|col| {
            let mut same_as = col.all_ancestral_same_as_columns(conn).unwrap_or_default();
            same_as.extend(col.associated_same_as_columns(conn).unwrap_or_default());
            same_as.extend(col.triangular_same_as_columns(conn).unwrap_or_default());
            same_as
        })
        .filter(|col| !columns.contains(col))
        .collect::<Vec<_>>();
    inferred_columns.sort_unstable();
    inferred_columns.dedup();

    let mut flowchart = FlowchartBuilder::default();

    let column_of_interest_class = flowchart
        .style_class(
            StyleClassBuilder::default()
                .name("column-of-interest")
                .expect("Valid class name")
                .property(StyleProperty::Stroke(Color::pastel_red()))
                .expect("Valid property")
                .property(StyleProperty::Fill(Color::pastel_red().lighten(10)))
                .expect("Valid property"),
        )
        .expect("Valid style class");
    let directly_involved_column_class = flowchart
        .style_class(
            StyleClassBuilder::default()
                .name("directly-involved-column")
                .expect("Valid class name")
                .property(StyleProperty::Stroke(Color::pastel_blue()))
                .expect("Valid property")
                .property(StyleProperty::Fill(Color::pastel_blue().lighten(10)))
                .expect("Valid property"),
        )
        .expect("Valid style class");
    let undirectly_involved_column_class = flowchart
        .style_class(
            StyleClassBuilder::default()
                .name("undirectly-involved-column")
                .expect("Valid class name")
                .property(StyleProperty::Stroke(Color::pastel_cyan()))
                .expect("Valid property")
                .property(StyleProperty::StrokeDasharray(5, 5))
                .expect("Valid property")
                .property(StyleProperty::Fill(Color::pastel_cyan().lighten(10)))
                .expect("Valid property"),
        )
        .expect("Valid style class");

    let mut column_nodes: HashMap<&Column, Rc<FlowchartNode>> = HashMap::new();
    // First, we populate the column nodes.
    for column in columns {
        let mut node_builder = FlowchartNodeBuilder::default()
            .label(&column.column_name)
            .expect("Valid label")
            .shape(FlowchartNodeShape::RoundEdges);
        node_builder = node_builder
            .style_class(Rc::clone(if column == column_of_interest {
                &column_of_interest_class
            } else {
                &directly_involved_column_class
            }))
            .unwrap();
        column_nodes.insert(column, flowchart.node(node_builder).unwrap());
    }

    // First, we populate the undirectly involved column nodes.
    for column in &inferred_columns {
        column_nodes.insert(
            column,
            flowchart
                .node(
                    FlowchartNodeBuilder::default()
                        .label(&column.column_name)
                        .expect("Valid label")
                        .shape(FlowchartNodeShape::RoundEdges)
                        .style_class(Rc::clone(&undirectly_involved_column_class))
                        .unwrap(),
                )
                .unwrap(),
        );
    }

    // Next, we define the relationships between the columns.
    for (column, column_node) in &column_nodes {
        let ancestral_same_as_columns = column.ancestral_same_as_columns(conn)?;
        for ancestral_same_as_column in &ancestral_same_as_columns {
            if let Some(ancestral_column_node) = column_nodes.get(&ancestral_same_as_column) {
                flowchart
                    .edge(
                        FlowchartEdgeBuilder::default()
                            .source(column_node.clone())
                            .unwrap()
                            .destination(ancestral_column_node.clone())
                            .unwrap()
                            .label("ancestral same as")
                            .unwrap()
                            .right_arrow_shape(ArrowShape::Normal)
                            .unwrap(),
                    )
                    .unwrap();
            }
        }

        for inferred_ancestral_same_as_column in &column.all_ancestral_same_as_columns(conn)? {
            if ancestral_same_as_columns.contains(inferred_ancestral_same_as_column) {
                // We have already added this edge.
                continue;
            }

            if let Some(inferred_ancestral_same_as_column_node) =
                column_nodes.get(&inferred_ancestral_same_as_column)
            {
                flowchart
                    .edge(
                        FlowchartEdgeBuilder::default()
                            .source(column_node.clone())
                            .unwrap()
                            .destination(inferred_ancestral_same_as_column_node.clone())
                            .unwrap()
                            .label("inferred ancestral same as")
                            .unwrap()
                            .line_style(LineStyle::Dashed)
                            .right_arrow_shape(ArrowShape::Normal)
                            .unwrap(),
                    )
                    .unwrap();
            }
        }

        let associated_same_as_columns = column.associated_same_as_columns(conn)?;
        for associated_same_as_column in &associated_same_as_columns {
            if let Some(associated_column_node) = column_nodes.get(&associated_same_as_column) {
                flowchart
                    .edge(
                        FlowchartEdgeBuilder::default()
                            .source(column_node.clone())
                            .unwrap()
                            .destination(associated_column_node.clone())
                            .unwrap()
                            .label("associated same as")
                            .unwrap()
                            .right_arrow_shape(ArrowShape::Normal)
                            .unwrap(),
                    )
                    .unwrap();
            }
        }
    }

    let mut tables = columns
        .iter()
        .chain(inferred_columns.iter())
        .map(|col| col.table(conn))
        .collect::<Result<Vec<_>, _>>()?;
    tables.sort_unstable();
    tables.dedup();
    let mut table_nodes: HashMap<&Table, Rc<FlowchartNode>> = HashMap::new();

    if tables.len() > 1 {
        // First, we create the table nodes, which include the column nodes as subnodes.
        for table in &tables {
            let mut table_node_builder =
                FlowchartNodeBuilder::default().label(&table.table_name).expect("Valid label");

            for (column, column_node) in &column_nodes {
                if table.has_column(column) {
                    table_node_builder =
                        table_node_builder.subnode(column_node.clone()).expect("Valid subnode");
                }
            }

            table_nodes.insert(table, flowchart.node(table_node_builder).unwrap());
        }

        // Next, we include the connections relative to the tables which extend other
        // tables.
        for (table, table_node) in &table_nodes {
            let extension_tables = table.extension_tables(conn)?;

            for extended_table in &extension_tables {
                if let Some(extended_table_node) = table_nodes.get(&extended_table) {
                    flowchart
                        .edge(
                            FlowchartEdgeBuilder::default()
                                .source(table_node.clone())
                                .unwrap()
                                .destination(extended_table_node.clone())
                                .unwrap()
                                .label("extends")
                                .unwrap()
                                .right_arrow_shape(ArrowShape::Normal)
                                .unwrap(),
                        )
                        .unwrap();
                }
            }
            for ancestor_table in table.ancestral_extension_tables(conn)? {
                if extension_tables.contains(&ancestor_table) {
                    // We have already added this edge.
                    continue;
                }

                if let Some(ancestor_table_node) = table_nodes.get(&ancestor_table) {
                    flowchart
                        .edge(
                            FlowchartEdgeBuilder::default()
                                .source(table_node.clone())
                                .unwrap()
                                .destination(ancestor_table_node.clone())
                                .unwrap()
                                .label("descendant of")
                                .unwrap()
                                .line_style(LineStyle::Dashed)
                                .right_arrow_shape(ArrowShape::Normal)
                                .unwrap(),
                        )
                        .unwrap();
                }
            }

            for associated_table in table.associated_tables(conn)? {
                if let Some(associated_table_node) = table_nodes.get(&associated_table) {
                    flowchart
                        .edge(
                            FlowchartEdgeBuilder::default()
                                .source(table_node.clone())
                                .unwrap()
                                .destination(associated_table_node.clone())
                                .unwrap()
                                .label("associated with")
                                .unwrap()
                                .right_arrow_shape(ArrowShape::Circle)
                                .unwrap(),
                        )
                        .unwrap();
                }
            }
        }
    }

    let mut schemas = tables.iter().map(|table| table.table_schema.as_str()).collect::<Vec<_>>();
    schemas.sort_unstable();
    schemas.dedup();

    if schemas.len() > 1 {
        for schema in schemas {
            let mut schema_node_builder =
                FlowchartNodeBuilder::default().label(schema).expect("Valid label");

            for (table, table_node) in &table_nodes {
                if table.table_schema == schema {
                    schema_node_builder =
                        schema_node_builder.subnode(table_node.clone()).expect("Valid subnode");
                }
            }

            flowchart.node(schema_node_builder).unwrap();
        }
    }

    Ok(flowchart.into())
}
