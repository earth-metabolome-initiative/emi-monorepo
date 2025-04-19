#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::observation_subjects::ObservationSubject
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableObservationSubject;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableObservationSubjectBuilder;
}
