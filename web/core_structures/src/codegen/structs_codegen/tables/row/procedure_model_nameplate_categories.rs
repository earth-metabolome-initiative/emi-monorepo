impl From<
    crate::codegen::structs_codegen::tables::procedure_model_nameplate_categories::ProcedureModelNameplateCategory,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::procedure_model_nameplate_categories::ProcedureModelNameplateCategory,
    ) -> Self {
        super::Row::ProcedureModelNameplateCategory(value)
    }
}
impl TryFrom<super::Row>
for crate::codegen::structs_codegen::tables::procedure_model_nameplate_categories::ProcedureModelNameplateCategory {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::ProcedureModelNameplateCategory(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
