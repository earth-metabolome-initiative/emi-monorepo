#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableParentProcedureTemplateAttributes {
    ParentProcedureTemplate,
    ChildProcedureTemplate,
    Snoozable,
    Copiable,
    Repeatable,
    Skippable,
    CreatedBy,
    CreatedAt,
}
impl core::str::FromStr for InsertableParentProcedureTemplateAttributes {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ParentProcedureTemplate" => Ok(Self::ParentProcedureTemplate),
            "ChildProcedureTemplate" => Ok(Self::ChildProcedureTemplate),
            "Snoozable" => Ok(Self::Snoozable),
            "Copiable" => Ok(Self::Copiable),
            "Repeatable" => Ok(Self::Repeatable),
            "Skippable" => Ok(Self::Skippable),
            "CreatedBy" => Ok(Self::CreatedBy),
            "CreatedAt" => Ok(Self::CreatedAt),
            "parent_procedure_template" => Ok(Self::ParentProcedureTemplate),
            "child_procedure_template" => Ok(Self::ChildProcedureTemplate),
            "snoozable" => Ok(Self::Snoozable),
            "copiable" => Ok(Self::Copiable),
            "repeatable" => Ok(Self::Repeatable),
            "skippable" => Ok(Self::Skippable),
            "created_by" => Ok(Self::CreatedBy),
            "created_at" => Ok(Self::CreatedAt),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl core::fmt::Display for InsertableParentProcedureTemplateAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::ParentProcedureTemplate => write!(f, "parent_procedure_template"),
            Self::ChildProcedureTemplate => write!(f, "child_procedure_template"),
            Self::Snoozable => write!(f, "snoozable"),
            Self::Copiable => write!(f, "copiable"),
            Self::Repeatable => write!(f, "repeatable"),
            Self::Skippable => write!(f, "skippable"),
            Self::CreatedBy => write!(f, "created_by"),
            Self::CreatedAt => write!(f, "created_at"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::parent_procedure_templates::parent_procedure_templates
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableParentProcedureTemplate {
    pub(crate) parent_procedure_template: i32,
    pub(crate) child_procedure_template: i32,
    pub(crate) snoozable: bool,
    pub(crate) copiable: bool,
    pub(crate) repeatable: bool,
    pub(crate) skippable: bool,
    pub(crate) created_by: i32,
    pub(crate) created_at: ::rosetta_timestamp::TimestampUTC,
}
impl InsertableParentProcedureTemplate {
    pub fn parent_procedure_template<C: diesel::connection::LoadConnection>(
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
                self.parent_procedure_template,
            ),
            conn,
        )
    }
    pub fn child_procedure_template<C: diesel::connection::LoadConnection>(
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
                self.child_procedure_template,
            ),
            conn,
        )
    }
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
}
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableParentProcedureTemplateBuilder {
    pub(crate) parent_procedure_template: Option<i32>,
    pub(crate) child_procedure_template: Option<i32>,
    pub(crate) snoozable: Option<bool>,
    pub(crate) copiable: Option<bool>,
    pub(crate) repeatable: Option<bool>,
    pub(crate) skippable: Option<bool>,
    pub(crate) created_by: Option<i32>,
    pub(crate) created_at: Option<::rosetta_timestamp::TimestampUTC>,
}
impl Default for InsertableParentProcedureTemplateBuilder {
    fn default() -> Self {
        Self {
            parent_procedure_template: Default::default(),
            child_procedure_template: Default::default(),
            snoozable: Some(false),
            copiable: Some(false),
            repeatable: Some(false),
            skippable: Some(false),
            created_by: Default::default(),
            created_at: Some(rosetta_timestamp::TimestampUTC::default()),
        }
    }
}
/// Trait defining setters for attributes of an instance of
/// `ParentProcedureTemplate` or descendant tables.
pub trait ParentProcedureTemplateBuildable: Sized {
    /// Attributes required to build the insertable.
    type Attributes;
    /// Sets the value of the
    /// `public.parent_procedure_templates.parent_procedure_template` column.
    ///
    /// # Arguments
    /// * `parent_procedure_template`: The value to set for the
    ///   `public.parent_procedure_templates.parent_procedure_template` column.
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
    fn parent_procedure_template(
        self,
        parent_procedure_template: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the
    /// `public.parent_procedure_templates.child_procedure_template` column.
    ///
    /// # Arguments
    /// * `child_procedure_template`: The value to set for the
    ///   `public.parent_procedure_templates.child_procedure_template` column.
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
    fn child_procedure_template(
        self,
        child_procedure_template: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the `public.parent_procedure_templates.snoozable`
    /// column.
    ///
    /// # Arguments
    /// * `snoozable`: The value to set for the
    ///   `public.parent_procedure_templates.snoozable` column.
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
    /// * If the provided value cannot be converted to the required type `bool`.
    /// * If the provided value does not pass schema-defined validation.
    fn snoozable<S>(
        self,
        snoozable: S,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        S: TryInto<bool>,
        validation_errors::SingleFieldError: From<<S as TryInto<bool>>::Error>;
    /// Sets the value of the `public.parent_procedure_templates.copiable`
    /// column.
    ///
    /// # Arguments
    /// * `copiable`: The value to set for the
    ///   `public.parent_procedure_templates.copiable` column.
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
    /// * If the provided value cannot be converted to the required type `bool`.
    /// * If the provided value does not pass schema-defined validation.
    fn copiable<C>(
        self,
        copiable: C,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        C: TryInto<bool>,
        validation_errors::SingleFieldError: From<<C as TryInto<bool>>::Error>;
    /// Sets the value of the `public.parent_procedure_templates.repeatable`
    /// column.
    ///
    /// # Arguments
    /// * `repeatable`: The value to set for the
    ///   `public.parent_procedure_templates.repeatable` column.
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
    /// * If the provided value cannot be converted to the required type `bool`.
    /// * If the provided value does not pass schema-defined validation.
    fn repeatable<R>(
        self,
        repeatable: R,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        R: TryInto<bool>,
        validation_errors::SingleFieldError: From<<R as TryInto<bool>>::Error>;
    /// Sets the value of the `public.parent_procedure_templates.skippable`
    /// column.
    ///
    /// # Arguments
    /// * `skippable`: The value to set for the
    ///   `public.parent_procedure_templates.skippable` column.
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
    /// * If the provided value cannot be converted to the required type `bool`.
    /// * If the provided value does not pass schema-defined validation.
    fn skippable<S>(
        self,
        skippable: S,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        S: TryInto<bool>,
        validation_errors::SingleFieldError: From<<S as TryInto<bool>>::Error>;
    /// Sets the value of the `public.parent_procedure_templates.created_by`
    /// column.
    ///
    /// # Arguments
    /// * `created_by`: The value to set for the
    ///   `public.parent_procedure_templates.created_by` column.
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
    /// Sets the value of the `public.parent_procedure_templates.created_at`
    /// column.
    ///
    /// # Arguments
    /// * `created_at`: The value to set for the
    ///   `public.parent_procedure_templates.created_at` column.
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
impl ParentProcedureTemplateBuildable for InsertableParentProcedureTemplateBuilder {
    type Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableParentProcedureTemplateAttributes;
    /// Sets the value of the
    /// `public.parent_procedure_templates.parent_procedure_template` column.
    fn parent_procedure_template(
        mut self,
        parent_procedure_template: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let Some(child_procedure_template) = self.child_procedure_template {
            pgrx_validation::must_be_distinct_i32(
                    parent_procedure_template,
                    child_procedure_template,
                )
                .map_err(|e| {
                    e
                        .rename_fields(
                            crate::codegen::structs_codegen::tables::insertables::InsertableParentProcedureTemplateAttributes::ParentProcedureTemplate,
                            crate::codegen::structs_codegen::tables::insertables::InsertableParentProcedureTemplateAttributes::ChildProcedureTemplate,
                        )
                })?;
        }
        self.parent_procedure_template = Some(parent_procedure_template);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.parent_procedure_templates.child_procedure_template` column.
    fn child_procedure_template(
        mut self,
        child_procedure_template: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let Some(parent_procedure_template) = self.parent_procedure_template {
            pgrx_validation::must_be_distinct_i32(
                    parent_procedure_template,
                    child_procedure_template,
                )
                .map_err(|e| {
                    e
                        .rename_fields(
                            crate::codegen::structs_codegen::tables::insertables::InsertableParentProcedureTemplateAttributes::ParentProcedureTemplate,
                            crate::codegen::structs_codegen::tables::insertables::InsertableParentProcedureTemplateAttributes::ChildProcedureTemplate,
                        )
                })?;
        }
        self.child_procedure_template = Some(child_procedure_template);
        Ok(self)
    }
    /// Sets the value of the `public.parent_procedure_templates.snoozable`
    /// column.
    fn snoozable<S>(
        mut self,
        snoozable: S,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        S: TryInto<bool>,
        validation_errors::SingleFieldError: From<<S as TryInto<bool>>::Error>,
    {
        let snoozable = snoozable.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(InsertableParentProcedureTemplateAttributes::Snoozable)
        })?;
        self.snoozable = Some(snoozable);
        Ok(self)
    }
    /// Sets the value of the `public.parent_procedure_templates.copiable`
    /// column.
    fn copiable<C>(
        mut self,
        copiable: C,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        C: TryInto<bool>,
        validation_errors::SingleFieldError: From<<C as TryInto<bool>>::Error>,
    {
        let copiable = copiable.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(InsertableParentProcedureTemplateAttributes::Copiable)
        })?;
        self.copiable = Some(copiable);
        Ok(self)
    }
    /// Sets the value of the `public.parent_procedure_templates.repeatable`
    /// column.
    fn repeatable<R>(
        mut self,
        repeatable: R,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        R: TryInto<bool>,
        validation_errors::SingleFieldError: From<<R as TryInto<bool>>::Error>,
    {
        let repeatable = repeatable.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(InsertableParentProcedureTemplateAttributes::Repeatable)
        })?;
        self.repeatable = Some(repeatable);
        Ok(self)
    }
    /// Sets the value of the `public.parent_procedure_templates.skippable`
    /// column.
    fn skippable<S>(
        mut self,
        skippable: S,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        S: TryInto<bool>,
        validation_errors::SingleFieldError: From<<S as TryInto<bool>>::Error>,
    {
        let skippable = skippable.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(InsertableParentProcedureTemplateAttributes::Skippable)
        })?;
        self.skippable = Some(skippable);
        Ok(self)
    }
    /// Sets the value of the `public.parent_procedure_templates.created_by`
    /// column.
    fn created_by(
        mut self,
        created_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.created_by = Some(created_by);
        Ok(self)
    }
    /// Sets the value of the `public.parent_procedure_templates.created_at`
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
                .rename_field(InsertableParentProcedureTemplateAttributes::CreatedAt)
        })?;
        self.created_at = Some(created_at);
        Ok(self)
    }
}
impl web_common_traits::prelude::SetPrimaryKey for InsertableParentProcedureTemplateBuilder {
    type PrimaryKey = (i32, i32);
    fn set_primary_key(self, _primary_key: Self::PrimaryKey) -> Self {
        self
    }
}
impl<C> web_common_traits::database::TryInsertGeneric<C>
for InsertableParentProcedureTemplateBuilder
where
    Self: web_common_traits::database::InsertableVariant<
        C,
        UserId = i32,
        Row = crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate,
        Error = web_common_traits::database::InsertError<
            InsertableParentProcedureTemplateAttributes,
        >,
    >,
{
    type Attributes = InsertableParentProcedureTemplateAttributes;
    fn is_complete(&self) -> bool {
        self.parent_procedure_template.is_some()
            && self.child_procedure_template.is_some() && self.snoozable.is_some()
            && self.copiable.is_some() && self.repeatable.is_some()
            && self.skippable.is_some() && self.created_by.is_some()
            && self.created_at.is_some()
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
        let insertable: crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate = self
            .insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
