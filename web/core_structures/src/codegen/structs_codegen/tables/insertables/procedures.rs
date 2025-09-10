#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ProcedureAttribute {
    Procedure,
    ProcedureTemplate,
    ParentProcedure,
    ParentProcedureTemplate,
    PredecessorProcedure,
    PredecessorProcedureTemplate,
    MostConcreteTable,
    CreatedBy,
    CreatedAt,
    UpdatedBy,
    UpdatedAt,
    NumberOfCompletedSubprocedures,
}
impl core::str::FromStr for ProcedureAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Procedure" => Ok(Self::Procedure),
            "ProcedureTemplate" => Ok(Self::ProcedureTemplate),
            "ParentProcedure" => Ok(Self::ParentProcedure),
            "ParentProcedureTemplate" => Ok(Self::ParentProcedureTemplate),
            "PredecessorProcedure" => Ok(Self::PredecessorProcedure),
            "PredecessorProcedureTemplate" => Ok(Self::PredecessorProcedureTemplate),
            "MostConcreteTable" => Ok(Self::MostConcreteTable),
            "CreatedBy" => Ok(Self::CreatedBy),
            "CreatedAt" => Ok(Self::CreatedAt),
            "UpdatedBy" => Ok(Self::UpdatedBy),
            "UpdatedAt" => Ok(Self::UpdatedAt),
            "NumberOfCompletedSubprocedures" => Ok(Self::NumberOfCompletedSubprocedures),
            "procedure" => Ok(Self::Procedure),
            "procedure_template" => Ok(Self::ProcedureTemplate),
            "parent_procedure" => Ok(Self::ParentProcedure),
            "parent_procedure_template" => Ok(Self::ParentProcedureTemplate),
            "predecessor_procedure" => Ok(Self::PredecessorProcedure),
            "predecessor_procedure_template" => Ok(Self::PredecessorProcedureTemplate),
            "most_concrete_table" => Ok(Self::MostConcreteTable),
            "created_by" => Ok(Self::CreatedBy),
            "created_at" => Ok(Self::CreatedAt),
            "updated_by" => Ok(Self::UpdatedBy),
            "updated_at" => Ok(Self::UpdatedAt),
            "number_of_completed_subprocedures" => Ok(Self::NumberOfCompletedSubprocedures),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl core::fmt::Display for ProcedureAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Procedure => write!(f, "procedures.procedure"),
            Self::ProcedureTemplate => write!(f, "procedures.procedure_template"),
            Self::ParentProcedure => write!(f, "procedures.parent_procedure"),
            Self::ParentProcedureTemplate => {
                write!(f, "procedures.parent_procedure_template")
            }
            Self::PredecessorProcedure => write!(f, "procedures.predecessor_procedure"),
            Self::PredecessorProcedureTemplate => {
                write!(f, "procedures.predecessor_procedure_template")
            }
            Self::MostConcreteTable => write!(f, "procedures.most_concrete_table"),
            Self::CreatedBy => write!(f, "procedures.created_by"),
            Self::CreatedAt => write!(f, "procedures.created_at"),
            Self::UpdatedBy => write!(f, "procedures.updated_by"),
            Self::UpdatedAt => write!(f, "procedures.updated_at"),
            Self::NumberOfCompletedSubprocedures => {
                write!(f, "procedures.number_of_completed_subprocedures")
            }
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(table_name = crate::codegen::diesel_codegen::tables::procedures::procedures)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableProcedure {
    pub(crate) procedure: ::rosetta_uuid::Uuid,
    pub(crate) procedure_template: i32,
    pub(crate) parent_procedure: Option<::rosetta_uuid::Uuid>,
    pub(crate) parent_procedure_template: Option<i32>,
    pub(crate) predecessor_procedure: Option<::rosetta_uuid::Uuid>,
    pub(crate) predecessor_procedure_template: Option<i32>,
    pub(crate) most_concrete_table: String,
    pub(crate) created_by: i32,
    pub(crate) created_at: ::rosetta_timestamp::TimestampUTC,
    pub(crate) updated_by: i32,
    pub(crate) updated_at: ::rosetta_timestamp::TimestampUTC,
    pub(crate) number_of_completed_subprocedures: i16,
}
impl InsertableProcedure {
    pub fn created_by<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::codegen::structs_codegen::tables::users::User, diesel::result::Error>
    where
        crate::codegen::structs_codegen::tables::users::User: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::users::User::read(self.created_by, conn)
    }
    pub fn parent_procedure<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::procedures::Procedure>,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedures::Procedure:
            web_common_traits::database::Read<C>,
    {
        use diesel::OptionalExtension;
        use web_common_traits::database::Read;
        let Some(parent_procedure) = self.parent_procedure else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::procedures::Procedure::read(parent_procedure, conn)
            .optional()
    }
    #[cfg(feature = "postgres")]
    pub fn procedures_parent_procedure_parent_procedure_template_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::procedures::Procedure>,
        diesel::result::Error,
    > {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl,
            associations::HasTable,
        };
        let Some(parent_procedure) = self.parent_procedure else {
            return Ok(None);
        };
        let Some(parent_procedure_template) = self.parent_procedure_template else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::procedures::Procedure::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedures::procedures::dsl::procedure
                    .eq(parent_procedure)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedures::procedures::dsl::procedure_template
                            .eq(parent_procedure_template),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedures::Procedure,
            >(conn)
            .optional()
    }
    pub fn parent_procedure_template<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate>,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate:
            web_common_traits::database::Read<C>,
    {
        use diesel::OptionalExtension;
        use web_common_traits::database::Read;
        let Some(parent_procedure_template) = self.parent_procedure_template else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate::read(
            parent_procedure_template,
            conn,
        )
        .optional()
    }
    pub fn procedures_parent_procedure_template_predecessor_procedure_fkey<
        C: diesel::connection::LoadConnection,
    >(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<
            crate::codegen::structs_codegen::tables::next_procedure_templates::NextProcedureTemplate,
        >,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::next_procedure_templates::NextProcedureTemplate: web_common_traits::database::Read<
            C,
        >,
    {
        use diesel::OptionalExtension;
        use web_common_traits::database::Read;
        let Some(parent_procedure_template) = self.parent_procedure_template else {
            return Ok(None);
        };
        let Some(predecessor_procedure_template) = self.predecessor_procedure_template else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::next_procedure_templates::NextProcedureTemplate::read(
                (
                    parent_procedure_template,
                    predecessor_procedure_template,
                    self.procedure_template,
                ),
                conn,
            )
            .optional()
    }
    pub fn procedures_parent_procedure_template_procedure_template_fkey<
        C: diesel::connection::LoadConnection,
    >(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<
            crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate,
        >,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate: web_common_traits::database::Read<
            C,
        >,
    {
        use diesel::OptionalExtension;
        use web_common_traits::database::Read;
        let Some(parent_procedure_template) = self.parent_procedure_template else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate::read(
                (parent_procedure_template, self.procedure_template),
                conn,
            )
            .optional()
    }
    pub fn predecessor_procedure<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::procedures::Procedure>,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedures::Procedure:
            web_common_traits::database::Read<C>,
    {
        use diesel::OptionalExtension;
        use web_common_traits::database::Read;
        let Some(predecessor_procedure) = self.predecessor_procedure else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::procedures::Procedure::read(
            predecessor_procedure,
            conn,
        )
        .optional()
    }
    #[cfg(feature = "postgres")]
    pub fn procedures_predecessor_procedure_predecessor_procedure_tem_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::procedures::Procedure>,
        diesel::result::Error,
    > {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl,
            associations::HasTable,
        };
        let Some(predecessor_procedure) = self.predecessor_procedure else {
            return Ok(None);
        };
        let Some(predecessor_procedure_template) = self.predecessor_procedure_template else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::procedures::Procedure::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedures::procedures::dsl::procedure
                    .eq(predecessor_procedure)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedures::procedures::dsl::procedure_template
                            .eq(predecessor_procedure_template),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedures::Procedure,
            >(conn)
            .optional()
    }
    pub fn predecessor_procedure_template<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate>,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate:
            web_common_traits::database::Read<C>,
    {
        use diesel::OptionalExtension;
        use web_common_traits::database::Read;
        let Some(predecessor_procedure_template) = self.predecessor_procedure_template else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate::read(
            predecessor_procedure_template,
            conn,
        )
        .optional()
    }
    pub fn procedure_template<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate::read(
            self.procedure_template,
            conn,
        )
    }
    pub fn updated_by<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::codegen::structs_codegen::tables::users::User, diesel::result::Error>
    where
        crate::codegen::structs_codegen::tables::users::User: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::users::User::read(self.updated_by, conn)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Hash, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableProcedureBuilder {
    pub(crate) procedure: Option<::rosetta_uuid::Uuid>,
    pub(crate) procedure_template: Option<i32>,
    pub(crate) parent_procedure: Option<::rosetta_uuid::Uuid>,
    pub(crate) parent_procedure_template: Option<i32>,
    pub(crate) predecessor_procedure: Option<::rosetta_uuid::Uuid>,
    pub(crate) predecessor_procedure_template: Option<i32>,
    pub(crate) most_concrete_table: Option<String>,
    pub(crate) created_by: Option<i32>,
    pub(crate) created_at: Option<::rosetta_timestamp::TimestampUTC>,
    pub(crate) updated_by: Option<i32>,
    pub(crate) updated_at: Option<::rosetta_timestamp::TimestampUTC>,
    pub(crate) number_of_completed_subprocedures: Option<i16>,
}
impl From<InsertableProcedureBuilder>
    for web_common_traits::database::IdOrBuilder<::rosetta_uuid::Uuid, InsertableProcedureBuilder>
{
    fn from(builder: InsertableProcedureBuilder) -> Self {
        Self::Builder(builder)
    }
}
impl Default for InsertableProcedureBuilder {
    fn default() -> Self {
        Self {
            procedure: Some(rosetta_uuid::Uuid::new_v4()),
            procedure_template: Default::default(),
            parent_procedure: Default::default(),
            parent_procedure_template: Default::default(),
            predecessor_procedure: Default::default(),
            predecessor_procedure_template: Default::default(),
            most_concrete_table: Default::default(),
            created_by: Default::default(),
            created_at: Some(rosetta_timestamp::TimestampUTC::default()),
            updated_by: Default::default(),
            updated_at: Some(rosetta_timestamp::TimestampUTC::default()),
            number_of_completed_subprocedures: Some(0i16),
        }
    }
}
impl common_traits::builder::IsCompleteBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableProcedureBuilder
{
    fn is_complete(&self) -> bool {
        self.procedure.is_some()
            && self.procedure_template.is_some()
            && self.most_concrete_table.is_some()
            && self.created_by.is_some()
            && self.created_at.is_some()
            && self.updated_by.is_some()
            && self.updated_at.is_some()
            && self.number_of_completed_subprocedures.is_some()
    }
}
/// Trait defining setters for attributes of an instance of `Procedure` or
/// descendant tables.
pub trait ProcedureSettable: Sized {
    /// Attributes required to build the insertable.
    type Attributes;
    /// Sets the value of the `public.procedures.procedure` column.
    ///
    /// # Arguments
    /// * `procedure`: The value to set for the `public.procedures.procedure`
    ///   column.
    ///
    /// # Implementation details
    /// This method accepts a reference to a generic value which can be
    /// converted to the required type for the column. This allows passing
    /// values of different types, as long as they can be converted to the
    /// required type using the `TryFrom` trait. The method, additionally,
    /// employs same-as and inferred same-as rules to ensure that the
    /// schema-defined ancestral tables and associated table values associated
    /// to the current column (if any) are also set appropriately.
    ///
    /// # Errors
    /// * If the provided value cannot be converted to the required type
    ///   `::rosetta_uuid::Uuid`.
    /// * If the provided value does not pass schema-defined validation.
    fn procedure<P>(
        self,
        procedure: P,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        P: web_common_traits::database::PrimaryKeyLike<PrimaryKey = ::rosetta_uuid::Uuid>;
    /// Sets the value of the `public.procedures.procedure_template` column.
    ///
    /// # Arguments
    /// * `procedure_template`: The value to set for the
    ///   `public.procedures.procedure_template` column.
    ///
    /// # Implementation details
    /// This method accepts a reference to a generic value which can be
    /// converted to the required type for the column. This allows passing
    /// values of different types, as long as they can be converted to the
    /// required type using the `TryFrom` trait. The method, additionally,
    /// employs same-as and inferred same-as rules to ensure that the
    /// schema-defined ancestral tables and associated table values associated
    /// to the current column (if any) are also set appropriately.
    ///
    /// # Errors
    /// * If the provided value cannot be converted to the required type `i32`.
    /// * If the provided value does not pass schema-defined validation.
    fn procedure_template<PT>(
        self,
        procedure_template: PT,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PT: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
    /// Sets the value of the `public.procedures.parent_procedure` column.
    ///
    /// # Arguments
    /// * `parent_procedure`: The value to set for the
    ///   `public.procedures.parent_procedure` column.
    ///
    /// # Implementation details
    /// This method accepts a reference to a generic value which can be
    /// converted to the required type for the column. This allows passing
    /// values of different types, as long as they can be converted to the
    /// required type using the `TryFrom` trait. The method, additionally,
    /// employs same-as and inferred same-as rules to ensure that the
    /// schema-defined ancestral tables and associated table values associated
    /// to the current column (if any) are also set appropriately.
    ///
    /// # Errors
    /// * If the provided value cannot be converted to the required type
    ///   `::rosetta_uuid::Uuid`.
    /// * If the provided value does not pass schema-defined validation.
    fn parent_procedure<PP>(
        self,
        parent_procedure: PP,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PP: web_common_traits::database::MaybePrimaryKeyLike<PrimaryKey = ::rosetta_uuid::Uuid>;
    /// Sets the value of the `public.procedures.parent_procedure_template`
    /// column.
    ///
    /// # Arguments
    /// * `parent_procedure_template`: The value to set for the
    ///   `public.procedures.parent_procedure_template` column.
    ///
    /// # Implementation details
    /// This method accepts a reference to a generic value which can be
    /// converted to the required type for the column. This allows passing
    /// values of different types, as long as they can be converted to the
    /// required type using the `TryFrom` trait. The method, additionally,
    /// employs same-as and inferred same-as rules to ensure that the
    /// schema-defined ancestral tables and associated table values associated
    /// to the current column (if any) are also set appropriately.
    ///
    /// # Errors
    /// * If the provided value cannot be converted to the required type `i32`.
    /// * If the provided value does not pass schema-defined validation.
    fn parent_procedure_template<PPT>(
        self,
        parent_procedure_template: PPT,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PPT: web_common_traits::database::MaybePrimaryKeyLike<PrimaryKey = i32>;
    /// Sets the value of the `public.procedures.predecessor_procedure` column.
    ///
    /// # Arguments
    /// * `predecessor_procedure`: The value to set for the
    ///   `public.procedures.predecessor_procedure` column.
    ///
    /// # Implementation details
    /// This method accepts a reference to a generic value which can be
    /// converted to the required type for the column. This allows passing
    /// values of different types, as long as they can be converted to the
    /// required type using the `TryFrom` trait. The method, additionally,
    /// employs same-as and inferred same-as rules to ensure that the
    /// schema-defined ancestral tables and associated table values associated
    /// to the current column (if any) are also set appropriately.
    ///
    /// # Errors
    /// * If the provided value cannot be converted to the required type
    ///   `::rosetta_uuid::Uuid`.
    /// * If the provided value does not pass schema-defined validation.
    fn predecessor_procedure<PP>(
        self,
        predecessor_procedure: PP,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PP: web_common_traits::database::MaybePrimaryKeyLike<PrimaryKey = ::rosetta_uuid::Uuid>;
    /// Sets the value of the `public.procedures.predecessor_procedure_template`
    /// column.
    ///
    /// # Arguments
    /// * `predecessor_procedure_template`: The value to set for the
    ///   `public.procedures.predecessor_procedure_template` column.
    ///
    /// # Implementation details
    /// This method accepts a reference to a generic value which can be
    /// converted to the required type for the column. This allows passing
    /// values of different types, as long as they can be converted to the
    /// required type using the `TryFrom` trait. The method, additionally,
    /// employs same-as and inferred same-as rules to ensure that the
    /// schema-defined ancestral tables and associated table values associated
    /// to the current column (if any) are also set appropriately.
    ///
    /// # Errors
    /// * If the provided value cannot be converted to the required type `i32`.
    /// * If the provided value does not pass schema-defined validation.
    fn predecessor_procedure_template<PPT>(
        self,
        predecessor_procedure_template: PPT,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PPT: web_common_traits::database::MaybePrimaryKeyLike<PrimaryKey = i32>;
    /// Sets the value of the `public.procedures.created_by` column.
    ///
    /// # Arguments
    /// * `created_by`: The value to set for the `public.procedures.created_by`
    ///   column.
    ///
    /// # Implementation details
    /// This method accepts a reference to a generic value which can be
    /// converted to the required type for the column. This allows passing
    /// values of different types, as long as they can be converted to the
    /// required type using the `TryFrom` trait. The method, additionally,
    /// employs same-as and inferred same-as rules to ensure that the
    /// schema-defined ancestral tables and associated table values associated
    /// to the current column (if any) are also set appropriately.
    ///
    /// # Errors
    /// * If the provided value cannot be converted to the required type `i32`.
    /// * If the provided value does not pass schema-defined validation.
    fn created_by<CB>(
        self,
        created_by: CB,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        CB: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
    /// Sets the value of the `public.procedures.created_at` column.
    ///
    /// # Arguments
    /// * `created_at`: The value to set for the `public.procedures.created_at`
    ///   column.
    ///
    /// # Implementation details
    /// This method accepts a reference to a generic value which can be
    /// converted to the required type for the column. This allows passing
    /// values of different types, as long as they can be converted to the
    /// required type using the `TryFrom` trait. The method, additionally,
    /// employs same-as and inferred same-as rules to ensure that the
    /// schema-defined ancestral tables and associated table values associated
    /// to the current column (if any) are also set appropriately.
    ///
    /// # Errors
    /// * If the provided value cannot be converted to the required type
    ///   `::rosetta_timestamp::TimestampUTC`.
    /// * If the provided value does not pass schema-defined validation.
    fn created_at<CA>(
        self,
        created_at: CA,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        CA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::SingleFieldError:
            From<<CA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error>;
    /// Sets the value of the `public.procedures.updated_by` column.
    ///
    /// # Arguments
    /// * `updated_by`: The value to set for the `public.procedures.updated_by`
    ///   column.
    ///
    /// # Implementation details
    /// This method accepts a reference to a generic value which can be
    /// converted to the required type for the column. This allows passing
    /// values of different types, as long as they can be converted to the
    /// required type using the `TryFrom` trait. The method, additionally,
    /// employs same-as and inferred same-as rules to ensure that the
    /// schema-defined ancestral tables and associated table values associated
    /// to the current column (if any) are also set appropriately.
    ///
    /// # Errors
    /// * If the provided value cannot be converted to the required type `i32`.
    /// * If the provided value does not pass schema-defined validation.
    fn updated_by<UB>(
        self,
        updated_by: UB,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        UB: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
    /// Sets the value of the `public.procedures.updated_at` column.
    ///
    /// # Arguments
    /// * `updated_at`: The value to set for the `public.procedures.updated_at`
    ///   column.
    ///
    /// # Implementation details
    /// This method accepts a reference to a generic value which can be
    /// converted to the required type for the column. This allows passing
    /// values of different types, as long as they can be converted to the
    /// required type using the `TryFrom` trait. The method, additionally,
    /// employs same-as and inferred same-as rules to ensure that the
    /// schema-defined ancestral tables and associated table values associated
    /// to the current column (if any) are also set appropriately.
    ///
    /// # Errors
    /// * If the provided value cannot be converted to the required type
    ///   `::rosetta_timestamp::TimestampUTC`.
    /// * If the provided value does not pass schema-defined validation.
    fn updated_at<UA>(
        self,
        updated_at: UA,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        UA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::SingleFieldError:
            From<<UA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error>;
    /// Sets the value of the
    /// `public.procedures.number_of_completed_subprocedures` column.
    ///
    /// # Arguments
    /// * `number_of_completed_subprocedures`: The value to set for the
    ///   `public.procedures.number_of_completed_subprocedures` column.
    ///
    /// # Implementation details
    /// This method accepts a reference to a generic value which can be
    /// converted to the required type for the column. This allows passing
    /// values of different types, as long as they can be converted to the
    /// required type using the `TryFrom` trait. The method, additionally,
    /// employs same-as and inferred same-as rules to ensure that the
    /// schema-defined ancestral tables and associated table values associated
    /// to the current column (if any) are also set appropriately.
    ///
    /// # Errors
    /// * If the provided value cannot be converted to the required type `i16`.
    /// * If the provided value does not pass schema-defined validation.
    fn number_of_completed_subprocedures<NOCS>(
        self,
        number_of_completed_subprocedures: NOCS,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        NOCS: TryInto<i16>,
        validation_errors::SingleFieldError: From<<NOCS as TryInto<i16>>::Error>;
}
impl ProcedureSettable for InsertableProcedureBuilder {
    type Attributes = crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute;
    /// Sets the value of the `public.procedures.procedure` column.
    fn procedure<P>(
        mut self,
        procedure: P,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        P: web_common_traits::database::PrimaryKeyLike<PrimaryKey = ::rosetta_uuid::Uuid>,
    {
        let procedure = <P as web_common_traits::database::PrimaryKeyLike>::primary_key(&procedure);
        if let Some(parent_procedure) = self.parent_procedure {
            pgrx_validation::must_be_distinct_uuid(procedure, parent_procedure)
                .map_err(|e| {
                    e
                        .rename_fields(
                            crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute::Procedure,
                            crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute::ParentProcedure,
                        )
                })?;
        }
        if let Some(predecessor_procedure) = self.predecessor_procedure {
            pgrx_validation::must_be_distinct_uuid(procedure, predecessor_procedure)
                .map_err(|e| {
                    e
                        .rename_fields(
                            crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute::Procedure,
                            crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute::PredecessorProcedure,
                        )
                })?;
        }
        self.procedure = Some(procedure);
        Ok(self)
    }
    /// Sets the value of the `public.procedures.procedure_template` column.
    fn procedure_template<PT>(
        mut self,
        procedure_template: PT,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PT: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let procedure_template =
            <PT as web_common_traits::database::PrimaryKeyLike>::primary_key(&procedure_template);
        if let Some(predecessor_procedure_template) = self.predecessor_procedure_template {
            pgrx_validation::must_be_distinct_i32(
                    procedure_template,
                    predecessor_procedure_template,
                )
                .map_err(|e| {
                    e
                        .rename_fields(
                            crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute::ProcedureTemplate,
                            crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute::PredecessorProcedureTemplate,
                        )
                })?;
        }
        if let Some(parent_procedure_template) = self.parent_procedure_template {
            pgrx_validation::must_be_distinct_i32(
                    procedure_template,
                    parent_procedure_template,
                )
                .map_err(|e| {
                    e
                        .rename_fields(
                            crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute::ProcedureTemplate,
                            crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute::ParentProcedureTemplate,
                        )
                })?;
        }
        self.procedure_template = Some(procedure_template);
        Ok(self)
    }
    /// Sets the value of the `public.procedures.parent_procedure` column.
    ///
    /// # Implementation notes
    /// This method also set the values of other columns, due to
    /// same-as relationships or inferred values.
    ///
    /// ## Mermaid illustration
    ///
    /// ```mermaid
    /// flowchart BT
    /// classDef column-of-interest stroke: #f0746c,fill: #f49f9a
    /// classDef directly-involved-column stroke: #6c74f0,fill: #9a9ff4
    /// v0@{shape: rounded, label: "parent_procedure"}
    /// class v0 column-of-interest
    /// v1@{shape: rounded, label: "parent_procedure_template"}
    /// class v1 directly-involved-column
    /// v0 -.->|"`foreign defines`"| v1
    /// ```
    fn parent_procedure<PP>(
        mut self,
        parent_procedure: PP,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PP: web_common_traits::database::MaybePrimaryKeyLike<PrimaryKey = ::rosetta_uuid::Uuid>,
    {
        let parent_procedure =
            <PP as web_common_traits::database::MaybePrimaryKeyLike>::maybe_primary_key(
                &parent_procedure,
            );
        if let (Some(parent_procedure), Some(procedure)) = (parent_procedure, self.procedure) {
            pgrx_validation::must_be_distinct_uuid(procedure, parent_procedure)
                .map_err(|e| {
                    e
                        .rename_fields(
                            crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute::Procedure,
                            crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute::ParentProcedure,
                        )
                })?;
        }
        self.parent_procedure = parent_procedure;
        Ok(self)
    }
    /// Sets the value of the `public.procedures.parent_procedure_template`
    /// column.
    fn parent_procedure_template<PPT>(
        mut self,
        parent_procedure_template: PPT,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PPT: web_common_traits::database::MaybePrimaryKeyLike<PrimaryKey = i32>,
    {
        let parent_procedure_template =
            <PPT as web_common_traits::database::MaybePrimaryKeyLike>::maybe_primary_key(
                &parent_procedure_template,
            );
        if let (Some(procedure_template), Some(parent_procedure_template)) =
            (self.procedure_template, parent_procedure_template)
        {
            pgrx_validation::must_be_distinct_i32(
                    procedure_template,
                    parent_procedure_template,
                )
                .map_err(|e| {
                    e
                        .rename_fields(
                            crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute::ProcedureTemplate,
                            crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute::ParentProcedureTemplate,
                        )
                })?;
        }
        self.parent_procedure_template = parent_procedure_template;
        Ok(self)
    }
    /// Sets the value of the `public.procedures.predecessor_procedure` column.
    ///
    /// # Implementation notes
    /// This method also set the values of other columns, due to
    /// same-as relationships or inferred values.
    ///
    /// ## Mermaid illustration
    ///
    /// ```mermaid
    /// flowchart BT
    /// classDef column-of-interest stroke: #f0746c,fill: #f49f9a
    /// classDef directly-involved-column stroke: #6c74f0,fill: #9a9ff4
    /// v0@{shape: rounded, label: "predecessor_procedure"}
    /// class v0 column-of-interest
    /// v1@{shape: rounded, label: "predecessor_procedure_template"}
    /// class v1 directly-involved-column
    /// v0 -.->|"`foreign defines`"| v1
    /// ```
    fn predecessor_procedure<PP>(
        mut self,
        predecessor_procedure: PP,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PP: web_common_traits::database::MaybePrimaryKeyLike<PrimaryKey = ::rosetta_uuid::Uuid>,
    {
        let predecessor_procedure =
            <PP as web_common_traits::database::MaybePrimaryKeyLike>::maybe_primary_key(
                &predecessor_procedure,
            );
        if let (Some(predecessor_procedure), Some(procedure)) =
            (predecessor_procedure, self.procedure)
        {
            pgrx_validation::must_be_distinct_uuid(procedure, predecessor_procedure)
                .map_err(|e| {
                    e
                        .rename_fields(
                            crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute::Procedure,
                            crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute::PredecessorProcedure,
                        )
                })?;
        }
        self.predecessor_procedure = predecessor_procedure;
        Ok(self)
    }
    /// Sets the value of the `public.procedures.predecessor_procedure_template`
    /// column.
    fn predecessor_procedure_template<PPT>(
        mut self,
        predecessor_procedure_template: PPT,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PPT: web_common_traits::database::MaybePrimaryKeyLike<PrimaryKey = i32>,
    {
        let predecessor_procedure_template =
            <PPT as web_common_traits::database::MaybePrimaryKeyLike>::maybe_primary_key(
                &predecessor_procedure_template,
            );
        if let (Some(procedure_template), Some(predecessor_procedure_template)) =
            (self.procedure_template, predecessor_procedure_template)
        {
            pgrx_validation::must_be_distinct_i32(
                    procedure_template,
                    predecessor_procedure_template,
                )
                .map_err(|e| {
                    e
                        .rename_fields(
                            crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute::ProcedureTemplate,
                            crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute::PredecessorProcedureTemplate,
                        )
                })?;
        }
        self.predecessor_procedure_template = predecessor_procedure_template;
        Ok(self)
    }
    /// Sets the value of the `public.procedures.created_by` column.
    ///
    /// # Implementation notes
    /// This method also set the values of other columns, due to
    /// same-as relationships or inferred values.
    ///
    /// ## Mermaid illustration
    ///
    /// ```mermaid
    /// flowchart BT
    /// classDef column-of-interest stroke: #f0746c,fill: #f49f9a
    /// classDef directly-involved-column stroke: #6c74f0,fill: #9a9ff4
    /// v0@{shape: rounded, label: "created_by"}
    /// class v0 column-of-interest
    /// v1@{shape: rounded, label: "updated_by"}
    /// class v1 directly-involved-column
    /// ```
    fn created_by<CB>(
        mut self,
        created_by: CB,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        CB: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let created_by =
            <CB as web_common_traits::database::PrimaryKeyLike>::primary_key(&created_by);
        self = self.updated_by(created_by)?;
        self.created_by = Some(created_by);
        Ok(self)
    }
    /// Sets the value of the `public.procedures.created_at` column.
    fn created_at<CA>(
        mut self,
        created_at: CA,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        CA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::SingleFieldError:
            From<<CA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error>,
    {
        let created_at = created_at.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(ProcedureAttribute::CreatedAt)
        })?;
        if let Some(updated_at) = self.updated_at {
            pgrx_validation::must_be_smaller_than_utc(created_at, updated_at)
                .map_err(|e| {
                    e
                        .rename_fields(
                            crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute::CreatedAt,
                            crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute::UpdatedAt,
                        )
                })?;
        }
        self.created_at = Some(created_at);
        Ok(self)
    }
    /// Sets the value of the `public.procedures.updated_by` column.
    fn updated_by<UB>(
        mut self,
        updated_by: UB,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        UB: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let updated_by =
            <UB as web_common_traits::database::PrimaryKeyLike>::primary_key(&updated_by);
        self.updated_by = Some(updated_by);
        Ok(self)
    }
    /// Sets the value of the `public.procedures.updated_at` column.
    fn updated_at<UA>(
        mut self,
        updated_at: UA,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        UA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::SingleFieldError:
            From<<UA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error>,
    {
        let updated_at = updated_at.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(ProcedureAttribute::UpdatedAt)
        })?;
        if let Some(created_at) = self.created_at {
            pgrx_validation::must_be_smaller_than_utc(created_at, updated_at)
                .map_err(|e| {
                    e
                        .rename_fields(
                            crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute::CreatedAt,
                            crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute::UpdatedAt,
                        )
                })?;
        }
        self.updated_at = Some(updated_at);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.procedures.number_of_completed_subprocedures` column.
    fn number_of_completed_subprocedures<NOCS>(
        mut self,
        number_of_completed_subprocedures: NOCS,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        NOCS: TryInto<i16>,
        validation_errors::SingleFieldError: From<<NOCS as TryInto<i16>>::Error>,
    {
        let number_of_completed_subprocedures =
            number_of_completed_subprocedures.try_into().map_err(|err| {
                validation_errors::SingleFieldError::from(err)
                    .rename_field(ProcedureAttribute::NumberOfCompletedSubprocedures)
            })?;
        pgrx_validation::must_be_positive_i16(number_of_completed_subprocedures)
            .map_err(|e| {
                e
                    .rename_field(
                        crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute::NumberOfCompletedSubprocedures,
                    )
            })?;
        self.number_of_completed_subprocedures = Some(number_of_completed_subprocedures);
        Ok(self)
    }
}
impl web_common_traits::database::MostConcreteTable for InsertableProcedureBuilder {
    fn set_most_concrete_table(&mut self, table_name: &str) {
        if self.most_concrete_table.is_none() {
            self.most_concrete_table = Some(table_name.to_owned());
        }
    }
}
impl web_common_traits::prelude::SetPrimaryKey for InsertableProcedureBuilder {
    type PrimaryKey = ::rosetta_uuid::Uuid;
    fn set_primary_key(mut self, primary_key: Self::PrimaryKey) -> Self {
        self.procedure = Some(primary_key);
        self
    }
}
impl<C> web_common_traits::database::TryInsertGeneric<C> for InsertableProcedureBuilder
where
    Self: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::codegen::structs_codegen::tables::procedures::Procedure,
            Error = web_common_traits::database::InsertError<ProcedureAttribute>,
        >,
{
    type Attributes = ProcedureAttribute;
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, web_common_traits::database::InsertError<Self::Attributes>> {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::procedures::Procedure =
            self.insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
