impl<
    C: diesel::connection::LoadConnection,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableAddressBuilder
where
    <C as diesel::Connection>::Backend: diesel::backend::DieselReserveSpecialization,
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::addresses::Address as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableAddress as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::addresses::Address as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::addresses::Address,
    >,
    C: diesel::connection::LoadConnection,
{
    type Row = crate::codegen::structs_codegen::tables::addresses::Address;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableAddress;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::AddressAttribute,
    >;
    type UserId = i32;
    fn insert(
        self,
        user_id: Self::UserId,
        conn: &mut C,
    ) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableAddress = self
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
        let city_id = self
            .city_id
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::AddressAttribute::CityId,
                ),
            )?;
        let street_name = self
            .street_name
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::AddressAttribute::StreetName,
                ),
            )?;
        let street_number = self
            .street_number
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::AddressAttribute::StreetNumber,
                ),
            )?;
        let postal_code = self
            .postal_code
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::AddressAttribute::PostalCode,
                ),
            )?;
        let geolocation = self
            .geolocation
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::AddressAttribute::Geolocation,
                ),
            )?;
        Ok(Self::InsertableVariant {
            city_id,
            street_name,
            street_number,
            postal_code,
            geolocation,
        })
    }
}
