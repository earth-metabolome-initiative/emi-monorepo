#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Insertable
    for crate::codegen::structs_codegen::tables::observations::Observation
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableObservation;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableObservationBuilder;
}
