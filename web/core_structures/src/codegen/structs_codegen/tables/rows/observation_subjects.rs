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
impl From<super::Rows>
    for Option<
        Vec<crate::codegen::structs_codegen::tables::observation_subjects::ObservationSubject>,
    >
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::ObservationSubject(v) => Some(v),
            _ => None,
        }
    }
}
