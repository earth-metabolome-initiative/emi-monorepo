#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::volumetric_processables::VolumetricProcessable
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableVolumetricProcessable;
    type InsertableBuilder = crate::codegen::structs_codegen::tables::insertables::InsertableVolumetricProcessableBuilder;
}
