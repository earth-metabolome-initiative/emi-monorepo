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
impl TryFrom<super::Rows>
for Vec<
    crate::codegen::structs_codegen::tables::commercial_packaging_models::CommercialPackagingModel,
> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::CommercialPackagingModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
