//! Submodule providing an iterator struct which uses a listener pattern trait
//! to traverse a `ProcedureTemplateGraph`.

use std::collections::VecDeque;

use core_structures::{ProcedureTemplate, ProcedureTemplateAssetModel};

use crate::{
    PTGListener, ProcedureTemplateGraph,
    structs::{HierarchyLike, OwnershipLike, TaskGraph},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
enum CurrentNodeVisitState {
    #[default]
    Unvisited,
    Visiting,
    Visited,
}

#[derive(Debug, Clone)]
#[allow(clippy::type_complexity)]
/// Visitor pattern trait for traversing a `ProcedureTemplateGraph`.
pub struct PTGVisitor<'graph, G, L> {
    /// The graph to traverse.
    graph: &'graph G,
    /// The listener to apply at each node.
    listener: L,
    /// Iterator over foreign procedure templates.
    foreign_procedures_iter: core::slice::Iter<'graph, ProcedureTemplate>,
    /// Procedure template asset models employed by the current node.
    ptam_iter: Option<Vec<&'graph ProcedureTemplateAssetModel>>,
    /// Parents stack
    parents: Vec<&'graph ProcedureTemplate>,
    /// The current node being visited.
    current_node: Option<&'graph ProcedureTemplate>,
    /// The state of the current node being visited.
    current_node_state: CurrentNodeVisitState,
    /// Stores whether
    /// Stack of nodes to visit in the current task graph.
    nodes_to_visit: Vec<(
        &'graph TaskGraph,
        VecDeque<(&'graph ProcedureTemplate, Vec<&'graph ProcedureTemplate>)>,
    )>,
}

impl<'graph, G, L> PTGVisitor<'graph, G, L>
where
    G: AsRef<ProcedureTemplateGraph>,
{
    /// Creates a new `PTGVisitor`.
    pub fn new(graph: &'graph G, listener: L) -> Self {
        Self {
            foreign_procedures_iter: graph.as_ref().foreign_procedure_templates(),
            parents: Vec::new(),
            current_node: Some(graph.as_ref().root_procedure_template()),
            graph,
            listener,
            current_node_state: CurrentNodeVisitState::Unvisited,
            ptam_iter: None,
            nodes_to_visit: Vec::new(),
        }
    }

    /// Returns a reference to the underlying graph.
    pub fn graph(&self) -> &'graph ProcedureTemplateGraph {
        self.graph.as_ref()
    }

    /// Returns a reference to the underlying listener.
    pub fn listener(&self) -> &L {
        &self.listener
    }

    /// Returns a mutable reference to the underlying listener.
    pub fn listener_mut(&mut self) -> &mut L {
        &mut self.listener
    }
}

impl ProcedureTemplateGraph {
    /// Returns an iterator which traverses the graph using the provided
    /// listener.
    ///
    /// # Arguments
    ///
    /// * `listener` - The listener to apply at each node.
    pub fn visit_with<'graph, L>(&'graph self, listener: L) -> PTGVisitor<'graph, Self, L>
    where
        L: PTGListener<'graph>,
    {
        PTGVisitor::new(self, listener)
    }
}

impl<'graph, G, L> Iterator for PTGVisitor<'graph, G, L>
where
    G: AsRef<ProcedureTemplateGraph>,
    L: PTGListener<'graph>,
{
    type Item = Result<L::Output, L::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(foreign_procedure_template_id) = self.foreign_procedures_iter.next() {
            return Some(
                self.listener.enter_foreign_procedure_template(foreign_procedure_template_id),
            );
        }

        let current_node = self.current_node?;

        Some(match self.current_node_state {
            CurrentNodeVisitState::Unvisited => {
                // We determine the visit outcome of visiting the current node.
                let outcome =
                    self.listener.enter_procedure_template(self.parents.as_slice(), current_node);

                // When we are starting to visit a node, we setup its recursion.
                if let Some(task_graph) = self.graph.as_ref().task_graph_of(current_node) {
                    // If the node is associated with a task, we need to explore and
                    // visit its task graph.
                    // We push the current node to the parents stack, set the
                    // current node to the root of the task graph, and push the
                    // task graph to the stack of nodes to visit.
                    self.parents.push(current_node);
                    let root_node = task_graph.root_node();
                    self.current_node = Some(root_node);
                    // Since we are changing the current node, we would need
                    // to change its state to unvisited, but this is already
                    // the value of `current_node_state`, so no action is needed.

                    // We push the task graph to the stack of nodes to visit, with
                    // the root node as the first node to visit, and an empty
                    // predecessors list.
                    self.nodes_to_visit
                        .push((task_graph, VecDeque::from([(root_node, Vec::new())])));
                } else {
                    // Otherwise, we are at a leaf node, and we need to visit all
                    // its owned procedure template asset models.
                    self.ptam_iter =
                        Some(self.graph.as_ref().employed_by(self.current_node?).collect());
                    // We change the state of the current node to visiting, so that
                    // we can start visiting its owned procedure template asset models.
                    self.current_node_state = CurrentNodeVisitState::Visiting;
                }

                outcome
            }
            CurrentNodeVisitState::Visiting => {
                // When we are visiting a node, so we can either be visiting its
                // owned procedure template asset models, or its task graph.
                if let Some(ptam_iter) = &mut self.ptam_iter {
                    // If there is a procedure template asset model iterator, then this indicates
                    // that we are visiting a leaf node, and we need to visit all its owned
                    // procedure template asset models.
                    if let Some(ptam) = ptam_iter.pop() {
                        // If there are still owned procedure template asset models to visit,
                        // we visit the next one.
                        self.listener.enter_leaf_ptam(self.parents.as_slice(), current_node, ptam)
                    } else {
                        // Otherwise, we have finished visiting all owned procedure template asset
                        // models. We change the state of the current node
                        // to visited, so that we can move on to
                        // the next node in the task graph, if any.
                        self.ptam_iter = None;
                        self.current_node_state = CurrentNodeVisitState::Visited;
                        self.listener
                            .leave_procedure_template(self.parents.as_slice(), current_node)
                    }
                } else if let Some((task_graph, nodes)) = self.nodes_to_visit.last_mut() {
                    // If there is a task graph to visit, then we are visiting a non-leaf node,
                    // and the current node is expected to be a node within the task graph.
                    assert_eq!(
                        *task_graph,
                        self.graph.as_ref().task_graph_of(current_node).unwrap(),
                        "The current node should be the one associated with the task graph we are visiting."
                    );

                    // If there are still nodes to visit in the task graph, we expect that we
                    // currently have finished visiting the current inner one,
                    // got sent back to the parent node, and now we need to
                    // continue exploring its task graph.
                    if let Some((node, _predecessors)) = nodes.front() {
                        self.current_node = Some(node);
                        self.current_node_state = CurrentNodeVisitState::Unvisited;
                        self.parents.push(current_node);
                        self.next()?
                    } else {
                        // Otherwise, we have finished visiting all nodes in the task graph.
                        // We pop the task graph from the stack and change the state of the current
                        // node to visited.
                        self.nodes_to_visit.pop();
                        self.current_node_state = CurrentNodeVisitState::Visited;
                        self.listener
                            .leave_procedure_template(self.parents.as_slice(), current_node)
                    }
                } else {
                    unreachable!(
                        "If we are visiting a node, we must either be visiting its owned procedure template asset models, or its task graph."
                    )
                }
            }
            CurrentNodeVisitState::Visited => {
                // If we have finished visiting the node, we need to move to its parent.
                // If there are no parents, then we have finished visiting the entire graph.
                if let Some((task_graph, nodes)) = self.nodes_to_visit.last_mut() {
                    // And we ask the listener to filter the successors of the current node, so
                    // to only explore the ones that are relevant to the
                    // current context.
                    let successors = match self
                        .listener
                        .filter_successors(task_graph.successors(current_node))
                    {
                        Ok(successors) => successors,
                        Err(e) => return Some(Err(e)),
                    };

                    let (node, mut predecessors) = nodes.pop_front().unwrap();
                    assert_eq!(
                        node, current_node,
                        "The current node should be the one we are visiting."
                    );

                    if let Err(err) = self.listener.continue_task(
                        self.parents.as_slice(),
                        predecessors.as_slice(),
                        current_node,
                    ) {
                        return Some(Err(err));
                    }

                    predecessors.push(current_node);

                    // We push the successors to the nodes to visit stack, so that we can
                    // explore them later.
                    for successor in successors {
                        nodes.push_back((successor, predecessors.clone()));
                    }
                }

                self.current_node_state = CurrentNodeVisitState::Visiting;
                self.current_node = self.parents.pop();
                self.next()?
            }
        })
    }
}
