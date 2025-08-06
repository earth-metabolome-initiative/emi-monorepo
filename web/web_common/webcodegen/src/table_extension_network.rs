//! Submodule defining the network of same-as columns.

use std::collections::HashMap;

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
    traits::{Edges, MonopartiteGraph, MonoplexGraph, MonoplexMonopartiteGraph},
};
use proc_macro2::TokenStream;
use sorted_vec::prelude::SortedVec;

use crate::{Column, KeyColumnUsage, Table, errors::WebCodeGenError};

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
    /// Creates a new `TableExtensionNetwork` from a PostgreSQL connection.
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

        Self::from_tables(conn, tables)
    }

    /// Creates a new `TableExtensionNetwork` from a PostgreSQL connection.
    ///
    /// # Arguments
    ///
    /// * `conn`: A mutable reference to a PostgreSQL connection.
    /// * `table_catalog`: The catalog of the table to load columns from.
    /// * `table_schema`: An optional schema of the table to load columns from.
    pub fn from_tables(
        conn: &mut PgConnection,
        tables: Vec<Table>,
    ) -> Result<Self, WebCodeGenError> {
        let tables = SortedVec::try_from(tables).expect("Tables should be sorted and unique");

        let mut edges = tables
            .iter()
            .enumerate()
            .filter_map(|(src_id, table)| {
                let same_as_edges: Vec<(usize, usize)> = table
                    .extension_tables(conn)
                    .ok()?
                    .into_iter()
                    .filter_map(|same_as_column| {
                        let dst_id = tables.binary_search(&same_as_column).ok()?;
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

        Ok(Self { extension_graph, transposed_extension_graph })
    }

    /// Returns a dot plot representation of the column same-as network.
    pub fn to_dot(&self) -> String {
        let mut dot = String::new();
        dot.push_str("digraph TableExtensionNetwork {\n");

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

        // First, we add the nodes.
        for (table_id, table) in self.extension_graph.nodes().enumerate() {
            // We skip the columns which do not have any same-as relationships.
            if !self.extension_graph.has_successors(table_id)
                && !self.transposed_extension_graph.has_successors(table_id)
            {
                continue;
            }

            let mut fillcolors = Vec::new();
            let mut color = "#666666";
            if self.is_diamond_bottom(&table).is_some() {
                fillcolors.push("#FFE6CC");
                color = "#D79B00";
            }
            if self.is_diamond_top(&table) {
                fillcolors.push("#D5E8D4");
                color = "#82B366";
            }
            if self.is_diamond_side(&table) {
                fillcolors.push("#E1D5E7");
                color = "#9673A6";
            }
            assert!(
                fillcolors.len() < 3,
                "A table cannot be both a diamond top and a diamond bottom at the same time"
            );

            if fillcolors.is_empty() {
                fillcolors.push("#F5F5F5");
            }

            let fillcolors = fillcolors.join(":");

            dot.push_str(&format!(
                "  T{table_id} [color=\"{color}\", label=\"{}\", fillcolor=\"{fillcolors}\", style=\"filled,rounded\", gradientangle=0];\n",
                table.table_name
            ));
        }

        // Now, we add the edges for the same-as relationships.
        for (src_id, dst_id) in self.extension_graph.edges().sparse_coordinates() {
            dot.push_str(&format!(
                "  T{src_id} -> T{dst_id} [color=\"gray\", label=\"extends\"];\n",
            ));
        }

        dot.push_str("}\n");

        dot
    }

    /// Returns whether the provided table is a diamond bottom.
    ///
    /// # Arguments
    ///
    /// * `table`: A reference to the table to check.
    ///
    /// # Implementation details
    ///
    /// A `diamond bottom` is a table that sits at the bottom of an extension
    /// DAG, and has two or more parents which then join into a single table
    /// at the top, which we call the `diamond top`.
    pub fn is_diamond_bottom(&self, table: &Table) -> Option<&Table> {
        let table_id = self
            .extension_graph
            .nodes_vocabulary()
            .binary_search(table)
            .expect("Failed to get node ID for the table");

        // If the table does not have at least two predecessors, it cannot be a diamond
        // bottom
        if self.extension_graph.successors(table_id).count() < 2 {
            return None;
        }

        let paths = self.extension_graph.unique_paths_from(table_id);
        if paths.len() < 2 {
            return None;
        }

        // First, we check that the last node in each path is the same,
        // otherwise something is wrong with the graph.
        let mut last_node = paths[0].last().copied()?;
        let mut number_of_nodes_from_end = 1;
        assert!(
            paths.iter().all(|path| path.last() == Some(&last_node)),
            "Paths do not end with the same node: {:?}",
            paths
        );

        loop {
            let current_last_node = paths[0][paths[0].len() - number_of_nodes_from_end];
            if !paths
                .iter()
                .all(|path| path[path.len() - number_of_nodes_from_end] == current_last_node)
            {
                break;
            }

            last_node = current_last_node;
            number_of_nodes_from_end += 1;
        }

        self.extension_graph.nodes_vocabulary().get(last_node)
    }

    /// Returns whether the provided table is a diamond top.
    ///
    /// # Arguments
    ///
    /// * `table`: A reference to the table to check.
    ///
    /// # Implementation details
    ///
    /// See the documentation for `is_diamond_bottom`.
    pub fn is_diamond_top(&self, table: &Table) -> bool {
        // A node is a diamond top if it has a descendant that is a diamond bottom
        // with this table as its diamond top.
        let table_id = self
            .extension_graph
            .nodes_vocabulary()
            .binary_search(table)
            .expect("Failed to get node ID for the table");

        let descendants = self.transposed_extension_graph.successors_set(table_id);

        descendants.iter().any(|&descendant_id| {
            let descendant_table = self
                .extension_graph
                .nodes_vocabulary()
                .get(descendant_id)
                .expect("Failed to get descendant table");
            self.is_diamond_bottom(descendant_table)
                .map_or(false, |bottom_table| bottom_table == table)
        })
    }

    /// Returns whether the provided table is a diamond side.
    ///
    /// # Arguments
    ///
    /// * `table`: A reference to the table to check.
    ///
    /// # Implementation details
    ///
    /// Other than the definitions for `is_diamond_bottom` and `is_diamond_top`,
    /// a table is a diamond side if its parent is a `diamong top`.
    pub fn is_diamond_side(&self, table: &Table) -> bool {
        let table_id = self
            .extension_graph
            .nodes_vocabulary()
            .binary_search(table)
            .expect("Failed to get node ID for the table");

        self.extension_graph.successors(table_id).any(|extended_table_id| {
            let extended_table = self
                .extension_graph
                .nodes_vocabulary()
                .get(extended_table_id)
                .expect("Failed to get extended table");
            self.is_diamond_top(extended_table)
        })
    }

    /// Returns the columns associated with the ancestors of the current table.
    ///
    /// # Arguments
    ///
    /// * `table`: A reference to the table to generate generics for.
    /// * `covered_successors`: A mutable reference to a hashmap that tracks
    ///   whether an ancestor extended by the current table has already been
    ///   covered by a predecessor.
    /// * `conn`: A mutable reference to a PostgreSQL connection.
    ///
    /// # Implementation details
    ///
    /// The columns from the ancestors which are covered by same-as
    /// relationships with columns in the descendant tables are removed as
    /// they should be handled fully by the descendant setter methods via
    /// the same-as relationships.
    ///
    /// # Errors
    ///
    /// Returns an error if the generics cannot be generated.
    pub fn ancestors_columns<'a>(
        &'a self,
        table: &'a Table,
        covered_successors: &mut HashMap<usize, bool>,
        conn: &mut PgConnection,
    ) -> Result<HashMap<&'a Table, Vec<Column>>, WebCodeGenError> {
        let table_id = self
            .extension_graph
            .nodes_vocabulary()
            .binary_search(table)
            .expect("Failed to get node ID for the table");

        let columns = table.columns(conn)?;
        let same_as_columns = columns
            .iter()
            .flat_map(|column| column.same_as_columns(conn).unwrap_or_default().into_iter())
            .collect::<Vec<_>>();

        let mut ancestors_columns = HashMap::new();

        self.extension_graph.successors(table_id).for_each(|extended_table_id| {
            let extended_table = self
                .extension_graph
                .nodes_vocabulary()
                .get(extended_table_id)
                .expect("Failed to get extended table");
            // If the extended table is already covered, its generic
            // parameters will be the primary key of the table, otherwise
            // we recursively determine it as the builder type with its own
            // generics.
            match covered_successors.entry(extended_table_id) {
                // If the extended table has already been covered, we
                // return its primary key type as the generic parameter.
                std::collections::hash_map::Entry::Occupied(_) => {}
                // If the extended table has not been covered yet, we
                // recursively determine its builder type generics.
                std::collections::hash_map::Entry::Vacant(entry) => {
                    entry.insert(true);
                    let ancestor_columns = self
                        .ancestors_columns(extended_table, covered_successors, conn)
                        .unwrap_or_default()
                        .into_iter()
                        .map(|(ancestor_table, columns)| {
                            (
                                ancestor_table,
                                columns
                                    .into_iter()
                                    .filter(|column| {
                                        // We filter out both the columns that are part of the
                                        // same-as
                                        // relationships with the current table, as those values
                                        // will be
                                        // handled by the descendant setter methods.
                                        !same_as_columns.contains(column)
                                    })
                                    .collect::<Vec<_>>(),
                            )
                        })
                        .collect::<HashMap<_, _>>();

                    ancestors_columns.insert(
                        extended_table,
                        ancestor_columns
                            .values()
                            .flat_map(|columns| columns.iter().cloned())
                            .collect(),
                    );
                }
            }
        });

        ancestors_columns.insert(table, table.insertable_columns(conn, false)?);

        Ok(ancestors_columns)
    }

    /// Returns the generics with the expected default values for the provided
    /// table.
    ///
    /// # Arguments
    ///
    /// * `table`: A reference to the table to generate generics for.
    ///
    /// # Errors
    ///
    /// Returns an error if the generics cannot be generated.
    fn generics_for_table_builder_type(
        &self,
        table: &Table,
        covered_successors: &mut HashMap<usize, bool>,
        conn: &mut PgConnection,
    ) -> Result<Option<TokenStream>, WebCodeGenError> {
        let table_id = self
            .extension_graph
            .nodes_vocabulary()
            .binary_search(table)
            .expect("Failed to get node ID for the table");

        // If the table is at the root of the extension graph, it has no
        // builder type generics.
        if !self.extension_graph.has_successors(table_id) {
            return Ok(None);
        }

        let generics = self
            .extension_graph
            .successors(table_id)
            .map(|extended_table_id| {
                let extended_table = self
                    .extension_graph
                    .nodes_vocabulary()
                    .get(extended_table_id)
                    .expect("Failed to get extended table");
                // If the extended table is already covered, its generic
                // parameters will be the primary key of the table, otherwise
                // we recursively determine it as the builder type with its own
                // generics.
                match covered_successors.entry(extended_table_id) {
                    // If the extended table has already been covered, we
                    // return its primary key type as the generic parameter.
                    std::collections::hash_map::Entry::Occupied(_) => {
                        let primary_key_ty = extended_table.primary_key_type(conn)?;
                        Ok(quote::quote! { Option<#primary_key_ty> })
                    }
                    // If the extended table has not been covered yet, we
                    // recursively determine its builder type generics.
                    std::collections::hash_map::Entry::Vacant(entry) => {
                        entry.insert(true);
                        let extended_builder_generics = self.generics_for_table_builder_type(
                            extended_table,
                            covered_successors,
                            conn,
                        )?;
                        let extended_builder_type = extended_table.insertable_builder_ty()?;
                        Ok(quote::quote! { #extended_builder_type #extended_builder_generics })
                    }
                }
            })
            .collect::<Result<Vec<_>, WebCodeGenError>>()?;

        Ok(Some(quote::quote! {
            <#(#generics),*>
        }))
    }

    /// Returns the generics with the expected default values for the provided
    /// table.
    ///
    /// # Arguments
    ///
    /// * `table`: A reference to the table to generate generics for.
    /// * `conn`: A mutable reference to a PostgreSQL connection.
    ///
    /// # Errors
    ///
    /// Returns an error if the generics cannot be generated.
    pub fn generics_for_table_builder_definition(
        &self,
        table: &Table,
        conn: &mut PgConnection,
    ) -> Result<Option<TokenStream>, WebCodeGenError> {
        let table_id = self
            .extension_graph
            .nodes_vocabulary()
            .binary_search(table)
            .expect("Failed to get node ID for the table");

        // If the table is at the root of the extension graph, it has no
        // builder type generics.
        if !self.extension_graph.has_successors(table_id) {
            return Ok(None);
        }

        // We create a hashmap to map whether an ancestor extended by the current
        // table has already been handled by a predecessor or not.
        let mut covered_successors = HashMap::new();

        // Otherwise, for each extended table, we generate the set of generics
        // with the expected default values to define the builder type.
        let generics = self
            .extension_graph
            .successors(table_id)
            .map(|extended_table_id| {
                let extended_table = self
                    .extension_graph
                    .nodes_vocabulary()
                    .get(extended_table_id)
                    .expect("Failed to get extended table");
                let extended_table_ident = extended_table.struct_ident()?;
                let extended_builder_type = extended_table.insertable_builder_ty()?;
                let extended_builder_generics = self.generics_for_table_builder_type(
                    extended_table,
                    &mut covered_successors,
                    conn,
                )?;
                Ok(quote::quote! {
                    #extended_table_ident=#extended_builder_type #extended_builder_generics
                })
            })
            .collect::<Result<Vec<_>, WebCodeGenError>>()?;

        Ok(Some(quote::quote! {
            <#(#generics),*>
        }))
    }

    /// Returns the number of immediate ancestors of the provided `table`.
    ///
    /// # Arguments
    ///
    /// * `table`: A reference to the table to check.
    pub fn number_of_extended_tables(&self, table: &Table) -> usize {
        let table_id = self
            .extension_graph
            .nodes_vocabulary()
            .binary_search(table)
            .expect("Failed to get node ID for the table");

        self.extension_graph.out_degree(table_id)
    }

    /// Returns the generics for the table builder implementation.
    ///
    /// # Arguments
    ///
    /// * `table`: A reference to the table to generate generics for.
    ///
    /// # Errors
    ///
    /// Returns an error if the generics cannot be generated.
    pub fn generics_for_table_builder_implementation(
        &self,
        table: &Table,
    ) -> Result<Option<TokenStream>, WebCodeGenError> {
        let table_id = self
            .extension_graph
            .nodes_vocabulary()
            .binary_search(table)
            .expect("Failed to get node ID for the table");

        // If the table is at the root of the extension graph, it has no
        // builder type generics.
        if !self.extension_graph.has_successors(table_id) {
            return Ok(None);
        }

        // Otherwise, for each extended table, we generate the set of generics
        // with the expected default values to define the builder type.
        let generics = self
            .extension_graph
            .successors(table_id)
            .map(|extended_table_id| {
                let extended_table = self
                    .extension_graph
                    .nodes_vocabulary()
                    .get(extended_table_id)
                    .expect("Failed to get extended table");
                let extended_table_ident = extended_table.struct_ident()?;
                Ok(quote::quote! {
                    #extended_table_ident
                })
            })
            .collect::<Result<Vec<_>, WebCodeGenError>>()?;

        Ok(Some(quote::quote! {
            <#(#generics),*>
        }))
    }

    /// Returns the path of extensions to get from the provided `table` to the
    /// provided `column`.
    ///
    /// # Arguments
    ///
    /// * `table`: A reference to the table to start from.
    /// * `column`: A reference to the column to find the path to.
    pub(crate) fn extension_tree(
        &self,
        table: &Table,
        columns: &[Column],
    ) -> GenericGraph<&'_ SortedVec<Table>, SquareCSR2D<CSR2D<usize, usize, usize>>> {
        let paths = columns
            .iter()
            .filter_map(|column| self.extension_path(table, column))
            .map(|mut path| {
                path.insert(0, table);
                path
            })
            .collect::<Vec<_>>();

        let mut edges = paths
            .iter()
            .flat_map(|path| {
                path.iter().zip(path.iter().skip(1)).map(|(src, dst)| {
                    (
                        self.extension_graph.nodes_vocabulary().binary_search(src).unwrap(),
                        self.extension_graph.nodes_vocabulary().binary_search(dst).unwrap(),
                    )
                })
            })
            .collect::<Vec<_>>();

        edges.sort_unstable();
        edges.dedup();

        GenericMonoplexMonopartiteGraphBuilder::default()
            .nodes(self.extension_graph.nodes_vocabulary())
            .edges(
                DiEdgesBuilder::default()
                    .expected_shape(self.extension_graph.number_of_nodes())
                    .edges(edges)
                    .build()
                    .expect("Failed to build edges for the column same-as network"),
            )
            .build()
            .expect("Failed to build the extension tree graph")
    }

    /// Returns the path of extensions to get from the provided `table` to the
    /// provided `column`.
    ///
    /// # Arguments
    ///
    /// * `table`: A reference to the table to start from.
    /// * `column`: A reference to the column to find the path to.
    pub(crate) fn extension_path(&self, table: &Table, column: &Column) -> Option<Vec<&Table>> {
        // If the provided table contains the provided column, then the path is empty.
        if table.table_name == column.table_name
            && table.table_schema == column.table_schema
            && table.table_catalog == column.table_catalog
        {
            return Some(Vec::new());
        }

        let table_id = self
            .extension_graph
            .nodes_vocabulary()
            .binary_search(table)
            .expect("Failed to get node ID for the table");

        for ancestor_id in self.extension_graph.successors(table_id) {
            let ancestor_table = self
                .extension_graph
                .nodes_vocabulary()
                .get(ancestor_id)
                .expect("Failed to get ancestor table");
            match self.extension_path(ancestor_table, column) {
                Some(mut ancestor_path) => {
                    ancestor_path.insert(0, ancestor_table);
                    return Some(ancestor_path);
                }
                None => continue,
            }
        }

        None
    }

    /// Returns the path of extension foreign keys to get from the provided
    /// `table` to the provided `column`.
    ///
    /// # Arguments
    ///
    /// * `table`: A reference to the table to start from.
    /// * `column`: A reference to the column to find the path to.
    /// * `conn`: A mutable reference to a PostgreSQL connection.
    pub(crate) fn extension_foreign_keys_path(
        &self,
        table: &Table,
        column: &Column,
        conn: &mut PgConnection,
    ) -> Option<Vec<KeyColumnUsage>> {
        // If the provided table contains the provided column, then the path is empty.
        if table.table_name == column.table_name
            && table.table_schema == column.table_schema
            && table.table_catalog == column.table_catalog
        {
            return Some(Vec::new());
        }

        let table_id = self
            .extension_graph
            .nodes_vocabulary()
            .binary_search(table)
            .expect("Failed to get node ID for the table");

        for ancestor_id in self.extension_graph.successors(table_id) {
            let ancestor_table = self
                .extension_graph
                .nodes_vocabulary()
                .get(ancestor_id)
                .expect("Failed to get ancestor table");

            match self.extension_foreign_keys_path(ancestor_table, column, conn) {
                Some(mut ancestor_foreign_keys_path) => {
                    // We identify which of the foreign keys is the one that bridges
                    // from the current table to the ancestor table.
                    let foreign_key = table
                        .foreign_keys(conn)
                        .unwrap_or_default()
                        .into_iter()
                        .find(|fk| {
                            fk.is_extension(conn).unwrap_or(false)
                                && fk
                                    .foreign_table(conn)
                                    .ok()
                                    .flatten()
                                    .map_or(false, |fk_table| &fk_table == ancestor_table)
                        })
                        .unwrap();

                    ancestor_foreign_keys_path.insert(0, foreign_key);
                    return Some(ancestor_foreign_keys_path);
                }
                None => continue,
            }
        }

        None
    }

    /// Returns the extension tables for the provided table.
    ///
    /// # Arguments
    ///
    /// * `table`: A reference to the table to find the extension tables for.
    pub fn extension_tables(&self, table: &Table) -> Vec<&Table> {
        let table_id = self
            .extension_graph
            .nodes_vocabulary()
            .binary_search(table)
            .expect("Failed to get node ID for the table");

        self.extension_graph
            .successors(table_id)
            .map(|extended_table_id| {
                self.extension_graph
                    .nodes_vocabulary()
                    .get(extended_table_id)
                    .expect("Failed to get extended table")
            })
            .collect()
    }

    /// Returns the extension foreign keys for the provided table.
    ///
    /// # Arguments
    ///
    /// * `table`: A reference to the table to find the extension foreign keys
    ///   for.
    ///
    /// # Implementation details
    ///
    /// This method retrieves the foreign keys that are part of the
    /// extension tables of the provided table. It sorts the foreign keys
    /// based on the position of their foreign tables in the list of
    /// extension tables, ensuring that the foreign keys are returned in
    /// the same order as the extension tables.
    pub fn extension_foreign_keys(
        &self,
        table: &Table,
        conn: &mut PgConnection,
    ) -> Result<Vec<KeyColumnUsage>, WebCodeGenError> {
        let extension_tables = self.extension_tables(table);
        let extension_foreign_keys = table.extension_foreign_keys(conn)?;
        let mut sorted_extension_foreign_keys = extension_foreign_keys.clone();
        for extension_foreign_key in extension_foreign_keys {
            let foreign_table = extension_foreign_key
                .foreign_table(conn)?
                .expect("Failed to get foreign table for the extension foreign key");
            let expected_position = extension_tables
                .iter()
                .position(|&t| t == &foreign_table)
                .expect("Failed to find the foreign table in the extension tables");
            sorted_extension_foreign_keys[expected_position] = extension_foreign_key;
        }

        Ok(sorted_extension_foreign_keys)
    }
}
