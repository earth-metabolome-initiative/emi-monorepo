impl<
    C: diesel::connection::LoadConnection,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableColorBuilder
where
    <C as diesel::Connection>::Backend: diesel::backend::DieselReserveSpecialization,
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::colors::Color as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableColor as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::colors::Color as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::colors::Color,
    >,
    C: diesel::connection::LoadConnection,
{
    type Row = crate::codegen::structs_codegen::tables::colors::Color;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableColor;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::ColorAttribute,
    >;
    type UserId = i32;
    fn insert(
        self,
        user_id: Self::UserId,
        conn: &mut C,
    ) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableColor = self
            .try_insert(user_id, conn)?;
        Ok(
            diesel::insert_into(Self::Row::table())
                .values(insertable_struct)
                .get_result(conn)?,
        )
    }
    fn try_insert(
        self,
        _user_id: i32,
        _conn: &mut C,
    ) -> Result<Self::InsertableVariant, Self::Error> {
        let name = self
            .name
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::ColorAttribute::Name,
                ),
            )?;
        let hexadecimal_value = self
            .hexadecimal_value
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::ColorAttribute::HexadecimalValue,
                ),
            )?;
        let description = self
            .description
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::ColorAttribute::Description,
                ),
            )?;
        Ok(Self::InsertableVariant {
            name,
            hexadecimal_value,
            description,
        })
    }
}
