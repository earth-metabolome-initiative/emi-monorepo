//! Submodule providing a method returning the procedure template asset
//! models [`StyleClass`] objects.

use core_structures::ProcedureTemplateAssetModel;
use mermaid::{
    prelude::{Color, FlowchartBuilder, StyleClassBuilder, StyleProperty},
    traits::DiagramBuilder,
};

pub(super) fn ptam_node_class_name(ptam: &ProcedureTemplateAssetModel) -> String {
    format!("ptam_node_{}", ptam.id)
}

pub(super) fn ptam_edge_class_name(ptam: &ProcedureTemplateAssetModel) -> String {
    format!("ptam_edge_{}", ptam.id)
}

pub(super) fn ptam_classes<'graph, I>(
    ptams: I,
    builder: &mut FlowchartBuilder,
) -> Result<(), mermaid::FlowchartError>
where
    I: Iterator<Item = &'graph ProcedureTemplateAssetModel>,
{
    let mut ptam_builders = Vec::new();
    for procedure_template_asset_model in ptams {
        ptam_builders.push((
            StyleClassBuilder::default()
                .name(ptam_node_class_name(procedure_template_asset_model))
                .unwrap(),
            StyleClassBuilder::default()
                .name(ptam_edge_class_name(procedure_template_asset_model))
                .unwrap(),
        ));
    }

    let colors = Color::maximally_distinct(ptam_builders.len() as u16, 70, 80);

    for ((node_builder, edge_builder), color) in ptam_builders.into_iter().zip(colors.into_iter()) {
        builder.style_class(
            node_builder
                .property(StyleProperty::Fill(color))
                .unwrap()
                .property(StyleProperty::Stroke(color.darken(30)))
                .unwrap(),
        )?;
        builder
            .style_class(edge_builder.property(StyleProperty::Stroke(color.darken(40))).unwrap())?;
    }

    Ok(())
}
