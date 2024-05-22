use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct EmptyFilter;

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct BioOttRankFilter {
    pub icon_id: Option<i32>,
    pub color_id: Option<i32>,
}


#[cfg(feature = "frontend")]
impl BioOttRankFilter {

    pub fn as_filter_expression(&self) -> gluesql::core::ast_builder::ExprNode<'_> {
        let mut filter: gluesql::core::ast_builder::ExprNode<'_> = gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into();
        if let Some(icon_id) = &self.icon_id {
            filter = filter.and(gluesql::core::ast_builder::col("bio_ott_ranks.icon_id").eq(icon_id.to_string()));
        }

        if let Some(color_id) = &self.color_id {
            filter = filter.and(gluesql::core::ast_builder::col("bio_ott_ranks.color_id").eq(color_id.to_string()));
        }

        filter
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct BioOttTaxonItemFilter {
    pub ott_rank_id: Option<i32>,
    pub domain_id: Option<i32>,
    pub kingdom_id: Option<i32>,
    pub phylum_id: Option<i32>,
    pub class_id: Option<i32>,
    pub order_id: Option<i32>,
    pub family_id: Option<i32>,
    pub genus_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub icon_id: Option<i32>,
    pub color_id: Option<i32>,
}


#[cfg(feature = "frontend")]
impl BioOttTaxonItemFilter {

    pub fn as_filter_expression(&self) -> gluesql::core::ast_builder::ExprNode<'_> {
        let mut filter: gluesql::core::ast_builder::ExprNode<'_> = gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into();
        if let Some(ott_rank_id) = &self.ott_rank_id {
            filter = filter.and(gluesql::core::ast_builder::col("bio_ott_taxon_items.ott_rank_id").eq(ott_rank_id.to_string()));
        }

        if let Some(domain_id) = &self.domain_id {
            filter = filter.and(gluesql::core::ast_builder::col("bio_ott_taxon_items.domain_id").eq(domain_id.to_string()));
        }

        if let Some(kingdom_id) = &self.kingdom_id {
            filter = filter.and(gluesql::core::ast_builder::col("bio_ott_taxon_items.kingdom_id").eq(kingdom_id.to_string()));
        }

        if let Some(phylum_id) = &self.phylum_id {
            filter = filter.and(gluesql::core::ast_builder::col("bio_ott_taxon_items.phylum_id").eq(phylum_id.to_string()));
        }

        if let Some(class_id) = &self.class_id {
            filter = filter.and(gluesql::core::ast_builder::col("bio_ott_taxon_items.class_id").eq(class_id.to_string()));
        }

        if let Some(order_id) = &self.order_id {
            filter = filter.and(gluesql::core::ast_builder::col("bio_ott_taxon_items.order_id").eq(order_id.to_string()));
        }

        if let Some(family_id) = &self.family_id {
            filter = filter.and(gluesql::core::ast_builder::col("bio_ott_taxon_items.family_id").eq(family_id.to_string()));
        }

        if let Some(genus_id) = &self.genus_id {
            filter = filter.and(gluesql::core::ast_builder::col("bio_ott_taxon_items.genus_id").eq(genus_id.to_string()));
        }

        if let Some(parent_id) = &self.parent_id {
            filter = filter.and(gluesql::core::ast_builder::col("bio_ott_taxon_items.parent_id").eq(parent_id.to_string()));
        }

        if let Some(icon_id) = &self.icon_id {
            filter = filter.and(gluesql::core::ast_builder::col("bio_ott_taxon_items.icon_id").eq(icon_id.to_string()));
        }

        if let Some(color_id) = &self.color_id {
            filter = filter.and(gluesql::core::ast_builder::col("bio_ott_taxon_items.color_id").eq(color_id.to_string()));
        }

        filter
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct DerivedSampleFilter {
    pub created_by: Option<i32>,
    pub parent_sample_id: Option<Uuid>,
    pub child_sample_id: Option<Uuid>,
}


#[cfg(feature = "frontend")]
impl DerivedSampleFilter {

    pub fn as_filter_expression(&self) -> gluesql::core::ast_builder::ExprNode<'_> {
        let mut filter: gluesql::core::ast_builder::ExprNode<'_> = gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into();
        if let Some(created_by) = &self.created_by {
            filter = filter.and(gluesql::core::ast_builder::col("derived_samples.created_by").eq(created_by.to_string()));
        }

        if let Some(parent_sample_id) = &self.parent_sample_id {
            filter = filter.and(gluesql::core::ast_builder::col("derived_samples.parent_sample_id").eq(parent_sample_id.to_string()));
        }

        if let Some(child_sample_id) = &self.child_sample_id {
            filter = filter.and(gluesql::core::ast_builder::col("derived_samples.child_sample_id").eq(child_sample_id.to_string()));
        }

        filter
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct DocumentFormatFilter {
    pub icon_id: Option<i32>,
    pub color_id: Option<i32>,
}


#[cfg(feature = "frontend")]
impl DocumentFormatFilter {

    pub fn as_filter_expression(&self) -> gluesql::core::ast_builder::ExprNode<'_> {
        let mut filter: gluesql::core::ast_builder::ExprNode<'_> = gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into();
        if let Some(icon_id) = &self.icon_id {
            filter = filter.and(gluesql::core::ast_builder::col("document_formats.icon_id").eq(icon_id.to_string()));
        }

        if let Some(color_id) = &self.color_id {
            filter = filter.and(gluesql::core::ast_builder::col("document_formats.color_id").eq(color_id.to_string()));
        }

        filter
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct LoginProviderFilter {
    pub icon_id: Option<i32>,
    pub color_id: Option<i32>,
}


#[cfg(feature = "frontend")]
impl LoginProviderFilter {

    pub fn as_filter_expression(&self) -> gluesql::core::ast_builder::ExprNode<'_> {
        let mut filter: gluesql::core::ast_builder::ExprNode<'_> = gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into();
        if let Some(icon_id) = &self.icon_id {
            filter = filter.and(gluesql::core::ast_builder::col("login_providers.icon_id").eq(icon_id.to_string()));
        }

        if let Some(color_id) = &self.color_id {
            filter = filter.and(gluesql::core::ast_builder::col("login_providers.color_id").eq(color_id.to_string()));
        }

        filter
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct NotificationFilter {
    pub user_id: Option<i32>,
}


#[cfg(feature = "frontend")]
impl NotificationFilter {

    pub fn as_filter_expression(&self) -> gluesql::core::ast_builder::ExprNode<'_> {
        let mut filter: gluesql::core::ast_builder::ExprNode<'_> = gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into();
        if let Some(user_id) = &self.user_id {
            filter = filter.and(gluesql::core::ast_builder::col("notifications.user_id").eq(user_id.to_string()));
        }

        filter
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct ObservationFilter {
    pub created_by: Option<i32>,
    pub updated_by: Option<i32>,
    pub project_id: Option<i32>,
    pub individual_id: Option<Uuid>,
}


#[cfg(feature = "frontend")]
impl ObservationFilter {

    pub fn as_filter_expression(&self) -> gluesql::core::ast_builder::ExprNode<'_> {
        let mut filter: gluesql::core::ast_builder::ExprNode<'_> = gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into();
        if let Some(created_by) = &self.created_by {
            filter = filter.and(gluesql::core::ast_builder::col("observations.created_by").eq(created_by.to_string()));
        }

        if let Some(updated_by) = &self.updated_by {
            filter = filter.and(gluesql::core::ast_builder::col("observations.updated_by").eq(updated_by.to_string()));
        }

        if let Some(project_id) = &self.project_id {
            filter = filter.and(gluesql::core::ast_builder::col("observations.project_id").eq(project_id.to_string()));
        }

        if let Some(individual_id) = &self.individual_id {
            filter = filter.and(gluesql::core::ast_builder::col("observations.individual_id").eq(individual_id.to_string()));
        }

        filter
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct OrganizationFilter {
    pub country_id: Option<i32>,
}


#[cfg(feature = "frontend")]
impl OrganizationFilter {

    pub fn as_filter_expression(&self) -> gluesql::core::ast_builder::ExprNode<'_> {
        let mut filter: gluesql::core::ast_builder::ExprNode<'_> = gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into();
        if let Some(country_id) = &self.country_id {
            filter = filter.and(gluesql::core::ast_builder::col("organizations.country_id").eq(country_id.to_string()));
        }

        filter
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct ProjectStateFilter {
    pub icon_id: Option<i32>,
    pub color_id: Option<i32>,
}


#[cfg(feature = "frontend")]
impl ProjectStateFilter {

    pub fn as_filter_expression(&self) -> gluesql::core::ast_builder::ExprNode<'_> {
        let mut filter: gluesql::core::ast_builder::ExprNode<'_> = gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into();
        if let Some(icon_id) = &self.icon_id {
            filter = filter.and(gluesql::core::ast_builder::col("project_states.icon_id").eq(icon_id.to_string()));
        }

        if let Some(color_id) = &self.color_id {
            filter = filter.and(gluesql::core::ast_builder::col("project_states.color_id").eq(color_id.to_string()));
        }

        filter
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Default, Eq)]
pub struct ProjectFilter {
    pub state_id: Option<i32>,
    pub icon_id: Option<i32>,
    pub color_id: Option<i32>,
    pub parent_project_id: Option<i32>,
    pub created_by: Option<i32>,
    pub updated_by: Option<i32>,
}


#[cfg(feature = "frontend")]
impl ProjectFilter {

    pub fn as_filter_expression(&self) -> gluesql::core::ast_builder::ExprNode<'_> {
        let mut filter: gluesql::core::ast_builder::ExprNode<'_> = gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into();
        if let Some(state_id) = &self.state_id {
            filter = filter.and(gluesql::core::ast_builder::col("projects.state_id").eq(state_id.to_string()));
        }

        if let Some(icon_id) = &self.icon_id {
            filter = filter.and(gluesql::core::ast_builder::col("projects.icon_id").eq(icon_id.to_string()));
        }

        if let Some(color_id) = &self.color_id {
            filter = filter.and(gluesql::core::ast_builder::col("projects.color_id").eq(color_id.to_string()));
        }

        if let Some(parent_project_id) = &self.parent_project_id {
            filter = filter.and(gluesql::core::ast_builder::col("projects.parent_project_id").eq(parent_project_id.to_string()));
        }

        if let Some(created_by) = &self.created_by {
            filter = filter.and(gluesql::core::ast_builder::col("projects.created_by").eq(created_by.to_string()));
        }

        if let Some(updated_by) = &self.updated_by {
            filter = filter.and(gluesql::core::ast_builder::col("projects.updated_by").eq(updated_by.to_string()));
        }

        filter
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct ProjectsTeamsRoleInvitationFilter {
    pub table_id: Option<i32>,
    pub team_id: Option<i32>,
    pub role_id: Option<i32>,
    pub created_by: Option<i32>,
}


#[cfg(feature = "frontend")]
impl ProjectsTeamsRoleInvitationFilter {

    pub fn as_filter_expression(&self) -> gluesql::core::ast_builder::ExprNode<'_> {
        let mut filter: gluesql::core::ast_builder::ExprNode<'_> = gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into();
        if let Some(table_id) = &self.table_id {
            filter = filter.and(gluesql::core::ast_builder::col("projects_teams_role_invitations.table_id").eq(table_id.to_string()));
        }

        if let Some(team_id) = &self.team_id {
            filter = filter.and(gluesql::core::ast_builder::col("projects_teams_role_invitations.team_id").eq(team_id.to_string()));
        }

        if let Some(role_id) = &self.role_id {
            filter = filter.and(gluesql::core::ast_builder::col("projects_teams_role_invitations.role_id").eq(role_id.to_string()));
        }

        if let Some(created_by) = &self.created_by {
            filter = filter.and(gluesql::core::ast_builder::col("projects_teams_role_invitations.created_by").eq(created_by.to_string()));
        }

        filter
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct ProjectsTeamsRoleRequestFilter {
    pub table_id: Option<i32>,
    pub team_id: Option<i32>,
    pub role_id: Option<i32>,
    pub created_by: Option<i32>,
}


#[cfg(feature = "frontend")]
impl ProjectsTeamsRoleRequestFilter {

    pub fn as_filter_expression(&self) -> gluesql::core::ast_builder::ExprNode<'_> {
        let mut filter: gluesql::core::ast_builder::ExprNode<'_> = gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into();
        if let Some(table_id) = &self.table_id {
            filter = filter.and(gluesql::core::ast_builder::col("projects_teams_role_requests.table_id").eq(table_id.to_string()));
        }

        if let Some(team_id) = &self.team_id {
            filter = filter.and(gluesql::core::ast_builder::col("projects_teams_role_requests.team_id").eq(team_id.to_string()));
        }

        if let Some(role_id) = &self.role_id {
            filter = filter.and(gluesql::core::ast_builder::col("projects_teams_role_requests.role_id").eq(role_id.to_string()));
        }

        if let Some(created_by) = &self.created_by {
            filter = filter.and(gluesql::core::ast_builder::col("projects_teams_role_requests.created_by").eq(created_by.to_string()));
        }

        filter
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct ProjectsTeamsRoleFilter {
    pub table_id: Option<i32>,
    pub team_id: Option<i32>,
    pub role_id: Option<i32>,
    pub created_by: Option<i32>,
}


#[cfg(feature = "frontend")]
impl ProjectsTeamsRoleFilter {

    pub fn as_filter_expression(&self) -> gluesql::core::ast_builder::ExprNode<'_> {
        let mut filter: gluesql::core::ast_builder::ExprNode<'_> = gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into();
        if let Some(table_id) = &self.table_id {
            filter = filter.and(gluesql::core::ast_builder::col("projects_teams_roles.table_id").eq(table_id.to_string()));
        }

        if let Some(team_id) = &self.team_id {
            filter = filter.and(gluesql::core::ast_builder::col("projects_teams_roles.team_id").eq(team_id.to_string()));
        }

        if let Some(role_id) = &self.role_id {
            filter = filter.and(gluesql::core::ast_builder::col("projects_teams_roles.role_id").eq(role_id.to_string()));
        }

        if let Some(created_by) = &self.created_by {
            filter = filter.and(gluesql::core::ast_builder::col("projects_teams_roles.created_by").eq(created_by.to_string()));
        }

        filter
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct ProjectsUsersRoleInvitationFilter {
    pub table_id: Option<i32>,
    pub user_id: Option<i32>,
    pub role_id: Option<i32>,
    pub created_by: Option<i32>,
}


#[cfg(feature = "frontend")]
impl ProjectsUsersRoleInvitationFilter {

    pub fn as_filter_expression(&self) -> gluesql::core::ast_builder::ExprNode<'_> {
        let mut filter: gluesql::core::ast_builder::ExprNode<'_> = gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into();
        if let Some(table_id) = &self.table_id {
            filter = filter.and(gluesql::core::ast_builder::col("projects_users_role_invitations.table_id").eq(table_id.to_string()));
        }

        if let Some(user_id) = &self.user_id {
            filter = filter.and(gluesql::core::ast_builder::col("projects_users_role_invitations.user_id").eq(user_id.to_string()));
        }

        if let Some(role_id) = &self.role_id {
            filter = filter.and(gluesql::core::ast_builder::col("projects_users_role_invitations.role_id").eq(role_id.to_string()));
        }

        if let Some(created_by) = &self.created_by {
            filter = filter.and(gluesql::core::ast_builder::col("projects_users_role_invitations.created_by").eq(created_by.to_string()));
        }

        filter
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct ProjectsUsersRoleRequestFilter {
    pub table_id: Option<i32>,
    pub user_id: Option<i32>,
    pub role_id: Option<i32>,
    pub created_by: Option<i32>,
}


#[cfg(feature = "frontend")]
impl ProjectsUsersRoleRequestFilter {

    pub fn as_filter_expression(&self) -> gluesql::core::ast_builder::ExprNode<'_> {
        let mut filter: gluesql::core::ast_builder::ExprNode<'_> = gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into();
        if let Some(table_id) = &self.table_id {
            filter = filter.and(gluesql::core::ast_builder::col("projects_users_role_requests.table_id").eq(table_id.to_string()));
        }

        if let Some(user_id) = &self.user_id {
            filter = filter.and(gluesql::core::ast_builder::col("projects_users_role_requests.user_id").eq(user_id.to_string()));
        }

        if let Some(role_id) = &self.role_id {
            filter = filter.and(gluesql::core::ast_builder::col("projects_users_role_requests.role_id").eq(role_id.to_string()));
        }

        if let Some(created_by) = &self.created_by {
            filter = filter.and(gluesql::core::ast_builder::col("projects_users_role_requests.created_by").eq(created_by.to_string()));
        }

        filter
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct ProjectsUsersRoleFilter {
    pub table_id: Option<i32>,
    pub user_id: Option<i32>,
    pub role_id: Option<i32>,
    pub created_by: Option<i32>,
}


#[cfg(feature = "frontend")]
impl ProjectsUsersRoleFilter {

    pub fn as_filter_expression(&self) -> gluesql::core::ast_builder::ExprNode<'_> {
        let mut filter: gluesql::core::ast_builder::ExprNode<'_> = gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into();
        if let Some(table_id) = &self.table_id {
            filter = filter.and(gluesql::core::ast_builder::col("projects_users_roles.table_id").eq(table_id.to_string()));
        }

        if let Some(user_id) = &self.user_id {
            filter = filter.and(gluesql::core::ast_builder::col("projects_users_roles.user_id").eq(user_id.to_string()));
        }

        if let Some(role_id) = &self.role_id {
            filter = filter.and(gluesql::core::ast_builder::col("projects_users_roles.role_id").eq(role_id.to_string()));
        }

        if let Some(created_by) = &self.created_by {
            filter = filter.and(gluesql::core::ast_builder::col("projects_users_roles.created_by").eq(created_by.to_string()));
        }

        filter
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct RoleFilter {
    pub icon_id: Option<i32>,
    pub color_id: Option<i32>,
}


#[cfg(feature = "frontend")]
impl RoleFilter {

    pub fn as_filter_expression(&self) -> gluesql::core::ast_builder::ExprNode<'_> {
        let mut filter: gluesql::core::ast_builder::ExprNode<'_> = gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into();
        if let Some(icon_id) = &self.icon_id {
            filter = filter.and(gluesql::core::ast_builder::col("roles.icon_id").eq(icon_id.to_string()));
        }

        if let Some(color_id) = &self.color_id {
            filter = filter.and(gluesql::core::ast_builder::col("roles.color_id").eq(color_id.to_string()));
        }

        filter
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct SampleBioOttTaxonItemFilter {
    pub created_by: Option<i32>,
    pub sample_id: Option<Uuid>,
    pub taxon_id: Option<i32>,
}


#[cfg(feature = "frontend")]
impl SampleBioOttTaxonItemFilter {

    pub fn as_filter_expression(&self) -> gluesql::core::ast_builder::ExprNode<'_> {
        let mut filter: gluesql::core::ast_builder::ExprNode<'_> = gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into();
        if let Some(created_by) = &self.created_by {
            filter = filter.and(gluesql::core::ast_builder::col("sample_bio_ott_taxon_items.created_by").eq(created_by.to_string()));
        }

        if let Some(sample_id) = &self.sample_id {
            filter = filter.and(gluesql::core::ast_builder::col("sample_bio_ott_taxon_items.sample_id").eq(sample_id.to_string()));
        }

        if let Some(taxon_id) = &self.taxon_id {
            filter = filter.and(gluesql::core::ast_builder::col("sample_bio_ott_taxon_items.taxon_id").eq(taxon_id.to_string()));
        }

        filter
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct SampleContainerCategoryFilter {
    pub icon_id: Option<i32>,
    pub color_id: Option<i32>,
}


#[cfg(feature = "frontend")]
impl SampleContainerCategoryFilter {

    pub fn as_filter_expression(&self) -> gluesql::core::ast_builder::ExprNode<'_> {
        let mut filter: gluesql::core::ast_builder::ExprNode<'_> = gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into();
        if let Some(icon_id) = &self.icon_id {
            filter = filter.and(gluesql::core::ast_builder::col("sample_container_categories.icon_id").eq(icon_id.to_string()));
        }

        if let Some(color_id) = &self.color_id {
            filter = filter.and(gluesql::core::ast_builder::col("sample_container_categories.color_id").eq(color_id.to_string()));
        }

        filter
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct SampleContainerFilter {
    pub category_id: Option<i32>,
    pub created_by: Option<i32>,
}


#[cfg(feature = "frontend")]
impl SampleContainerFilter {

    pub fn as_filter_expression(&self) -> gluesql::core::ast_builder::ExprNode<'_> {
        let mut filter: gluesql::core::ast_builder::ExprNode<'_> = gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into();
        if let Some(category_id) = &self.category_id {
            filter = filter.and(gluesql::core::ast_builder::col("sample_containers.category_id").eq(category_id.to_string()));
        }

        if let Some(created_by) = &self.created_by {
            filter = filter.and(gluesql::core::ast_builder::col("sample_containers.created_by").eq(created_by.to_string()));
        }

        filter
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct SampleStateFilter {
    pub icon_id: Option<i32>,
    pub color_id: Option<i32>,
}


#[cfg(feature = "frontend")]
impl SampleStateFilter {

    pub fn as_filter_expression(&self) -> gluesql::core::ast_builder::ExprNode<'_> {
        let mut filter: gluesql::core::ast_builder::ExprNode<'_> = gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into();
        if let Some(icon_id) = &self.icon_id {
            filter = filter.and(gluesql::core::ast_builder::col("sample_states.icon_id").eq(icon_id.to_string()));
        }

        if let Some(color_id) = &self.color_id {
            filter = filter.and(gluesql::core::ast_builder::col("sample_states.color_id").eq(color_id.to_string()));
        }

        filter
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct SampledIndividualBioOttTaxonItemFilter {
    pub created_by: Option<i32>,
    pub sampled_individual_id: Option<Uuid>,
    pub taxon_id: Option<i32>,
}


#[cfg(feature = "frontend")]
impl SampledIndividualBioOttTaxonItemFilter {

    pub fn as_filter_expression(&self) -> gluesql::core::ast_builder::ExprNode<'_> {
        let mut filter: gluesql::core::ast_builder::ExprNode<'_> = gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into();
        if let Some(created_by) = &self.created_by {
            filter = filter.and(gluesql::core::ast_builder::col("sampled_individual_bio_ott_taxon_items.created_by").eq(created_by.to_string()));
        }

        if let Some(sampled_individual_id) = &self.sampled_individual_id {
            filter = filter.and(gluesql::core::ast_builder::col("sampled_individual_bio_ott_taxon_items.sampled_individual_id").eq(sampled_individual_id.to_string()));
        }

        if let Some(taxon_id) = &self.taxon_id {
            filter = filter.and(gluesql::core::ast_builder::col("sampled_individual_bio_ott_taxon_items.taxon_id").eq(taxon_id.to_string()));
        }

        filter
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct SampledIndividualFilter {
    pub project_id: Option<i32>,
    pub created_by: Option<i32>,
    pub updated_by: Option<i32>,
}


#[cfg(feature = "frontend")]
impl SampledIndividualFilter {

    pub fn as_filter_expression(&self) -> gluesql::core::ast_builder::ExprNode<'_> {
        let mut filter: gluesql::core::ast_builder::ExprNode<'_> = gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into();
        if let Some(project_id) = &self.project_id {
            filter = filter.and(gluesql::core::ast_builder::col("sampled_individuals.project_id").eq(project_id.to_string()));
        }

        if let Some(created_by) = &self.created_by {
            filter = filter.and(gluesql::core::ast_builder::col("sampled_individuals.created_by").eq(created_by.to_string()));
        }

        if let Some(updated_by) = &self.updated_by {
            filter = filter.and(gluesql::core::ast_builder::col("sampled_individuals.updated_by").eq(updated_by.to_string()));
        }

        filter
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct SampleFilter {
    pub container_id: Option<i32>,
    pub created_by: Option<i32>,
    pub sampled_by: Option<i32>,
    pub updated_by: Option<i32>,
    pub state: Option<i32>,
}


#[cfg(feature = "frontend")]
impl SampleFilter {

    pub fn as_filter_expression(&self) -> gluesql::core::ast_builder::ExprNode<'_> {
        let mut filter: gluesql::core::ast_builder::ExprNode<'_> = gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into();
        if let Some(container_id) = &self.container_id {
            filter = filter.and(gluesql::core::ast_builder::col("samples.container_id").eq(container_id.to_string()));
        }

        if let Some(created_by) = &self.created_by {
            filter = filter.and(gluesql::core::ast_builder::col("samples.created_by").eq(created_by.to_string()));
        }

        if let Some(sampled_by) = &self.sampled_by {
            filter = filter.and(gluesql::core::ast_builder::col("samples.sampled_by").eq(sampled_by.to_string()));
        }

        if let Some(updated_by) = &self.updated_by {
            filter = filter.and(gluesql::core::ast_builder::col("samples.updated_by").eq(updated_by.to_string()));
        }

        if let Some(state) = &self.state {
            filter = filter.and(gluesql::core::ast_builder::col("samples.state").eq(state.to_string()));
        }

        filter
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct SamplesTeamsRoleInvitationFilter {
    pub table_id: Option<Uuid>,
    pub team_id: Option<i32>,
    pub role_id: Option<i32>,
    pub created_by: Option<i32>,
}


#[cfg(feature = "frontend")]
impl SamplesTeamsRoleInvitationFilter {

    pub fn as_filter_expression(&self) -> gluesql::core::ast_builder::ExprNode<'_> {
        let mut filter: gluesql::core::ast_builder::ExprNode<'_> = gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into();
        if let Some(table_id) = &self.table_id {
            filter = filter.and(gluesql::core::ast_builder::col("samples_teams_role_invitations.table_id").eq(table_id.to_string()));
        }

        if let Some(team_id) = &self.team_id {
            filter = filter.and(gluesql::core::ast_builder::col("samples_teams_role_invitations.team_id").eq(team_id.to_string()));
        }

        if let Some(role_id) = &self.role_id {
            filter = filter.and(gluesql::core::ast_builder::col("samples_teams_role_invitations.role_id").eq(role_id.to_string()));
        }

        if let Some(created_by) = &self.created_by {
            filter = filter.and(gluesql::core::ast_builder::col("samples_teams_role_invitations.created_by").eq(created_by.to_string()));
        }

        filter
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct SamplesTeamsRoleRequestFilter {
    pub table_id: Option<Uuid>,
    pub team_id: Option<i32>,
    pub role_id: Option<i32>,
    pub created_by: Option<i32>,
}


#[cfg(feature = "frontend")]
impl SamplesTeamsRoleRequestFilter {

    pub fn as_filter_expression(&self) -> gluesql::core::ast_builder::ExprNode<'_> {
        let mut filter: gluesql::core::ast_builder::ExprNode<'_> = gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into();
        if let Some(table_id) = &self.table_id {
            filter = filter.and(gluesql::core::ast_builder::col("samples_teams_role_requests.table_id").eq(table_id.to_string()));
        }

        if let Some(team_id) = &self.team_id {
            filter = filter.and(gluesql::core::ast_builder::col("samples_teams_role_requests.team_id").eq(team_id.to_string()));
        }

        if let Some(role_id) = &self.role_id {
            filter = filter.and(gluesql::core::ast_builder::col("samples_teams_role_requests.role_id").eq(role_id.to_string()));
        }

        if let Some(created_by) = &self.created_by {
            filter = filter.and(gluesql::core::ast_builder::col("samples_teams_role_requests.created_by").eq(created_by.to_string()));
        }

        filter
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct SamplesTeamsRoleFilter {
    pub table_id: Option<Uuid>,
    pub team_id: Option<i32>,
    pub role_id: Option<i32>,
    pub created_by: Option<i32>,
}


#[cfg(feature = "frontend")]
impl SamplesTeamsRoleFilter {

    pub fn as_filter_expression(&self) -> gluesql::core::ast_builder::ExprNode<'_> {
        let mut filter: gluesql::core::ast_builder::ExprNode<'_> = gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into();
        if let Some(table_id) = &self.table_id {
            filter = filter.and(gluesql::core::ast_builder::col("samples_teams_roles.table_id").eq(table_id.to_string()));
        }

        if let Some(team_id) = &self.team_id {
            filter = filter.and(gluesql::core::ast_builder::col("samples_teams_roles.team_id").eq(team_id.to_string()));
        }

        if let Some(role_id) = &self.role_id {
            filter = filter.and(gluesql::core::ast_builder::col("samples_teams_roles.role_id").eq(role_id.to_string()));
        }

        if let Some(created_by) = &self.created_by {
            filter = filter.and(gluesql::core::ast_builder::col("samples_teams_roles.created_by").eq(created_by.to_string()));
        }

        filter
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct SamplesUsersRoleInvitationFilter {
    pub table_id: Option<Uuid>,
    pub user_id: Option<i32>,
    pub role_id: Option<i32>,
    pub created_by: Option<i32>,
}


#[cfg(feature = "frontend")]
impl SamplesUsersRoleInvitationFilter {

    pub fn as_filter_expression(&self) -> gluesql::core::ast_builder::ExprNode<'_> {
        let mut filter: gluesql::core::ast_builder::ExprNode<'_> = gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into();
        if let Some(table_id) = &self.table_id {
            filter = filter.and(gluesql::core::ast_builder::col("samples_users_role_invitations.table_id").eq(table_id.to_string()));
        }

        if let Some(user_id) = &self.user_id {
            filter = filter.and(gluesql::core::ast_builder::col("samples_users_role_invitations.user_id").eq(user_id.to_string()));
        }

        if let Some(role_id) = &self.role_id {
            filter = filter.and(gluesql::core::ast_builder::col("samples_users_role_invitations.role_id").eq(role_id.to_string()));
        }

        if let Some(created_by) = &self.created_by {
            filter = filter.and(gluesql::core::ast_builder::col("samples_users_role_invitations.created_by").eq(created_by.to_string()));
        }

        filter
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct SamplesUsersRoleRequestFilter {
    pub table_id: Option<Uuid>,
    pub user_id: Option<i32>,
    pub role_id: Option<i32>,
    pub created_by: Option<i32>,
}


#[cfg(feature = "frontend")]
impl SamplesUsersRoleRequestFilter {

    pub fn as_filter_expression(&self) -> gluesql::core::ast_builder::ExprNode<'_> {
        let mut filter: gluesql::core::ast_builder::ExprNode<'_> = gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into();
        if let Some(table_id) = &self.table_id {
            filter = filter.and(gluesql::core::ast_builder::col("samples_users_role_requests.table_id").eq(table_id.to_string()));
        }

        if let Some(user_id) = &self.user_id {
            filter = filter.and(gluesql::core::ast_builder::col("samples_users_role_requests.user_id").eq(user_id.to_string()));
        }

        if let Some(role_id) = &self.role_id {
            filter = filter.and(gluesql::core::ast_builder::col("samples_users_role_requests.role_id").eq(role_id.to_string()));
        }

        if let Some(created_by) = &self.created_by {
            filter = filter.and(gluesql::core::ast_builder::col("samples_users_role_requests.created_by").eq(created_by.to_string()));
        }

        filter
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct SamplesUsersRoleFilter {
    pub table_id: Option<Uuid>,
    pub user_id: Option<i32>,
    pub role_id: Option<i32>,
    pub created_by: Option<i32>,
}


#[cfg(feature = "frontend")]
impl SamplesUsersRoleFilter {

    pub fn as_filter_expression(&self) -> gluesql::core::ast_builder::ExprNode<'_> {
        let mut filter: gluesql::core::ast_builder::ExprNode<'_> = gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into();
        if let Some(table_id) = &self.table_id {
            filter = filter.and(gluesql::core::ast_builder::col("samples_users_roles.table_id").eq(table_id.to_string()));
        }

        if let Some(user_id) = &self.user_id {
            filter = filter.and(gluesql::core::ast_builder::col("samples_users_roles.user_id").eq(user_id.to_string()));
        }

        if let Some(role_id) = &self.role_id {
            filter = filter.and(gluesql::core::ast_builder::col("samples_users_roles.role_id").eq(role_id.to_string()));
        }

        if let Some(created_by) = &self.created_by {
            filter = filter.and(gluesql::core::ast_builder::col("samples_users_roles.created_by").eq(created_by.to_string()));
        }

        filter
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct SpectraFilter {
    pub spectra_collection_id: Option<i32>,
}


#[cfg(feature = "frontend")]
impl SpectraFilter {

    pub fn as_filter_expression(&self) -> gluesql::core::ast_builder::ExprNode<'_> {
        let mut filter: gluesql::core::ast_builder::ExprNode<'_> = gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into();
        if let Some(spectra_collection_id) = &self.spectra_collection_id {
            filter = filter.and(gluesql::core::ast_builder::col("spectra.spectra_collection_id").eq(spectra_collection_id.to_string()));
        }

        filter
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct SpectraCollectionFilter {
    pub sample_id: Option<Uuid>,
    pub created_by: Option<i32>,
    pub updated_by: Option<i32>,
}


#[cfg(feature = "frontend")]
impl SpectraCollectionFilter {

    pub fn as_filter_expression(&self) -> gluesql::core::ast_builder::ExprNode<'_> {
        let mut filter: gluesql::core::ast_builder::ExprNode<'_> = gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into();
        if let Some(sample_id) = &self.sample_id {
            filter = filter.and(gluesql::core::ast_builder::col("spectra_collections.sample_id").eq(sample_id.to_string()));
        }

        if let Some(created_by) = &self.created_by {
            filter = filter.and(gluesql::core::ast_builder::col("spectra_collections.created_by").eq(created_by.to_string()));
        }

        if let Some(updated_by) = &self.updated_by {
            filter = filter.and(gluesql::core::ast_builder::col("spectra_collections.updated_by").eq(updated_by.to_string()));
        }

        filter
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct SpectraCollectionsTeamsRoleInvitationFilter {
    pub table_id: Option<i32>,
    pub team_id: Option<i32>,
    pub role_id: Option<i32>,
    pub created_by: Option<i32>,
}


#[cfg(feature = "frontend")]
impl SpectraCollectionsTeamsRoleInvitationFilter {

    pub fn as_filter_expression(&self) -> gluesql::core::ast_builder::ExprNode<'_> {
        let mut filter: gluesql::core::ast_builder::ExprNode<'_> = gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into();
        if let Some(table_id) = &self.table_id {
            filter = filter.and(gluesql::core::ast_builder::col("spectra_collections_teams_role_invitations.table_id").eq(table_id.to_string()));
        }

        if let Some(team_id) = &self.team_id {
            filter = filter.and(gluesql::core::ast_builder::col("spectra_collections_teams_role_invitations.team_id").eq(team_id.to_string()));
        }

        if let Some(role_id) = &self.role_id {
            filter = filter.and(gluesql::core::ast_builder::col("spectra_collections_teams_role_invitations.role_id").eq(role_id.to_string()));
        }

        if let Some(created_by) = &self.created_by {
            filter = filter.and(gluesql::core::ast_builder::col("spectra_collections_teams_role_invitations.created_by").eq(created_by.to_string()));
        }

        filter
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct SpectraCollectionsTeamsRoleRequestFilter {
    pub table_id: Option<i32>,
    pub team_id: Option<i32>,
    pub role_id: Option<i32>,
    pub created_by: Option<i32>,
}


#[cfg(feature = "frontend")]
impl SpectraCollectionsTeamsRoleRequestFilter {

    pub fn as_filter_expression(&self) -> gluesql::core::ast_builder::ExprNode<'_> {
        let mut filter: gluesql::core::ast_builder::ExprNode<'_> = gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into();
        if let Some(table_id) = &self.table_id {
            filter = filter.and(gluesql::core::ast_builder::col("spectra_collections_teams_role_requests.table_id").eq(table_id.to_string()));
        }

        if let Some(team_id) = &self.team_id {
            filter = filter.and(gluesql::core::ast_builder::col("spectra_collections_teams_role_requests.team_id").eq(team_id.to_string()));
        }

        if let Some(role_id) = &self.role_id {
            filter = filter.and(gluesql::core::ast_builder::col("spectra_collections_teams_role_requests.role_id").eq(role_id.to_string()));
        }

        if let Some(created_by) = &self.created_by {
            filter = filter.and(gluesql::core::ast_builder::col("spectra_collections_teams_role_requests.created_by").eq(created_by.to_string()));
        }

        filter
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct SpectraCollectionsTeamsRoleFilter {
    pub table_id: Option<i32>,
    pub team_id: Option<i32>,
    pub role_id: Option<i32>,
    pub created_by: Option<i32>,
}


#[cfg(feature = "frontend")]
impl SpectraCollectionsTeamsRoleFilter {

    pub fn as_filter_expression(&self) -> gluesql::core::ast_builder::ExprNode<'_> {
        let mut filter: gluesql::core::ast_builder::ExprNode<'_> = gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into();
        if let Some(table_id) = &self.table_id {
            filter = filter.and(gluesql::core::ast_builder::col("spectra_collections_teams_roles.table_id").eq(table_id.to_string()));
        }

        if let Some(team_id) = &self.team_id {
            filter = filter.and(gluesql::core::ast_builder::col("spectra_collections_teams_roles.team_id").eq(team_id.to_string()));
        }

        if let Some(role_id) = &self.role_id {
            filter = filter.and(gluesql::core::ast_builder::col("spectra_collections_teams_roles.role_id").eq(role_id.to_string()));
        }

        if let Some(created_by) = &self.created_by {
            filter = filter.and(gluesql::core::ast_builder::col("spectra_collections_teams_roles.created_by").eq(created_by.to_string()));
        }

        filter
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct SpectraCollectionsUsersRoleInvitationFilter {
    pub table_id: Option<i32>,
    pub user_id: Option<i32>,
    pub role_id: Option<i32>,
    pub created_by: Option<i32>,
}


#[cfg(feature = "frontend")]
impl SpectraCollectionsUsersRoleInvitationFilter {

    pub fn as_filter_expression(&self) -> gluesql::core::ast_builder::ExprNode<'_> {
        let mut filter: gluesql::core::ast_builder::ExprNode<'_> = gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into();
        if let Some(table_id) = &self.table_id {
            filter = filter.and(gluesql::core::ast_builder::col("spectra_collections_users_role_invitations.table_id").eq(table_id.to_string()));
        }

        if let Some(user_id) = &self.user_id {
            filter = filter.and(gluesql::core::ast_builder::col("spectra_collections_users_role_invitations.user_id").eq(user_id.to_string()));
        }

        if let Some(role_id) = &self.role_id {
            filter = filter.and(gluesql::core::ast_builder::col("spectra_collections_users_role_invitations.role_id").eq(role_id.to_string()));
        }

        if let Some(created_by) = &self.created_by {
            filter = filter.and(gluesql::core::ast_builder::col("spectra_collections_users_role_invitations.created_by").eq(created_by.to_string()));
        }

        filter
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct SpectraCollectionsUsersRoleRequestFilter {
    pub table_id: Option<i32>,
    pub user_id: Option<i32>,
    pub role_id: Option<i32>,
    pub created_by: Option<i32>,
}


#[cfg(feature = "frontend")]
impl SpectraCollectionsUsersRoleRequestFilter {

    pub fn as_filter_expression(&self) -> gluesql::core::ast_builder::ExprNode<'_> {
        let mut filter: gluesql::core::ast_builder::ExprNode<'_> = gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into();
        if let Some(table_id) = &self.table_id {
            filter = filter.and(gluesql::core::ast_builder::col("spectra_collections_users_role_requests.table_id").eq(table_id.to_string()));
        }

        if let Some(user_id) = &self.user_id {
            filter = filter.and(gluesql::core::ast_builder::col("spectra_collections_users_role_requests.user_id").eq(user_id.to_string()));
        }

        if let Some(role_id) = &self.role_id {
            filter = filter.and(gluesql::core::ast_builder::col("spectra_collections_users_role_requests.role_id").eq(role_id.to_string()));
        }

        if let Some(created_by) = &self.created_by {
            filter = filter.and(gluesql::core::ast_builder::col("spectra_collections_users_role_requests.created_by").eq(created_by.to_string()));
        }

        filter
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct SpectraCollectionsUsersRoleFilter {
    pub table_id: Option<i32>,
    pub user_id: Option<i32>,
    pub role_id: Option<i32>,
    pub created_by: Option<i32>,
}


#[cfg(feature = "frontend")]
impl SpectraCollectionsUsersRoleFilter {

    pub fn as_filter_expression(&self) -> gluesql::core::ast_builder::ExprNode<'_> {
        let mut filter: gluesql::core::ast_builder::ExprNode<'_> = gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into();
        if let Some(table_id) = &self.table_id {
            filter = filter.and(gluesql::core::ast_builder::col("spectra_collections_users_roles.table_id").eq(table_id.to_string()));
        }

        if let Some(user_id) = &self.user_id {
            filter = filter.and(gluesql::core::ast_builder::col("spectra_collections_users_roles.user_id").eq(user_id.to_string()));
        }

        if let Some(role_id) = &self.role_id {
            filter = filter.and(gluesql::core::ast_builder::col("spectra_collections_users_roles.role_id").eq(role_id.to_string()));
        }

        if let Some(created_by) = &self.created_by {
            filter = filter.and(gluesql::core::ast_builder::col("spectra_collections_users_roles.created_by").eq(created_by.to_string()));
        }

        filter
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct TeamStateFilter {
    pub icon_id: Option<i32>,
    pub color_id: Option<i32>,
}


#[cfg(feature = "frontend")]
impl TeamStateFilter {

    pub fn as_filter_expression(&self) -> gluesql::core::ast_builder::ExprNode<'_> {
        let mut filter: gluesql::core::ast_builder::ExprNode<'_> = gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into();
        if let Some(icon_id) = &self.icon_id {
            filter = filter.and(gluesql::core::ast_builder::col("team_states.icon_id").eq(icon_id.to_string()));
        }

        if let Some(color_id) = &self.color_id {
            filter = filter.and(gluesql::core::ast_builder::col("team_states.color_id").eq(color_id.to_string()));
        }

        filter
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct TeamFilter {
    pub icon_id: Option<i32>,
    pub color_id: Option<i32>,
    pub parent_team_id: Option<i32>,
    pub created_by: Option<i32>,
    pub updated_by: Option<i32>,
}


#[cfg(feature = "frontend")]
impl TeamFilter {

    pub fn as_filter_expression(&self) -> gluesql::core::ast_builder::ExprNode<'_> {
        let mut filter: gluesql::core::ast_builder::ExprNode<'_> = gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into();
        if let Some(icon_id) = &self.icon_id {
            filter = filter.and(gluesql::core::ast_builder::col("teams.icon_id").eq(icon_id.to_string()));
        }

        if let Some(color_id) = &self.color_id {
            filter = filter.and(gluesql::core::ast_builder::col("teams.color_id").eq(color_id.to_string()));
        }

        if let Some(parent_team_id) = &self.parent_team_id {
            filter = filter.and(gluesql::core::ast_builder::col("teams.parent_team_id").eq(parent_team_id.to_string()));
        }

        if let Some(created_by) = &self.created_by {
            filter = filter.and(gluesql::core::ast_builder::col("teams.created_by").eq(created_by.to_string()));
        }

        if let Some(updated_by) = &self.updated_by {
            filter = filter.and(gluesql::core::ast_builder::col("teams.updated_by").eq(updated_by.to_string()));
        }

        filter
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct TeamsTeamsRoleInvitationFilter {
    pub table_id: Option<i32>,
    pub team_id: Option<i32>,
    pub role_id: Option<i32>,
    pub created_by: Option<i32>,
}


#[cfg(feature = "frontend")]
impl TeamsTeamsRoleInvitationFilter {

    pub fn as_filter_expression(&self) -> gluesql::core::ast_builder::ExprNode<'_> {
        let mut filter: gluesql::core::ast_builder::ExprNode<'_> = gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into();
        if let Some(table_id) = &self.table_id {
            filter = filter.and(gluesql::core::ast_builder::col("teams_teams_role_invitations.table_id").eq(table_id.to_string()));
        }

        if let Some(team_id) = &self.team_id {
            filter = filter.and(gluesql::core::ast_builder::col("teams_teams_role_invitations.team_id").eq(team_id.to_string()));
        }

        if let Some(role_id) = &self.role_id {
            filter = filter.and(gluesql::core::ast_builder::col("teams_teams_role_invitations.role_id").eq(role_id.to_string()));
        }

        if let Some(created_by) = &self.created_by {
            filter = filter.and(gluesql::core::ast_builder::col("teams_teams_role_invitations.created_by").eq(created_by.to_string()));
        }

        filter
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct TeamsUsersRoleInvitationFilter {
    pub table_id: Option<i32>,
    pub user_id: Option<i32>,
    pub role_id: Option<i32>,
    pub created_by: Option<i32>,
}


#[cfg(feature = "frontend")]
impl TeamsUsersRoleInvitationFilter {

    pub fn as_filter_expression(&self) -> gluesql::core::ast_builder::ExprNode<'_> {
        let mut filter: gluesql::core::ast_builder::ExprNode<'_> = gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into();
        if let Some(table_id) = &self.table_id {
            filter = filter.and(gluesql::core::ast_builder::col("teams_users_role_invitations.table_id").eq(table_id.to_string()));
        }

        if let Some(user_id) = &self.user_id {
            filter = filter.and(gluesql::core::ast_builder::col("teams_users_role_invitations.user_id").eq(user_id.to_string()));
        }

        if let Some(role_id) = &self.role_id {
            filter = filter.and(gluesql::core::ast_builder::col("teams_users_role_invitations.role_id").eq(role_id.to_string()));
        }

        if let Some(created_by) = &self.created_by {
            filter = filter.and(gluesql::core::ast_builder::col("teams_users_role_invitations.created_by").eq(created_by.to_string()));
        }

        filter
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct TeamsUsersRoleRequestFilter {
    pub table_id: Option<i32>,
    pub user_id: Option<i32>,
    pub role_id: Option<i32>,
    pub created_by: Option<i32>,
}


#[cfg(feature = "frontend")]
impl TeamsUsersRoleRequestFilter {

    pub fn as_filter_expression(&self) -> gluesql::core::ast_builder::ExprNode<'_> {
        let mut filter: gluesql::core::ast_builder::ExprNode<'_> = gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into();
        if let Some(table_id) = &self.table_id {
            filter = filter.and(gluesql::core::ast_builder::col("teams_users_role_requests.table_id").eq(table_id.to_string()));
        }

        if let Some(user_id) = &self.user_id {
            filter = filter.and(gluesql::core::ast_builder::col("teams_users_role_requests.user_id").eq(user_id.to_string()));
        }

        if let Some(role_id) = &self.role_id {
            filter = filter.and(gluesql::core::ast_builder::col("teams_users_role_requests.role_id").eq(role_id.to_string()));
        }

        if let Some(created_by) = &self.created_by {
            filter = filter.and(gluesql::core::ast_builder::col("teams_users_role_requests.created_by").eq(created_by.to_string()));
        }

        filter
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct TeamsUsersRoleFilter {
    pub table_id: Option<i32>,
    pub user_id: Option<i32>,
    pub role_id: Option<i32>,
    pub created_by: Option<i32>,
}


#[cfg(feature = "frontend")]
impl TeamsUsersRoleFilter {

    pub fn as_filter_expression(&self) -> gluesql::core::ast_builder::ExprNode<'_> {
        let mut filter: gluesql::core::ast_builder::ExprNode<'_> = gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into();
        if let Some(table_id) = &self.table_id {
            filter = filter.and(gluesql::core::ast_builder::col("teams_users_roles.table_id").eq(table_id.to_string()));
        }

        if let Some(user_id) = &self.user_id {
            filter = filter.and(gluesql::core::ast_builder::col("teams_users_roles.user_id").eq(user_id.to_string()));
        }

        if let Some(role_id) = &self.role_id {
            filter = filter.and(gluesql::core::ast_builder::col("teams_users_roles.role_id").eq(role_id.to_string()));
        }

        if let Some(created_by) = &self.created_by {
            filter = filter.and(gluesql::core::ast_builder::col("teams_users_roles.created_by").eq(created_by.to_string()));
        }

        filter
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct UserEmailFilter {
    pub created_by: Option<i32>,
    pub login_provider_id: Option<i32>,
}


#[cfg(feature = "frontend")]
impl UserEmailFilter {

    pub fn as_filter_expression(&self) -> gluesql::core::ast_builder::ExprNode<'_> {
        let mut filter: gluesql::core::ast_builder::ExprNode<'_> = gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into();
        if let Some(created_by) = &self.created_by {
            filter = filter.and(gluesql::core::ast_builder::col("user_emails.created_by").eq(created_by.to_string()));
        }

        if let Some(login_provider_id) = &self.login_provider_id {
            filter = filter.and(gluesql::core::ast_builder::col("user_emails.login_provider_id").eq(login_provider_id.to_string()));
        }

        filter
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct UsersUsersRoleInvitationFilter {
    pub table_id: Option<i32>,
    pub user_id: Option<i32>,
    pub role_id: Option<i32>,
    pub created_by: Option<i32>,
}


#[cfg(feature = "frontend")]
impl UsersUsersRoleInvitationFilter {

    pub fn as_filter_expression(&self) -> gluesql::core::ast_builder::ExprNode<'_> {
        let mut filter: gluesql::core::ast_builder::ExprNode<'_> = gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into();
        if let Some(table_id) = &self.table_id {
            filter = filter.and(gluesql::core::ast_builder::col("users_users_role_invitations.table_id").eq(table_id.to_string()));
        }

        if let Some(user_id) = &self.user_id {
            filter = filter.and(gluesql::core::ast_builder::col("users_users_role_invitations.user_id").eq(user_id.to_string()));
        }

        if let Some(role_id) = &self.role_id {
            filter = filter.and(gluesql::core::ast_builder::col("users_users_role_invitations.role_id").eq(role_id.to_string()));
        }

        if let Some(created_by) = &self.created_by {
            filter = filter.and(gluesql::core::ast_builder::col("users_users_role_invitations.created_by").eq(created_by.to_string()));
        }

        filter
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct UsersUsersRoleRequestFilter {
    pub table_id: Option<i32>,
    pub user_id: Option<i32>,
    pub role_id: Option<i32>,
    pub created_by: Option<i32>,
}


#[cfg(feature = "frontend")]
impl UsersUsersRoleRequestFilter {

    pub fn as_filter_expression(&self) -> gluesql::core::ast_builder::ExprNode<'_> {
        let mut filter: gluesql::core::ast_builder::ExprNode<'_> = gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into();
        if let Some(table_id) = &self.table_id {
            filter = filter.and(gluesql::core::ast_builder::col("users_users_role_requests.table_id").eq(table_id.to_string()));
        }

        if let Some(user_id) = &self.user_id {
            filter = filter.and(gluesql::core::ast_builder::col("users_users_role_requests.user_id").eq(user_id.to_string()));
        }

        if let Some(role_id) = &self.role_id {
            filter = filter.and(gluesql::core::ast_builder::col("users_users_role_requests.role_id").eq(role_id.to_string()));
        }

        if let Some(created_by) = &self.created_by {
            filter = filter.and(gluesql::core::ast_builder::col("users_users_role_requests.created_by").eq(created_by.to_string()));
        }

        filter
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct UsersUsersRoleFilter {
    pub table_id: Option<i32>,
    pub user_id: Option<i32>,
    pub role_id: Option<i32>,
    pub created_by: Option<i32>,
}


#[cfg(feature = "frontend")]
impl UsersUsersRoleFilter {

    pub fn as_filter_expression(&self) -> gluesql::core::ast_builder::ExprNode<'_> {
        let mut filter: gluesql::core::ast_builder::ExprNode<'_> = gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into();
        if let Some(table_id) = &self.table_id {
            filter = filter.and(gluesql::core::ast_builder::col("users_users_roles.table_id").eq(table_id.to_string()));
        }

        if let Some(user_id) = &self.user_id {
            filter = filter.and(gluesql::core::ast_builder::col("users_users_roles.user_id").eq(user_id.to_string()));
        }

        if let Some(role_id) = &self.role_id {
            filter = filter.and(gluesql::core::ast_builder::col("users_users_roles.role_id").eq(role_id.to_string()));
        }

        if let Some(created_by) = &self.created_by {
            filter = filter.and(gluesql::core::ast_builder::col("users_users_roles.created_by").eq(created_by.to_string()));
        }

        filter
    }
}

