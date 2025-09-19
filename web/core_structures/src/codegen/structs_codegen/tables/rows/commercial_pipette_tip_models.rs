impl From<
    crate::codegen::structs_codegen::tables::commercial_pipette_tip_models::CommercialPipetteTipModel,
> for super::Rows {
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_pipette_tip_models::CommercialPipetteTipModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::commercial_pipette_tip_models::CommercialPipetteTipModel,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::commercial_pipette_tip_models::CommercialPipetteTipModel,
        >,
    ) -> Self {
        super::Rows::CommercialPipetteTipModel(value)
    }
}
impl From<super::Rows>
for Option<
    Vec<
        crate::codegen::structs_codegen::tables::commercial_pipette_tip_models::CommercialPipetteTipModel,
    >,
> {
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::CommercialPipetteTipModel(v) => Some(v),
            _ => None,
        }
    }
}
