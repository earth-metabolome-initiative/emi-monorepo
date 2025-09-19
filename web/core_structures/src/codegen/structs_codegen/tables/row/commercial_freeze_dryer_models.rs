impl From<
    crate::codegen::structs_codegen::tables::commercial_freeze_dryer_models::CommercialFreezeDryerModel,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_freeze_dryer_models::CommercialFreezeDryerModel,
    ) -> Self {
        super::Row::CommercialFreezeDryerModel(value)
    }
}
impl From<super::Row>
for Option<
    crate::codegen::structs_codegen::tables::commercial_freeze_dryer_models::CommercialFreezeDryerModel,
> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::CommercialFreezeDryerModel(v) => Some(v),
            _ => None,
        }
    }
}
