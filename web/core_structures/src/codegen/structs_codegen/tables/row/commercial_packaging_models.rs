impl From<
    crate::codegen::structs_codegen::tables::commercial_packaging_models::CommercialPackagingModel,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_packaging_models::CommercialPackagingModel,
    ) -> Self {
        super::Row::CommercialPackagingModel(value)
    }
}
impl TryFrom<super::Row>
for crate::codegen::structs_codegen::tables::commercial_packaging_models::CommercialPackagingModel {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::CommercialPackagingModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
