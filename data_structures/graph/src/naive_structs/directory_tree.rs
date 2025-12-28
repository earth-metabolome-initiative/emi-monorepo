//! Submodule providing a graph which represents a directory tree structure.

use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

use sorted_vec::prelude::SortedVec;

use crate::{
    prelude::{DiGraph, GenericEdgesBuilder},
    traits::{EdgesBuilder, MonopartiteGraph, MonoplexGraph},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// A directory tree represented as a directed graph.
pub struct DirectoryTree {
    graph: DiGraph<PathBuf>,
}

fn recursive_build_tree(
    path: &Path,
    path_id: usize,
    nodes: &mut Vec<PathBuf>,
    edges: &mut Vec<(usize, usize)>,
) {
    if let Ok(entries) = path.read_dir() {
        let mut entries = entries.flatten().collect::<Vec<_>>();
        entries.sort_by_key(std::fs::DirEntry::path);
        for entry in entries {
            let child_path = entry.path();
            let child_id = nodes.len();
            edges.push((path_id, child_id));
            nodes.push(child_path.clone());
            if child_path.is_dir() {
                recursive_build_tree(&child_path, child_id, nodes, edges);
            }
        }
    }
}

impl From<PathBuf> for DirectoryTree {
    fn from(path: PathBuf) -> Self {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();

        nodes.push(path.clone());
        recursive_build_tree(&path, 0, &mut nodes, &mut edges);
        edges.sort_unstable();

        let edges = GenericEdgesBuilder::default()
            .edges(edges)
            .expected_shape(nodes.len())
            .build()
            .expect("Failed to build directory tree edges");

        let graph = DiGraph::try_from((
            SortedVec::try_from(nodes).expect("Failed to build directory tree nodes"),
            edges,
        ))
        .expect("Failed to build graph");

        DirectoryTree { graph }
    }
}

fn display_node(
    f: &mut std::fmt::Formatter<'_>,
    graph: &DiGraph<PathBuf>,
    node: usize,
    indent_level: usize,
) -> std::fmt::Result {
    for successor in graph.successors(node) {
        for _ in 0..indent_level {
            write!(f, "    ")?; // 4 spaces per indent level
        }
        let path = &graph.nodes_vocabulary()[successor];
        let last_segment = path.file_name().unwrap_or_default();
        writeln!(f, "/{}", last_segment.display())?;
        display_node(f, graph, successor, indent_level + 1)?;
    }
    Ok(())
}

impl Display for DirectoryTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // We display the directory tree in a simple text format, with indentation
        // representing depth.
        let base_path = &self.graph.nodes_vocabulary()[0];
        writeln!(f, "{}", base_path.display())?;
        display_node(f, &self.graph, 0, 1)
    }
}
