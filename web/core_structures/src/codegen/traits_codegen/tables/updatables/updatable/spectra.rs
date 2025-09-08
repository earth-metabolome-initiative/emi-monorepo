impl<C: diesel::connection::LoadConnection> web_common_traits::database::Updatable<C>
    for crate::Spectrum
where
    crate::DigitalAsset: web_common_traits::database::Read<C>,
    crate::DigitalAsset: web_common_traits::database::Updatable<C, UserId = i32>,
    crate::SpectraCollection: web_common_traits::database::Read<C>,
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
