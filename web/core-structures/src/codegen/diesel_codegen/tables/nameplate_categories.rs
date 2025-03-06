#[cfg(feature = "diesel")]
diesel::table! { public . nameplate_categories (id) { name -> diesel :: sql_types :: Text , material_id -> diesel :: sql_types :: SmallInt , permanence -> diesel :: sql_types :: Text , description -> diesel :: sql_types :: Text , icon_id -> diesel :: sql_types :: SmallInt , color_id -> diesel :: sql_types :: SmallInt , id -> diesel :: sql_types :: SmallInt } }
