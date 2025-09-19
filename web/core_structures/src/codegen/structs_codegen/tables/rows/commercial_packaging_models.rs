impl From<
    crate::codegen::structs_codegen::tables::commercial_packaging_models::CommercialPackagingModel,
> for super::Rows {
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_packaging_models::CommercialPackagingModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::commercial_packaging_models::CommercialPackagingModel,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::commercial_packaging_models::CommercialPackagingModel,
        >,
    ) -> Self {
        super::Rows::CommercialPackagingModel(value)
    }
}
impl From<super::Rows>
for Option<
    Vec<
        crate::codegen::structs_codegen::tables::commercial_packaging_models::CommercialPackagingModel,
    >,
> {
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::CommercialPackagingModel(v) => Some(v),
            _ => None,
        }
    }
}
