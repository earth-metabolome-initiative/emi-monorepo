//! Submodule defining the network of same-as columns.

use std::collections::HashMap;

use ::graph::{
    prelude::{
        Builder, DiEdgesBuilder, DiGraph, GenericMonoplexMonopartiteGraphBuilder, UndiGraph,
    },
    traits::{EdgesBuilder, MonopartiteGraphBuilder, MonoplexGraphBuilder},
};
use algebra::{
    impls::{CSR2D, SquareCSR2D},
    prelude::TransposableMatrix2D,
};
use diesel::PgConnection;
use graph::{
    prelude::{GenericGraph, UndiEdgesBuilder},
    traits::{Edges, MonopartiteGraph, MonoplexGraph, MonoplexMonopartiteGraph},
};
use sorted_vec::prelude::SortedVec;

use crate::{Column, Table, errors::WebCodeGenError};

#[derive(Debug, Clone, Hash)]
/// A network of columns that are "same as" each other.
pub struct ColumnSameAsNetwork {
    /// The directed graph representing the "same as" relationships between
    /// columns.
    same_as_graph: DiGraph<Column>,
    /// The transposed same-as graph, which allows for efficient access to
    /// predecessors.
    transposed_same_as_graph: DiGraph<Column>,
    /// The inferred directed graph representing the "same as" relationships
    /// between columns.
    inferred_same_as_graph: UndiGraph<Column>,
}

impl ColumnSameAsNetwork {
    /// Returns an iterator over all columns in the network.
    pub fn columns(&self) -> impl Iterator<Item = Column> {
        self.same_as_graph.nodes()
    }

    /// Creates a new `ColumnSameAsNetwork` from a PostgreSQL connection.
    ///
    /// # Arguments
    ///
    /// * `conn`: A mutable reference to a PostgreSQL connection.
    /// * `table_catalog`: The catalog of the table to load columns from.
    /// * `table_schema`: An optional schema of the table to load columns from.
    pub fn from_tables(conn: &mut PgConnection, tables: &[Table]) -> Result<Self, WebCodeGenError> {
        let mut columns: Vec<Column> = Vec::new();

        for table in tables {
            columns.extend(table.columns(conn)?);
        }

        columns.sort_unstable();

        let columns = SortedVec::try_from(columns).expect("Columns should be sorted and unique");

        let mut node_tuples = Vec::new();
        let mut undirected_node_tuples = Vec::new();

        for (src_id, column) in columns.iter().enumerate() {
            for same_as_column in column.same_as_columns(conn)? {
                if let Ok(dst_id) = columns.binary_search(&same_as_column) {
                    node_tuples.push((src_id, dst_id));
                    undirected_node_tuples.push(if src_id < dst_id {
                        (src_id, dst_id)
                    } else {
                        (dst_id, src_id)
                    });
                }
            }
        }

        node_tuples.sort_unstable();
        undirected_node_tuples.sort_unstable();

        let edges = DiEdgesBuilder::default()
            .expected_shape(columns.len())
            .edges(node_tuples.clone())
            .build()
            .expect("Failed to build edges for the column same-as network");

        let undirected_edges = UndiEdgesBuilder::default()
            .expected_shape(columns.len())
            .edges(undirected_node_tuples)
            .build()
            .expect(&format!(
                "Failed to build undirected edges for the column same-as network with {} columns",
                columns.len()
            ));

        let same_as_graph: GenericGraph<
            SortedVec<Column>,
            SquareCSR2D<CSR2D<usize, usize, usize>>,
        > = GenericMonoplexMonopartiteGraphBuilder::default()
            .nodes(columns.clone())
            .edges(edges.clone())
            .build()
            .expect("Failed to build the column same-as network graph");

        let undirected_edges_same_as_graph: UndiGraph<Column> =
            GenericMonoplexMonopartiteGraphBuilder::default()
                .nodes(columns.clone())
                .edges(undirected_edges)
                .build()
                .expect("Failed to build the column same-as network graph");

        let transposed_edges = edges.transpose();

        let transposed_same_as_graph = GenericMonoplexMonopartiteGraphBuilder::default()
            .nodes(columns.clone())
            .edges(transposed_edges)
            .build()
            .expect("Failed to build the transposed column same-as network graph");

        let mut missing_edges: Vec<(usize, usize)> = Vec::new();

        for src_id in undirected_edges_same_as_graph.node_ids() {
            let src_column = &undirected_edges_same_as_graph.nodes_vocabulary()[src_id];
            let src_table = src_column.table(conn)?;
            for dst_id in undirected_edges_same_as_graph.successors_set(src_id) {
                if undirected_edges_same_as_graph.has_successor(src_id, dst_id)
                    || undirected_edges_same_as_graph.has_successor(dst_id, src_id)
                {
                    continue;
                }

                let dst_column = &undirected_edges_same_as_graph.nodes_vocabulary()[dst_id];
                let dst_table = dst_column.table(conn)?;

                // We check whether the two tables must be inserted together.
                if !src_table.must_be_inserted_alongside_with(&dst_table, conn)? {
                    continue;
                }

                if src_table == dst_table {
                    continue;
                }

                if src_id < dst_id {
                    missing_edges.push((src_id, dst_id));
                } else {
                    // Ensure the order is consistent for undirected edges.
                    missing_edges.push((dst_id, src_id));
                }
            }
        }

        missing_edges.sort_unstable();
        missing_edges.dedup();

        let inferred_edges = UndiEdgesBuilder::default()
            .expected_shape(columns.len())
            .edges(missing_edges.into_iter())
            .build()
            .expect("Failed to build inferred edges for the column same-as network");

        let inferred_same_as_graph = GenericMonoplexMonopartiteGraphBuilder::default()
            .nodes(columns.clone())
            .edges(inferred_edges)
            .build()
            .expect("Failed to build the inferred column same-as network graph");

        Ok(Self { same_as_graph, transposed_same_as_graph, inferred_same_as_graph })
    }

    /// Returns a dot plot representation of the column same-as network.
    pub fn to_dot(&self, conn: &mut PgConnection) -> Result<String, WebCodeGenError> {
        let mut dot = String::new();
        dot.push_str("digraph ColumnSameAsNetwork {\n");

        dot.push_str(
            "graph [
            fontname=\"Helvetica\",
            fontsize=12,
            bgcolor=white,
            splines=true,
            nodesep=0.6,
            concentrate=true,
            compound=true,
        ];\n",
        );
        dot.push_str("node [shape=box, style=rounded, fontname=\"Helvetica\"];\n");
        dot.push_str("edge [fontname=\"Helvetica\"];\n");

        let mut tables: HashMap<Table, Vec<(usize, Column)>> = HashMap::new();

        // First, we add the nodes.
        for (column_id, column) in self.same_as_graph.nodes().enumerate() {
            // We skip the columns which do not have any same-as relationships.
            if !self.same_as_graph.has_successors(column_id)
                && !self.transposed_same_as_graph.has_successors(column_id)
            {
                continue;
            }

            let table = column.table(conn)?;

            match tables.get_mut(&table) {
                Some(columns) => columns.push((column_id, column.clone())),
                None => {
                    tables.insert(table, vec![(column_id, column.clone())]);
                }
            }
        }

        for (table, columns) in tables {
            // We define a new subgraph for each table.
            dot.push_str(&format!("subgraph cluster_{} {{\n", table.table_name.replace(' ', "_")));

            for (column_id, column) in columns {
                dot.push_str(&format!(
                    "  C{column_id} [color=\"blue\", fillcolor=\"#ffffff\", label=\"{}\\n{}\"];\n",
                    table.table_name, column.column_name
                ));
            }
            dot.push_str(&format!("  label=\"{}\";\n", table.table_name.replace(' ', "_")));
            dot.push_str("  style=\"filled, rounded\";\n");
            dot.push_str("  fillcolor=\"#f0f0f0\";\n");
            dot.push_str("}\n");
        }

        // Now, we add the edges for the same-as relationships.
        for (src_id, dst_id) in self.same_as_graph.edges().sparse_coordinates() {
            dot.push_str(&format!(
                "  C{} -> C{} [color=\"green\", label=\"same as\"];\n",
                src_id, dst_id
            ));
        }

        // Add the inferred edges.
        for (src_id, dst_id) in self.inferred_same_as_graph.edges().sparse_coordinates() {
            if src_id >= dst_id {
                continue; // Avoid duplicate edges in the undirected graph
            }
            dot.push_str(&format!(
                "  C{} -> C{} [dir=\"both\", color=\"red\", label=\"inferred\", style=dashed];\n",
                src_id, dst_id
            ));
        }

        dot.push_str("}\n");
        Ok(dot)
    }

    /// Returns the same-as columns associated to the given column and
    /// target table.
    ///
    /// # Arguments
    ///
    /// * `source_table`: The column for which to find same-as columns.
    /// * `target_table`: The target table to which the same-as columns should
    ///   belong.
    /// * `starting_from`: An optional column from which to start the search, if
    ///   any.
    /// * `passing_through`: A column that must be passed through in the same-as
    ///   relationship.
    ///
    /// # Errors
    ///
    /// * Returns an error if there is an issue with the database connection.
    pub(crate) fn same_as_columns(
        &self,
        source_table: &Table,
        target_table: &Table,
        starting_from: Option<&Column>,
        passing_through: &Column,
        conn: &mut PgConnection,
    ) -> Result<Vec<(&Column, &Column)>, WebCodeGenError> {
        let mut valid_traffic_nodes: Vec<usize> = Vec::new();
        let mut same_as_columns = self
            .same_as_graph
            .sparse_coordinates()
            .filter_map(|(src_id, dst_id)| {
                let src_column = self.same_as_graph.nodes_vocabulary().get(src_id)?;
                let dst_column = self.same_as_graph.nodes_vocabulary().get(dst_id)?;

                if let Some(starting_from) = starting_from {
                    if src_column != starting_from {
                        return None;
                    }
                }

                if src_column.table_name != source_table.table_name
                    || src_column.table_schema != source_table.table_schema
                    || dst_column.table_name != target_table.table_name
                    || dst_column.table_schema != target_table.table_schema
                {
                    return None;
                }

                // We check if this source column appears in foreign keys with the
                // provided passing through column, and has in the foreign columns
                // the target table.
                for foreign_key in src_column.foreign_keys(conn).ok()? {
                    let local_columns = foreign_key.columns(conn).ok()?;
                    if !local_columns.contains(passing_through) {
                        continue;
                    }
                    let foreign_columns = foreign_key.foreign_columns(conn).ok()?;
                    if !foreign_columns.contains(dst_column) {
                        continue;
                    }

                    valid_traffic_nodes.push(dst_id);
                    return Some((src_column, dst_column));
                }

                None
            })
            .collect::<Vec<(&Column, &Column)>>();

        let inferred_same_as_edges = self
            .inferred_same_as_graph
            .sparse_coordinates()
            .filter_map(|(src_id, dst_id)| {
                let src_column = self.inferred_same_as_graph.nodes_vocabulary().get(src_id)?;
                let dst_column = self.inferred_same_as_graph.nodes_vocabulary().get(dst_id)?;

                if let Some(starting_from) = starting_from {
                    if src_column != starting_from {
                        return None;
                    }
                }

                if src_column.table_name == source_table.table_name
                    && src_column.table_schema == source_table.table_schema
                    && dst_column.table_name == target_table.table_name
                    && dst_column.table_schema == target_table.table_schema
                    && valid_traffic_nodes.iter().any(|traffic_node_id| {
                        self.same_as_graph.is_reachable_through(src_id, dst_id, *traffic_node_id)
                    })
                {
                    Some((src_column, dst_column))
                } else {
                    None
                }
            })
            .collect::<Vec<(&Column, &Column)>>();

        same_as_columns.extend(inferred_same_as_edges);

        // We check that the provided column and the same-as columns have shared
        // ancestors. If they have not, the same-as graph is in an illegal state.
        for (left, right) in same_as_columns.iter() {
            if !left.has_compatible_data_type(&right, conn)? {
                panic!(
                    "The column {left} is in a same-as relationship with column {right}, but the two columns do not share any ancestor. This indicates an illegal state in the same-as network.",
                );
            }
        }

        Ok(same_as_columns)
    }

    /// Returns the same-as columns associated with the provided table and
    /// its ancestors, passing through the primary key of the source table.
    ///
    /// # Arguments
    ///
    /// * `source_table`: The table for which to find same-as columns in its
    ///   ancestors.
    /// * `starting_from`: An optional column from which to start the search, if
    ///  any.
    /// * `filter_ancestors`: If true, filters out columns that are already
    ///   handled by more distant ancestors.
    /// * `conn`: A mutable reference to a PostgreSQL connection.
    ///
    /// # Errors
    ///
    /// * Returns an error if there is an issue with the database connection.
    ///
    /// # Panics
    ///
    /// * If the current table has a composite primary key, as this is not
    ///   supported in extension hierarchies.
    pub(crate) fn ancestral_same_as_columns(
        &self,
        table: &Table,
        starting_from: Option<&Column>,
        filter_ancestors: bool,
        conn: &mut PgConnection,
    ) -> Result<Vec<(&Column, &Column)>, WebCodeGenError> {
        let primary_key_columns = table.primary_key_columns(conn)?;
        if primary_key_columns.len() != 1 {
            panic!(
                "Table {table} with composite primary keys are not supported in extension hierarchies"
            );
        }
        let primary_key_column = &primary_key_columns[0];

        let mut same_as_columns = Vec::new();

        // We identify the same-as columns in the ancestors, of which we
        // determine in particular the destination nodes. Such nodes
        // are then filtered from the set we return, as they are already
        // handled in the ancestors and should not be duplicated.
        let mut destination_columns_handled_by_ancestors: Vec<&Column> = Vec::new();

        if filter_ancestors {
            for ancestor in table.ancestral_extension_tables(conn)? {
                let primary_key_columns = ancestor.primary_key_columns(conn)?;
                if primary_key_columns.len() != 1 {
                    panic!(
                        "Table {ancestor} with composite primary keys are not supported in extension hierarchies"
                    );
                }
                let primary_key_column = &primary_key_columns[0];
                for ancestor2 in ancestor.ancestral_extension_tables(conn)? {
                    let ancestor_same_as_columns = self.same_as_columns(
                        &ancestor,
                        &ancestor2,
                        None,
                        primary_key_column,
                        conn,
                    )?;
                    for (_, dst_column) in ancestor_same_as_columns {
                        destination_columns_handled_by_ancestors.push(dst_column);
                    }
                }
            }
        }

        for ancestor in table.ancestral_extension_tables(conn)? {
            same_as_columns.extend(
                self.same_as_columns(table, &ancestor, starting_from, primary_key_column, conn)?
                    .into_iter()
                    .filter(|(_, dst_column)| {
                        !destination_columns_handled_by_ancestors.contains(dst_column)
                    }),
            );
        }

        Ok(same_as_columns)
    }
}
