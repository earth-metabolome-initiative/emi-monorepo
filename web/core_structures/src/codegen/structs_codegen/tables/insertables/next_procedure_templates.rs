#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum NextProcedureTemplateAttribute {
    Parent,
    Current,
    SuccessorId,
    CreatedBy,
    CreatedAt,
}
impl core::str::FromStr for NextProcedureTemplateAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Parent" => Ok(Self::Parent),
            "Current" => Ok(Self::Current),
            "SuccessorId" => Ok(Self::SuccessorId),
            "CreatedBy" => Ok(Self::CreatedBy),
            "CreatedAt" => Ok(Self::CreatedAt),
            "parent" => Ok(Self::Parent),
            "current" => Ok(Self::Current),
            "successor_id" => Ok(Self::SuccessorId),
            "created_by" => Ok(Self::CreatedBy),
            "created_at" => Ok(Self::CreatedAt),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl core::fmt::Display for NextProcedureTemplateAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Parent => write!(f, "parent"),
            Self::Current => write!(f, "current"),
            Self::SuccessorId => write!(f, "successor_id"),
            Self::CreatedBy => write!(f, "created_by"),
            Self::CreatedAt => write!(f, "created_at"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::next_procedure_templates::next_procedure_templates
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableNextProcedureTemplate {
    pub(crate) parent: i32,
    pub(crate) current: i32,
    pub(crate) successor_id: i32,
    pub(crate) created_by: i32,
    pub(crate) created_at: ::rosetta_timestamp::TimestampUTC,
}
impl InsertableNextProcedureTemplate {
    pub fn created_by<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::users::User,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::users::User: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::users::User as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::users::User as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::users::User as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::users::User as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::users::User as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::users::User as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::users::User,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::users::User::table(),
                self.created_by,
            ),
            conn,
        )
    }
    pub fn current<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate::table(),
                self.current,
            ),
            conn,
        )
    }
    pub fn next_procedure_templates_parent_current_fkey<
        C: diesel::connection::LoadConnection,
    >(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate::table(),
                (self.parent, self.current),
            ),
            conn,
        )
    }
    pub fn parent<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate::table(),
                self.parent,
            ),
            conn,
        )
    }
    pub fn next_procedure_templates_parent_successor_id_fkey<
        C: diesel::connection::LoadConnection,
    >(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate::table(),
                (self.parent, self.successor_id),
            ),
            conn,
        )
    }
    pub fn successor<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate::table(),
                self.successor_id,
            ),
            conn,
        )
    }
}
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableNextProcedureTemplateBuilder {
    pub(crate) parent: Option<i32>,
    pub(crate) current: Option<i32>,
    pub(crate) successor_id: Option<i32>,
    pub(crate) created_by: Option<i32>,
    pub(crate) created_at: Option<::rosetta_timestamp::TimestampUTC>,
}
impl Default for InsertableNextProcedureTemplateBuilder {
    fn default() -> Self {
        Self {
            parent: Default::default(),
            current: Default::default(),
            successor_id: Default::default(),
            created_by: Default::default(),
            created_at: Some(rosetta_timestamp::TimestampUTC::default()),
        }
    }
}
/// Trait defining setters for attributes of an instance of
/// `NextProcedureTemplate` or descendant tables.
pub trait NextProcedureTemplateSettable: Sized {
    /// Attributes required to build the insertable.
    type Attributes;
    /// Sets the value of the `public.next_procedure_templates.parent` column.
    ///
    /// # Arguments
    /// * `parent`: The value to set for the
    ///   `public.next_procedure_templates.parent` column.
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
    fn parent(
        self,
        parent: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the `public.next_procedure_templates.current` column.
    ///
    /// # Arguments
    /// * `current`: The value to set for the
    ///   `public.next_procedure_templates.current` column.
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
    fn current(
        self,
        current: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the `public.next_procedure_templates.successor_id`
    /// column.
    ///
    /// # Arguments
    /// * `successor_id`: The value to set for the
    ///   `public.next_procedure_templates.successor_id` column.
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
    fn successor(
        self,
        successor_id: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the `public.next_procedure_templates.created_by`
    /// column.
    ///
    /// # Arguments
    /// * `created_by`: The value to set for the
    ///   `public.next_procedure_templates.created_by` column.
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
    fn created_by(
        self,
        created_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the `public.next_procedure_templates.created_at`
    /// column.
    ///
    /// # Arguments
    /// * `created_at`: The value to set for the
    ///   `public.next_procedure_templates.created_at` column.
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
}
impl NextProcedureTemplateSettable for InsertableNextProcedureTemplateBuilder {
    type Attributes =
        crate::codegen::structs_codegen::tables::insertables::NextProcedureTemplateAttribute;
    /// Sets the value of the `public.next_procedure_templates.parent` column.
    fn parent(
        mut self,
        parent: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.parent = Some(parent);
        Ok(self)
    }
    /// Sets the value of the `public.next_procedure_templates.current` column.
    fn current(
        mut self,
        current: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let Some(successor_id) = self.successor_id {
            pgrx_validation::must_be_distinct_i32(current, successor_id)
                .map_err(|e| {
                    e
                        .rename_fields(
                            crate::codegen::structs_codegen::tables::insertables::NextProcedureTemplateAttribute::Current,
                            crate::codegen::structs_codegen::tables::insertables::NextProcedureTemplateAttribute::SuccessorId,
                        )
                })?;
        }
        self.current = Some(current);
        Ok(self)
    }
    /// Sets the value of the `public.next_procedure_templates.successor_id`
    /// column.
    fn successor(
        mut self,
        successor_id: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let Some(current) = self.current {
            pgrx_validation::must_be_distinct_i32(current, successor_id)
                .map_err(|e| {
                    e
                        .rename_fields(
                            crate::codegen::structs_codegen::tables::insertables::NextProcedureTemplateAttribute::Current,
                            crate::codegen::structs_codegen::tables::insertables::NextProcedureTemplateAttribute::SuccessorId,
                        )
                })?;
        }
        self.successor_id = Some(successor_id);
        Ok(self)
    }
    /// Sets the value of the `public.next_procedure_templates.created_by`
    /// column.
    fn created_by(
        mut self,
        created_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.created_by = Some(created_by);
        Ok(self)
    }
    /// Sets the value of the `public.next_procedure_templates.created_at`
    /// column.
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
                .rename_field(NextProcedureTemplateAttribute::CreatedAt)
        })?;
        self.created_at = Some(created_at);
        Ok(self)
    }
}
impl web_common_traits::prelude::SetPrimaryKey for InsertableNextProcedureTemplateBuilder {
    type PrimaryKey = (i32, i32, i32);
    fn set_primary_key(self, _primary_key: Self::PrimaryKey) -> Self {
        self
    }
}
impl<C> web_common_traits::database::TryInsertGeneric<C>
for InsertableNextProcedureTemplateBuilder
where
    Self: web_common_traits::database::InsertableVariant<
        C,
        UserId = i32,
        Row = crate::codegen::structs_codegen::tables::next_procedure_templates::NextProcedureTemplate,
        Error = web_common_traits::database::InsertError<NextProcedureTemplateAttribute>,
    >,
{
    type Attributes = NextProcedureTemplateAttribute;
    fn is_complete(&self) -> bool {
        self.parent.is_some() && self.current.is_some() && self.successor_id.is_some()
            && self.created_by.is_some() && self.created_at.is_some()
    }
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<
        Self::PrimaryKey,
        web_common_traits::database::InsertError<Self::Attributes>,
    > {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::next_procedure_templates::NextProcedureTemplate = self
            .insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
