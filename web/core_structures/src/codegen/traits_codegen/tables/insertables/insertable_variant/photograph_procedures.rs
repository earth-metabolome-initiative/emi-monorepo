impl<
    C: diesel::connection::LoadConnection,
    Procedure,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureBuilder<
    Procedure,
>
where
    <C as diesel::Connection>::Backend: diesel::backend::DieselReserveSpecialization,
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::photograph_procedures::PhotographProcedure as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedure as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::photograph_procedures::PhotographProcedure as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::photograph_procedures::PhotographProcedure,
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
        Attributes = crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureAttributes,
    >,
{
    type Row = crate::codegen::structs_codegen::tables::photograph_procedures::PhotographProcedure;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedure;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureAttributes,
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
            "photograph_procedures",
        )?;
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedure = self
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
                    crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureAttributes::ProcedureTemplate,
                ),
            )?;
        let foreign_procedure_template = self
            .foreign_procedure_template
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureAttributes::ForeignProcedureTemplate,
                ),
            )?;
        let foreign_procedure = self
            .foreign_procedure
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureAttributes::ForeignProcedure,
                ),
            )?;
        let photographed_asset = self
            .photographed_asset
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureAttributes::PhotographedAsset,
                ),
            )?;
        let photographed_with_model = self
            .photographed_with_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureAttributes::PhotographedWithModel,
                ),
            )?;
        let procedure = self
            .procedure
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureAttributes::Extension(
                    crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureExtensionAttributes::Procedure(
                        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAttributes::Procedure,
                    ),
                ))
            })?;
        Ok(Self::InsertableVariant {
            procedure,
            procedure_template,
            foreign_procedure_template,
            foreign_procedure,
            photographed_asset,
            photographed_with: self.photographed_with,
            photographed_with_model,
        })
    }
}
