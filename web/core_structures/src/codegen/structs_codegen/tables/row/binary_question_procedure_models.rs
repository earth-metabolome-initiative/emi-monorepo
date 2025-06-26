impl From<
    crate::codegen::structs_codegen::tables::binary_question_procedure_models::BinaryQuestionProcedureModel,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::binary_question_procedure_models::BinaryQuestionProcedureModel,
    ) -> Self {
        super::Row::BinaryQuestionProcedureModel(value)
    }
}
impl TryFrom<super::Row>
for crate::codegen::structs_codegen::tables::binary_question_procedure_models::BinaryQuestionProcedureModel {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::BinaryQuestionProcedureModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
