impl From<
    crate::codegen::structs_codegen::tables::commercial_pipette_tip_models::CommercialPipetteTipModel,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_pipette_tip_models::CommercialPipetteTipModel,
    ) -> Self {
        super::Row::CommercialPipetteTipModel(value)
    }
}
impl TryFrom<super::Row>
for crate::codegen::structs_codegen::tables::commercial_pipette_tip_models::CommercialPipetteTipModel {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::CommercialPipetteTipModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
