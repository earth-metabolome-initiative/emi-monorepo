impl From<crate::codegen::structs_codegen::tables::pipette_models::PipetteModel> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::pipette_models::PipetteModel) -> Self {
        super::Row::PipetteModel(value)
    }
}
impl From<super::Row>
    for Option<crate::codegen::structs_codegen::tables::pipette_models::PipetteModel>
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::PipetteModel(v) => Some(v),
            _ => None,
        }
    }
}
