#![cfg(feature = "postgres")]
//! Submodule providing a `ProcedureModelDot` trait for creating a DOT
//! representation of a procedure model.

use std::collections::HashMap;

use ::graph::{
    prelude::{DiEdgesBuilder, DiGraph, GenericMonoplexMonopartiteGraphBuilder},
    traits::{
        EdgesBuilder, Graph, MonopartiteGraph, MonopartiteGraphBuilder, MonoplexGraph,
        MonoplexGraphBuilder,
    },
};
use algebra::{impls::SquareCSR2D, prelude::Kahn};
use diesel::{Identifiable, PgConnection};
use sorted_vec::prelude::SortedVec;
use web_common_traits::{
    database::Read,
    prelude::{Builder, ExtensionTable},
};

use crate::{NextProcedureModel, ParentProcedureModel, ProcedureModel, ProcedureModelTrackable};

const RED: &str = "\"#EF3340\"";
const PASTEL_COLORS: [&str; 99] = [
    "#E7D27C", "#FFF8D5", "#DDA0DD", "#E6AAD2", "#F1BEB5", "#D3B8A1", "#9DD6AD", "#FAF8F6",
    "#A5E3E0", "#D5F6FB", "#D69759", "#B2C9E2", "#EF9967", "#8686AF", "#807DDB", "#C4D5E0",
    "#EFDFD8", "#BCC4E9", "#AEC6CF", "#F6C1B2", "#CFCFC4", "#E4C9E0", "#BBA151", "#CFAC94",
    "#FFA4A9", "#974C5E", "#BCBC82", "#C5CBE1", "#C25964", "#F0EBD8", "#F6B8D0", "#ED8698",
    "#E56E90", "#E8C6AA", "#75655A", "#EBCCFF", "#A3CCE3", "#C3D9C4", "#F8D0D2", "#E7D7CA",
    "#B4D9EF", "#63B7B7", "#DFFF8F", "#B2F4E3", "#D0E9C0", "#DDE0E4", "#D8CBE9", "#F9CCB6",
    "#AFC0EA", "#D4EDF0", "#EB618F", "#9AE2E3", "#D3C7A2", "#FFB347", "#D0A48D", "#E99FAA",
    "#F6F3A9", "#5E5CB2", "#FFD1E1", "#F4B5C5", "#FFF5B2", "#BD5F60", "#C8BFD6", "#BEDDF1",
    "#FFFAC8", "#9BCC9E", "#F8C57C", "#A16AD1", "#A4D8D8", "#D1FEB8", "#FFD5D9", "#E5C768",
    "#6ECDDB", "#944547", "#F5E2E4", "#9ECB91", "#FFFEE0", "#2F4C39", "#D7CAB7", "#B1C086",
    "#F7DFC2", "#C79098", "#EEF3F6", "#D4C6AA", "#B0E9D5", "#AFCF99", "#E5ECF8", "#E6E6FA",
    "#FFDDB3", "#E9C9AA", "#3D426B", "#BDFCC9", "#FFA38C", "#FCEACD", "#A7796D", "#DBDBDC",
    "#D05C39", "#DAD4B6", "#A2CFDD",
];

pub trait ProcedureModelDot: ExtensionTable<ProcedureModel>
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{
    /// Returns the name of the procedure model.
    fn name(&self, conn: &mut PgConnection) -> Result<String, diesel::result::Error> {
        use diesel::Identifiable;

        if let Some(model) = ProcedureModel::read(*self.id(), conn)? {
            Ok(model.name)
        } else {
            Err(diesel::result::Error::NotFound)
        }
    }

    /// Returns the topological graph of the procedure models.
    fn graph(
        &self,
        conn: &mut PgConnection,
    ) -> Result<DiGraph<ProcedureModel>, diesel::result::Error> {
        let mut child_procedures =
            ParentProcedureModel::from_parent_procedure_model_id(self.id(), conn)?
                .into_iter()
                .filter_map(|procedure| {
                    let child_procedure = procedure.child_procedure_model(conn).ok()?;
                    let mut child_procedures =
                        ParentProcedureModel::from_parent_procedure_model_id(
                            &child_procedure.id,
                            conn,
                        )
                        .ok()?
                        .into_iter()
                        .filter_map(|p| p.child_procedure_model(conn).ok())
                        .collect::<Vec<_>>();

                    if child_procedures.is_empty() {
                        child_procedures.push(child_procedure);
                    }

                    Some(child_procedures)
                })
                .flatten()
                .collect::<Vec<_>>();

        child_procedures.sort_unstable();

        let child_procedures: SortedVec<ProcedureModel> =
            SortedVec::try_from(child_procedures).unwrap();

        let edges: SquareCSR2D<_> = DiEdgesBuilder::default()
            .expected_shape(child_procedures.len())
            .edges(NextProcedureModel::from_parent_id(self.id(), conn)?.into_iter().filter_map(
                |next| {
                    let current = next.current(conn).ok()?;
                    let successor = next.successor(conn).ok()?;
                    Some((
                        child_procedures.binary_search(&current).unwrap(),
                        child_procedures.binary_search(&successor).unwrap(),
                    ))
                },
            ))
            .build()
            .unwrap();

        let graph: DiGraph<ProcedureModel> = GenericMonoplexMonopartiteGraphBuilder::default()
            .nodes(child_procedures)
            .edges(edges)
            .build()
            .unwrap();

        Ok(graph)
    }

    /// Returns the IDs of the nodes in the procedure model, if any.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection` for database
    ///   operations.
    ///
    /// # Errors
    ///
    /// * If an error occurs while retrieving the ID, it returns a
    ///   `diesel::result::Error`.
    fn nodes(&self, conn: &mut PgConnection) -> Result<Vec<String>, diesel::result::Error> {
        let graph = self.graph(conn)?;
        let ordering = if graph.has_nodes() {
            graph.edges().kahn().unwrap()
        } else {
            let trackables = ProcedureModelTrackable::from_procedure_model_id(self.id(), conn)?
                .into_iter()
                .map(|trackable| format!("T{}", trackable.id))
                .collect::<Vec<String>>();

            if trackables.is_empty() {
                return Ok(vec![]);
            }

            return Ok(vec![trackables[trackables.len() / 2].clone()]);
        };
        let procedures_vocabulary = graph.nodes_vocabulary();

        Ok(ordering
            .into_iter()
            .filter_map(|id| {
                let child_procedure = procedures_vocabulary.get(id).unwrap();
                let mut nodes = child_procedure.nodes(conn).ok()?;

                if nodes.is_empty() {
                    nodes.push(format!("P{}", child_procedure.id));
                }

                Some(nodes)
            })
            .flatten()
            .collect())
    }

    /// Returns the ID of the last node in the procedure model, if any.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection` for database
    ///   operations.
    ///
    /// # Errors
    ///
    /// * If an error occurs while retrieving the last node ID, it returns a
    ///   `diesel::result::Error`.
    fn last_node(&self, conn: &mut PgConnection) -> Result<Option<String>, diesel::result::Error> {
        Ok(self.nodes(conn)?.pop())
    }

    /// Returns the ID of the first node in the procedure model, if any.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection` for database
    ///   operations.
    ///
    /// # Errors
    ///
    /// * If an error occurs while retrieving the first node ID, it returns a
    ///   `diesel::result::Error`.
    fn first_node(&self, conn: &mut PgConnection) -> Result<Option<String>, diesel::result::Error> {
        Ok(self.nodes(conn)?.first().cloned())
    }

    /// Returns the trackable nodes associated with the procedure model.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection` for database
    ///   operations.
    ///
    /// # Errors
    ///
    /// * If an error occurs while retrieving the trackable nodes, it returns a
    ///   `diesel::result::Error`.
    fn trackable_nodes(&self, conn: &mut PgConnection) -> Result<String, diesel::result::Error> {
        let mut dot = String::new();
        for procedure_trackable in
            ProcedureModelTrackable::from_procedure_model_id(self.id(), conn)?
        {
            let bgcolor = PASTEL_COLORS
                .get(procedure_trackable.id as usize % PASTEL_COLORS.len())
                .expect("Color index should be valid");

            dot.push_str(&format!(
                "    T{} [label=\"{}\", shape=box, style=\"filled,rounded\", color=\"{bgcolor}\", fillcolor=\"{bgcolor}\"];\n",
                procedure_trackable.id, procedure_trackable.name
            ));
        }

        Ok(dot)
    }

    fn procedure_nodes_and_edges(
        &self,
        subgraph_prefix: &str,
        conn: &mut PgConnection,
    ) -> Result<String, diesel::result::Error> {
        let mut dot = String::new();

        let mut procedures = HashMap::new();

        for (i, subprocedure) in
            ParentProcedureModel::from_parent_procedure_model_id(self.id(), conn)?
                .into_iter()
                .enumerate()
        {
            let child_procedure = subprocedure.child_procedure_model(conn)?;

            if child_procedure.last_node(conn)?.is_none() {
                // If the child procedure has no trackables and no children, we display it as a
                // box
                dot.push_str(&format!(
                    "    P{} [label=\"{}\", shape=box, color={RED}];\n",
                    child_procedure.id,
                    child_procedure.name(conn)?,
                ));
                procedures.insert(child_procedure.id, None);
            } else {
                let subgraph_prefix = format!("{}_{}", subgraph_prefix, i);
                dot.push_str(child_procedure.to_subgraph(&subgraph_prefix, conn)?.as_str());
                procedures.insert(child_procedure.id, Some(subgraph_prefix));
            }
        }

        // We link next procedures to child procedures
        for next in NextProcedureModel::from_parent_id(self.id(), conn)? {
            let previous_procedure = next.current(conn)?;
            let next_procedure = next.successor(conn)?;
            let previous_subgraph = procedures
                .get(&previous_procedure.id)
                .expect("Previous procedure should be in the map");
            let next_subgraph =
                procedures.get(&next_procedure.id).expect("Next procedure should be in the map");

            let mut attributes = vec![format!("label=\"And then\""), format!("color={RED}")];

            if let Some(previous) = previous_subgraph {
                attributes.push(format!("ltail=cluster_{}", previous));
            }
            if let Some(next) = next_subgraph {
                attributes.push(format!("lhead=cluster_{}", next));
            }

            let previous_node = previous_procedure
                .last_node(conn)?
                .unwrap_or_else(|| format!("P{}", previous_procedure.id));
            let next_node = next_procedure
                .first_node(conn)?
                .unwrap_or_else(|| format!("P{}", next_procedure.id));
            dot.push_str(&format!(
                "    {} -> {} [{}];\n",
                previous_node,
                next_node,
                attributes.join(", ")
            ));
        }

        Ok(dot)
    }

    /// Generates a DOT representation of the procedure subgraph model.
    ///
    /// # Arguments
    ///
    /// * `subgraph_prefix` - A prefix for the subgraph label, typically the
    ///   procedure model's name.
    /// * `conn` - A mutable reference to a `PgConnection` for database
    ///   operations.
    ///
    /// # Errors
    ///
    /// * If an error occurs while generating the DOT representation, it returns
    ///   a `diesel::result::Error`.
    fn to_subgraph(
        &self,
        subgraph_prefix: &str,
        conn: &mut PgConnection,
    ) -> Result<String, diesel::result::Error> {
        if self.last_node(conn)?.is_none() {
            // If the procedure model has no trackables and no children, we display it as a
            // box
            return Ok(format!(
                "    P{} [label=\"{}\", shape=box, color={RED}];\n",
                self.id(),
                self.name(conn)?
            ));
        }

        let mut dot = String::new();

        dot.push_str(&format!("subgraph cluster_{} {{\n", subgraph_prefix));
        dot.push_str(format!("\tlabel=<<U>{}</U>>;\n", self.name(conn)?).as_str());
        dot.push_str("\tstyle=\"rounded\";\n");
        dot.push_str(&format!("\tcolor={RED};\n"));

        if ParentProcedureModel::from_parent_procedure_model_id(self.id(), conn)?.is_empty() {
            dot.push_str(self.trackable_nodes(conn)?.as_str());
        }
        // dot.push_str(self.shared_trackable_edges(conn)?.as_str());

        let mut procedures = HashMap::new();

        for (i, subprocedure) in
            ParentProcedureModel::from_parent_procedure_model_id(self.id(), conn)?
                .into_iter()
                .enumerate()
        {
            let child_procedure = subprocedure.child_procedure_model(conn)?;

            if child_procedure.last_node(conn)?.is_none() {
                // If the child procedure has no trackables and no children, we display it as a
                // box
                dot.push_str(&format!(
                    "\tP{} [label=\"{}\", shape=box, color={RED}];\n",
                    child_procedure.id,
                    child_procedure.name(conn)?
                ));
                procedures.insert(child_procedure.id, None);
            } else {
                let subgraph_prefix = format!("{}_{}", subgraph_prefix, i);
                dot.push_str(child_procedure.to_subgraph(&subgraph_prefix, conn)?.as_str());
                procedures.insert(child_procedure.id, Some(subgraph_prefix));
            }
        }

        // We link next procedures to child procedures
        for next in NextProcedureModel::from_parent_id(self.id(), conn)? {
            let previous_procedure = next.current(conn)?;
            let next_procedure = next.successor(conn)?;
            let previous_subgraph = procedures
                .get(&previous_procedure.id)
                .expect("Previous procedure should be in the map");
            let next_subgraph =
                procedures.get(&next_procedure.id).expect("Next procedure should be in the map");

            let mut attributes = vec![format!("label=\"And then\""), format!("color={RED}")];

            if let Some(previous) = previous_subgraph {
                attributes.push(format!("ltail=cluster_{}", previous));
            }
            if let Some(next) = next_subgraph {
                attributes.push(format!("lhead=cluster_{}", next));
            }

            let previous_node = previous_procedure
                .last_node(conn)?
                .unwrap_or_else(|| format!("P{}", previous_procedure.id));
            let next_node = next_procedure
                .last_node(conn)?
                .unwrap_or_else(|| format!("P{}", next_procedure.id));
            dot.push_str(&format!(
                "    {}:s -> {}:n [{}];\n",
                previous_node,
                next_node,
                attributes.join(", ")
            ));
        }

        dot.push_str("}\n");

        Ok(dot)
    }

    /// Generates a DOT representation of the procedure model.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection` for database
    ///   operations.
    ///
    /// # Errors
    ///
    /// * If an error occurs while generating the DOT representation, it returns
    ///   a `diesel::result::Error`.
    fn to_dot(&self, conn: &mut PgConnection) -> Result<String, diesel::result::Error> {
        let mut dot = String::new();

        dot.push_str("digraph G {\n");

        // We set the graph configuration
        dot.push_str(&format!(
            "\tgraph [\n\
            \t\tlabel=<<B>{}</B>>,\n\
            \t\tfontname=\"Helvetica\",\n\
            \t\tfontsize=12,\n\
            \t\tbgcolor=white,\n\
            \t\tsplines=true,\n\
            \t\tnodesep=0.6,\n\
            \t\tconcentrate=true,\n\
            \t\tcompound=true,\n\
            \t];\n",
            self.name(conn)?,
        ));

        dot.push_str("\tnode [style=rounded, fontname=\"Helvetica\"];\n");
        dot.push_str("\tedge [fontname=\"Helvetica\"];\n");
        // dot.push_str(&self.trackable_nodes(conn)?);
        // dot.push_str(&self.shared_trackable_edges(conn)?);

        dot.push_str(self.procedure_nodes_and_edges("", conn)?.as_str());

        dot.push_str("}\n");

        Ok(dot)
    }
}

impl<T> ProcedureModelDot for T
where
    T: ExtensionTable<ProcedureModel>,
    for<'a> &'a T: diesel::Identifiable<Id = &'a i32>,
{
}
