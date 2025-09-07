impl web_common_traits::database::Insertable for crate::SpatialRefSy {
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableSpatialRefSyBuilder;
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableSpatialRefSy;
}
