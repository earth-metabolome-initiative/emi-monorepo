#[derive(Clone, core :: fmt :: Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableProjectAttributes {
    Id,
    Name,
    Description,
    ParentProjectId,
    Budget,
    Expenses,
    CreatedBy,
    UpdatedBy,
    ExpectedEndDate,
    EndDate,
    Public,
    StateId,
    IconId,
    ColorId,
}
impl core::fmt::Display for InsertableProjectAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableProjectAttributes::Id => write!(f, "id"),
            InsertableProjectAttributes::Name => write!(f, "name"),
            InsertableProjectAttributes::Description => write!(f, "description"),
            InsertableProjectAttributes::ParentProjectId => write!(f, "parent_project_id"),
            InsertableProjectAttributes::Budget => write!(f, "budget"),
            InsertableProjectAttributes::Expenses => write!(f, "expenses"),
            InsertableProjectAttributes::CreatedBy => write!(f, "created_by"),
            InsertableProjectAttributes::UpdatedBy => write!(f, "updated_by"),
            InsertableProjectAttributes::ExpectedEndDate => write!(f, "expected_end_date"),
            InsertableProjectAttributes::EndDate => write!(f, "end_date"),
            InsertableProjectAttributes::Public => write!(f, "public"),
            InsertableProjectAttributes::StateId => write!(f, "state_id"),
            InsertableProjectAttributes::IconId => write!(f, "icon_id"),
            InsertableProjectAttributes::ColorId => write!(f, "color_id"),
        }
    }
}
#[cfg_attr(feature = "diesel", derive(diesel::Insertable))]
# [cfg_attr (feature = "diesel" , diesel (table_name = crate :: codegen :: diesel_codegen :: tables :: projects :: projects))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableProject {
    id: i32,
    name: String,
    description: String,
    parent_project_id: Option<i32>,
    budget: Option<f64>,
    expenses: Option<f64>,
    created_by: i32,
    updated_by: i32,
    expected_end_date: Option<chrono::NaiveDateTime>,
    end_date: Option<chrono::NaiveDateTime>,
    public: bool,
    state_id: i16,
    icon_id: i16,
    color_id: i16,
}
#[derive(Default)]
pub struct InsertableProjectBuilder {
    id: Option<i32>,
    name: Option<String>,
    description: Option<String>,
    parent_project_id: Option<i32>,
    budget: Option<f64>,
    expenses: Option<f64>,
    created_by: Option<i32>,
    updated_by: Option<i32>,
    expected_end_date: Option<chrono::NaiveDateTime>,
    end_date: Option<chrono::NaiveDateTime>,
    public: Option<bool>,
    state_id: Option<i16>,
    icon_id: Option<i16>,
    color_id: Option<i16>,
}
impl InsertableProjectBuilder {
    pub fn id(mut self, id: i32) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.id = Some(id);
        Ok(self)
    }
    pub fn name(
        mut self,
        name: String,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.name = Some(name);
        Ok(self)
    }
    pub fn description(
        mut self,
        description: String,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.description = Some(description);
        Ok(self)
    }
    pub fn parent_project_id(
        mut self,
        parent_project_id: Option<i32>,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.parent_project_id = parent_project_id;
        Ok(self)
    }
    pub fn budget(
        mut self,
        budget: Option<f64>,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.budget = budget;
        Ok(self)
    }
    pub fn expenses(
        mut self,
        expenses: Option<f64>,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.expenses = expenses;
        Ok(self)
    }
    pub fn created_by(
        mut self,
        created_by: i32,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.created_by = Some(created_by);
        Ok(self)
    }
    pub fn updated_by(
        mut self,
        updated_by: i32,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.updated_by = Some(updated_by);
        Ok(self)
    }
    pub fn expected_end_date(
        mut self,
        expected_end_date: Option<chrono::NaiveDateTime>,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.expected_end_date = expected_end_date;
        Ok(self)
    }
    pub fn end_date(
        mut self,
        end_date: Option<chrono::NaiveDateTime>,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.end_date = end_date;
        Ok(self)
    }
    pub fn public(
        mut self,
        public: bool,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.public = Some(public);
        Ok(self)
    }
    pub fn state_id(
        mut self,
        state_id: i16,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.state_id = Some(state_id);
        Ok(self)
    }
    pub fn icon_id(
        mut self,
        icon_id: i16,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.icon_id = Some(icon_id);
        Ok(self)
    }
    pub fn color_id(
        mut self,
        color_id: i16,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.color_id = Some(color_id);
        Ok(self)
    }
}
impl common_traits::prelude::Builder for InsertableProjectBuilder {
    type Error = web_common_traits::database::InsertError<InsertableProjectAttributes>;
    type Object = InsertableProject;
    type Attribute = InsertableProjectAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            id: self.id.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableProjectAttributes::Id,
                )
            })?,
            name: self.name.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableProjectAttributes::Name,
                )
            })?,
            description: self.description.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableProjectAttributes::Description,
                )
            })?,
            parent_project_id: self.parent_project_id,
            budget: self.budget,
            expenses: self.expenses,
            created_by: self.created_by.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableProjectAttributes::CreatedBy,
                )
            })?,
            updated_by: self.updated_by.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableProjectAttributes::UpdatedBy,
                )
            })?,
            expected_end_date: self.expected_end_date,
            end_date: self.end_date,
            public: self.public.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableProjectAttributes::Public,
                )
            })?,
            state_id: self.state_id.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableProjectAttributes::StateId,
                )
            })?,
            icon_id: self.icon_id.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableProjectAttributes::IconId,
                )
            })?,
            color_id: self.color_id.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableProjectAttributes::ColorId,
                )
            })?,
        })
    }
}
impl TryFrom<InsertableProject> for InsertableProjectBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableProject) -> Result<Self, Self::Error> {
        Ok(Self::default()
            .id(insertable_variant.id)?
            .name(insertable_variant.name)?
            .description(insertable_variant.description)?
            .parent_project_id(insertable_variant.parent_project_id)?
            .budget(insertable_variant.budget)?
            .expenses(insertable_variant.expenses)?
            .created_by(insertable_variant.created_by)?
            .updated_by(insertable_variant.updated_by)?
            .expected_end_date(insertable_variant.expected_end_date)?
            .end_date(insertable_variant.end_date)?
            .public(insertable_variant.public)?
            .state_id(insertable_variant.state_id)?
            .icon_id(insertable_variant.icon_id)?
            .color_id(insertable_variant.color_id)?)
    }
}
