#[cfg(feature = "postgres")]
impl web_common_traits::prelude::InsertableBuilder
for crate::codegen::structs_codegen::tables::insertables::InsertableSampleContainerBuilder {
    type Row = crate::codegen::structs_codegen::tables::sample_containers::SampleContainer;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableSampleContainer;
}
