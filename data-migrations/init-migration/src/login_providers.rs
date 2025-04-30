//! Login providers migration

use diesel_async::AsyncPgConnection;

pub(crate) async fn init_login_providers(
    _portal_conn: &mut AsyncPgConnection,
) -> Result<(), crate::error::Error> {
    todo!();
}
