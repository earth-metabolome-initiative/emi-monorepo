

    /// Returns whether the current check constraint is a `distinct` constraint,
    /// i.e. it checks whether two columns are distinct.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    /// * If their is an error while querying the database.
    pub fn is_distinct_constraint(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Option<Arc<Vec<Column>>>, WebCodeGenError> {
        let columns = self.columns(conn)?;
        if columns.len() != 2 {
            return Ok(None);
        }
        let parsed_check_clause = Parser::new(&PostgreSqlDialect {})
            .try_with_sql(&self.check_clause)
            .expect("Failed to parse check clause")
            .parse_expr()
            .expect("Failed to parse check clause");
        match parsed_check_clause {
            Expr::Function(function) => {
                if !function.name.to_string().starts_with("must_be_distinct") {
                    return Ok(None);
                }
                Ok(Some(columns))
            }
            _ => Ok(None),
        }
    }