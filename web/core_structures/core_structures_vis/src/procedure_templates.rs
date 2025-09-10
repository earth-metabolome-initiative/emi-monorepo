//! Submodule providing an illustration of a procedure and its subprocedures
//! using a Flowchart Diagram in Mermaid syntax.

use std::{collections::HashMap, rc::Rc};

use core_structures::{
    AssetModel, NextProcedureTemplate, ParentProcedureTemplate, ProcedureTemplate,
    ProcedureTemplateAssetModel,
};
use diesel::PgConnection;
use graph::{
    prelude::{DiEdgesBuilder, DiGraph, GenericMonoplexMonopartiteGraphBuilder},
    traits::{
        EdgesBuilder, MonopartiteGraph, MonopartiteGraphBuilder, MonoplexGraph,
        MonoplexGraphBuilder,
    },
};
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
use sorted_vec::prelude::SortedVec;
use web_common_traits::{
    prelude::Builder, procedures::ProcedureTemplate as ProcedureTemplateTrait,
};

use crate::MermaidDB;

fn asset_model_icon(asset_model: &AssetModel) -> Option<&'static str> {
    Some(match asset_model.most_concrete_table.as_str() {
        "digital_asset_models" => "fa:fa-file",
        "organism_models" => "fa:fa-bacterium",
        "phone_models" => "fa:fa-mobile-screen-button",
        "sample_models" => "fa:fa-vial",
        "container_models" => "fa:fa-box",
        "volumetric_container_models" => "fa:fa-flask-vial",
        "packaging_models" => "fa:fa-sheet-plastic",
        "freezer_models" => "fa:fa-snowflake",
        "freeze_dryer_models" => "fa:fa-icicles",
        "weighing_device_models" => "fa:fa-scale-unbalanced",
        "bead_models" => "fa:fa-drum",
        "pipette_models" => "fa:fa-eye-dropper",
        _ => return None,
    })
}

fn to_mermaid_node(
    metadata: &ProcedureTemplateMetadata,
    procedure_template_asset_model_nodes: &mut HashMap<
        ProcedureTemplateAssetModel,
        Rc<FlowchartNode>,
    >,
    ptam_classes: &HashMap<ProcedureTemplateAssetModel, (Rc<StyleClass>, Rc<StyleClass>)>,
    procedure_template_nodes: &mut HashMap<ProcedureTemplate, Rc<FlowchartNode>>,
    procedure_template_class: &Rc<StyleClass>,
    procedure_template: &ProcedureTemplate,
    builder: &mut FlowchartBuilder,
    parent_direction: Direction,
    conn: &mut PgConnection,
) -> Result<Rc<FlowchartNode>, crate::Error> {
    let mut node_builder = FlowchartNodeBuilder::default()
        .label(&procedure_template.name)?
        .style_class(procedure_template_class.clone())
        .map_err(FlowchartError::from)?
        .shape(FlowchartNodeShape::RoundEdges);

    if !metadata.has_subprocedures(procedure_template) {
        for procedure_template_asset_model in
            procedure_template.procedure_template_asset_models(conn)?
        {
            let label = if let Some(icon) =
                asset_model_icon(&procedure_template_asset_model.asset_model(conn)?)
            {
                format!("{} {}", icon, procedure_template_asset_model.name)
            } else {
                procedure_template_asset_model.name.clone()
            };

            let reference_ptam: &ProcedureTemplateAssetModel = metadata
                .root_reference_procedure_template_asset_model(&procedure_template_asset_model);
            let (node_style_class, edge_style_class) = &ptam_classes[reference_ptam];
            let procedure_template_asset_model_node_builder = FlowchartNodeBuilder::default()
                .label(&label)?
                .style_class(node_style_class.clone())
                .map_err(FlowchartError::from)?;

            let procedure_template_asset_model_node =
                builder.node(procedure_template_asset_model_node_builder)?;
            if !procedure_template_asset_model_nodes.contains_key(&procedure_template_asset_model) {
                procedure_template_asset_model_nodes.insert(
                    procedure_template_asset_model.clone(),
                    procedure_template_asset_model_node.clone(),
                );
            }
            if procedure_template_asset_model.procedure_template
                != procedure_template.procedure_template
            {
                builder.edge(
                    FlowchartEdgeBuilder::default()
                        .source(
                            procedure_template_asset_model_nodes
                                .get(&procedure_template_asset_model)
                                .unwrap()
                                .clone(),
                        )?
                        .destination(procedure_template_asset_model_node.clone())?
                        .line_style(LineStyle::Dashed)
                        .curve_style(CurveStyle::Natural)
                        .style_class(edge_style_class.clone())
                        .map_err(FlowchartError::from)?,
                )?;
            }
            node_builder = node_builder
                .subnode(procedure_template_asset_model_node)
                .map_err(FlowchartError::from)?;
        }
    } else {
        node_builder = node_builder.direction(parent_direction);
    }

    for subprocedure in metadata.subprocedures(procedure_template) {
        node_builder = node_builder.subnode(to_mermaid_node(
            metadata,
            procedure_template_asset_model_nodes,
            ptam_classes,
            procedure_template_nodes,
            procedure_template_class,
            &subprocedure,
            builder,
            parent_direction.flip(),
            conn,
        )?)?;
    }

    // Next, we chain add the edges representing which procedures are followed by
    // which.
    for subprocedure in
        NextProcedureTemplate::from_parent(procedure_template.procedure_template, conn)?
    {
        let current_procedure = subprocedure.predecessor(conn)?;
        let next_procedure = subprocedure.successor(conn)?;
        let source_node = &procedure_template_nodes[&current_procedure];
        let destination_node = &procedure_template_nodes[&next_procedure];
        builder.edge(
            FlowchartEdgeBuilder::default()
                .source(source_node.clone())?
                .destination(destination_node.clone())?
                .curve_style(CurveStyle::Natural)
                .right_arrow_shape(ArrowShape::Normal)?,
        )?;
    }

    let node = builder.node(node_builder)?;
    procedure_template_nodes.insert(procedure_template.clone(), node.clone());
    Ok(node)
}

/// Procedure structuring all metadata associated to a procedure template
pub struct ProcedureTemplateMetadata {
    /// The root procedure template
    root_procedure_template: ProcedureTemplate,
    /// All of the sub-procedure templates
    hierarchy_graph: DiGraph<ProcedureTemplate>,
    /// The foreign procedures referenced by some procedure template asset model
    /// which are not among either the root procedure template or any of its
    /// sub-procedure templates.
    foreign_procedure_templates: Vec<ProcedureTemplate>,
    /// The procedure template asset models associated to the root procedure
    /// template or any of its sub-procedure templates.
    procedure_template_asset_models: Vec<ProcedureTemplateAssetModel>,
}

fn load_subprocedure_templates(
    procedure_template: &ProcedureTemplate,
    conn: &mut PgConnection,
) -> Result<
    (Vec<ProcedureTemplate>, Vec<(ProcedureTemplate, ProcedureTemplate)>),
    diesel::result::Error,
> {
    let mut sub_procedures = Vec::new();
    let mut edges = Vec::new();

    for sub_procedure in
        ParentProcedureTemplate::from_parent(procedure_template.procedure_template, conn)?
    {
        let child_procedure = sub_procedure.child(conn)?;
        let (child_sub_procedures, child_edges) =
            load_subprocedure_templates(&child_procedure, conn)?;
        sub_procedures.push(child_procedure.clone());
        sub_procedures.extend(child_sub_procedures);
        edges.push((procedure_template.clone(), child_procedure));
        edges.extend(child_edges);
    }

    Ok((sub_procedures, edges))
}

impl ProcedureTemplateMetadata {
    /// Returns a reference to the root procedure template name.
    pub fn root_procedure_template_name(&self) -> &str {
        &self.root_procedure_template.name
    }

    /// Returns the root reference procedure template asset model.
    ///
    /// # Arguments
    ///
    /// * `procedure_template_asset_model` - The procedure template asset model
    ///   for which to find the reference.
    /// * `parent_procedure_template_trace` - The trace of parent procedure
    ///   templates from the root procedure template to the procedure template
    ///   of the provided procedure template asset model.
    fn root_reference_procedure_template_asset_model(
        &self,
        procedure_template_asset_model: &ProcedureTemplateAssetModel,
    ) -> &ProcedureTemplateAssetModel {
        if self.is_root_procedure_template_asset_model(procedure_template_asset_model)
            || self.is_foreign_procedure_template_asset_model(procedure_template_asset_model)
        {
            return self
                .procedure_template_asset_models
                .binary_search_by_key(&procedure_template_asset_model.id, |ptam| ptam.id)
                .map(|index| &self.procedure_template_asset_models[index])
                .expect("Procedure template asset model not found");
        }

        // We find the procedure template asset models associated with the last parent
        // in the trace.
        for ptam in self.procedure_template_asset_models.iter() {
            if ptam.based_on == Some(procedure_template_asset_model.id) {
                return self.root_reference_procedure_template_asset_model(ptam);
            }
        }

        unreachable!(
            "Should have found a reference procedure template asset model for the provided procedure template asset model \"{}\".",
            procedure_template_asset_model.name,
        );
    }

    /// Returns whether the provided procedure template identifier is foreign.
    fn is_foreign_procedure_template_id(&self, id: i32) -> bool {
        self.foreign_procedure_templates
            .binary_search_by_key(&id, |pt| pt.procedure_template)
            .is_ok()
    }

    /// Returns whether the provided procedure template has any sub-procedures.
    fn has_subprocedures(&self, procedure_template: &ProcedureTemplate) -> bool {
        self.hierarchy_graph.has_successors(
            self.hierarchy_graph
                .nodes_vocabulary()
                .binary_search(&procedure_template)
                .expect("Procedure template should be in the graph"),
        )
    }

    /// Returns an iterator over the sub-procedures of the provided procedure
    /// template.
    fn subprocedures(
        &self,
        procedure_template: &ProcedureTemplate,
    ) -> impl Iterator<Item = &ProcedureTemplate> {
        self.hierarchy_graph
            .successors(
                self.hierarchy_graph
                    .nodes_vocabulary()
                    .binary_search(&procedure_template)
                    .expect("Procedure template should be in the graph"),
            )
            .map(|index| &self.hierarchy_graph.nodes_vocabulary()[index])
    }

    /// Returns whether the provided procedure template asset model is from a
    /// foreign procedure template.
    fn is_foreign_procedure_template_asset_model(
        &self,
        ptam: &ProcedureTemplateAssetModel,
    ) -> bool {
        self.is_foreign_procedure_template_id(ptam.procedure_template)
    }

    /// Returns whether the provided procedure template asset model is from the
    /// root procedure template.
    fn is_root_procedure_template_asset_model(&self, ptam: &ProcedureTemplateAssetModel) -> bool {
        ptam.procedure_template == self.root_procedure_template.procedure_template
    }

    /// Returns an iterator over all procedure template asset models associated
    /// with the provided foreign procedure template.
    fn foreign_procedure_assets(
        &self,
        foreign_procedure_template: &ProcedureTemplate,
    ) -> impl Iterator<Item = &ProcedureTemplateAssetModel> {
        self.procedure_template_asset_models.iter().filter(move |ptam| {
            ptam.procedure_template == foreign_procedure_template.procedure_template
        })
    }

    /// Returns an iterator over all unique procedure template asset models
    /// associated with either the root procedure template or any of its
    /// sub-procedure templates.
    pub fn unique_procedure_template_asset_models(
        &self,
    ) -> impl Iterator<Item = &ProcedureTemplateAssetModel> {
        self.procedure_template_asset_models.iter().filter(|ptam| {
            self.is_root_procedure_template_asset_model(ptam)
                || self.is_foreign_procedure_template_asset_model(ptam)
        })
    }

    /// Returns the number of unique procedure template asset models associated.
    pub fn number_of_unique_procedure_template_asset_models(&self) -> u16 {
        let mut number_of_unique = 0;
        for procedure_template_asset_model in &self.procedure_template_asset_models {
            // If the identifier of the procedure template asset model is
            // foreign, we do not follow the `based_on` chain.
            if self.is_foreign_procedure_template_asset_model(procedure_template_asset_model)
                || self.is_root_procedure_template_asset_model(procedure_template_asset_model)
            {
                number_of_unique += 1;
            }
        }
        number_of_unique
    }

    /// Constructs a new `ProcedureTemplateMetadata` instance for the given
    /// root procedure template.
    pub fn new(
        root_procedure_template: ProcedureTemplate,
        conn: &mut PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        let (mut subprocedures, edges) =
            load_subprocedure_templates(&root_procedure_template, conn)?;
        subprocedures.push(root_procedure_template.clone());
        subprocedures.sort_unstable();
        subprocedures.dedup();
        let sorted_subprocedures = SortedVec::try_from(subprocedures.clone())
            .expect("Subprocedures should be unique and sorted");
        let mut edges: Vec<(usize, usize)> = edges
            .into_iter()
            .map(|(parent, child)| {
                let parent_index = sorted_subprocedures.binary_search(&parent).expect(&format!(
                    "Parent procedure \"{}\" should be in subprocedures",
                    parent.name
                ));
                let child_index = sorted_subprocedures.binary_search(&child).expect(&format!(
                    "Child procedure \"{}\" should be in subprocedures",
                    child.name
                ));
                (parent_index, child_index)
            })
            .collect();
        edges.sort_unstable();
        edges.dedup();
        let edges = DiEdgesBuilder::default()
            .expected_shape(sorted_subprocedures.len())
            .edges(edges)
            .build()
            .expect("Failed to build edges for the column same-as network");

        let mut foreign_procedure_templates = Vec::new();
        let mut procedure_template_asset_models = Vec::new();

        for sub_procedure in sorted_subprocedures.iter().chain(Some(&root_procedure_template)) {
            for procedure_template_asset_model in
                sub_procedure.procedure_template_asset_models(conn)?
            {
                if !sorted_subprocedures.iter().any(|s| {
                    s.procedure_template == procedure_template_asset_model.procedure_template
                }) {
                    let foreign_procedure_template =
                        procedure_template_asset_model.procedure_template(conn)?;
                    foreign_procedure_templates.push(foreign_procedure_template);
                }
                procedure_template_asset_models.push(procedure_template_asset_model);
            }
        }

        foreign_procedure_templates.sort_unstable_by_key(|pt| pt.procedure_template);
        foreign_procedure_templates.dedup_by_key(|pt| pt.procedure_template);
        procedure_template_asset_models.sort_unstable_by_key(|ptam| ptam.id);
        procedure_template_asset_models.dedup_by_key(|ptam| ptam.id);

        let hierarchy_graph: DiGraph<ProcedureTemplate> =
            GenericMonoplexMonopartiteGraphBuilder::default()
                .nodes(sorted_subprocedures.clone())
                .edges(edges)
                .build()
                .unwrap();

        Ok(Self {
            root_procedure_template,
            hierarchy_graph,
            foreign_procedure_templates,
            procedure_template_asset_models,
        })
    }
}

impl MermaidDB<PgConnection> for ProcedureTemplate {
    type Diagram = Flowchart;
    type Error = crate::Error;

    fn to_mermaid(&self, conn: &mut PgConnection) -> Result<Self::Diagram, Self::Error> {
        let parent_direction = Direction::TopToBottom;
        let metadata = ProcedureTemplateMetadata::new(self.clone(), conn)?;

        let mut builder = FlowchartBuilder::default().configuration(
            FlowchartConfigurationBuilder::default()
                .renderer(Renderer::EclipseLayoutKernel)
                .direction(parent_direction)
                .title(&metadata.root_procedure_template_name())?,
        )?;

        let colors = Color::maximally_distinct(
            metadata.number_of_unique_procedure_template_asset_models(),
            70,
            80,
        );

        let procedure_template_class = builder.style_class(
            StyleClassBuilder::default()
                .name("procedure_template")
                .map_err(FlowchartError::from)?
                .property(StyleProperty::Fill(Color::pastel_red()))
                .map_err(FlowchartError::from)?
                .property(StyleProperty::Stroke(Color::pastel_red().darken(20)))
                .map_err(FlowchartError::from)?,
        )?;

        let foreign_procedure_template_class = builder.style_class(
            StyleClassBuilder::default()
                .name("foreign_procedure_template")
                .map_err(FlowchartError::from)?
                .property(StyleProperty::StrokeDasharray(5, 5))
                .map_err(FlowchartError::from)?,
        )?;

        let mut ptam_classes = HashMap::new();
        for (procedure_template_asset_model, color) in
            metadata.unique_procedure_template_asset_models().zip(colors)
        {
            let stroke_color = color.darken(40);
            let mut node_style_class_builder = StyleClassBuilder::default()
                .name(format!("ptam_node_{}", procedure_template_asset_model.id))
                .map_err(FlowchartError::from)?
                .property(StyleProperty::Fill(color))
                .map_err(FlowchartError::from)?
                .property(StyleProperty::Stroke(stroke_color))
                .map_err(FlowchartError::from)?;

            if metadata.is_foreign_procedure_template_asset_model(procedure_template_asset_model) {
                node_style_class_builder = node_style_class_builder
                    .property(StyleProperty::StrokeDasharray(5, 5))
                    .map_err(FlowchartError::from)?;
            }

            let node_style_class = builder.style_class(node_style_class_builder)?;
            let edge_style_class = builder.style_class(
                StyleClassBuilder::default()
                    .name(format!("ptam_edge_{}", procedure_template_asset_model.id))
                    .map_err(FlowchartError::from)?
                    .property(StyleProperty::Stroke(stroke_color))
                    .map_err(FlowchartError::from)?,
            )?;
            ptam_classes.insert(
                procedure_template_asset_model.clone(),
                (node_style_class, edge_style_class),
            );
        }

        let mut procedure_template_asset_model_nodes = HashMap::new();
        for foreign_procedure_template in &metadata.foreign_procedure_templates {
            let mut foreign_procedure_template_node_builder = FlowchartNodeBuilder::default()
                .label(&foreign_procedure_template.name)?
                .shape(FlowchartNodeShape::RoundEdges)
                .style_class(foreign_procedure_template_class.clone())
                .map_err(FlowchartError::from)?;
            for foreign_ptam in metadata.foreign_procedure_assets(foreign_procedure_template) {
                let foreign_ptam_node_builder = FlowchartNodeBuilder::default()
                    .label(&foreign_ptam.name)?
                    .shape(FlowchartNodeShape::LRParallelogram)
                    .style_class(ptam_classes[foreign_ptam].clone().0)
                    .map_err(FlowchartError::from)?;
                let foreign_ptam_node = builder.node(foreign_ptam_node_builder)?;
                procedure_template_asset_model_nodes
                    .insert(foreign_ptam.clone(), foreign_ptam_node.clone());
                foreign_procedure_template_node_builder = foreign_procedure_template_node_builder
                    .subnode(foreign_ptam_node)
                    .map_err(FlowchartError::from)?;
            }
            let _ = builder.node(foreign_procedure_template_node_builder)?;
        }

        let mut procedure_template_nodes = HashMap::new();
        let _root_node = to_mermaid_node(
            &metadata,
            &mut procedure_template_asset_model_nodes,
            &ptam_classes,
            &mut procedure_template_nodes,
            &procedure_template_class,
            &metadata.root_procedure_template,
            &mut builder,
            parent_direction.flip(),
            conn,
        )?;

        Ok(builder.into())
    }
}
