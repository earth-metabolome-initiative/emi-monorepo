impl From<
    crate::codegen::structs_codegen::tables::procedure_model_nameplate_categories::ProcedureModelNameplateCategory,
> for super::Rows {
    fn from(
        value: crate::codegen::structs_codegen::tables::procedure_model_nameplate_categories::ProcedureModelNameplateCategory,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::procedure_model_nameplate_categories::ProcedureModelNameplateCategory,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::procedure_model_nameplate_categories::ProcedureModelNameplateCategory,
        >,
    ) -> Self {
        super::Rows::ProcedureModelNameplateCategory(value)
    }
}
impl TryFrom<super::Rows>
for Vec<
    crate::codegen::structs_codegen::tables::procedure_model_nameplate_categories::ProcedureModelNameplateCategory,
> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::ProcedureModelNameplateCategory(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
