#[cfg(feature = "diesel")]
diesel::table! { public . countries (id) { iso -> diesel :: sql_types :: Text , emoji -> diesel :: sql_types :: Text , unicode -> diesel :: sql_types :: Text , name -> diesel :: sql_types :: Text , id -> diesel :: sql_types :: SmallInt } }
