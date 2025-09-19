impl
    From<crate::codegen::structs_codegen::tables::commercial_pipette_models::CommercialPipetteModel>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_pipette_models::CommercialPipetteModel,
    ) -> Self {
        super::Row::CommercialPipetteModel(value)
    }
}
impl From<super::Row>
    for Option<
        crate::codegen::structs_codegen::tables::commercial_pipette_models::CommercialPipetteModel,
    >
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::CommercialPipetteModel(v) => Some(v),
            _ => None,
        }
    }
}
