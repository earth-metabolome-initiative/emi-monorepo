impl web_common_traits::database::InsertableVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableAddressBuilder
{
    type Row = crate::codegen::structs_codegen::tables::addresses::Address;
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableAddress;
    type UserId = i32;
}
impl<
    C: diesel::connection::LoadConnection,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableAddressBuilder
where
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
{
    fn insert(
        self,
        user_id: Self::UserId,
        conn: &mut C,
    ) -> Result<
        Self::Row,
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::AddressAttribute,
        >,
    > {
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
    ) -> Result<
        Self::InsertableVariant,
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::AddressAttribute,
        >,
    > {
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
        let house_number = self
            .house_number
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::AddressAttribute::HouseNumber,
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
            house_number,
            postal_code,
            geolocation,
        })
    }
}
