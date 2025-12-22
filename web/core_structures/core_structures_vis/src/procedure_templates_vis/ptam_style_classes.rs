//! Submodule providing a method returning the procedure template asset
//! models [`StyleClass`] objects.

use core_structures::ProcedureTemplateAssetModel;
use guided_procedures::{OwnershipLike, ProcedureTemplateGraph};
use mermaid::{
    prelude::{Color, FlowchartBuilder, StyleClassBuilder, StyleProperty, Unit},
    traits::DiagramBuilder,
};

pub(super) fn ptam_node_class_name(ptam: &ProcedureTemplateAssetModel) -> String {
    format!("ptam_node_{}", ptam.id)
}

pub(super) fn ptam_edge_class_name(ptam: &ProcedureTemplateAssetModel) -> String {
    format!("ptam_edge_{}", ptam.id)
}

pub(super) fn ptam_classes<'graph, I>(
    graph: &'graph ProcedureTemplateGraph,
    ptams: I,
    builder: &mut FlowchartBuilder,
) -> Result<(), mermaid::FlowchartError>
where
    I: Iterator<Item = &'graph ProcedureTemplateAssetModel>,
{
    let mut ptam_builders = Vec::new();
    for procedure_template_asset_model_id in ptams {
        let mut node = StyleClassBuilder::default()
            .name(ptam_node_class_name(procedure_template_asset_model_id))
            .unwrap()
            .property(StyleProperty::BorderRadius(Unit::Pixel(2)))
            .unwrap();
        let mut edge = StyleClassBuilder::default()
            .name(ptam_edge_class_name(procedure_template_asset_model_id))
            .unwrap();
        if graph.foreign_owned_ptam(procedure_template_asset_model_id) {
            node = node.property(StyleProperty::StrokeDasharray(5, 5)).unwrap();
            edge = edge.property(StyleProperty::StrokeDasharray(5, 5)).unwrap();
        }
        ptam_builders.push((node, edge));
    }

    let colors = Color::maximally_distinct(u16::try_from(ptam_builders.len()).unwrap(), 70, 80);

    for ((node_builder, edge_builder), color) in ptam_builders.into_iter().zip(colors.into_iter()) {
        builder.style_class(
            node_builder
                .property(StyleProperty::Fill(color))
                .unwrap()
                .property(StyleProperty::Stroke(color.darken(30)))
                .unwrap(),
        )?;
        builder.style_class(
            edge_builder
                .property(StyleProperty::Stroke(color.darken(30)))
                .unwrap()
                .property(StyleProperty::StrokeWidth(Unit::Pixel(2)))
                .unwrap(),
        )?;
    }

    Ok(())
}
