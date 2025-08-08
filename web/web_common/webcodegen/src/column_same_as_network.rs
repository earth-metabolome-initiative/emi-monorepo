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

#[derive(Debug, Clone)]
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
    /// Creates a new `ColumnSameAsNetwork` from a PostgreSQL connection.
    ///
    /// # Arguments
    ///
    /// * `conn`: A mutable reference to a PostgreSQL connection.
    /// * `table_catalog`: The catalog of the table to load columns from.
    /// * `table_schema`: An optional schema of the table to load columns from.
    pub fn new(
        conn: &mut PgConnection,
        table_catalog: &str,
        table_schema: Option<&str>,
    ) -> Result<Self, WebCodeGenError> {
        let mut tables = Table::load_all(conn, table_catalog, table_schema)?
            .into_iter()
            .filter(|table| !(table.is_temporary() || table.is_view()))
            .collect::<Vec<Table>>();

        tables.sort_unstable();

        Self::from_tables(conn, &tables)
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

        let mut edges = columns
            .iter()
            .enumerate()
            .filter_map(|(src_id, column)| {
                let same_as_edges: Vec<(usize, usize)> = column
                    .same_as_columns(conn)
                    .ok()?
                    .into_iter()
                    .filter_map(|same_as_column| {
                        let dst_id = columns.binary_search(&same_as_column).ok()?;
                        Some((src_id, dst_id))
                    })
                    .collect();
                Some(same_as_edges)
            })
            .flatten()
            .collect::<Vec<(usize, usize)>>();

        edges.sort_unstable();
        edges.dedup();

        let edges = DiEdgesBuilder::default()
            .expected_shape(columns.len())
            .edges(edges)
            .build()
            .expect("Failed to build edges for the column same-as network");

        let same_as_graph: GenericGraph<
            SortedVec<Column>,
            SquareCSR2D<CSR2D<usize, usize, usize>>,
        > = GenericMonoplexMonopartiteGraphBuilder::default()
            .nodes(columns.clone())
            .edges(edges.clone())
            .build()
            .expect("Failed to build the column same-as network graph");

        let transposed_edges = edges.transpose();

        let transposed_same_as_graph = GenericMonoplexMonopartiteGraphBuilder::default()
            .nodes(columns.clone())
            .edges(transposed_edges)
            .build()
            .expect("Failed to build the transposed column same-as network graph");

        let mut missing_edges: Vec<(usize, usize)> = Vec::new();

        for src_id in same_as_graph.node_ids() {
            for dst_id in same_as_graph.successors_set(src_id) {
                if same_as_graph.has_successor(src_id, dst_id)
                    || same_as_graph.has_successor(dst_id, src_id)
                {
                    continue;
                }

                let src_column = &same_as_graph.nodes_vocabulary()[src_id];
                let dst_column = &same_as_graph.nodes_vocabulary()[dst_id];

                let src_table = src_column.table(conn)?;
                let dst_table = dst_column.table(conn)?;

                if src_table == dst_table {
                    // If both columns are in the same table, we do not need to infer an edge.
                    continue;
                }

                let src_ancestral_extension_tables = src_table.ancestral_extension_tables(conn)?;
                let dst_ancestral_extension_tables = dst_table.ancestral_extension_tables(conn)?;

                // If the `src_table` is an extension of the `dst_table`, or vice versa,
                // we do not need to infer an edge.
                if src_ancestral_extension_tables.contains(&dst_table)
                    || dst_ancestral_extension_tables.contains(&src_table)
                {
                    continue;
                }

                // We check whether the two tables must be inserted together.
                if !src_table.must_be_inserted_alongside_with(&dst_table, conn)? {
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
    ///
    /// # Errors
    ///
    /// * Returns an error if there is an issue with the database connection.
    pub fn same_as_columns(
        &self,
        source_table: &Table,
        target_table: &Table,
        passing_through: &Column,
        conn: &mut PgConnection,
    ) -> Result<Vec<(&Column, &Column)>, WebCodeGenError> {
        Ok(self
            .same_as_graph
            .sparse_coordinates()
            .filter_map(move |(src_id, dst_id)| {
                let src_column = self.same_as_graph.nodes_vocabulary().get(src_id)?;
                let dst_column = self.same_as_graph.nodes_vocabulary().get(dst_id)?;

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

                    return Some((src_column, dst_column));
                }

                None
            })
            .collect())
    }

    /// Returns the inferred same-as columns associated to the given column and
    /// target table.
    ///
    /// # Arguments
    ///
    /// * `column`: The column for which to find inferred same-as columns.
    /// * `target_table`: The target table to which the inferred same-as columns
    ///   should belong.
    ///
    /// # Errors
    ///
    /// * Returns an error if there is an issue with the database connection.
    pub fn inferred_same_as_columns(
        &self,
        source_table: &Table,
        target_table: &Table,
        passing_through: &Column,
    ) -> Vec<(&Column, &Column)> {
        let Ok(passing_through_id) =
            self.inferred_same_as_graph.nodes_vocabulary().binary_search(passing_through)
        else {
            return Vec::new();
        };
        self.inferred_same_as_graph
            .sparse_coordinates()
            .filter_map(|(src_id, dst_id)| {
                let src_column = self.inferred_same_as_graph.nodes_vocabulary().get(src_id)?;
                let dst_column = self.inferred_same_as_graph.nodes_vocabulary().get(dst_id)?;
                if src_column.table_name == source_table.table_name
                    && src_column.table_schema == source_table.table_schema
                    && dst_column.table_name == target_table.table_name
                    && dst_column.table_schema == target_table.table_schema
                    && self.same_as_graph.is_reachable_through(src_id, dst_id, passing_through_id)
                {
                    Some((src_column, dst_column))
                } else {
                    None
                }
            })
            .collect()
    }
}
