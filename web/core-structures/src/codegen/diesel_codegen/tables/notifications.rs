#[cfg(feature = "diesel")]
diesel::table! { public . notifications (id) { id -> diesel :: sql_types :: Integer , user_id -> diesel :: sql_types :: Integer , operation -> diesel :: sql_types :: Text , table_name -> diesel :: sql_types :: Text , record -> diesel :: sql_types :: Text , read -> diesel :: sql_types :: Bool } }
