#[cfg(feature = "postgres")]
impl web_common_traits::prelude::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableObservationBuilder
{
    type Row = crate::codegen::structs_codegen::tables::observations::Observation;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableObservation;
}
