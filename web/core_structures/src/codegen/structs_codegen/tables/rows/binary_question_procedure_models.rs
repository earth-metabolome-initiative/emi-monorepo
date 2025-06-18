impl From<
    crate::codegen::structs_codegen::tables::binary_question_procedure_models::BinaryQuestionProcedureModel,
> for super::Rows {
    fn from(
        value: crate::codegen::structs_codegen::tables::binary_question_procedure_models::BinaryQuestionProcedureModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::binary_question_procedure_models::BinaryQuestionProcedureModel,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::binary_question_procedure_models::BinaryQuestionProcedureModel,
        >,
    ) -> Self {
        super::Rows::BinaryQuestionProcedureModel(value)
    }
}
impl TryFrom<super::Rows>
for Vec<
    crate::codegen::structs_codegen::tables::binary_question_procedure_models::BinaryQuestionProcedureModel,
> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::BinaryQuestionProcedureModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
