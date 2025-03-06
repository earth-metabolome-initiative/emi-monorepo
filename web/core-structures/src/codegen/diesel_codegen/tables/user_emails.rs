#[cfg(feature = "diesel")]
diesel::table! { public . user_emails (id) { id -> diesel :: sql_types :: Integer , email -> diesel :: sql_types :: Text , created_by -> diesel :: sql_types :: Integer , login_provider_id -> diesel :: sql_types :: SmallInt , created_at -> diesel :: sql_types :: Timestamp , primary_email -> diesel :: sql_types :: Bool } }
