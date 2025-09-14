//! Submodule providing an illustration of a procedure and its subprocedures
//! using a Flowchart Diagram in Mermaid syntax.

use std::rc::Rc;

use core_structures::{ProcedureTemplate, ProcedureTemplateAssetModel};
use diesel::PgConnection;
use guided_procedure_builder::{HierarchyLike, OwnershipLike, PTGVisitor, ProcedureTemplateGraph};
use mermaid::{
    FlowchartError,
    prelude::{
        ArrowShape, Color, ConfigurationBuilder, CurveStyle, DiagramBuilder, Direction, Flowchart,
        FlowchartBuilder, FlowchartConfigurationBuilder, FlowchartEdgeBuilder, FlowchartNode,
        FlowchartNodeBuilder, FlowchartNodeShape, LineStyle, Renderer, StyleClass,
        StyleClassBuilder, StyleProperty,
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

        Ok(Self { graph, builder, node_builders_stack: Vec::new() })
    }

    fn ptam_node_class(&self, ptam: &ProcedureTemplateAssetModel) -> Rc<StyleClass> {
        todo!()
    }

    fn ptam_edge_class(&self, ptam: &ProcedureTemplateAssetModel) -> Rc<StyleClass> {
        todo!()
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
        self.builder.get_node_by_id(node_id).expect("ProcedureTemplate node not found")
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
                    .label(&foreign_ptam.name)?
                    .shape(FlowchartNodeShape::LRParallelogram)
                    .style_class(self.ptam_node_class(foreign_ptam))
                    .unwrap(),
            )?;
            node_builder = node_builder.subnode(foreign_ptam_node.clone())?;
        }

        self.builder.node(node_builder)?;

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
        self.node_builders_stack.push(
            FlowchartNodeBuilder::default()
                .id(procedure_template_hash(parents, child))
                .label(&child.name)?
                .shape(FlowchartNodeShape::RoundEdges)
                .style_class(self.procedure_template_class())
                .unwrap(),
        );
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
                    .destination(self.get_pt_node(parents, child))?,
            )?;
        }
        Ok(())
    }

    fn leave_procedure_template(
        &mut self,
        parents: &[&ProcedureTemplate],
        child: &ProcedureTemplate,
    ) -> Result<(), Self::Error> {
        let node_builder = self.node_builders_stack.pop().unwrap();
        assert_eq!(
            node_builder.get_label().unwrap(),
            &child.name,
            "Mismatched node builder stack state"
        );
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

        let mut procedure_template_asset_model_node_builder = FlowchartNodeBuilder::default()
            .id(procedure_template_asset_model_hash(
                parents.iter().copied().chain(std::iter::once(leaf)),
                ptam,
            ))
            .label(&label)?;

        let reference_ptam = if self.graph.foreign_owned_ptam(ptam) {
            ptam
        } else {
            self.graph.root_analogue_ptam(&ptam)
        };

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
            // self.builder.edge(
            //     FlowchartEdgeBuilder::default()
            //         .source(ptam_nodes.get(&procedure_template_asset_model).
            // unwrap().clone())?
            //         .destination(procedure_template_asset_model_node.
            // clone())?         .line_style(LineStyle::Dashed)
            //         .style_class(self.ptam_edge_class(reference_ptam))
            //         .unwrap(),
            // )?;
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
        Ok(visualization.builder.into())
    }
}
