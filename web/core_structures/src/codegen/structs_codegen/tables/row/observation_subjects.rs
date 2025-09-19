impl From<crate::codegen::structs_codegen::tables::observation_subjects::ObservationSubject>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::observation_subjects::ObservationSubject,
    ) -> Self {
        super::Row::ObservationSubject(value)
    }
}
impl From<super::Row>
    for Option<crate::codegen::structs_codegen::tables::observation_subjects::ObservationSubject>
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::ObservationSubject(v) => Some(v),
            _ => None,
        }
    }
}
