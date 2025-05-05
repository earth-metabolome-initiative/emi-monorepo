impl From<crate::codegen::structs_codegen::tables::observation_subjects::ObservationSubject>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::observation_subjects::ObservationSubject,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::observation_subjects::ObservationSubject>>
    for super::Rows
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::observation_subjects::ObservationSubject,
        >,
    ) -> Self {
        super::Rows::ObservationSubject(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::observation_subjects::ObservationSubject>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::ObservationSubject(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
