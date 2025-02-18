//! Submodule proving a trait defining a taxon entry.

use crate::traits::taxonomy::Taxonomy;
use crate::traits::TaxonEntry;
use std::collections::HashMap;

/// Trait defining a taxon entry.
pub trait Taxon<'a>: Sized
where
    Self: 'a,
{
    /// Type of the identifier for the taxon.
    type Taxonomy: super::Taxonomy<Taxon<'a> = Self>;

    /// Returns the identifier of the taxon.
    fn id(&self) -> &'a <<Self::Taxonomy as super::Taxonomy>::TaxonEntry as super::TaxonEntry>::Id {
        self.entry().id()
    }

    /// Returns the name of the taxon.
    fn name(&self) -> &'a str {
        self.entry().name()
    }

    /// Returns the "ltree path" of the taxon.
    /// As per https://www.postgresql.org/docs/current/ltree.html
    /// A label is a sequence of alphanumeric characters, underscores, and hyphens. Valid alphanumeric character ranges are dependent on the database locale. For example, in C locale, the characters A-Za-z0-9_- are allowed. Labels must be no more than 1000 characters long.
    /// TODO : We might want to add control mechanisms for these specifications here.
    // fn ltree_path(&self) -> String {
    //     if let Some(parent) = self.parent() {
    //         // Here we recursively call the ltree_path function in order to return parents
    //         format!("{}.{}", parent.ltree_path(), self.name())
    //     } else {
    //         // this is the base case. We have no parents.
    //         self.name().to_owned()
    //     }
    // }

    fn ltree_path(
        &self,
        cache: &mut HashMap<
            <<Self::Taxonomy as super::Taxonomy>::TaxonEntry as super::TaxonEntry>::Id,
            String,
        >,
    ) -> String {
        if let Some(cached_path) = cache.get(self.id()) {
            return cached_path.clone();
        }

        let sanitized_name = Self::sanitize_ltree_label(self.name()); // Sanitize the taxon name

        let path = if let Some(parent) = self.parent() {
            format!("{}.{}", parent.ltree_path(cache), sanitized_name)
        } else {
            sanitized_name
        };

        cache.insert(self.id().clone(), path.clone());
        path
    }

    /// Sanitize the taxon names for LTREE compliance
    fn sanitize_ltree_label(label: &str) -> String {
        label
            .replace(".", "_") // Replace dots with underscores
            .replace("/", "_") // Replace slashes with underscores
            .replace(" ", "_") // Replace spaces with underscores
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '_') // Keep only valid characters
            .collect()
    }

    /// Returns the rank of the taxon.
    fn rank(
        &self,
    ) -> &'a <<Self::Taxonomy as super::Taxonomy>::TaxonEntry as super::TaxonEntry>::Rank {
        self.entry().rank()
    }

    /// Returns the identifier of the parent taxon.
    fn parent_id(
        &self,
    ) -> Option<&'a <<Self::Taxonomy as super::Taxonomy>::TaxonEntry as super::TaxonEntry>::Id>
    {
        self.entry().parent_id()
    }

    /// Returns the underlying taxon entry.
    fn entry(&self) -> &'a <Self::Taxonomy as super::Taxonomy>::TaxonEntry;

    /// Returns a reference to the underlying taxonomy.
    fn taxonomy(&self) -> &'a Self::Taxonomy;

    /// Returns the identifier of the parent taxon.
    fn parent(&self) -> Option<Self> {
        self.parent_id()
            .and_then(|id| self.taxonomy().taxon_by_id(id).ok())
    }

    /// Returns an iterator over the children of the taxon.
    fn children(&self) -> impl Iterator<Item = Self> {
        self.taxonomy()
            .taxons()
            .filter(move |taxon| taxon.parent_id() == Some(self.id()))
    }
}
