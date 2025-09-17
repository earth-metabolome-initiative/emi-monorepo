#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ParentProcedureTemplateAttribute {
    Parent,
    Child,
    CreatedBy,
    CreatedAt,
}
impl core::str::FromStr for ParentProcedureTemplateAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Parent" => Ok(Self::Parent),
            "Child" => Ok(Self::Child),
            "CreatedBy" => Ok(Self::CreatedBy),
            "CreatedAt" => Ok(Self::CreatedAt),
            "parent" => Ok(Self::Parent),
            "child" => Ok(Self::Child),
            "created_by" => Ok(Self::CreatedBy),
            "created_at" => Ok(Self::CreatedAt),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl common_traits::builder::Attributed
for crate::codegen::structs_codegen::tables::insertables::InsertableParentProcedureTemplateBuilder {
    type Attribute = ParentProcedureTemplateAttribute;
}
impl core::fmt::Display for ParentProcedureTemplateAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Parent => write!(f, "parent_procedure_templates.parent"),
            Self::Child => write!(f, "parent_procedure_templates.child"),
            Self::CreatedBy => write!(f, "parent_procedure_templates.created_by"),
            Self::CreatedAt => write!(f, "parent_procedure_templates.created_at"),
        }
    }
}
#[derive(Debug)]
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::parent_procedure_templates::parent_procedure_templates
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableParentProcedureTemplate {
    pub(crate) parent: i32,
    pub(crate) child: i32,
    pub(crate) created_by: i32,
    pub(crate) created_at: ::rosetta_timestamp::TimestampUTC,
}
impl InsertableParentProcedureTemplate {
    pub fn child<C: diesel::connection::LoadConnection>(
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
            self.child, conn,
        )
    }
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
    pub fn parent<C: diesel::connection::LoadConnection>(
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
            self.parent,
            conn,
        )
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Hash, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Builder for creating and inserting a new
/// [`ParentProcedureTemplate`](crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate).
///
///
/// # Implementation details
/// While this builder implements several methods, a reasonably complete
/// **basic** usage example (*which may not apply to your own specific use case,
/// please adapt accordingly*) is as follows:
///
/// ```rust,ignore
/// use core_structures::ParentProcedureTemplate;
/// use core_structures::tables::insertables::ParentProcedureTemplateSettable;
/// use web_common_traits::database::Insertable;
/// use web_common_traits::database::InsertableVariant;
///
/// let parent_procedure_template = ParentProcedureTemplate::new()
///    // Set mandatory fields
///    .child(child)?
///    .created_by(created_by)?
///    .parent(parent)?
///    // Optionally set fields with default values
///    .created_at(created_at)?
///    // Finally, insert the new record in the database
///    .insert(user.id, conn)?;
/// ```
pub struct InsertableParentProcedureTemplateBuilder {
    pub(crate) parent: Option<i32>,
    pub(crate) child: Option<i32>,
    pub(crate) created_by: Option<i32>,
    pub(crate) created_at: Option<::rosetta_timestamp::TimestampUTC>,
}
impl diesel::associations::HasTable for InsertableParentProcedureTemplateBuilder {
    type Table = crate::codegen::diesel_codegen::tables::parent_procedure_templates::parent_procedure_templates::table;
    fn table() -> Self::Table {
        crate::codegen::diesel_codegen::tables::parent_procedure_templates::parent_procedure_templates::table
    }
}
impl Default for InsertableParentProcedureTemplateBuilder {
    fn default() -> Self {
        Self {
            parent: Default::default(),
            child: Default::default(),
            created_by: Default::default(),
            created_at: Some(rosetta_timestamp::TimestampUTC::default()),
        }
    }
}
impl common_traits::builder::IsCompleteBuilder
for crate::codegen::structs_codegen::tables::insertables::InsertableParentProcedureTemplateBuilder {
    fn is_complete(&self) -> bool {
        self.parent.is_some() && self.child.is_some() && self.created_by.is_some()
            && self.created_at.is_some()
    }
}
/// Trait defining setters for attributes of an instance of
/// `ParentProcedureTemplate` or descendant tables.
pub trait ParentProcedureTemplateSettable: Sized {
    /// Error type returned when setting attributes.
    type Error;
    /// Sets the value of the `public.parent_procedure_templates.parent` column.
    ///
    /// # Arguments
    /// * `parent`: The value to set for the
    ///   `public.parent_procedure_templates.parent` column.
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
    fn parent<P>(self, parent: P) -> Result<Self, Self::Error>
    where
        P: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
    /// Sets the value of the `public.parent_procedure_templates.child` column.
    ///
    /// # Arguments
    /// * `child`: The value to set for the
    ///   `public.parent_procedure_templates.child` column.
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
    fn child<C>(self, child: C) -> Result<Self, Self::Error>
    where
        C: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
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
    fn created_by<CB>(self, created_by: CB) -> Result<Self, Self::Error>
    where
        CB: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
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
    fn created_at<CA>(self, created_at: CA) -> Result<Self, Self::Error>
    where
        CA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::SingleFieldError:
            From<<CA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error>;
}
impl ParentProcedureTemplateSettable for InsertableParentProcedureTemplateBuilder
where
    Self: common_traits::builder::Attributed<
        Attribute = crate::codegen::structs_codegen::tables::insertables::ParentProcedureTemplateAttribute,
    >,
{
    type Error = web_common_traits::database::InsertError<
        <Self as common_traits::builder::Attributed>::Attribute,
    >;
    ///Sets the value of the `public.parent_procedure_templates.parent` column.
    fn parent<P>(mut self, parent: P) -> Result<Self, Self::Error>
    where
        P: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let parent = <P as web_common_traits::database::PrimaryKeyLike>::primary_key(
            &parent,
        );
        if let Some(child) = self.child {
            pgrx_validation::must_be_distinct_i32(parent, child)
                .map_err(|e| {
                    e
                        .rename_fields(
                            crate::codegen::structs_codegen::tables::insertables::ParentProcedureTemplateAttribute::Parent,
                            crate::codegen::structs_codegen::tables::insertables::ParentProcedureTemplateAttribute::Child,
                        )
                })?;
        }
        self.parent = Some(parent);
        Ok(self)
    }
    ///Sets the value of the `public.parent_procedure_templates.child` column.
    fn child<C>(mut self, child: C) -> Result<Self, Self::Error>
    where
        C: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let child = <C as web_common_traits::database::PrimaryKeyLike>::primary_key(
            &child,
        );
        if let Some(parent) = self.parent {
            pgrx_validation::must_be_distinct_i32(parent, child)
                .map_err(|e| {
                    e
                        .rename_fields(
                            crate::codegen::structs_codegen::tables::insertables::ParentProcedureTemplateAttribute::Parent,
                            crate::codegen::structs_codegen::tables::insertables::ParentProcedureTemplateAttribute::Child,
                        )
                })?;
        }
        self.child = Some(child);
        Ok(self)
    }
    ///Sets the value of the `public.parent_procedure_templates.created_by` column.
    fn created_by<CB>(mut self, created_by: CB) -> Result<Self, Self::Error>
    where
        CB: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let created_by = <CB as web_common_traits::database::PrimaryKeyLike>::primary_key(
            &created_by,
        );
        self.created_by = Some(created_by);
        Ok(self)
    }
    ///Sets the value of the `public.parent_procedure_templates.created_at` column.
    fn created_at<CA>(mut self, created_at: CA) -> Result<Self, Self::Error>
    where
        CA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::SingleFieldError: From<
            <CA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error,
        >,
    {
        let created_at = created_at
            .try_into()
            .map_err(|err| {
                validation_errors::SingleFieldError::from(err)
                    .rename_field(ParentProcedureTemplateAttribute::CreatedAt)
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
    Self: web_common_traits::database::DispatchableInsertableVariant<
        C,
        Row = crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate,
        Error = web_common_traits::database::InsertError<
            ParentProcedureTemplateAttribute,
        >,
    >,
{
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<
        Self::PrimaryKey,
        web_common_traits::database::InsertError<ParentProcedureTemplateAttribute>,
    > {
        use diesel::Identifiable;
        use web_common_traits::database::DispatchableInsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate = self
            .insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
