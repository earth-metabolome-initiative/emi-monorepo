//! Submodule providing an illustration of a procedure and its subprocedures
//! using a Flowchart Diagram in Mermaid syntax.

use std::rc::Rc;

use core_structures::{ProcedureTemplate, ProcedureTemplateAssetModel};
use diesel::PgConnection;
use guided_procedures::{HierarchyLike, OwnershipLike, PTGListener, ProcedureTemplateGraph};
use mermaid::{
    prelude::{
        ArrowShape, ConfigurationBuilder, DiagramBuilder, Direction, Flowchart, FlowchartBuilder,
        FlowchartConfigurationBuilder, FlowchartEdgeBuilder, FlowchartNode, FlowchartNodeBuilder,
        FlowchartNodeShape, LineStyle, Renderer, StyleClass, StyleProperty, Unit,
    },
    traits::{EdgeBuilder, NodeBuilder},
};

mod foreign_procedure_template_class;
mod procedure_template_class;
mod ptam_style_classes;

use crate::{
    MermaidDB, asset_model_icon,
    procedure_templates_vis::procedure_template_class::{
        procedure_fill_color, procedure_stroke_color,
    },
    table_icons::procedure_template_icon,
};

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

        ptam_style_classes::ptam_classes(graph, graph.root_and_foreign_ptams(), &mut builder)?;
        builder.style_class(foreign_procedure_template_class::foreign_procedure_template_class())?;
        builder.style_class(procedure_template_class::procedure_arrow_class())?;

        Ok(Self {
            graph,
            builder,
            node_builders_stack: Vec::new(),
            required_procedures: FlowchartNodeBuilder::default()
                .label("**Required Procedures**")?
                .style_property(StyleProperty::Fill(procedure_fill_color()))
                .unwrap()
                .style_property(StyleProperty::Stroke(procedure_stroke_color()))
                .unwrap()
                .style_property(StyleProperty::BorderRadius(Unit::Pixel(3)))
                .unwrap()
                .style_property(StyleProperty::StrokeDasharray(5, 5))
                .unwrap(),
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

    fn get_pt_node(
        &self,
        parents: &[&ProcedureTemplate],
        child: &ProcedureTemplate,
    ) -> Rc<FlowchartNode> {
        let node_id = procedure_template_hash(parents, child);
        self.builder.get_node_by_id(node_id).unwrap_or_else(|| {
            panic!(
                "PT node {node_id} for \"{}\" with parents [{}] not found",
                child.name,
                parents.iter().map(|p| p.name.as_str()).collect::<Vec<_>>().join(", ")
            )
        })
    }

    fn get_ptam_node(
        &self,
        parents: &[&ProcedureTemplate],
        ptam: &ProcedureTemplateAssetModel,
    ) -> Rc<FlowchartNode> {
        let node_id = procedure_template_asset_model_hash(parents, ptam);
        self.builder.get_node_by_id(node_id).unwrap_or_else(|| {
            panic!(
                "PTAM node {node_id} for \"{}\" with parents {} not found",
                ptam.name,
                parents.iter().map(|p| p.name.as_str()).collect::<Vec<_>>().join(", ")
            )
        })
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

impl<'graph> PTGListener<'graph> for &mut ProcedureTemplateVisualization<'graph> {
    type Error = crate::Error;
    type FilteredSuccessors<I>
        = I
    where
        I: Iterator<Item = &'graph ProcedureTemplate>;
    type Output = ();

    fn enter_foreign_procedure_template(
        &mut self,
        foreign_procedure_template: &ProcedureTemplate,
    ) -> Result<(), Self::Error> {
        let procedure_name = procedure_template_icon(foreign_procedure_template_id).map_or_else(
            || format!("*{}*", foreign_procedure_template.name),
            |icon| format!("{} *{}*", icon, foreign_procedure_template.name),
        );

        let mut node_builder = FlowchartNodeBuilder::default()
            .label(procedure_name)?
            .shape(FlowchartNodeShape::RoundEdges)
            .style_property(StyleProperty::BorderRadius(Unit::Pixel(3)))
            .unwrap()
            .style_property(StyleProperty::Fill(procedure_fill_color().darken(2)))
            .unwrap()
            .style_property(StyleProperty::Stroke(procedure_stroke_color().darken(2)))
            .unwrap()
            .style_property(StyleProperty::StrokeDasharray(5, 5))
            .unwrap();
        for foreign_ptam in self.graph.foreign_ptams_of(foreign_procedure_template_id) {
            let foreign_ptam_node = self.builder.node(
                FlowchartNodeBuilder::default()
                    .id(procedure_template_asset_model_hash(
                        std::iter::once(foreign_procedure_template_id),
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

    fn filter_successors<I>(
        &mut self,
        successors: I,
    ) -> Result<Self::FilteredSuccessors<I>, Self::Error>
    where
        I: Iterator<Item = &'graph ProcedureTemplate>,
    {
        // We always want to visit all successors.
        Ok(successors)
    }

    fn enter_procedure_template(
        &mut self,
        parents: &[&ProcedureTemplate],
        child: &ProcedureTemplate,
    ) -> Result<(), Self::Error> {
        let node_id = procedure_template_hash(parents, child);
        let procedure_name = procedure_template_icon(child).map_or_else(
            || format!("*{}*", child.name),
            |icon| format!("{} *{}*", icon, child.name),
        );

        let mut node_builder = FlowchartNodeBuilder::default()
            .id(node_id)
            .label(&procedure_name)?
            .shape(FlowchartNodeShape::RoundEdges)
            .style_property(StyleProperty::BorderRadius(Unit::Pixel(3)))
            .unwrap();

        if let Some(previous_builder) = self.node_builders_stack.last() {
            for style_property in previous_builder.style_properties() {
                match style_property {
                    StyleProperty::Fill(fill) => {
                        node_builder = node_builder
                            .style_property(StyleProperty::Fill(fill.darken(3)))
                            .unwrap();
                    }
                    StyleProperty::Stroke(stroke) => {
                        node_builder = node_builder
                            .style_property(StyleProperty::Stroke(stroke.darken(3)))
                            .unwrap();
                    }
                    _ => {}
                }
            }
        } else {
            node_builder = node_builder
                .style_property(StyleProperty::Fill(procedure_fill_color()))
                .unwrap()
                .style_property(StyleProperty::Stroke(procedure_stroke_color()))
                .unwrap();
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
            let (root_leaf_node_parents, root_leaf_node) =
                self.graph.root_leaf_node_of(parents, child);
            for (sink_leaf_node_parents, sink_leaf_node) in
                self.graph.sink_leaf_nodes_of(parents, predecessor)
            {
                self.builder.edge(
                    FlowchartEdgeBuilder::default()
                        .source(self.get_pt_node(&sink_leaf_node_parents, sink_leaf_node))?
                        .destination(self.get_pt_node(&root_leaf_node_parents, root_leaf_node))?
                        .right_arrow_shape(ArrowShape::Normal)?
                        .style_class(
                            self.builder
                                .get_style_class_by_name(
                                    procedure_template_class::PROCEDURE_ARROW_CLASS_NAME,
                                )
                                .unwrap(),
                        )
                        .unwrap(),
                )?;
            }
        }
        Ok(())
    }

    fn leave_procedure_template(
        &mut self,
        parents: &[&ProcedureTemplate],
        child: &ProcedureTemplate,
    ) -> Result<(), Self::Error> {
        println!("{}Leaving procedure template: {}", "\t".repeat(parents.len()), child.name);

        let mut node_builder = self.node_builders_stack.pop().unwrap();

        // If we are back to the root, we skip adding the root node as we imply
        // that the overall diagram represents the root procedure template.
        if self.node_builders_stack.is_empty() {
            return Ok(());
        }

        if !node_builder.is_subgraph() {
            node_builder = node_builder.reset_direction();
        }

        let node = self.builder.node(node_builder.clone())?;

        if let Some(_parent) = parents.last() {
            let parent_node_builder = self.node_builders_stack.pop().unwrap();
            self.node_builders_stack.push(parent_node_builder.subnode(node.clone())?);
        }

        Ok(())
    }

    fn enter_leaf_ptam(
        &mut self,
        parents: &[&ProcedureTemplate],
        leaf: &ProcedureTemplate,
        ptam: &ProcedureTemplateAssetModel,
    ) -> Result<(), Self::Error> {
        let asset_model_id = self.graph.asset_model_of(ptam);
        let label = if let Some(icon) = asset_model_icon(asset_model_id) {
            format!("{} {}", icon, ptam.name)
        } else {
            ptam.name.clone()
        };

        let maybe_foreign_owner = self.graph.foreign_procedure_template_of(ptam);
        let (shape, reference_ptam) = if maybe_foreign_owner.is_some() {
            (FlowchartNodeShape::LRParallelogram, ptam)
        } else {
            (if ptam.procedure_template == leaf.procedure_template {
                FlowchartNodeShape::Rectangle
            } else {
                FlowchartNodeShape::Hexagon
            }, self.graph
            .reference_based_on_alias(parents, ptam)
            .unwrap_or_else(|| panic!("Expected PTAM \"{}\" from leaf PT \"{}\" to be either foreign-owned or have a reference based on alias using parents [{}]",
                ptam.name,
                leaf.name,
                parents.iter().map(|p| p.name.as_str()).collect::<Vec<_>>().join(", "))))
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
        graph.visit_with(&mut visualization).collect::<Result<(), Self::Error>>()?;
        if visualization.required_procedures.is_subgraph() {
            visualization.builder.node(visualization.required_procedures)?;
        }
        Ok(visualization.builder.into())
    }
}
