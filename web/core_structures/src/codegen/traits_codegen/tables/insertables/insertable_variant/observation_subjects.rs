impl web_common_traits::database::DispatchableInsertVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableObservationSubjectBuilder
{
    type Row = crate::codegen::structs_codegen::tables::observation_subjects::ObservationSubject;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::ObservationSubjectAttribute,
    >;
}
impl web_common_traits::database::InsertableVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableObservationSubjectBuilder
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableObservationSubject;
}
#[cfg(feature = "backend")]
impl web_common_traits::database::BackendInsertableVariant
    for crate::codegen::structs_codegen::tables::insertables::InsertableObservationSubjectBuilder
where
    Self: web_common_traits::database::DispatchableInsertableVariant<diesel::PgConnection>,
{
}
impl<
    C: diesel::connection::LoadConnection,
> web_common_traits::database::DispatchableInsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableObservationSubjectBuilder
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::observation_subjects::ObservationSubject as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableObservationSubject as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::observation_subjects::ObservationSubject as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::observation_subjects::ObservationSubject,
    >,
    Self: web_common_traits::database::InsertableVariant<
        C,
        InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableObservationSubject,
        Row = crate::codegen::structs_codegen::tables::observation_subjects::ObservationSubject,
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::ObservationSubjectAttribute,
        >,
    >,
{
    fn insert(self, user_id: i32, conn: &mut C) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::InsertableVariant;
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableObservationSubject = self
            .try_insert(user_id, conn)?;
        Ok(
            diesel::insert_into(Self::table())
                .values(insertable_struct)
                .get_result(conn)?,
        )
    }
}
impl<
    C: diesel::connection::LoadConnection,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableObservationSubjectBuilder
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::observation_subjects::ObservationSubject as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableObservationSubject as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::observation_subjects::ObservationSubject as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::observation_subjects::ObservationSubject,
    >,
{
    fn try_insert(
        self,
        _user_id: i32,
        _conn: &mut C,
    ) -> Result<Self::InsertableVariant, Self::Error> {
        let name = self
            .name
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::ObservationSubjectAttribute::Name,
                ),
            )?;
        let description = self
            .description
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::ObservationSubjectAttribute::Description,
                ),
            )?;
        let icon = self
            .icon
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::ObservationSubjectAttribute::Icon,
                ),
            )?;
        let color_id = self
            .color_id
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::ObservationSubjectAttribute::ColorId,
                ),
            )?;
        Ok(Self::InsertableVariant {
            name,
            description,
            icon,
            color_id,
        })
    }
}
