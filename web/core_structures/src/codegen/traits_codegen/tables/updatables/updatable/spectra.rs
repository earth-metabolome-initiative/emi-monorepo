impl<C: diesel::connection::LoadConnection> web_common_traits::database::Updatable<C>
for crate::Spectrum
where
    crate::DigitalAsset: diesel::Identifiable,
    <crate::DigitalAsset as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
        <crate::DigitalAsset as diesel::Identifiable>::Id,
    >,
    <<crate::DigitalAsset as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::DigitalAsset as diesel::Identifiable>::Id,
    >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
    <<<crate::DigitalAsset as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::DigitalAsset as diesel::Identifiable>::Id,
    >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
        'a,
        C,
        crate::DigitalAsset,
    >,
    crate::DigitalAsset: web_common_traits::database::Updatable<C, UserId = i32>,
    crate::SpectraCollection: diesel::Identifiable,
    <crate::SpectraCollection as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
        <crate::SpectraCollection as diesel::Identifiable>::Id,
    >,
    <<crate::SpectraCollection as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::SpectraCollection as diesel::Identifiable>::Id,
    >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
    <<<crate::SpectraCollection as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::SpectraCollection as diesel::Identifiable>::Id,
    >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
        'a,
        C,
        crate::SpectraCollection,
    >,
    crate::SpectraCollection: web_common_traits::database::Updatable<C, UserId = i32>,
{
    type UserId = i32;
    fn can_update(
        &self,
        user_id: Self::UserId,
        conn: &mut C,
    ) -> Result<bool, diesel::result::Error> {
        if !self.id(conn)?.can_update(user_id, conn)? {
            return Ok(false);
        }
        if !self.spectra_collection(conn)?.can_update(user_id, conn)? {
            return Ok(false);
        }
        Ok(true)
    }
}
