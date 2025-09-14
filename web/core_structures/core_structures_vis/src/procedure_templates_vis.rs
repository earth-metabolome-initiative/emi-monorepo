//! Submodule providing an illustration of a procedure and its subprocedures
//! using a Flowchart Diagram in Mermaid syntax.

use std::rc::Rc;

use core_structures::{ProcedureTemplate, ProcedureTemplateAssetModel};
use diesel::PgConnection;
use guided_procedure_builder::{HierarchyLike, OwnershipLike, PTGVisitor, ProcedureTemplateGraph};
use mermaid::{
    prelude::{
        ArrowShape, ConfigurationBuilder, DiagramBuilder, Direction, Flowchart, FlowchartBuilder,
        FlowchartConfigurationBuilder, FlowchartEdgeBuilder, FlowchartNode, FlowchartNodeBuilder,
        FlowchartNodeShape, LineStyle, Renderer, StyleClass,
    },
    traits::{EdgeBuilder, NodeBuilder},
};

mod foreign_procedure_template_class;
mod procedure_template_class;
mod ptam_style_classes;

use crate::{MermaidDB, asset_model_icon};

struct ProcedureTemplateVisualization<'graph> {
    graph: &'graph ProcedureTemplateGraph,
    builder: FlowchartBuilder,
    node_builders_stack: Vec<FlowchartNodeBuilder>,
    required_procedures: FlowchartNodeBuilder,
}

impl<'graph> ProcedureTemplateVisualization<'graph> {
    fn new(graph: &'graph ProcedureTemplateGraph) -> Result<Self, crate::Error> {
        let mut builder = FlowchartBuilder::default().configuration(
            FlowchartConfigurationBuilder::default()
                .renderer(Renderer::EclipseLayoutKernel)
                .direction(Direction::TopToBottom)
                .title(&graph.root_procedure_template_name())?,
        )?;

        builder.style_class(procedure_template_class::procedure_template_class())?;
        ptam_style_classes::ptam_classes(graph.root_and_foreign_ptams(), &mut builder)?;
        builder.style_class(foreign_procedure_template_class::foreign_procedure_template_class())?;

        Ok(Self {
            graph,
            builder,
            node_builders_stack: Vec::new(),
            required_procedures: FlowchartNodeBuilder::default()
                .label("**Required Procedures**")?,
        })
    }

    fn ptam_node_class(&self, ptam: &ProcedureTemplateAssetModel) -> Rc<StyleClass> {
        self.builder
            .get_style_class_by_name(&ptam_style_classes::ptam_node_class_name(ptam))
            .unwrap()
    }

    fn ptam_edge_class(&self, ptam: &ProcedureTemplateAssetModel) -> Rc<StyleClass> {
        self.builder
            .get_style_class_by_name(&ptam_style_classes::ptam_edge_class_name(ptam))
            .unwrap()
    }

    fn foreign_procedure_template_class(&self) -> Rc<StyleClass> {
        self.builder
            .get_style_class_by_name(
                foreign_procedure_template_class::FOREIGN_PROCEDURE_TEMPLATE_CLASS_NAME,
            )
            .unwrap()
            .clone()
    }

    fn procedure_template_class(&self) -> Rc<StyleClass> {
        self.builder
            .get_style_class_by_name(procedure_template_class::PROCEDURE_TEMPLATE_CLASS_NAME)
            .unwrap()
            .clone()
    }

    fn get_pt_node(
        &self,
        parents: &[&ProcedureTemplate],
        child: &ProcedureTemplate,
    ) -> Rc<FlowchartNode> {
        let node_id = procedure_template_hash(parents, child);
        self.builder.get_node_by_id(node_id).expect(&format!(
            "PT node {node_id} for \"{}\" with parents {} not found",
            child.name,
            parents.iter().map(|p| p.name.as_str()).collect::<Vec<_>>().join(", ")
        ))
    }

    fn get_ptam_node(
        &self,
        parents: &[&ProcedureTemplate],
        ptam: &ProcedureTemplateAssetModel,
    ) -> Rc<FlowchartNode> {
        let node_id = procedure_template_asset_model_hash(parents, ptam);
        self.builder.get_node_by_id(node_id).expect(&format!(
            "PTAM node {node_id} for \"{}\" with parents {} not found",
            ptam.name,
            parents.iter().map(|p| p.name.as_str()).collect::<Vec<_>>().join(", ")
        ))
    }
}

fn procedure_template_hash<I, PT1, PT2>(parents: I, child: PT1) -> u64
where
    I: IntoIterator<Item = PT2>,
    PT1: AsRef<ProcedureTemplate>,
    PT2: AsRef<ProcedureTemplate>,
{
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    for pt in parents {
        pt.as_ref().hash(&mut hasher);
    }
    child.as_ref().hash(&mut hasher);
    hasher.finish()
}

fn procedure_template_asset_model_hash<PT, I>(pts: I, ptam: &ProcedureTemplateAssetModel) -> u64
where
    I: IntoIterator<Item = PT>,
    PT: AsRef<ProcedureTemplate>,
{
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    for pt in pts {
        pt.as_ref().hash(&mut hasher);
    }
    ptam.hash(&mut hasher);
    hasher.finish()
}

impl PTGVisitor for ProcedureTemplateVisualization<'_> {
    type Error = crate::Error;
    type FilteredSuccessors<'graph, I>
        = I
    where
        I: Iterator<Item = &'graph ProcedureTemplate>;

    fn visit_foreign_procedure_template(
        &mut self,
        foreign_procedure_template: &ProcedureTemplate,
    ) -> Result<(), Self::Error> {
        let mut node_builder = FlowchartNodeBuilder::default()
            .label(&foreign_procedure_template.name)?
            .shape(FlowchartNodeShape::RoundEdges)
            .style_class(self.foreign_procedure_template_class())
            .unwrap();
        for foreign_ptam in self.graph.foreign_ptams_of(foreign_procedure_template) {
            let foreign_ptam_node = self.builder.node(
                FlowchartNodeBuilder::default()
                    .id(procedure_template_asset_model_hash(
                        std::iter::once(foreign_procedure_template),
                        foreign_ptam,
                    ))
                    .label(format!("*{}*", foreign_ptam.name))?
                    .shape(FlowchartNodeShape::LRParallelogram)
                    .style_class(self.ptam_node_class(foreign_ptam))
                    .unwrap(),
            )?;
            node_builder = node_builder.subnode(foreign_ptam_node.clone())?;
        }

        let node = self.builder.node(node_builder)?;
        self.required_procedures = self.required_procedures.clone().subnode(node)?;

        Ok(())
    }

    fn filter_successors<'graph, I>(
        &mut self,
        _parents: &[&ProcedureTemplate],
        _predecessors: &[&ProcedureTemplate],
        successors: I,
    ) -> Result<Self::FilteredSuccessors<'graph, I>, Self::Error>
    where
        I: Iterator<Item = &'graph ProcedureTemplate>,
    {
        // We always want to visit all successors.
        Ok(successors)
    }

    fn visit_procedure_template(
        &mut self,
        parents: &[&ProcedureTemplate],
        child: &ProcedureTemplate,
    ) -> Result<(), Self::Error> {
        let node_id = procedure_template_hash(parents, child);
        let mut node_builder = FlowchartNodeBuilder::default()
            .id(node_id)
            .label(&child.name)?
            .shape(FlowchartNodeShape::RoundEdges)
            .style_class(self.procedure_template_class())
            .unwrap();

        if let Some(previous_direction) =
            self.node_builders_stack.last().and_then(|previous| previous.get_direction())
        {
            node_builder = node_builder.direction(previous_direction.flip());
        } else {
            node_builder = node_builder.direction(Direction::TopToBottom);
        }

        self.node_builders_stack.push(node_builder);
        Ok(())
    }

    fn continue_task(
        &mut self,
        parents: &[&ProcedureTemplate],
        predecessors: &[&ProcedureTemplate],
        child: &ProcedureTemplate,
    ) -> Result<(), Self::Error> {
        if let Some(&predecessor) = predecessors.last() {
            self.builder.edge(
                FlowchartEdgeBuilder::default()
                    .source(self.get_pt_node(parents, predecessor))?
                    .destination(self.get_pt_node(parents, child))?
                    .right_arrow_shape(ArrowShape::Normal)?,
            )?;
        }
        Ok(())
    }

    fn leave_procedure_template(
        &mut self,
        parents: &[&ProcedureTemplate],
        child: &ProcedureTemplate,
    ) -> Result<(), Self::Error> {
        let mut node_builder = self.node_builders_stack.pop().unwrap();
        assert_eq!(
            node_builder.get_label().unwrap(),
            &child.name,
            "Mismatched node builder stack state"
        );

        if !node_builder.is_subgraph() {
            node_builder = node_builder.reset_direction();
        }

        let node = self.builder.node(node_builder.clone())?;

        if let Some(parent) = parents.last() {
            let parent_node_builder = self.node_builders_stack.pop().unwrap();
            assert_eq!(
                parent_node_builder.get_label().unwrap(),
                &parent.name,
                "Mismatched node builder stack state when adding child node"
            );

            self.node_builders_stack.push(parent_node_builder.subnode(node.clone())?);
        }

        Ok(())
    }

    fn visit_leaf_ptam(
        &mut self,
        parents: &[&ProcedureTemplate],
        leaf: &ProcedureTemplate,
        ptam: &ProcedureTemplateAssetModel,
    ) -> Result<(), Self::Error> {
        let asset_model = self.graph.asset_model_of(ptam);
        let label = if let Some(icon) = asset_model_icon(&asset_model) {
            format!("{} {}", icon, ptam.name)
        } else {
            ptam.name.clone()
        };

        let maybe_foreign_owner = self.graph.foreign_procedure_template_of(ptam);
        let (shape, reference_ptam) = if maybe_foreign_owner.is_some() {
            (FlowchartNodeShape::LRParallelogram, ptam)
        } else {
            (FlowchartNodeShape::Rectangle, self.graph
                .reference_based_on_alias(parents, &ptam)
                .expect(&format!(
                    "Expected PTAM \"{}\" from leaf PT \"{}\" to be either foreign-owned or have a reference based on alias using parents [{}]",
                    ptam.name,
                    leaf.name,
                    parents.iter().map(|p| p.name.as_str()).collect::<Vec<_>>().join(", ")
                )))
        };

        let procedure_template_asset_model_node_builder = FlowchartNodeBuilder::default()
            .id(procedure_template_asset_model_hash(
                parents.iter().copied().chain(std::iter::once(leaf)),
                reference_ptam,
            ))
            .shape(shape)
            .label(&label)?;

        let procedure_template_asset_model_node_builder =
            procedure_template_asset_model_node_builder
                .style_class(self.ptam_node_class(reference_ptam))
                .unwrap();

        let procedure_template_asset_model_node =
            self.builder.node(procedure_template_asset_model_node_builder)?;

        let current_node_builder = self.node_builders_stack.pop().unwrap();
        assert_eq!(
            current_node_builder.get_label().unwrap(),
            &leaf.name,
            "Mismatched node builder stack state when adding PTAM node"
        );
        self.node_builders_stack
            .push(current_node_builder.subnode(procedure_template_asset_model_node.clone())?);

        // If the procedure template asset model is not owned by the current
        // procedure template, we draw a dashed edge to indicate that it is a
        // reference to another procedure template asset model.
        if ptam.procedure_template != leaf.procedure_template {
            if let Some(foreign_owner) = maybe_foreign_owner {
                // We find the foreign owner procedure template.
                self.builder.edge(
                    FlowchartEdgeBuilder::default()
                        .source(self.get_ptam_node(&[foreign_owner], reference_ptam))?
                        .destination(procedure_template_asset_model_node.clone())?
                        .line_style(LineStyle::Dashed)
                        .style_class(self.ptam_edge_class(reference_ptam))
                        .unwrap(),
                )?;
            } else {
                let (parent, parents) = parents.split_last().unwrap();
                for closest_paths in self.graph.closest_paths_to_procedure_template_using_ptam(
                    parents,
                    parent,
                    leaf,
                    reference_ptam,
                    false,
                ) {
                    self.builder.edge(
                        FlowchartEdgeBuilder::default()
                            .source(self.get_ptam_node(&closest_paths, reference_ptam))?
                            .destination(procedure_template_asset_model_node.clone())?
                            .line_style(LineStyle::Dashed)
                            .style_class(self.ptam_edge_class(reference_ptam))
                            .unwrap(),
                    )?;
                }
            }
        }

        Ok(())
    }
}

impl MermaidDB<PgConnection> for ProcedureTemplate {
    type Diagram = Flowchart;
    type Error = crate::Error;

    fn to_mermaid(&self, conn: &mut PgConnection) -> Result<Self::Diagram, Self::Error> {
        let graph = ProcedureTemplateGraph::new(self, conn)?;
        let mut visualization = ProcedureTemplateVisualization::new(&graph)?;
        visualization.graph.visit(&mut visualization)?;
        if visualization.required_procedures.is_subgraph() {
            visualization.builder.node(visualization.required_procedures)?;
        }
        Ok(visualization.builder.into())
    }
}
