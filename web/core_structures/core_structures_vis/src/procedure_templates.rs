//! Submodule providing an illustration of a procedure and its subprocedures
//! using a Flowchart Diagram in Mermaid syntax.

use std::{collections::HashMap, rc::Rc};

use core_structures::{
    NextProcedureTemplate, ParentProcedureTemplate, ProcedureTemplate, ProcedureTemplateAssetModel,
};
use diesel::PgConnection;
use mermaid::{
    FlowchartError,
    prelude::{
        ArrowShape, Color, ConfigurationBuilder, DiagramBuilder, Direction, Flowchart,
        FlowchartBuilder, FlowchartConfigurationBuilder, FlowchartEdgeBuilder, FlowchartNode,
        FlowchartNodeBuilder, FlowchartNodeShape, Renderer, StyleClass, StyleClassBuilder,
        StyleProperty,
    },
    traits::{EdgeBuilder, NodeBuilder},
};

use crate::MermaidDB;

fn to_mermaid_node(
    root_procedure_template: &ProcedureTemplate,
    trackable_classes: &HashMap<&ProcedureTemplateAssetModel, Rc<StyleClass>>,
    procedure_template_class: &Rc<StyleClass>,
    procedure_template: &ProcedureTemplate,
    builder: &mut FlowchartBuilder,
    parent_direction: Direction,
    conn: &mut PgConnection,
) -> Result<Rc<FlowchartNode>, crate::Error> {
    let mut node_builder = FlowchartNodeBuilder::default()
        .label(&procedure_template.name)?
        .style_class(procedure_template_class.clone())
        .map_err(FlowchartError::from)?;

    let current_direction = parent_direction.flip();
    let subprocedures =
        ParentProcedureTemplate::from_parent(&procedure_template.procedure_template, conn)?;
    let next_procedures =
        NextProcedureTemplate::from_parent(&procedure_template.procedure_template, conn)?;

    if subprocedures.is_empty() {
        let procedure_trackables = ProcedureTemplateAssetModel::from_procedure_template(
            &procedure_template.procedure_template,
            conn,
        )?;
        if procedure_trackables.is_empty() {
            node_builder = node_builder.shape(FlowchartNodeShape::RoundEdges);
        } else {
            for procedure_template_trackable in procedure_trackables {
                let style_class = trackable_classes
                    .get(&procedure_template_trackable)
                    .expect("Style class not found");
                node_builder = node_builder.subnode(
                    builder.node(
                        FlowchartNodeBuilder::default()
                            .label(&procedure_template_trackable.name)?
                            .shape(FlowchartNodeShape::LRParallelogram)
                            .style_class(style_class.clone())
                            .map_err(FlowchartError::from)?,
                    )?,
                )?;
            }
            node_builder = node_builder.direction(current_direction);
        }
    } else {
        node_builder = node_builder.direction(current_direction);
    }

    for subprocedure in subprocedures {
        let child_procedure = subprocedure.child(conn)?;
        node_builder = node_builder.subnode(to_mermaid_node(
            root_procedure_template,
            trackable_classes,
            procedure_template_class,
            &child_procedure,
            builder,
            current_direction,
            conn,
        )?)?;
    }

    // Next, we chain add the edges representing which procedures are followed by
    // which.
    for subprocedure in next_procedures {
        let current_procedure = subprocedure.predecessor(conn)?;
        let next_procedure = subprocedure.successor(conn)?;
        builder.edge(
            FlowchartEdgeBuilder::default()
                .source(
                    builder
                        .get_node_by_label(&current_procedure.name)
                        .expect("Current procedure node not found"),
                )?
                .destination(
                    builder
                        .get_node_by_label(&next_procedure.name)
                        .expect("Next procedure node not found"),
                )?
                .right_arrow_shape(ArrowShape::Normal)?,
        )?;
    }

    Ok(builder.node(node_builder)?)
}

impl MermaidDB<PgConnection> for ProcedureTemplate {
    type Diagram = Flowchart;
    type Error = crate::Error;

    fn to_mermaid(&self, conn: &mut PgConnection) -> Result<Self::Diagram, Self::Error> {
        let parent_direction = Direction::LeftToRight;
        let mut builder = FlowchartBuilder::default().configuration(
            FlowchartConfigurationBuilder::default()
                .renderer(Renderer::EclipseLayoutKernel)
                .direction(parent_direction)
                .title(&self.name)?,
        )?;

        let procedure_trackables =
            ProcedureTemplateAssetModel::from_procedure_template(&self.procedure_template, conn)?;
        let colors = Color::maximally_distinct(procedure_trackables.len() as u16, 70, 80);
        let mut trackable_classes: HashMap<&ProcedureTemplateAssetModel, Rc<StyleClass>> =
            HashMap::with_capacity(procedure_trackables.len());

        let procedure_template_class = builder.style_class(
            StyleClassBuilder::default()
                .name("procedure_template")
                .map_err(FlowchartError::from)?
                .property(StyleProperty::Fill(Color::pastel_red()))
                .map_err(FlowchartError::from)?
                .property(StyleProperty::Stroke(Color::pastel_red().darken(20)))
                .map_err(FlowchartError::from)?,
        )?;

        for (trackable_index, (trackable, color)) in
            procedure_trackables.iter().zip(colors).enumerate()
        {
            let style_class = builder.style_class(
                StyleClassBuilder::default()
                    .name(format!("t{}", trackable_index))
                    .map_err(FlowchartError::from)?
                    .property(StyleProperty::Fill(color))
                    .map_err(FlowchartError::from)?
                    .property(StyleProperty::Stroke(color.darken(30)))
                    .map_err(FlowchartError::from)?,
            )?;
            trackable_classes.insert(trackable, style_class);
        }

        // We create a class for each trackable of the parent procedure.

        // We start creating the nodes for the procedure template.
        for subprocedure in ParentProcedureTemplate::from_parent(&self.procedure_template, conn)? {
            let child_procedure = subprocedure.child(conn)?;
            let _node = to_mermaid_node(
                self,
                &trackable_classes,
                &procedure_template_class,
                &child_procedure,
                &mut builder,
                parent_direction,
                conn,
            )?;
        }

        // Next, we chain add the edges representing which procedures are followed by
        // which.
        for subprocedure in NextProcedureTemplate::from_parent(&self.procedure_template, conn)? {
            let current_procedure = subprocedure.predecessor(conn)?;
            let next_procedure = subprocedure.successor(conn)?;
            builder.edge(
                FlowchartEdgeBuilder::default()
                    .source(
                        builder
                            .get_node_by_label(&current_procedure.name)
                            .expect("Current procedure node not found"),
                    )?
                    .destination(
                        builder
                            .get_node_by_label(&next_procedure.name)
                            .expect("Next procedure node not found"),
                    )?
                    .right_arrow_shape(ArrowShape::Normal)?,
            )?;
        }

        Ok(builder.into())
    }
}
