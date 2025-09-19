impl From<
    crate::codegen::structs_codegen::tables::commercial_pipette_tip_models::CommercialPipetteTipModel,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_pipette_tip_models::CommercialPipetteTipModel,
    ) -> Self {
        super::Row::CommercialPipetteTipModel(value)
    }
}
impl From<super::Row>
for Option<
    crate::codegen::structs_codegen::tables::commercial_pipette_tip_models::CommercialPipetteTipModel,
> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::CommercialPipetteTipModel(v) => Some(v),
            _ => None,
        }
    }
}
