impl From<crate::codegen::structs_codegen::tables::pipette_models::PipetteModel> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::pipette_models::PipetteModel) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::pipette_models::PipetteModel>>
    for super::Rows
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::pipette_models::PipetteModel>,
    ) -> Self {
        super::Rows::PipetteModel(value)
    }
}
impl From<super::Rows>
    for Option<Vec<crate::codegen::structs_codegen::tables::pipette_models::PipetteModel>>
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::PipetteModel(v) => Some(v),
            _ => None,
        }
    }
}
