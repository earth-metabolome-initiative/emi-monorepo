use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct EmptyFilter;

#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default, Eq, PartialOrd, Copy, Ord)]
pub struct BioOttRankFilter {
    pub icon_id: Option<i32>,
    pub color_id: Option<i32>,
}

unsafe impl Send for BioOttRankFilter {}
unsafe impl Sync for BioOttRankFilter {}

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

#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default, Eq, PartialOrd, Copy, Ord)]
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

unsafe impl Send for BioOttTaxonItemFilter {}
unsafe impl Sync for BioOttTaxonItemFilter {}

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

#[derive(PartialEq, Debug, Clone, Copy, serde::Serialize, serde::Deserialize, Default, Eq)]
pub struct DerivedSampleFilter {
    pub created_by: Option<i32>,
    pub updated_by: Option<i32>,
    pub parent_sample_id: Option<uuid::Uuid>,
    pub child_sample_id: Option<uuid::Uuid>,
    pub unit_id: Option<i32>,
}

unsafe impl Send for DerivedSampleFilter {}
unsafe impl Sync for DerivedSampleFilter {}

#[cfg(feature = "frontend")]
impl DerivedSampleFilter {

    pub fn as_filter_expression(&self) -> gluesql::core::ast_builder::ExprNode<'_> {
        let mut filter: gluesql::core::ast_builder::ExprNode<'_> = gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into();
        if let Some(created_by) = &self.created_by {
            filter = filter.and(gluesql::core::ast_builder::col("derived_samples.created_by").eq(created_by.to_string()));
        }

        if let Some(updated_by) = &self.updated_by {
            filter = filter.and(gluesql::core::ast_builder::col("derived_samples.updated_by").eq(updated_by.to_string()));
        }

        if let Some(parent_sample_id) = &self.parent_sample_id {
            filter = filter.and(gluesql::core::ast_builder::col("derived_samples.parent_sample_id").eq(parent_sample_id.to_string()));
        }

        if let Some(child_sample_id) = &self.child_sample_id {
            filter = filter.and(gluesql::core::ast_builder::col("derived_samples.child_sample_id").eq(child_sample_id.to_string()));
        }

        if let Some(unit_id) = &self.unit_id {
            filter = filter.and(gluesql::core::ast_builder::col("derived_samples.unit_id").eq(unit_id.to_string()));
        }

        filter
    }
}

#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default, Eq, PartialOrd, Copy, Ord)]
pub struct DocumentFormatFilter {
    pub icon_id: Option<i32>,
    pub color_id: Option<i32>,
}

unsafe impl Send for DocumentFormatFilter {}
unsafe impl Sync for DocumentFormatFilter {}

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

#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default, Eq, PartialOrd, Copy, Ord)]
pub struct LoginProviderFilter {
    pub icon_id: Option<i32>,
    pub color_id: Option<i32>,
}

unsafe impl Send for LoginProviderFilter {}
unsafe impl Sync for LoginProviderFilter {}

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

#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default, Eq, PartialOrd, Copy, Ord)]
pub struct MaterialFilter {
    pub icon_id: Option<i32>,
    pub color_id: Option<i32>,
}

unsafe impl Send for MaterialFilter {}
unsafe impl Sync for MaterialFilter {}

#[cfg(feature = "frontend")]
impl MaterialFilter {

    pub fn as_filter_expression(&self) -> gluesql::core::ast_builder::ExprNode<'_> {
        let mut filter: gluesql::core::ast_builder::ExprNode<'_> = gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into();
        if let Some(icon_id) = &self.icon_id {
            filter = filter.and(gluesql::core::ast_builder::col("materials.icon_id").eq(icon_id.to_string()));
        }

        if let Some(color_id) = &self.color_id {
            filter = filter.and(gluesql::core::ast_builder::col("materials.color_id").eq(color_id.to_string()));
        }

        filter
    }
}

#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default, Eq, PartialOrd, Copy, Ord)]
pub struct NameplateCategoryFilter {
    pub permanence_id: Option<i32>,
    pub material_id: Option<i32>,
    pub icon_id: Option<i32>,
    pub color_id: Option<i32>,
}

unsafe impl Send for NameplateCategoryFilter {}
unsafe impl Sync for NameplateCategoryFilter {}

#[cfg(feature = "frontend")]
impl NameplateCategoryFilter {

    pub fn as_filter_expression(&self) -> gluesql::core::ast_builder::ExprNode<'_> {
        let mut filter: gluesql::core::ast_builder::ExprNode<'_> = gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into();
        if let Some(permanence_id) = &self.permanence_id {
            filter = filter.and(gluesql::core::ast_builder::col("nameplate_categories.permanence_id").eq(permanence_id.to_string()));
        }

        if let Some(material_id) = &self.material_id {
            filter = filter.and(gluesql::core::ast_builder::col("nameplate_categories.material_id").eq(material_id.to_string()));
        }

        if let Some(icon_id) = &self.icon_id {
            filter = filter.and(gluesql::core::ast_builder::col("nameplate_categories.icon_id").eq(icon_id.to_string()));
        }

        if let Some(color_id) = &self.color_id {
            filter = filter.and(gluesql::core::ast_builder::col("nameplate_categories.color_id").eq(color_id.to_string()));
        }

        filter
    }
}

#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Eq, PartialOrd, Copy, Ord, Default)]
pub struct NameplateFilter {
    pub project_id: Option<i32>,
    pub category_id: Option<i32>,
    pub created_by: Option<i32>,
    pub updated_by: Option<i32>,
}

unsafe impl Send for NameplateFilter {}
unsafe impl Sync for NameplateFilter {}

#[cfg(feature = "frontend")]
impl NameplateFilter {

    pub fn as_filter_expression(&self) -> gluesql::core::ast_builder::ExprNode<'_> {
        let mut filter: gluesql::core::ast_builder::ExprNode<'_> = gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into();
        if let Some(project_id) = &self.project_id {
            filter = filter.and(gluesql::core::ast_builder::col("nameplates.project_id").eq(project_id.to_string()));
        }

        if let Some(category_id) = &self.category_id {
            filter = filter.and(gluesql::core::ast_builder::col("nameplates.category_id").eq(category_id.to_string()));
        }

        if let Some(created_by) = &self.created_by {
            filter = filter.and(gluesql::core::ast_builder::col("nameplates.created_by").eq(created_by.to_string()));
        }

        if let Some(updated_by) = &self.updated_by {
            filter = filter.and(gluesql::core::ast_builder::col("nameplates.updated_by").eq(updated_by.to_string()));
        }

        filter
    }
}

#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default, Eq, PartialOrd, Copy, Ord)]
pub struct NotificationFilter {
    pub user_id: Option<i32>,
}

unsafe impl Send for NotificationFilter {}
unsafe impl Sync for NotificationFilter {}

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

#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default, Eq, PartialOrd, Copy, Ord)]
pub struct ObservationSubjectFilter {
    pub icon_id: Option<i32>,
    pub color_id: Option<i32>,
}

unsafe impl Send for ObservationSubjectFilter {}
unsafe impl Sync for ObservationSubjectFilter {}

#[cfg(feature = "frontend")]
impl ObservationSubjectFilter {

    pub fn as_filter_expression(&self) -> gluesql::core::ast_builder::ExprNode<'_> {
        let mut filter: gluesql::core::ast_builder::ExprNode<'_> = gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into();
        if let Some(icon_id) = &self.icon_id {
            filter = filter.and(gluesql::core::ast_builder::col("observation_subjects.icon_id").eq(icon_id.to_string()));
        }

        if let Some(color_id) = &self.color_id {
            filter = filter.and(gluesql::core::ast_builder::col("observation_subjects.color_id").eq(color_id.to_string()));
        }

        filter
    }
}

#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default, Eq, Copy)]
pub struct ObservationFilter {
    pub parent_observation_id: Option<uuid::Uuid>,
    pub created_by: Option<i32>,
    pub updated_by: Option<i32>,
    pub project_id: Option<i32>,
    pub organism_id: Option<uuid::Uuid>,
    pub sample_id: Option<uuid::Uuid>,
    pub subject_id: Option<i32>,
}

unsafe impl Send for ObservationFilter {}
unsafe impl Sync for ObservationFilter {}

#[cfg(feature = "frontend")]
impl ObservationFilter {

    pub fn as_filter_expression(&self) -> gluesql::core::ast_builder::ExprNode<'_> {
        let mut filter: gluesql::core::ast_builder::ExprNode<'_> = gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into();
        if let Some(parent_observation_id) = &self.parent_observation_id {
            filter = filter.and(gluesql::core::ast_builder::col("observations.parent_observation_id").eq(parent_observation_id.to_string()));
        }

        if let Some(created_by) = &self.created_by {
            filter = filter.and(gluesql::core::ast_builder::col("observations.created_by").eq(created_by.to_string()));
        }

        if let Some(updated_by) = &self.updated_by {
            filter = filter.and(gluesql::core::ast_builder::col("observations.updated_by").eq(updated_by.to_string()));
        }

        if let Some(project_id) = &self.project_id {
            filter = filter.and(gluesql::core::ast_builder::col("observations.project_id").eq(project_id.to_string()));
        }

        if let Some(organism_id) = &self.organism_id {
            filter = filter.and(gluesql::core::ast_builder::col("observations.organism_id").eq(organism_id.to_string()));
        }

        if let Some(sample_id) = &self.sample_id {
            filter = filter.and(gluesql::core::ast_builder::col("observations.sample_id").eq(sample_id.to_string()));
        }

        if let Some(subject_id) = &self.subject_id {
            filter = filter.and(gluesql::core::ast_builder::col("observations.subject_id").eq(subject_id.to_string()));
        }

        filter
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Copy, serde::Serialize, serde::Deserialize, Default)]
pub struct OrganismBioOttTaxonItemFilter {
    pub created_by: Option<i32>,
    pub organism_id: Option<uuid::Uuid>,
    pub taxon_id: Option<i32>,
}

unsafe impl Send for OrganismBioOttTaxonItemFilter {}
unsafe impl Sync for OrganismBioOttTaxonItemFilter {}

#[cfg(feature = "frontend")]
impl OrganismBioOttTaxonItemFilter {

    pub fn as_filter_expression(&self) -> gluesql::core::ast_builder::ExprNode<'_> {
        let mut filter: gluesql::core::ast_builder::ExprNode<'_> = gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into();
        if let Some(created_by) = &self.created_by {
            filter = filter.and(gluesql::core::ast_builder::col("organism_bio_ott_taxon_items.created_by").eq(created_by.to_string()));
        }

        if let Some(organism_id) = &self.organism_id {
            filter = filter.and(gluesql::core::ast_builder::col("organism_bio_ott_taxon_items.organism_id").eq(organism_id.to_string()));
        }

        if let Some(taxon_id) = &self.taxon_id {
            filter = filter.and(gluesql::core::ast_builder::col("organism_bio_ott_taxon_items.taxon_id").eq(taxon_id.to_string()));
        }

        filter
    }
}

#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default, Eq, Copy)]
pub struct OrganismFilter {
    pub host_organism_id: Option<uuid::Uuid>,
    pub sample_id: Option<uuid::Uuid>,
    pub nameplate_id: Option<i32>,
    pub project_id: Option<i32>,
    pub created_by: Option<i32>,
    pub updated_by: Option<i32>,
}

unsafe impl Send for OrganismFilter {}
unsafe impl Sync for OrganismFilter {}

#[cfg(feature = "frontend")]
impl OrganismFilter {

    pub fn as_filter_expression(&self) -> gluesql::core::ast_builder::ExprNode<'_> {
        let mut filter: gluesql::core::ast_builder::ExprNode<'_> = gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into();
        if let Some(host_organism_id) = &self.host_organism_id {
            filter = filter.and(gluesql::core::ast_builder::col("organisms.host_organism_id").eq(host_organism_id.to_string()));
        }

        if let Some(sample_id) = &self.sample_id {
            filter = filter.and(gluesql::core::ast_builder::col("organisms.sample_id").eq(sample_id.to_string()));
        }

        if let Some(nameplate_id) = &self.nameplate_id {
            filter = filter.and(gluesql::core::ast_builder::col("organisms.nameplate_id").eq(nameplate_id.to_string()));
        }

        if let Some(project_id) = &self.project_id {
            filter = filter.and(gluesql::core::ast_builder::col("organisms.project_id").eq(project_id.to_string()));
        }

        if let Some(created_by) = &self.created_by {
            filter = filter.and(gluesql::core::ast_builder::col("organisms.created_by").eq(created_by.to_string()));
        }

        if let Some(updated_by) = &self.updated_by {
            filter = filter.and(gluesql::core::ast_builder::col("organisms.updated_by").eq(updated_by.to_string()));
        }

        filter
    }
}

#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default, Eq, PartialOrd, Copy, Ord)]
pub struct OrganizationFilter {
    pub country_id: Option<i32>,
}

unsafe impl Send for OrganizationFilter {}
unsafe impl Sync for OrganizationFilter {}

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

#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default, Eq, PartialOrd, Copy, Ord)]
pub struct PermanenceCategoryFilter {
    pub icon_id: Option<i32>,
    pub color_id: Option<i32>,
}

unsafe impl Send for PermanenceCategoryFilter {}
unsafe impl Sync for PermanenceCategoryFilter {}

#[cfg(feature = "frontend")]
impl PermanenceCategoryFilter {

    pub fn as_filter_expression(&self) -> gluesql::core::ast_builder::ExprNode<'_> {
        let mut filter: gluesql::core::ast_builder::ExprNode<'_> = gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into();
        if let Some(icon_id) = &self.icon_id {
            filter = filter.and(gluesql::core::ast_builder::col("permanence_categories.icon_id").eq(icon_id.to_string()));
        }

        if let Some(color_id) = &self.color_id {
            filter = filter.and(gluesql::core::ast_builder::col("permanence_categories.color_id").eq(color_id.to_string()));
        }

        filter
    }
}

#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default, Eq, PartialOrd, Copy, Ord)]
pub struct ProjectStateFilter {
    pub icon_id: Option<i32>,
    pub color_id: Option<i32>,
}

unsafe impl Send for ProjectStateFilter {}
unsafe impl Sync for ProjectStateFilter {}

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

#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default, Eq, PartialOrd, Copy, Ord)]
pub struct ProjectFilter {
    pub state_id: Option<i32>,
    pub icon_id: Option<i32>,
    pub color_id: Option<i32>,
    pub parent_project_id: Option<i32>,
    pub created_by: Option<i32>,
    pub updated_by: Option<i32>,
}

unsafe impl Send for ProjectFilter {}
unsafe impl Sync for ProjectFilter {}

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

#[derive(Eq, PartialEq, PartialOrd, Debug, Clone, Copy, Ord, serde::Serialize, serde::Deserialize, Default)]
pub struct ProjectsTeamsRoleInvitationFilter {
    pub table_id: Option<i32>,
    pub team_id: Option<i32>,
    pub role_id: Option<i32>,
    pub created_by: Option<i32>,
}

unsafe impl Send for ProjectsTeamsRoleInvitationFilter {}
unsafe impl Sync for ProjectsTeamsRoleInvitationFilter {}

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

#[derive(Eq, PartialEq, PartialOrd, Debug, Clone, Copy, Ord, serde::Serialize, serde::Deserialize, Default)]
pub struct ProjectsTeamsRoleRequestFilter {
    pub table_id: Option<i32>,
    pub team_id: Option<i32>,
    pub role_id: Option<i32>,
    pub created_by: Option<i32>,
}

unsafe impl Send for ProjectsTeamsRoleRequestFilter {}
unsafe impl Sync for ProjectsTeamsRoleRequestFilter {}

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

#[derive(Eq, PartialEq, PartialOrd, Debug, Clone, Copy, Ord, serde::Serialize, serde::Deserialize, Default)]
pub struct ProjectsTeamsRoleFilter {
    pub table_id: Option<i32>,
    pub team_id: Option<i32>,
    pub role_id: Option<i32>,
    pub created_by: Option<i32>,
}

unsafe impl Send for ProjectsTeamsRoleFilter {}
unsafe impl Sync for ProjectsTeamsRoleFilter {}

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

#[derive(Eq, PartialEq, PartialOrd, Debug, Clone, Copy, Ord, serde::Serialize, serde::Deserialize, Default)]
pub struct ProjectsUsersRoleInvitationFilter {
    pub table_id: Option<i32>,
    pub user_id: Option<i32>,
    pub role_id: Option<i32>,
    pub created_by: Option<i32>,
}

unsafe impl Send for ProjectsUsersRoleInvitationFilter {}
unsafe impl Sync for ProjectsUsersRoleInvitationFilter {}

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

#[derive(Eq, PartialEq, PartialOrd, Debug, Clone, Copy, Ord, serde::Serialize, serde::Deserialize, Default)]
pub struct ProjectsUsersRoleRequestFilter {
    pub table_id: Option<i32>,
    pub user_id: Option<i32>,
    pub role_id: Option<i32>,
    pub created_by: Option<i32>,
}

unsafe impl Send for ProjectsUsersRoleRequestFilter {}
unsafe impl Sync for ProjectsUsersRoleRequestFilter {}

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

#[derive(Eq, PartialEq, PartialOrd, Debug, Clone, Copy, Ord, serde::Serialize, serde::Deserialize, Default)]
pub struct ProjectsUsersRoleFilter {
    pub table_id: Option<i32>,
    pub user_id: Option<i32>,
    pub role_id: Option<i32>,
    pub created_by: Option<i32>,
}

unsafe impl Send for ProjectsUsersRoleFilter {}
unsafe impl Sync for ProjectsUsersRoleFilter {}

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

#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default, Eq, PartialOrd, Copy, Ord)]
pub struct RoleFilter {
    pub icon_id: Option<i32>,
    pub color_id: Option<i32>,
}

unsafe impl Send for RoleFilter {}
unsafe impl Sync for RoleFilter {}

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

#[derive(Eq, PartialEq, Debug, Clone, Copy, serde::Serialize, serde::Deserialize, Default)]
pub struct SampleBioOttTaxonItemFilter {
    pub created_by: Option<i32>,
    pub sample_id: Option<uuid::Uuid>,
    pub taxon_id: Option<i32>,
}

unsafe impl Send for SampleBioOttTaxonItemFilter {}
unsafe impl Sync for SampleBioOttTaxonItemFilter {}

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

#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default, Eq, PartialOrd, Copy, Ord)]
pub struct SampleContainerCategoryFilter {
    pub material_id: Option<i32>,
    pub icon_id: Option<i32>,
    pub color_id: Option<i32>,
}

unsafe impl Send for SampleContainerCategoryFilter {}
unsafe impl Sync for SampleContainerCategoryFilter {}

#[cfg(feature = "frontend")]
impl SampleContainerCategoryFilter {

    pub fn as_filter_expression(&self) -> gluesql::core::ast_builder::ExprNode<'_> {
        let mut filter: gluesql::core::ast_builder::ExprNode<'_> = gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into();
        if let Some(material_id) = &self.material_id {
            filter = filter.and(gluesql::core::ast_builder::col("sample_container_categories.material_id").eq(material_id.to_string()));
        }

        if let Some(icon_id) = &self.icon_id {
            filter = filter.and(gluesql::core::ast_builder::col("sample_container_categories.icon_id").eq(icon_id.to_string()));
        }

        if let Some(color_id) = &self.color_id {
            filter = filter.and(gluesql::core::ast_builder::col("sample_container_categories.color_id").eq(color_id.to_string()));
        }

        filter
    }
}

#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default, Eq, PartialOrd, Copy, Ord)]
pub struct SampleContainerFilter {
    pub project_id: Option<i32>,
    pub category_id: Option<i32>,
    pub created_by: Option<i32>,
    pub updated_by: Option<i32>,
}

unsafe impl Send for SampleContainerFilter {}
unsafe impl Sync for SampleContainerFilter {}

#[cfg(feature = "frontend")]
impl SampleContainerFilter {

    pub fn as_filter_expression(&self) -> gluesql::core::ast_builder::ExprNode<'_> {
        let mut filter: gluesql::core::ast_builder::ExprNode<'_> = gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into();
        if let Some(project_id) = &self.project_id {
            filter = filter.and(gluesql::core::ast_builder::col("sample_containers.project_id").eq(project_id.to_string()));
        }

        if let Some(category_id) = &self.category_id {
            filter = filter.and(gluesql::core::ast_builder::col("sample_containers.category_id").eq(category_id.to_string()));
        }

        if let Some(created_by) = &self.created_by {
            filter = filter.and(gluesql::core::ast_builder::col("sample_containers.created_by").eq(created_by.to_string()));
        }

        if let Some(updated_by) = &self.updated_by {
            filter = filter.and(gluesql::core::ast_builder::col("sample_containers.updated_by").eq(updated_by.to_string()));
        }

        filter
    }
}

#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default, Eq, PartialOrd, Copy, Ord)]
pub struct SampleStateFilter {
    pub icon_id: Option<i32>,
    pub color_id: Option<i32>,
}

unsafe impl Send for SampleStateFilter {}
unsafe impl Sync for SampleStateFilter {}

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

#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default, Eq, PartialOrd, Copy, Ord)]
pub struct SampleFilter {
    pub container_id: Option<i32>,
    pub project_id: Option<i32>,
    pub created_by: Option<i32>,
    pub sampled_by: Option<i32>,
    pub updated_by: Option<i32>,
    pub state_id: Option<i32>,
}

unsafe impl Send for SampleFilter {}
unsafe impl Sync for SampleFilter {}

#[cfg(feature = "frontend")]
impl SampleFilter {

    pub fn as_filter_expression(&self) -> gluesql::core::ast_builder::ExprNode<'_> {
        let mut filter: gluesql::core::ast_builder::ExprNode<'_> = gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into();
        if let Some(container_id) = &self.container_id {
            filter = filter.and(gluesql::core::ast_builder::col("samples.container_id").eq(container_id.to_string()));
        }

        if let Some(project_id) = &self.project_id {
            filter = filter.and(gluesql::core::ast_builder::col("samples.project_id").eq(project_id.to_string()));
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

        if let Some(state_id) = &self.state_id {
            filter = filter.and(gluesql::core::ast_builder::col("samples.state_id").eq(state_id.to_string()));
        }

        filter
    }
}

#[derive(Eq, PartialEq, PartialOrd, Debug, Clone, Copy, Ord, serde::Serialize, serde::Deserialize, Default)]
pub struct SpectraFilter {
    pub spectra_collection_id: Option<i32>,
}

unsafe impl Send for SpectraFilter {}
unsafe impl Sync for SpectraFilter {}

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

#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default, Eq, Copy)]
pub struct SpectraCollectionFilter {
    pub sample_id: Option<uuid::Uuid>,
    pub created_by: Option<i32>,
    pub updated_by: Option<i32>,
}

unsafe impl Send for SpectraCollectionFilter {}
unsafe impl Sync for SpectraCollectionFilter {}

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

#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default, Eq, PartialOrd, Copy, Ord)]
pub struct TeamStateFilter {
    pub icon_id: Option<i32>,
    pub color_id: Option<i32>,
}

unsafe impl Send for TeamStateFilter {}
unsafe impl Sync for TeamStateFilter {}

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

#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default, Eq, PartialOrd, Copy, Ord)]
pub struct TeamFilter {
    pub icon_id: Option<i32>,
    pub color_id: Option<i32>,
    pub state_id: Option<i32>,
    pub parent_team_id: Option<i32>,
    pub created_by: Option<i32>,
    pub updated_by: Option<i32>,
}

unsafe impl Send for TeamFilter {}
unsafe impl Sync for TeamFilter {}

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

        if let Some(state_id) = &self.state_id {
            filter = filter.and(gluesql::core::ast_builder::col("teams.state_id").eq(state_id.to_string()));
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

#[derive(Eq, PartialEq, PartialOrd, Debug, Clone, Copy, Ord, serde::Serialize, serde::Deserialize, Default)]
pub struct TeamsTeamsRoleInvitationFilter {
    pub table_id: Option<i32>,
    pub team_id: Option<i32>,
    pub role_id: Option<i32>,
    pub created_by: Option<i32>,
}

unsafe impl Send for TeamsTeamsRoleInvitationFilter {}
unsafe impl Sync for TeamsTeamsRoleInvitationFilter {}

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

#[derive(Eq, PartialEq, PartialOrd, Debug, Clone, Copy, Ord, serde::Serialize, serde::Deserialize, Default)]
pub struct TeamsUsersRoleInvitationFilter {
    pub table_id: Option<i32>,
    pub user_id: Option<i32>,
    pub role_id: Option<i32>,
    pub created_by: Option<i32>,
}

unsafe impl Send for TeamsUsersRoleInvitationFilter {}
unsafe impl Sync for TeamsUsersRoleInvitationFilter {}

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

#[derive(Eq, PartialEq, PartialOrd, Debug, Clone, Copy, Ord, serde::Serialize, serde::Deserialize, Default)]
pub struct TeamsUsersRoleRequestFilter {
    pub table_id: Option<i32>,
    pub user_id: Option<i32>,
    pub role_id: Option<i32>,
    pub created_by: Option<i32>,
}

unsafe impl Send for TeamsUsersRoleRequestFilter {}
unsafe impl Sync for TeamsUsersRoleRequestFilter {}

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

#[derive(Eq, PartialEq, PartialOrd, Debug, Clone, Copy, Ord, serde::Serialize, serde::Deserialize, Default)]
pub struct TeamsUsersRoleFilter {
    pub table_id: Option<i32>,
    pub user_id: Option<i32>,
    pub role_id: Option<i32>,
    pub created_by: Option<i32>,
}

unsafe impl Send for TeamsUsersRoleFilter {}
unsafe impl Sync for TeamsUsersRoleFilter {}

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

#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default, Eq, PartialOrd, Copy, Ord)]
pub struct UnitFilter {
    pub icon_id: Option<i32>,
    pub color_id: Option<i32>,
}

unsafe impl Send for UnitFilter {}
unsafe impl Sync for UnitFilter {}

#[cfg(feature = "frontend")]
impl UnitFilter {

    pub fn as_filter_expression(&self) -> gluesql::core::ast_builder::ExprNode<'_> {
        let mut filter: gluesql::core::ast_builder::ExprNode<'_> = gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into();
        if let Some(icon_id) = &self.icon_id {
            filter = filter.and(gluesql::core::ast_builder::col("units.icon_id").eq(icon_id.to_string()));
        }

        if let Some(color_id) = &self.color_id {
            filter = filter.and(gluesql::core::ast_builder::col("units.color_id").eq(color_id.to_string()));
        }

        filter
    }
}

#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default, Eq, PartialOrd, Copy, Ord)]
pub struct UserEmailFilter {
    pub created_by: Option<i32>,
    pub login_provider_id: Option<i32>,
}

unsafe impl Send for UserEmailFilter {}
unsafe impl Sync for UserEmailFilter {}

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

#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default, Eq, PartialOrd, Copy, Ord)]
pub struct UserFilter {
    pub organization_id: Option<i32>,
}

unsafe impl Send for UserFilter {}
unsafe impl Sync for UserFilter {}

#[cfg(feature = "frontend")]
impl UserFilter {

    pub fn as_filter_expression(&self) -> gluesql::core::ast_builder::ExprNode<'_> {
        let mut filter: gluesql::core::ast_builder::ExprNode<'_> = gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into();
        if let Some(organization_id) = &self.organization_id {
            filter = filter.and(gluesql::core::ast_builder::col("users.organization_id").eq(organization_id.to_string()));
        }

        filter
    }
}

#[derive(Eq, PartialEq, PartialOrd, Debug, Clone, Copy, Ord, serde::Serialize, serde::Deserialize, Default)]
pub struct UsersUsersRoleInvitationFilter {
    pub table_id: Option<i32>,
    pub user_id: Option<i32>,
    pub role_id: Option<i32>,
    pub created_by: Option<i32>,
}

unsafe impl Send for UsersUsersRoleInvitationFilter {}
unsafe impl Sync for UsersUsersRoleInvitationFilter {}

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

#[derive(Eq, PartialEq, PartialOrd, Debug, Clone, Copy, Ord, serde::Serialize, serde::Deserialize, Default)]
pub struct UsersUsersRoleRequestFilter {
    pub table_id: Option<i32>,
    pub user_id: Option<i32>,
    pub role_id: Option<i32>,
    pub created_by: Option<i32>,
}

unsafe impl Send for UsersUsersRoleRequestFilter {}
unsafe impl Sync for UsersUsersRoleRequestFilter {}

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

#[derive(Eq, PartialEq, PartialOrd, Debug, Clone, Copy, Ord, serde::Serialize, serde::Deserialize, Default)]
pub struct UsersUsersRoleFilter {
    pub table_id: Option<i32>,
    pub user_id: Option<i32>,
    pub role_id: Option<i32>,
    pub created_by: Option<i32>,
}

unsafe impl Send for UsersUsersRoleFilter {}
unsafe impl Sync for UsersUsersRoleFilter {}

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
