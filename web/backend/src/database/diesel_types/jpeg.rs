use diesel::serialize::{Output, ToSql, WriteTuple};
use diesel::{
    backend::Backend,
    deserialize::{FromSql, FromSqlRow},
    expression::AsExpression,
};
use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    FromSqlRow,
    AsExpression,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Default,
    Serialize,
    Deserialize,
)]
#[diesel(sql_type = crate::database::sql_type_bindings::Jpeg)]
pub struct JPEG(Vec<u8>);

impl ToSql<crate::database::sql_type_bindings::Jpeg, diesel::pg::Pg> for JPEG {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, diesel::pg::Pg>) -> diesel::serialize::Result {
        WriteTuple::<(diesel::sql_types::Bytea,)>::write_tuple(
            &(self.0.clone(),),
            &mut out.reborrow(),
        )
    }
}

impl FromSql<crate::database::sql_type_bindings::Jpeg, diesel::pg::Pg> for JPEG {
    fn from_sql(
        bytes: <diesel::pg::Pg as Backend>::RawValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
        let (data,) = <(Vec<u8>,) as FromSql<
            diesel::sql_types::Record<(diesel::sql_types::Bytea,)>,
            diesel::pg::Pg,
        >>::from_sql(bytes)?;
        // TODO! VALIDATION HERE
        Ok(JPEG(data))
    }
}

impl From<Vec<u8>> for JPEG {
    fn from(data: Vec<u8>) -> Self {
        JPEG(data)
    }
}

impl From<JPEG> for Vec<u8> {
    fn from(data: JPEG) -> Vec<u8> {
        data.0
    }
}

impl From<web_common::types::JPEG> for JPEG {
    fn from(data: web_common::types::JPEG) -> Self {
        JPEG(data.into())
    }
}

impl From<JPEG> for web_common::types::JPEG {
    fn from(data: JPEG) -> web_common::types::JPEG {
        data.0.into()
    }
}
