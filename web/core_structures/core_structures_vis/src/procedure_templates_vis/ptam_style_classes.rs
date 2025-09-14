//! Submodule providing a method returning the procedure template asset
//! models [`StyleClass`] objects.

use core_structures::ProcedureTemplateAssetModel;
use mermaid::{
    prelude::{Color, FlowchartBuilder, StyleClassBuilder, StyleProperty},
    traits::DiagramBuilder,
};
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
                .name(format!("ptam_node_{}", procedure_template_asset_model.id))
                .unwrap(),
            StyleClassBuilder::default()
                .name(format!("ptam_edge_{}", procedure_template_asset_model.id))
                .unwrap(),
        ));
    }

    let colors = Color::maximally_distinct(ptam_builders.len() as u16, 70, 80);

    for ((node_builder, edge_builder), color) in ptam_builders.into_iter().zip(colors.into_iter()) {
        let stroke_color = color.darken(30);
        builder.style_class(
            node_builder
                .property(StyleProperty::Fill(color))
                .unwrap()
                .property(StyleProperty::Stroke(stroke_color))
                .unwrap(),
        )?;
        builder.style_class(edge_builder.property(StyleProperty::Stroke(stroke_color)).unwrap())?;
    }

    Ok(())
}
