#[cfg(feature = "diesel")]
diesel::table! { public . units (id) { name -> diesel :: sql_types :: Text , unit -> diesel :: sql_types :: Text , icon_id -> diesel :: sql_types :: SmallInt , color_id -> diesel :: sql_types :: SmallInt , id -> diesel :: sql_types :: SmallInt } }
