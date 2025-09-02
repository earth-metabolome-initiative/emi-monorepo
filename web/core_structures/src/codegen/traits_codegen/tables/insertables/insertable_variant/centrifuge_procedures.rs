impl<
    C: diesel::connection::LoadConnection,
    Procedure,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureBuilder<
    Procedure,
>
where
    <C as diesel::Connection>::Backend: diesel::backend::DieselReserveSpecialization,
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::centrifuge_procedures::CentrifugeProcedure as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedure as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::centrifuge_procedures::CentrifugeProcedure as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::centrifuge_procedures::CentrifugeProcedure,
    >,
    C: diesel::connection::LoadConnection,
    Procedure: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = ::rosetta_uuid::Uuid,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureBuilder: web_common_traits::database::TryInsertGeneric<
        C,
        Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAttributes,
        PrimaryKey = ::rosetta_uuid::Uuid,
    >,
    Self: crate::codegen::structs_codegen::tables::insertables::ProcedureBuildable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureAttributes,
    >,
{
    type Row = crate::codegen::structs_codegen::tables::centrifuge_procedures::CentrifugeProcedure;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedure;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureAttributes,
    >;
    type UserId = i32;
    fn insert(
        mut self,
        user_id: Self::UserId,
        conn: &mut C,
    ) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        self = <Self as crate::codegen::structs_codegen::tables::insertables::ProcedureBuildable>::most_concrete_table(
            self,
            "centrifuge_procedures",
        )?;
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedure = self
            .try_insert(user_id, conn)?;
        Ok(
            diesel::insert_into(Self::Row::table())
                .values(insertable_struct)
                .get_result(conn)?,
        )
    }
    fn try_insert(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::InsertableVariant, Self::Error> {
        let procedure_template = self
            .procedure_template
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureAttributes::ProcedureTemplate,
                ),
            )?;
        let foreign_procedure_template = self
            .foreign_procedure_template
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureAttributes::ForeignProcedureTemplate,
                ),
            )?;
        let foreign_procedure = self
            .foreign_procedure
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureAttributes::ForeignProcedure,
                ),
            )?;
        let centrifuged_container = self
            .centrifuged_container
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureAttributes::CentrifugedContainer,
                ),
            )?;
        let centrifuged_with_model = self
            .centrifuged_with_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureAttributes::CentrifugedWithModel,
                ),
            )?;
        let procedure = self
            .procedure
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureAttributes::Extension(
                    crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureExtensionAttributes::Procedure(
                        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAttributes::Procedure,
                    ),
                ))
            })?;
        Ok(Self::InsertableVariant {
            procedure,
            procedure_template,
            foreign_procedure_template,
            foreign_procedure,
            centrifuged_container,
            centrifuged_with_model,
            centrifuged_with: self.centrifuged_with,
        })
    }
}
