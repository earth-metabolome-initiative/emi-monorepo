#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableVolumetricProcessableBuilder
{
    type Row =
        crate::codegen::structs_codegen::tables::volumetric_processables::VolumetricProcessable;
    type Product =
        crate::codegen::structs_codegen::tables::insertables::InsertableVolumetricProcessable;
}
