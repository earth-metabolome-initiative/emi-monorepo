impl
    From<crate::codegen::structs_codegen::tables::commercial_freezer_models::CommercialFreezerModel>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_freezer_models::CommercialFreezerModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::commercial_freezer_models::CommercialFreezerModel,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::commercial_freezer_models::CommercialFreezerModel,
        >,
    ) -> Self {
        super::Rows::CommercialFreezerModel(value)
    }
}
impl From<super::Rows>
for Option<
    Vec<
        crate::codegen::structs_codegen::tables::commercial_freezer_models::CommercialFreezerModel,
    >,
> {
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::CommercialFreezerModel(v) => Some(v),
            _ => None,
        }
    }
}
