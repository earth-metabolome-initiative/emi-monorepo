//! Submodule defining the network of same-as columns.

use ::graph::{
    prelude::{Builder, DiEdgesBuilder, DiGraph, GenericMonoplexMonopartiteGraphBuilder},
    traits::{EdgesBuilder, MonopartiteGraphBuilder, MonoplexGraphBuilder},
};
use algebra::{
    impls::{CSR2D, SquareCSR2D},
    prelude::TransposableMatrix2D,
};
use diesel::PgConnection;
use graph::{
    prelude::GenericGraph,
    traits::{MonopartiteGraph, MonoplexMonopartiteGraph},
};
use sorted_vec::prelude::SortedVec;

use crate::Table;

#[derive(Debug, Clone)]
/// A network of columns that are "extend" each other.
pub struct TableExtensionNetwork {
    /// The directed graph representing the "extend" relationships between
    /// columns.
    pub(crate) extension_graph: DiGraph<Table>,
    /// The transposed same-as graph, which allows for efficient access to
    /// predecessors.
    transposed_extension_graph: DiGraph<Table>,
}

impl TableExtensionNetwork {
    /// Creates a new `TableExtensionNetwork` from a `PostgreSQL` connection.
    ///
    /// # Arguments
    ///
    /// * `conn`: A mutable reference to a `PostgreSQL` connection.
    /// * `table_catalog`: The catalog of the table to load columns from.
    /// * `table_schema`: An optional schema of the table to load columns from.
    pub(crate) fn from_tables(conn: &mut PgConnection, tables: Vec<Table>) -> Self {
        let tables = SortedVec::try_from(tables).expect("Tables should be sorted and unique");

        let mut edges = tables
            .iter()
            .enumerate()
            .filter_map(|(src_id, table)| {
                let same_as_edges: Vec<(usize, usize)> = table
                    .extension_tables(conn)
                    .ok()?
                    .iter()
                    .filter_map(|same_as_column| {
                        let dst_id = tables.binary_search(same_as_column).ok()?;
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
            .expected_shape(tables.len())
            .edges(edges)
            .build()
            .expect("Failed to build edges for the column same-as network");

        let extension_graph: GenericGraph<
            SortedVec<Table>,
            SquareCSR2D<CSR2D<usize, usize, usize>>,
        > = GenericMonoplexMonopartiteGraphBuilder::default()
            .nodes(tables.clone())
            .edges(edges.clone())
            .build()
            .expect("Failed to build the column same-as network graph");

        let transposed_edges = edges.transpose();

        let transposed_extension_graph = GenericMonoplexMonopartiteGraphBuilder::default()
            .nodes(tables)
            .edges(transposed_edges)
            .build()
            .expect("Failed to build the transposed column same-as network graph");

        Self { extension_graph, transposed_extension_graph }
    }

    #[must_use]
    /// Returns all the descendant tables of the provided table in the
    /// extension network.
    ///
    /// # Arguments
    ///
    /// * `table`: A reference to the table to find the descendants for.
    ///
    /// # Panics
    ///
    /// * If the provided table is not part of the extension network.
    pub fn descendants(&self, table: &Table) -> Vec<&Table> {
        let table_id = self
            .extension_graph
            .nodes_vocabulary()
            .binary_search(table)
            .expect("Failed to get node ID for the table");

        let mut descendants = Vec::new();

        for successor_id in self.transposed_extension_graph.successors_set(table_id) {
            descendants.push(
                self.extension_graph
                    .nodes_vocabulary()
                    .get(successor_id)
                    .expect("Failed to get descendant table"),
            );
        }

        descendants
    }
}
