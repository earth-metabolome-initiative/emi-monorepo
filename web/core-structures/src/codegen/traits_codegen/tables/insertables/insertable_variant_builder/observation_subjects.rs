#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableObservationSubjectBuilder
{
    type Row = crate::codegen::structs_codegen::tables::observation_subjects::ObservationSubject;
    type Product =
        crate::codegen::structs_codegen::tables::insertables::InsertableObservationSubject;
}
