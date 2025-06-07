impl From<crate::codegen::structs_codegen::tables::parent_procedure_models::ParentProcedureModel>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::parent_procedure_models::ParentProcedureModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl
    From<
        Vec<crate::codegen::structs_codegen::tables::parent_procedure_models::ParentProcedureModel>,
    > for super::Rows
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::parent_procedure_models::ParentProcedureModel,
        >,
    ) -> Self {
        super::Rows::ParentProcedureModel(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::parent_procedure_models::ParentProcedureModel>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::ParentProcedureModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
