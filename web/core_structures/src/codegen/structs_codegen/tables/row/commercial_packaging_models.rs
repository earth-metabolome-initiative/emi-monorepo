impl From<
    crate::codegen::structs_codegen::tables::commercial_packaging_models::CommercialPackagingModel,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_packaging_models::CommercialPackagingModel,
    ) -> Self {
        super::Row::CommercialPackagingModel(value)
    }
}
impl From<super::Row>
for Option<
    crate::codegen::structs_codegen::tables::commercial_packaging_models::CommercialPackagingModel,
> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::CommercialPackagingModel(v) => Some(v),
            _ => None,
        }
    }
}
