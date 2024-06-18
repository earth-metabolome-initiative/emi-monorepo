//! This file is automatically generated, do not modify it directly.
use super::*;
#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct UserEmail {
    pub id: i32,
    pub email: String,
    pub created_by: i32,
    pub created_at: chrono::NaiveDateTime,
    pub login_provider_id: i32,
    pub primary_email: bool,
}

unsafe impl Send for UserEmail {}
unsafe impl Sync for UserEmail {}
impl Tabular for UserEmail {
    const TABLE: crate::database::Table = crate::database::Table::UserEmails;
}
impl Describable for UserEmail {
    fn description(&self) -> Option<&str> {
        None
    }
}
impl Colorable for UserEmail {
    fn color(&self) -> Option<&str> {
        None
    }
}

impl Filtrable for crate::database::flat_variants::UserEmail {
    type Filter = crate::database::filter_variants::UserEmailFilter;
}
#[cfg(feature = "frontend")]
impl AllRecords for UserEmail {
    fn all_records<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        filter: Option<&<Self as Filtrable>::Filter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> impl std::future::Future<Output = Result<Vec<Self>, crate::api::ApiError>> {
        Self::all(filter, limit, offset, connection)
    }
}
#[cfg(feature = "frontend")]
impl UserEmail {
    /// Get the id attribute.
    pub fn get_id<E>(&self) -> &E
    where
        i32: AsRef<E>,
    {
        self.id.as_ref()
    }

    /// Get the email attribute.
    pub fn get_email<E>(&self) -> &E
    where
        String: AsRef<E>,
    {
        self.email.as_ref()
    }

    /// Get the created_by attribute.
    pub fn get_created_by<E>(&self) -> &E
    where
        i32: AsRef<E>,
    {
        self.created_by.as_ref()
    }

    /// Get the created_at attribute.
    pub fn get_created_at<E>(&self) -> &E
    where
        chrono::NaiveDateTime: AsRef<E>,
    {
        self.created_at.as_ref()
    }

    /// Get the login_provider_id attribute.
    pub fn get_login_provider_id<E>(&self) -> &E
    where
        i32: AsRef<E>,
    {
        self.login_provider_id.as_ref()
    }

    /// Whether the UserEmail is primary_email.
    pub fn is_primary_email<E>(&self) -> &E
    where
        bool: AsRef<E>,
    {
        self.primary_email.as_ref()
    }

    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::text(self.email),
            gluesql::core::ast_builder::num(self.created_by),
            gluesql::core::ast_builder::timestamp(self.created_at.to_string()),
            gluesql::core::ast_builder::num(self.login_provider_id),
            (self.primary_email.into()),
        ]
    }

    /// Insert the UserEmail into the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn insert<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, crate::api::ApiError> {
        use gluesql::core::ast_builder::*;
        Ok(table("user_emails")
            .insert()
            .columns("id, email, created_by, created_at, login_provider_id, primary_email")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
            .map(|payload| match payload {
                gluesql::prelude::Payload::Insert(number_of_inserted_rows) => {
                    number_of_inserted_rows
                }
                _ => unreachable!("Payload must be an Insert"),
            })?)
    }

    /// Get the UserEmail from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to check.
    /// * `connection` - The connection to the database.
    pub async fn get<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, crate::api::ApiError> {
        use gluesql::core::ast_builder::*;
        let select_row = table("user_emails")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, email, created_by, created_at, login_provider_id, primary_email")
            .limit(1)
            .execute(connection)
            .await?;
        Ok(select_row
            .select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete the UserEmail from the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn delete<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, crate::api::ApiError> {
        Self::delete_from_id(self.id, connection).await
    }

    /// Delete the UserEmail from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    pub async fn delete_from_id<
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    >(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, crate::api::ApiError> {
        use gluesql::core::ast_builder::*;
        Ok(table("user_emails")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
            .map(|payload| match payload {
                gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                _ => unreachable!("Payload must be a Delete"),
            })?)
    }

    /// Update the struct in the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn update<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, crate::api::ApiError> {
        use gluesql::core::ast_builder::*;
        table("user_emails")
            .update()
            .set("id", gluesql::core::ast_builder::num(self.id))
            .set("email", gluesql::core::ast_builder::text(self.email))
            .set(
                "created_by",
                gluesql::core::ast_builder::num(self.created_by),
            )
            .set(
                "created_at",
                gluesql::core::ast_builder::timestamp(self.created_at.to_string()),
            )
            .set(
                "login_provider_id",
                gluesql::core::ast_builder::num(self.login_provider_id),
            )
            .set("primary_email", self.primary_email)
            .execute(connection)
            .await
            .map(|payload| match payload {
                gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                _ => unreachable!("Expected Payload::Update"),
            })
            .map_err(crate::api::ApiError::from)
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// * `connection` - The connection to the database.
    pub async fn update_or_insert<
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    >(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, crate::api::ApiError> {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all UserEmail from the database.
    ///
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    pub async fn all<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        filter: Option<&crate::database::filter_variants::UserEmailFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, crate::api::ApiError> {
        use gluesql::core::ast_builder::*;
        let select_row = table("user_emails")
            .select()
            .filter(filter.map_or_else(
                || {
                    gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true))
                        .into()
                },
                |filter| filter.as_filter_expression(),
            ))
            .project("id, email, created_by, created_at, login_provider_id, primary_email")
            .order_by("primary_email DESC")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row
            .select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::I32(id) => id.clone(),
                _ => unreachable!("Expected I32"),
            },
            email: match row.get("email").unwrap() {
                gluesql::prelude::Value::Str(email) => email.clone(),
                _ => unreachable!("Expected Str"),
            },
            created_by: match row.get("created_by").unwrap() {
                gluesql::prelude::Value::I32(created_by) => created_by.clone(),
                _ => unreachable!("Expected I32"),
            },
            created_at: match row.get("created_at").unwrap() {
                gluesql::prelude::Value::Timestamp(created_at) => created_at.clone(),
                _ => unreachable!("Expected Timestamp"),
            },
            login_provider_id: match row.get("login_provider_id").unwrap() {
                gluesql::prelude::Value::I32(login_provider_id) => login_provider_id.clone(),
                _ => unreachable!("Expected I32"),
            },
            primary_email: match row.get("primary_email").unwrap() {
                gluesql::prelude::Value::Bool(primary_email) => primary_email.clone(),
                _ => unreachable!("Expected Bool"),
            },
        }
    }
}