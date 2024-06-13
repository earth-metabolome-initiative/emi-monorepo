use crate::database::*;
use crate::traits::GuessImageFormat;
/// A struct that is associated to a table in the database.

pub trait Tabular {
    const TABLE: Table;
}

/// A struct that is associated to a filter struct.
pub trait Filtrable: PartialEq {
    type Filter: serde::Serialize + PartialEq + Clone;
}

/// A struct that may be associated to a textual description.
pub trait Describable {
    fn description(&self) -> Option<&str>;
}

/// A struct that may be associated to a color.
pub trait Colorable {
    fn color(&self) -> Option<&str>;
}

#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct BioOttRank {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub icon_id: i32,
    pub color_id: i32,
}

unsafe impl Send for BioOttRank {}
unsafe impl Sync for BioOttRank {}
impl Tabular for BioOttRank {
    const TABLE: Table = Table::BioOttRanks;
}
impl Describable for BioOttRank {
    fn description(&self) -> Option<&str> {
        Some(self.description.as_str())
    }
}
impl Colorable for BioOttRank {
    fn color(&self) -> Option<&str> {
        None
    }
}

impl Filtrable for BioOttRank {
    type Filter = BioOttRankFilter;
}
#[cfg(feature = "frontend")]
impl BioOttRank {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::text(self.name),
            gluesql::core::ast_builder::text(self.description),
            gluesql::core::ast_builder::num(self.icon_id),
            gluesql::core::ast_builder::num(self.color_id),
        ]
    }

    /// Insert the BioOttRank into the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn insert<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut> (
self: crate::database::flat_variants::BioOttRank,
connection: &mut gluesql::prelude::Glue<C>,
) -> Result<usize, gluesql::prelude::Error>    {
        use gluesql::core::ast_builder::*;
        table("bio_ott_ranks")
            .insert()
            .columns("id, name, description, icon_id, color_id")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get BioOttRank from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("bio_ott_ranks")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, name, description, icon_id, color_id")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete BioOttRank from the database.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("bio_ott_ranks")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of BioOttRank from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("bio_ott_ranks")
            .update()        
.set("id", gluesql::core::ast_builder::num(self.id))        
.set("name", gluesql::core::ast_builder::text(self.name))        
.set("description", gluesql::core::ast_builder::text(self.description))        
.set("icon_id", gluesql::core::ast_builder::num(self.icon_id))        
.set("color_id", gluesql::core::ast_builder::num(self.color_id))            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all BioOttRank from the database.
    ///
    /// # Arguments
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        filter: Option<&BioOttRankFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("bio_ott_ranks")
            .select()
            .filter(filter.map_or_else(|| gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into(), |filter| filter.as_filter_expression()))
           .project("id, name, description, icon_id, color_id")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::I32(id) => id.clone(),
                _ => unreachable!("Expected I32")
            },
            name: match row.get("name").unwrap() {
                gluesql::prelude::Value::Str(name) => name.clone(),
                _ => unreachable!("Expected Str")
            },
            description: match row.get("description").unwrap() {
                gluesql::prelude::Value::Str(description) => description.clone(),
                _ => unreachable!("Expected Str")
            },
            icon_id: match row.get("icon_id").unwrap() {
                gluesql::prelude::Value::I32(icon_id) => icon_id.clone(),
                _ => unreachable!("Expected I32")
            },
            color_id: match row.get("color_id").unwrap() {
                gluesql::prelude::Value::I32(color_id) => color_id.clone(),
                _ => unreachable!("Expected I32")
            },
        }
    }
}
#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct BioOttTaxonItem {
    pub id: i32,
    pub name: String,
    pub ott_id: i32,
    pub ott_rank_id: i32,
    pub wikidata_id: Option<i32>,
    pub ncbi_id: Option<i32>,
    pub gbif_id: Option<i32>,
    pub irmng_id: Option<i32>,
    pub worms_id: Option<i32>,
    pub domain_id: Option<i32>,
    pub kingdom_id: Option<i32>,
    pub phylum_id: Option<i32>,
    pub class_id: Option<i32>,
    pub order_id: Option<i32>,
    pub family_id: Option<i32>,
    pub genus_id: Option<i32>,
    pub parent_id: i32,
    pub icon_id: i32,
    pub color_id: i32,
}

unsafe impl Send for BioOttTaxonItem {}
unsafe impl Sync for BioOttTaxonItem {}
impl Tabular for BioOttTaxonItem {
    const TABLE: Table = Table::BioOttTaxonItems;
}
impl Describable for BioOttTaxonItem {
    fn description(&self) -> Option<&str> {
        None
    }
}
impl Colorable for BioOttTaxonItem {
    fn color(&self) -> Option<&str> {
        None
    }
}

impl Filtrable for BioOttTaxonItem {
    type Filter = BioOttTaxonItemFilter;
}
#[cfg(feature = "frontend")]
impl BioOttTaxonItem {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::text(self.name),
            gluesql::core::ast_builder::num(self.ott_id),
            gluesql::core::ast_builder::num(self.ott_rank_id),
            match self.wikidata_id {
                Some(wikidata_id) => gluesql::core::ast_builder::num(wikidata_id),
                None => gluesql::core::ast_builder::null(),
            },
            match self.ncbi_id {
                Some(ncbi_id) => gluesql::core::ast_builder::num(ncbi_id),
                None => gluesql::core::ast_builder::null(),
            },
            match self.gbif_id {
                Some(gbif_id) => gluesql::core::ast_builder::num(gbif_id),
                None => gluesql::core::ast_builder::null(),
            },
            match self.irmng_id {
                Some(irmng_id) => gluesql::core::ast_builder::num(irmng_id),
                None => gluesql::core::ast_builder::null(),
            },
            match self.worms_id {
                Some(worms_id) => gluesql::core::ast_builder::num(worms_id),
                None => gluesql::core::ast_builder::null(),
            },
            match self.domain_id {
                Some(domain_id) => gluesql::core::ast_builder::num(domain_id),
                None => gluesql::core::ast_builder::null(),
            },
            match self.kingdom_id {
                Some(kingdom_id) => gluesql::core::ast_builder::num(kingdom_id),
                None => gluesql::core::ast_builder::null(),
            },
            match self.phylum_id {
                Some(phylum_id) => gluesql::core::ast_builder::num(phylum_id),
                None => gluesql::core::ast_builder::null(),
            },
            match self.class_id {
                Some(class_id) => gluesql::core::ast_builder::num(class_id),
                None => gluesql::core::ast_builder::null(),
            },
            match self.order_id {
                Some(order_id) => gluesql::core::ast_builder::num(order_id),
                None => gluesql::core::ast_builder::null(),
            },
            match self.family_id {
                Some(family_id) => gluesql::core::ast_builder::num(family_id),
                None => gluesql::core::ast_builder::null(),
            },
            match self.genus_id {
                Some(genus_id) => gluesql::core::ast_builder::num(genus_id),
                None => gluesql::core::ast_builder::null(),
            },
            gluesql::core::ast_builder::num(self.parent_id),
            gluesql::core::ast_builder::num(self.icon_id),
            gluesql::core::ast_builder::num(self.color_id),
        ]
    }

    /// Insert the BioOttTaxonItem into the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn insert<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut> (
self: crate::database::flat_variants::BioOttTaxonItem,
connection: &mut gluesql::prelude::Glue<C>,
) -> Result<usize, gluesql::prelude::Error>    {
        use gluesql::core::ast_builder::*;
        table("bio_ott_taxon_items")
            .insert()
            .columns("id, name, ott_id, ott_rank_id, wikidata_id, ncbi_id, gbif_id, irmng_id, worms_id, domain_id, kingdom_id, phylum_id, class_id, order_id, family_id, genus_id, parent_id, icon_id, color_id")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get BioOttTaxonItem from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("bio_ott_taxon_items")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, name, ott_id, ott_rank_id, wikidata_id, ncbi_id, gbif_id, irmng_id, worms_id, domain_id, kingdom_id, phylum_id, class_id, order_id, family_id, genus_id, parent_id, icon_id, color_id")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete BioOttTaxonItem from the database.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("bio_ott_taxon_items")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of BioOttTaxonItem from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let mut update_row = table("bio_ott_taxon_items")
            .update()        
.set("id", gluesql::core::ast_builder::num(self.id))        
.set("name", gluesql::core::ast_builder::text(self.name))        
.set("ott_id", gluesql::core::ast_builder::num(self.ott_id))        
.set("ott_rank_id", gluesql::core::ast_builder::num(self.ott_rank_id))        
.set("parent_id", gluesql::core::ast_builder::num(self.parent_id))        
.set("icon_id", gluesql::core::ast_builder::num(self.icon_id))        
.set("color_id", gluesql::core::ast_builder::num(self.color_id));
        if let Some(wikidata_id) = self.wikidata_id {
            update_row = update_row.set("wikidata_id", gluesql::core::ast_builder::num(wikidata_id));
        }
        if let Some(ncbi_id) = self.ncbi_id {
            update_row = update_row.set("ncbi_id", gluesql::core::ast_builder::num(ncbi_id));
        }
        if let Some(gbif_id) = self.gbif_id {
            update_row = update_row.set("gbif_id", gluesql::core::ast_builder::num(gbif_id));
        }
        if let Some(irmng_id) = self.irmng_id {
            update_row = update_row.set("irmng_id", gluesql::core::ast_builder::num(irmng_id));
        }
        if let Some(worms_id) = self.worms_id {
            update_row = update_row.set("worms_id", gluesql::core::ast_builder::num(worms_id));
        }
        if let Some(domain_id) = self.domain_id {
            update_row = update_row.set("domain_id", gluesql::core::ast_builder::num(domain_id));
        }
        if let Some(kingdom_id) = self.kingdom_id {
            update_row = update_row.set("kingdom_id", gluesql::core::ast_builder::num(kingdom_id));
        }
        if let Some(phylum_id) = self.phylum_id {
            update_row = update_row.set("phylum_id", gluesql::core::ast_builder::num(phylum_id));
        }
        if let Some(class_id) = self.class_id {
            update_row = update_row.set("class_id", gluesql::core::ast_builder::num(class_id));
        }
        if let Some(order_id) = self.order_id {
            update_row = update_row.set("order_id", gluesql::core::ast_builder::num(order_id));
        }
        if let Some(family_id) = self.family_id {
            update_row = update_row.set("family_id", gluesql::core::ast_builder::num(family_id));
        }
        if let Some(genus_id) = self.genus_id {
            update_row = update_row.set("genus_id", gluesql::core::ast_builder::num(genus_id));
        }
            update_row.execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all BioOttTaxonItem from the database.
    ///
    /// # Arguments
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        filter: Option<&BioOttTaxonItemFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("bio_ott_taxon_items")
            .select()
            .filter(filter.map_or_else(|| gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into(), |filter| filter.as_filter_expression()))
           .project("id, name, ott_id, ott_rank_id, wikidata_id, ncbi_id, gbif_id, irmng_id, worms_id, domain_id, kingdom_id, phylum_id, class_id, order_id, family_id, genus_id, parent_id, icon_id, color_id")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::I32(id) => id.clone(),
                _ => unreachable!("Expected I32")
            },
            name: match row.get("name").unwrap() {
                gluesql::prelude::Value::Str(name) => name.clone(),
                _ => unreachable!("Expected Str")
            },
            ott_id: match row.get("ott_id").unwrap() {
                gluesql::prelude::Value::I32(ott_id) => ott_id.clone(),
                _ => unreachable!("Expected I32")
            },
            ott_rank_id: match row.get("ott_rank_id").unwrap() {
                gluesql::prelude::Value::I32(ott_rank_id) => ott_rank_id.clone(),
                _ => unreachable!("Expected I32")
            },
            wikidata_id: match row.get("wikidata_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::I32(wikidata_id) => Some(wikidata_id.clone()),
                _ => unreachable!("Expected I32")
            },
            ncbi_id: match row.get("ncbi_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::I32(ncbi_id) => Some(ncbi_id.clone()),
                _ => unreachable!("Expected I32")
            },
            gbif_id: match row.get("gbif_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::I32(gbif_id) => Some(gbif_id.clone()),
                _ => unreachable!("Expected I32")
            },
            irmng_id: match row.get("irmng_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::I32(irmng_id) => Some(irmng_id.clone()),
                _ => unreachable!("Expected I32")
            },
            worms_id: match row.get("worms_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::I32(worms_id) => Some(worms_id.clone()),
                _ => unreachable!("Expected I32")
            },
            domain_id: match row.get("domain_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::I32(domain_id) => Some(domain_id.clone()),
                _ => unreachable!("Expected I32")
            },
            kingdom_id: match row.get("kingdom_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::I32(kingdom_id) => Some(kingdom_id.clone()),
                _ => unreachable!("Expected I32")
            },
            phylum_id: match row.get("phylum_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::I32(phylum_id) => Some(phylum_id.clone()),
                _ => unreachable!("Expected I32")
            },
            class_id: match row.get("class_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::I32(class_id) => Some(class_id.clone()),
                _ => unreachable!("Expected I32")
            },
            order_id: match row.get("order_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::I32(order_id) => Some(order_id.clone()),
                _ => unreachable!("Expected I32")
            },
            family_id: match row.get("family_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::I32(family_id) => Some(family_id.clone()),
                _ => unreachable!("Expected I32")
            },
            genus_id: match row.get("genus_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::I32(genus_id) => Some(genus_id.clone()),
                _ => unreachable!("Expected I32")
            },
            parent_id: match row.get("parent_id").unwrap() {
                gluesql::prelude::Value::I32(parent_id) => parent_id.clone(),
                _ => unreachable!("Expected I32")
            },
            icon_id: match row.get("icon_id").unwrap() {
                gluesql::prelude::Value::I32(icon_id) => icon_id.clone(),
                _ => unreachable!("Expected I32")
            },
            color_id: match row.get("color_id").unwrap() {
                gluesql::prelude::Value::I32(color_id) => color_id.clone(),
                _ => unreachable!("Expected I32")
            },
        }
    }
}
#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct Color {
    pub id: i32,
    pub name: String,
    pub hexadecimal_value: String,
    pub description: String,
}

unsafe impl Send for Color {}
unsafe impl Sync for Color {}
impl Tabular for Color {
    const TABLE: Table = Table::Colors;
}
impl Describable for Color {
    fn description(&self) -> Option<&str> {
        Some(self.description.as_str())
    }
}
impl Colorable for Color {
    fn color(&self) -> Option<&str> {
        Some(self.name.as_str())
    }
}

impl Filtrable for Color {
    type Filter = EmptyFilter;
}
#[cfg(feature = "frontend")]
impl Color {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::text(self.name),
            gluesql::core::ast_builder::text(self.hexadecimal_value),
            gluesql::core::ast_builder::text(self.description),
        ]
    }

    /// Insert the Color into the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn insert<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut> (
self: crate::database::flat_variants::Color,
connection: &mut gluesql::prelude::Glue<C>,
) -> Result<usize, gluesql::prelude::Error>    {
        use gluesql::core::ast_builder::*;
        table("colors")
            .insert()
            .columns("id, name, hexadecimal_value, description")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get Color from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("colors")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, name, hexadecimal_value, description")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete Color from the database.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("colors")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of Color from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("colors")
            .update()        
.set("id", gluesql::core::ast_builder::num(self.id))        
.set("name", gluesql::core::ast_builder::text(self.name))        
.set("hexadecimal_value", gluesql::core::ast_builder::text(self.hexadecimal_value))        
.set("description", gluesql::core::ast_builder::text(self.description))            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all Color from the database.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("colors")
            .select()
           .project("id, name, hexadecimal_value, description")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::I32(id) => id.clone(),
                _ => unreachable!("Expected I32")
            },
            name: match row.get("name").unwrap() {
                gluesql::prelude::Value::Str(name) => name.clone(),
                _ => unreachable!("Expected Str")
            },
            hexadecimal_value: match row.get("hexadecimal_value").unwrap() {
                gluesql::prelude::Value::Str(hexadecimal_value) => hexadecimal_value.clone(),
                _ => unreachable!("Expected Str")
            },
            description: match row.get("description").unwrap() {
                gluesql::prelude::Value::Str(description) => description.clone(),
                _ => unreachable!("Expected Str")
            },
        }
    }
}
#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct Country {
    pub id: i32,
    pub iso: String,
    pub emoji: String,
    pub unicode: String,
    pub name: String,
}

unsafe impl Send for Country {}
unsafe impl Sync for Country {}
impl Tabular for Country {
    const TABLE: Table = Table::Countries;
}
impl Describable for Country {
    fn description(&self) -> Option<&str> {
        None
    }
}
impl Colorable for Country {
    fn color(&self) -> Option<&str> {
        None
    }
}

impl Filtrable for Country {
    type Filter = EmptyFilter;
}
#[cfg(feature = "frontend")]
impl Country {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::text(self.iso),
            gluesql::core::ast_builder::text(self.emoji),
            gluesql::core::ast_builder::text(self.unicode),
            gluesql::core::ast_builder::text(self.name),
        ]
    }

    /// Insert the Country into the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn insert<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut> (
self: crate::database::flat_variants::Country,
connection: &mut gluesql::prelude::Glue<C>,
) -> Result<usize, gluesql::prelude::Error>    {
        use gluesql::core::ast_builder::*;
        table("countries")
            .insert()
            .columns("id, iso, emoji, unicode, name")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get Country from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("countries")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, iso, emoji, unicode, name")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete Country from the database.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("countries")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of Country from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("countries")
            .update()        
.set("id", gluesql::core::ast_builder::num(self.id))        
.set("iso", gluesql::core::ast_builder::text(self.iso))        
.set("emoji", gluesql::core::ast_builder::text(self.emoji))        
.set("unicode", gluesql::core::ast_builder::text(self.unicode))        
.set("name", gluesql::core::ast_builder::text(self.name))            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all Country from the database.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("countries")
            .select()
           .project("id, iso, emoji, unicode, name")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::I32(id) => id.clone(),
                _ => unreachable!("Expected I32")
            },
            iso: match row.get("iso").unwrap() {
                gluesql::prelude::Value::Str(iso) => iso.clone(),
                _ => unreachable!("Expected Str")
            },
            emoji: match row.get("emoji").unwrap() {
                gluesql::prelude::Value::Str(emoji) => emoji.clone(),
                _ => unreachable!("Expected Str")
            },
            unicode: match row.get("unicode").unwrap() {
                gluesql::prelude::Value::Str(unicode) => unicode.clone(),
                _ => unreachable!("Expected Str")
            },
            name: match row.get("name").unwrap() {
                gluesql::prelude::Value::Str(name) => name.clone(),
                _ => unreachable!("Expected Str")
            },
        }
    }
}
#[derive(PartialEq, Debug, Clone, Copy, serde::Serialize, serde::Deserialize, Default)]
pub struct DerivedSample {
    pub created_by: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_by: i32,
    pub updated_at: chrono::NaiveDateTime,
    pub parent_sample_id: uuid::Uuid,
    pub child_sample_id: uuid::Uuid,
    pub quantity: f64,
    pub unit_id: i32,
}

unsafe impl Send for DerivedSample {}
unsafe impl Sync for DerivedSample {}
impl Tabular for DerivedSample {
    const TABLE: Table = Table::DerivedSamples;
}
impl Describable for DerivedSample {
    fn description(&self) -> Option<&str> {
        None
    }
}
impl Colorable for DerivedSample {
    fn color(&self) -> Option<&str> {
        None
    }
}

impl Filtrable for DerivedSample {
    type Filter = DerivedSampleFilter;
}
#[cfg(feature = "frontend")]
impl DerivedSample {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.created_by),
            gluesql::core::ast_builder::timestamp(self.created_at.to_string()),
            gluesql::core::ast_builder::num(self.updated_by),
            gluesql::core::ast_builder::timestamp(self.updated_at.to_string()),
            gluesql::core::ast_builder::uuid(self.parent_sample_id.to_string()),
            gluesql::core::ast_builder::uuid(self.child_sample_id.to_string()),
            gluesql::core::ast_builder::num(self.quantity),
            gluesql::core::ast_builder::num(self.unit_id),
        ]
    }

    /// Insert the DerivedSample into the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn insert<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut> (
self: crate::database::flat_variants::DerivedSample,
connection: &mut gluesql::prelude::Glue<C>,
) -> Result<usize, gluesql::prelude::Error>    {
        use gluesql::core::ast_builder::*;
        table("derived_samples")
            .insert()
            .columns("created_by, created_at, updated_by, updated_at, parent_sample_id, child_sample_id, quantity, unit_id")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get DerivedSample from the database by its ID.
    ///
    /// # Arguments
    /// * `( parent_sample_id, child_sample_id )` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        ( parent_sample_id, child_sample_id ): ( uuid::Uuid, uuid::Uuid ),
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("derived_samples")
            .select()
            .filter(col("parent_sample_id").eq(parent_sample_id.to_string()))
            .filter(col("child_sample_id").eq(child_sample_id.to_string()))
            .project("created_by, created_at, updated_by, updated_at, parent_sample_id, child_sample_id, quantity, unit_id")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete DerivedSample from the database.
    ///
    /// # Arguments
    /// * `( parent_sample_id, child_sample_id )` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        ( parent_sample_id, child_sample_id ): ( uuid::Uuid, uuid::Uuid ),
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("derived_samples")
            .delete()
            .filter(col("parent_sample_id").eq(parent_sample_id.to_string()))
            .filter(col("child_sample_id").eq(child_sample_id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of DerivedSample from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(( self.parent_sample_id, self.child_sample_id ), connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("derived_samples")
            .update()        
.set("created_by", gluesql::core::ast_builder::num(self.created_by))        
.set("created_at", gluesql::core::ast_builder::timestamp(self.created_at.to_string()))        
.set("updated_by", gluesql::core::ast_builder::num(self.updated_by))        
.set("updated_at", gluesql::core::ast_builder::timestamp(self.updated_at.to_string()))        
.set("parent_sample_id", gluesql::core::ast_builder::uuid(self.parent_sample_id.to_string()))        
.set("child_sample_id", gluesql::core::ast_builder::uuid(self.child_sample_id.to_string()))        
.set("quantity", gluesql::core::ast_builder::num(self.quantity))        
.set("unit_id", gluesql::core::ast_builder::num(self.unit_id))            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all DerivedSample from the database.
    ///
    /// # Arguments
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        filter: Option<&DerivedSampleFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("derived_samples")
            .select()
            .filter(filter.map_or_else(|| gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into(), |filter| filter.as_filter_expression()))
           .project("created_by, created_at, updated_by, updated_at, parent_sample_id, child_sample_id, quantity, unit_id")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    /// Get all DerivedSample from the database ordered by the `updated_at` column.
    ///
    /// # Arguments
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all_by_updated_at<C>(
        filter: Option<&DerivedSampleFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("derived_samples")
            .select()
            .filter(filter.map_or_else(|| gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into(), |filter| filter.as_filter_expression()))
           .project("created_by, created_at, updated_by, updated_at, parent_sample_id, child_sample_id, quantity, unit_id")
            .order_by("updated_at desc")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            created_by: match row.get("created_by").unwrap() {
                gluesql::prelude::Value::I32(created_by) => created_by.clone(),
                _ => unreachable!("Expected I32")
            },
            created_at: match row.get("created_at").unwrap() {
                gluesql::prelude::Value::Timestamp(created_at) => created_at.clone(),
                _ => unreachable!("Expected Timestamp")
            },
            updated_by: match row.get("updated_by").unwrap() {
                gluesql::prelude::Value::I32(updated_by) => updated_by.clone(),
                _ => unreachable!("Expected I32")
            },
            updated_at: match row.get("updated_at").unwrap() {
                gluesql::prelude::Value::Timestamp(updated_at) => updated_at.clone(),
                _ => unreachable!("Expected Timestamp")
            },
            parent_sample_id: match row.get("parent_sample_id").unwrap() {
                gluesql::prelude::Value::Uuid(parent_sample_id) => uuid::Uuid::from_u128(*parent_sample_id),
                _ => unreachable!("Expected Uuid"),
            },
            child_sample_id: match row.get("child_sample_id").unwrap() {
                gluesql::prelude::Value::Uuid(child_sample_id) => uuid::Uuid::from_u128(*child_sample_id),
                _ => unreachable!("Expected Uuid"),
            },
            quantity: match row.get("quantity").unwrap() {
                gluesql::prelude::Value::F64(quantity) => quantity.clone(),
                _ => unreachable!("Expected F64")
            },
            unit_id: match row.get("unit_id").unwrap() {
                gluesql::prelude::Value::I32(unit_id) => unit_id.clone(),
                _ => unreachable!("Expected I32")
            },
        }
    }
}
#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct DocumentFormat {
    pub id: i32,
    pub extension: String,
    pub mime_type: String,
    pub description: String,
    pub icon_id: i32,
    pub color_id: i32,
}

unsafe impl Send for DocumentFormat {}
unsafe impl Sync for DocumentFormat {}
impl Tabular for DocumentFormat {
    const TABLE: Table = Table::DocumentFormats;
}
impl Describable for DocumentFormat {
    fn description(&self) -> Option<&str> {
        Some(self.description.as_str())
    }
}
impl Colorable for DocumentFormat {
    fn color(&self) -> Option<&str> {
        None
    }
}

impl Filtrable for DocumentFormat {
    type Filter = DocumentFormatFilter;
}
#[cfg(feature = "frontend")]
impl DocumentFormat {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::text(self.extension),
            gluesql::core::ast_builder::text(self.mime_type),
            gluesql::core::ast_builder::text(self.description),
            gluesql::core::ast_builder::num(self.icon_id),
            gluesql::core::ast_builder::num(self.color_id),
        ]
    }

    /// Insert the DocumentFormat into the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn insert<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut> (
self: crate::database::flat_variants::DocumentFormat,
connection: &mut gluesql::prelude::Glue<C>,
) -> Result<usize, gluesql::prelude::Error>    {
        use gluesql::core::ast_builder::*;
        table("document_formats")
            .insert()
            .columns("id, extension, mime_type, description, icon_id, color_id")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get DocumentFormat from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("document_formats")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, extension, mime_type, description, icon_id, color_id")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete DocumentFormat from the database.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("document_formats")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of DocumentFormat from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("document_formats")
            .update()        
.set("id", gluesql::core::ast_builder::num(self.id))        
.set("extension", gluesql::core::ast_builder::text(self.extension))        
.set("mime_type", gluesql::core::ast_builder::text(self.mime_type))        
.set("description", gluesql::core::ast_builder::text(self.description))        
.set("icon_id", gluesql::core::ast_builder::num(self.icon_id))        
.set("color_id", gluesql::core::ast_builder::num(self.color_id))            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all DocumentFormat from the database.
    ///
    /// # Arguments
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        filter: Option<&DocumentFormatFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("document_formats")
            .select()
            .filter(filter.map_or_else(|| gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into(), |filter| filter.as_filter_expression()))
           .project("id, extension, mime_type, description, icon_id, color_id")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::I32(id) => id.clone(),
                _ => unreachable!("Expected I32")
            },
            extension: match row.get("extension").unwrap() {
                gluesql::prelude::Value::Str(extension) => extension.clone(),
                _ => unreachable!("Expected Str")
            },
            mime_type: match row.get("mime_type").unwrap() {
                gluesql::prelude::Value::Str(mime_type) => mime_type.clone(),
                _ => unreachable!("Expected Str")
            },
            description: match row.get("description").unwrap() {
                gluesql::prelude::Value::Str(description) => description.clone(),
                _ => unreachable!("Expected Str")
            },
            icon_id: match row.get("icon_id").unwrap() {
                gluesql::prelude::Value::I32(icon_id) => icon_id.clone(),
                _ => unreachable!("Expected I32")
            },
            color_id: match row.get("color_id").unwrap() {
                gluesql::prelude::Value::I32(color_id) => color_id.clone(),
                _ => unreachable!("Expected I32")
            },
        }
    }
}
#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct FontAwesomeIcon {
    pub id: i32,
    pub name: String,
    pub description: String,
}

unsafe impl Send for FontAwesomeIcon {}
unsafe impl Sync for FontAwesomeIcon {}
impl Tabular for FontAwesomeIcon {
    const TABLE: Table = Table::FontAwesomeIcons;
}
impl Describable for FontAwesomeIcon {
    fn description(&self) -> Option<&str> {
        Some(self.description.as_str())
    }
}
impl Colorable for FontAwesomeIcon {
    fn color(&self) -> Option<&str> {
        None
    }
}

impl Filtrable for FontAwesomeIcon {
    type Filter = EmptyFilter;
}
#[cfg(feature = "frontend")]
impl FontAwesomeIcon {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::text(self.name),
            gluesql::core::ast_builder::text(self.description),
        ]
    }

    /// Insert the FontAwesomeIcon into the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn insert<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut> (
self: crate::database::flat_variants::FontAwesomeIcon,
connection: &mut gluesql::prelude::Glue<C>,
) -> Result<usize, gluesql::prelude::Error>    {
        use gluesql::core::ast_builder::*;
        table("font_awesome_icons")
            .insert()
            .columns("id, name, description")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get FontAwesomeIcon from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("font_awesome_icons")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, name, description")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete FontAwesomeIcon from the database.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("font_awesome_icons")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of FontAwesomeIcon from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("font_awesome_icons")
            .update()        
.set("id", gluesql::core::ast_builder::num(self.id))        
.set("name", gluesql::core::ast_builder::text(self.name))        
.set("description", gluesql::core::ast_builder::text(self.description))            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all FontAwesomeIcon from the database.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("font_awesome_icons")
            .select()
           .project("id, name, description")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::I32(id) => id.clone(),
                _ => unreachable!("Expected I32")
            },
            name: match row.get("name").unwrap() {
                gluesql::prelude::Value::Str(name) => name.clone(),
                _ => unreachable!("Expected Str")
            },
            description: match row.get("description").unwrap() {
                gluesql::prelude::Value::Str(description) => description.clone(),
                _ => unreachable!("Expected Str")
            },
        }
    }
}
#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct LoginProvider {
    pub id: i32,
    pub name: String,
    pub icon_id: i32,
    pub color_id: i32,
    pub client_id_var_name: String,
    pub redirect_uri_var_name: String,
    pub oauth_url: String,
    pub scope: String,
}

unsafe impl Send for LoginProvider {}
unsafe impl Sync for LoginProvider {}
impl Tabular for LoginProvider {
    const TABLE: Table = Table::LoginProviders;
}
impl Describable for LoginProvider {
    fn description(&self) -> Option<&str> {
        None
    }
}
impl Colorable for LoginProvider {
    fn color(&self) -> Option<&str> {
        None
    }
}

impl Filtrable for LoginProvider {
    type Filter = LoginProviderFilter;
}
#[cfg(feature = "frontend")]
impl LoginProvider {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::text(self.name),
            gluesql::core::ast_builder::num(self.icon_id),
            gluesql::core::ast_builder::num(self.color_id),
            gluesql::core::ast_builder::text(self.client_id_var_name),
            gluesql::core::ast_builder::text(self.redirect_uri_var_name),
            gluesql::core::ast_builder::text(self.oauth_url),
            gluesql::core::ast_builder::text(self.scope),
        ]
    }

    /// Insert the LoginProvider into the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn insert<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut> (
self: crate::database::flat_variants::LoginProvider,
connection: &mut gluesql::prelude::Glue<C>,
) -> Result<usize, gluesql::prelude::Error>    {
        use gluesql::core::ast_builder::*;
        table("login_providers")
            .insert()
            .columns("id, name, icon_id, color_id, client_id_var_name, redirect_uri_var_name, oauth_url, scope")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get LoginProvider from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("login_providers")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, name, icon_id, color_id, client_id_var_name, redirect_uri_var_name, oauth_url, scope")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete LoginProvider from the database.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("login_providers")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of LoginProvider from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("login_providers")
            .update()        
.set("id", gluesql::core::ast_builder::num(self.id))        
.set("name", gluesql::core::ast_builder::text(self.name))        
.set("icon_id", gluesql::core::ast_builder::num(self.icon_id))        
.set("color_id", gluesql::core::ast_builder::num(self.color_id))        
.set("client_id_var_name", gluesql::core::ast_builder::text(self.client_id_var_name))        
.set("redirect_uri_var_name", gluesql::core::ast_builder::text(self.redirect_uri_var_name))        
.set("oauth_url", gluesql::core::ast_builder::text(self.oauth_url))        
.set("scope", gluesql::core::ast_builder::text(self.scope))            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all LoginProvider from the database.
    ///
    /// # Arguments
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        filter: Option<&LoginProviderFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("login_providers")
            .select()
            .filter(filter.map_or_else(|| gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into(), |filter| filter.as_filter_expression()))
           .project("id, name, icon_id, color_id, client_id_var_name, redirect_uri_var_name, oauth_url, scope")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::I32(id) => id.clone(),
                _ => unreachable!("Expected I32")
            },
            name: match row.get("name").unwrap() {
                gluesql::prelude::Value::Str(name) => name.clone(),
                _ => unreachable!("Expected Str")
            },
            icon_id: match row.get("icon_id").unwrap() {
                gluesql::prelude::Value::I32(icon_id) => icon_id.clone(),
                _ => unreachable!("Expected I32")
            },
            color_id: match row.get("color_id").unwrap() {
                gluesql::prelude::Value::I32(color_id) => color_id.clone(),
                _ => unreachable!("Expected I32")
            },
            client_id_var_name: match row.get("client_id_var_name").unwrap() {
                gluesql::prelude::Value::Str(client_id_var_name) => client_id_var_name.clone(),
                _ => unreachable!("Expected Str")
            },
            redirect_uri_var_name: match row.get("redirect_uri_var_name").unwrap() {
                gluesql::prelude::Value::Str(redirect_uri_var_name) => redirect_uri_var_name.clone(),
                _ => unreachable!("Expected Str")
            },
            oauth_url: match row.get("oauth_url").unwrap() {
                gluesql::prelude::Value::Str(oauth_url) => oauth_url.clone(),
                _ => unreachable!("Expected Str")
            },
            scope: match row.get("scope").unwrap() {
                gluesql::prelude::Value::Str(scope) => scope.clone(),
                _ => unreachable!("Expected Str")
            },
        }
    }
}
#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct Material {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub icon_id: i32,
    pub color_id: i32,
}

unsafe impl Send for Material {}
unsafe impl Sync for Material {}
impl Tabular for Material {
    const TABLE: Table = Table::Materials;
}
impl Describable for Material {
    fn description(&self) -> Option<&str> {
        Some(self.description.as_str())
    }
}
impl Colorable for Material {
    fn color(&self) -> Option<&str> {
        None
    }
}

impl Filtrable for Material {
    type Filter = MaterialFilter;
}
#[cfg(feature = "frontend")]
impl Material {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::text(self.name),
            gluesql::core::ast_builder::text(self.description),
            gluesql::core::ast_builder::num(self.icon_id),
            gluesql::core::ast_builder::num(self.color_id),
        ]
    }

    /// Insert the Material into the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn insert<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut> (
self: crate::database::flat_variants::Material,
connection: &mut gluesql::prelude::Glue<C>,
) -> Result<usize, gluesql::prelude::Error>    {
        use gluesql::core::ast_builder::*;
        table("materials")
            .insert()
            .columns("id, name, description, icon_id, color_id")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get Material from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("materials")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, name, description, icon_id, color_id")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete Material from the database.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("materials")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of Material from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("materials")
            .update()        
.set("id", gluesql::core::ast_builder::num(self.id))        
.set("name", gluesql::core::ast_builder::text(self.name))        
.set("description", gluesql::core::ast_builder::text(self.description))        
.set("icon_id", gluesql::core::ast_builder::num(self.icon_id))        
.set("color_id", gluesql::core::ast_builder::num(self.color_id))            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all Material from the database.
    ///
    /// # Arguments
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        filter: Option<&MaterialFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("materials")
            .select()
            .filter(filter.map_or_else(|| gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into(), |filter| filter.as_filter_expression()))
           .project("id, name, description, icon_id, color_id")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::I32(id) => id.clone(),
                _ => unreachable!("Expected I32")
            },
            name: match row.get("name").unwrap() {
                gluesql::prelude::Value::Str(name) => name.clone(),
                _ => unreachable!("Expected Str")
            },
            description: match row.get("description").unwrap() {
                gluesql::prelude::Value::Str(description) => description.clone(),
                _ => unreachable!("Expected Str")
            },
            icon_id: match row.get("icon_id").unwrap() {
                gluesql::prelude::Value::I32(icon_id) => icon_id.clone(),
                _ => unreachable!("Expected I32")
            },
            color_id: match row.get("color_id").unwrap() {
                gluesql::prelude::Value::I32(color_id) => color_id.clone(),
                _ => unreachable!("Expected I32")
            },
        }
    }
}
#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct NameplateCategory {
    pub id: i32,
    pub name: String,
    pub permanence_id: i32,
    pub material_id: i32,
    pub description: String,
    pub icon_id: i32,
    pub color_id: i32,
}

unsafe impl Send for NameplateCategory {}
unsafe impl Sync for NameplateCategory {}
impl Tabular for NameplateCategory {
    const TABLE: Table = Table::NameplateCategories;
}
impl Describable for NameplateCategory {
    fn description(&self) -> Option<&str> {
        Some(self.description.as_str())
    }
}
impl Colorable for NameplateCategory {
    fn color(&self) -> Option<&str> {
        None
    }
}

impl Filtrable for NameplateCategory {
    type Filter = NameplateCategoryFilter;
}
#[cfg(feature = "frontend")]
impl NameplateCategory {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::text(self.name),
            gluesql::core::ast_builder::num(self.permanence_id),
            gluesql::core::ast_builder::num(self.material_id),
            gluesql::core::ast_builder::text(self.description),
            gluesql::core::ast_builder::num(self.icon_id),
            gluesql::core::ast_builder::num(self.color_id),
        ]
    }

    /// Insert the NameplateCategory into the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn insert<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut> (
self: crate::database::flat_variants::NameplateCategory,
connection: &mut gluesql::prelude::Glue<C>,
) -> Result<usize, gluesql::prelude::Error>    {
        use gluesql::core::ast_builder::*;
        table("nameplate_categories")
            .insert()
            .columns("id, name, permanence_id, material_id, description, icon_id, color_id")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get NameplateCategory from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("nameplate_categories")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, name, permanence_id, material_id, description, icon_id, color_id")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete NameplateCategory from the database.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("nameplate_categories")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of NameplateCategory from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("nameplate_categories")
            .update()        
.set("id", gluesql::core::ast_builder::num(self.id))        
.set("name", gluesql::core::ast_builder::text(self.name))        
.set("permanence_id", gluesql::core::ast_builder::num(self.permanence_id))        
.set("material_id", gluesql::core::ast_builder::num(self.material_id))        
.set("description", gluesql::core::ast_builder::text(self.description))        
.set("icon_id", gluesql::core::ast_builder::num(self.icon_id))        
.set("color_id", gluesql::core::ast_builder::num(self.color_id))            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all NameplateCategory from the database.
    ///
    /// # Arguments
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        filter: Option<&NameplateCategoryFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("nameplate_categories")
            .select()
            .filter(filter.map_or_else(|| gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into(), |filter| filter.as_filter_expression()))
           .project("id, name, permanence_id, material_id, description, icon_id, color_id")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::I32(id) => id.clone(),
                _ => unreachable!("Expected I32")
            },
            name: match row.get("name").unwrap() {
                gluesql::prelude::Value::Str(name) => name.clone(),
                _ => unreachable!("Expected Str")
            },
            permanence_id: match row.get("permanence_id").unwrap() {
                gluesql::prelude::Value::I32(permanence_id) => permanence_id.clone(),
                _ => unreachable!("Expected I32")
            },
            material_id: match row.get("material_id").unwrap() {
                gluesql::prelude::Value::I32(material_id) => material_id.clone(),
                _ => unreachable!("Expected I32")
            },
            description: match row.get("description").unwrap() {
                gluesql::prelude::Value::Str(description) => description.clone(),
                _ => unreachable!("Expected Str")
            },
            icon_id: match row.get("icon_id").unwrap() {
                gluesql::prelude::Value::I32(icon_id) => icon_id.clone(),
                _ => unreachable!("Expected I32")
            },
            color_id: match row.get("color_id").unwrap() {
                gluesql::prelude::Value::I32(color_id) => color_id.clone(),
                _ => unreachable!("Expected I32")
            },
        }
    }
}
#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Nameplate {
    pub id: i32,
    pub barcode: String,
    pub project_id: i32,
    pub category_id: i32,
    pub created_by: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_by: i32,
    pub updated_at: chrono::NaiveDateTime,
    pub geolocation: crate::types::Point,
}

unsafe impl Send for Nameplate {}
unsafe impl Sync for Nameplate {}
impl Tabular for Nameplate {
    const TABLE: Table = Table::Nameplates;
}
impl Describable for Nameplate {
    fn description(&self) -> Option<&str> {
        None
    }
}
impl Colorable for Nameplate {
    fn color(&self) -> Option<&str> {
        None
    }
}

impl Filtrable for Nameplate {
    type Filter = NameplateFilter;
}
#[cfg(feature = "frontend")]
impl Nameplate {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::text(self.barcode),
            gluesql::core::ast_builder::num(self.project_id),
            gluesql::core::ast_builder::num(self.category_id),
            gluesql::core::ast_builder::num(self.created_by),
            gluesql::core::ast_builder::timestamp(self.created_at.to_string()),
            gluesql::core::ast_builder::num(self.updated_by),
            gluesql::core::ast_builder::timestamp(self.updated_at.to_string()),
            gluesql::core::ast_builder::function::point(gluesql::core::ast_builder::num(self.geolocation.x), gluesql::core::ast_builder::num(self.geolocation.y)),
        ]
    }

    /// Insert the Nameplate into the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn insert<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut> (
self: crate::database::flat_variants::Nameplate,
connection: &mut gluesql::prelude::Glue<C>,
) -> Result<usize, gluesql::prelude::Error>    {
        use gluesql::core::ast_builder::*;
        table("nameplates")
            .insert()
            .columns("id, barcode, project_id, category_id, created_by, created_at, updated_by, updated_at, geolocation")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get Nameplate from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("nameplates")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, barcode, project_id, category_id, created_by, created_at, updated_by, updated_at, geolocation")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete Nameplate from the database.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("nameplates")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of Nameplate from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("nameplates")
            .update()        
.set("id", gluesql::core::ast_builder::num(self.id))        
.set("barcode", gluesql::core::ast_builder::text(self.barcode))        
.set("project_id", gluesql::core::ast_builder::num(self.project_id))        
.set("category_id", gluesql::core::ast_builder::num(self.category_id))        
.set("created_by", gluesql::core::ast_builder::num(self.created_by))        
.set("created_at", gluesql::core::ast_builder::timestamp(self.created_at.to_string()))        
.set("updated_by", gluesql::core::ast_builder::num(self.updated_by))        
.set("updated_at", gluesql::core::ast_builder::timestamp(self.updated_at.to_string()))        
.set("geolocation", gluesql::core::ast_builder::function::point(gluesql::core::ast_builder::num(self.geolocation.x), gluesql::core::ast_builder::num(self.geolocation.y)))            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all Nameplate from the database.
    ///
    /// # Arguments
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        filter: Option<&NameplateFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("nameplates")
            .select()
            .filter(filter.map_or_else(|| gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into(), |filter| filter.as_filter_expression()))
           .project("id, barcode, project_id, category_id, created_by, created_at, updated_by, updated_at, geolocation")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    /// Get all Nameplate from the database ordered by the `updated_at` column.
    ///
    /// # Arguments
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all_by_updated_at<C>(
        filter: Option<&NameplateFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("nameplates")
            .select()
            .filter(filter.map_or_else(|| gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into(), |filter| filter.as_filter_expression()))
           .project("id, barcode, project_id, category_id, created_by, created_at, updated_by, updated_at, geolocation")
            .order_by("updated_at desc")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::I32(id) => id.clone(),
                _ => unreachable!("Expected I32")
            },
            barcode: match row.get("barcode").unwrap() {
                gluesql::prelude::Value::Str(barcode) => barcode.clone(),
                _ => unreachable!("Expected Str")
            },
            project_id: match row.get("project_id").unwrap() {
                gluesql::prelude::Value::I32(project_id) => project_id.clone(),
                _ => unreachable!("Expected I32")
            },
            category_id: match row.get("category_id").unwrap() {
                gluesql::prelude::Value::I32(category_id) => category_id.clone(),
                _ => unreachable!("Expected I32")
            },
            created_by: match row.get("created_by").unwrap() {
                gluesql::prelude::Value::I32(created_by) => created_by.clone(),
                _ => unreachable!("Expected I32")
            },
            created_at: match row.get("created_at").unwrap() {
                gluesql::prelude::Value::Timestamp(created_at) => created_at.clone(),
                _ => unreachable!("Expected Timestamp")
            },
            updated_by: match row.get("updated_by").unwrap() {
                gluesql::prelude::Value::I32(updated_by) => updated_by.clone(),
                _ => unreachable!("Expected I32")
            },
            updated_at: match row.get("updated_at").unwrap() {
                gluesql::prelude::Value::Timestamp(updated_at) => updated_at.clone(),
                _ => unreachable!("Expected Timestamp")
            },
            geolocation: match row.get("geolocation").unwrap() {
                gluesql::prelude::Value::Point(geolocation) => geolocation.clone().into(),
                _ => unreachable!("Expected Bytea"),
            },
        }
    }
}
#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct Notification {
    pub id: i32,
    pub user_id: i32,
    pub operation: String,
    pub table_name: String,
    pub record: String,
    pub read: bool,
}

unsafe impl Send for Notification {}
unsafe impl Sync for Notification {}
impl Tabular for Notification {
    const TABLE: Table = Table::Notifications;
}
impl Describable for Notification {
    fn description(&self) -> Option<&str> {
        None
    }
}
impl Colorable for Notification {
    fn color(&self) -> Option<&str> {
        None
    }
}

impl Filtrable for Notification {
    type Filter = NotificationFilter;
}
#[cfg(feature = "frontend")]
impl Notification {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::num(self.user_id),
            gluesql::core::ast_builder::text(self.operation),
            gluesql::core::ast_builder::text(self.table_name),
            gluesql::core::ast_builder::text(self.record),
            (self.read.into()),
        ]
    }

    /// Insert the Notification into the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn insert<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut> (
self: crate::database::flat_variants::Notification,
connection: &mut gluesql::prelude::Glue<C>,
) -> Result<usize, gluesql::prelude::Error>    {
        use gluesql::core::ast_builder::*;
        table("notifications")
            .insert()
            .columns("id, user_id, operation, table_name, record, read")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get Notification from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("notifications")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, user_id, operation, table_name, record, read")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete Notification from the database.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("notifications")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of Notification from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("notifications")
            .update()        
.set("id", gluesql::core::ast_builder::num(self.id))        
.set("user_id", gluesql::core::ast_builder::num(self.user_id))        
.set("operation", gluesql::core::ast_builder::text(self.operation))        
.set("table_name", gluesql::core::ast_builder::text(self.table_name))        
.set("record", gluesql::core::ast_builder::text(self.record))        
.set("read", self.read)            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all Notification from the database.
    ///
    /// # Arguments
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        filter: Option<&NotificationFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("notifications")
            .select()
            .filter(filter.map_or_else(|| gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into(), |filter| filter.as_filter_expression()))
           .project("id, user_id, operation, table_name, record, read")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::I32(id) => id.clone(),
                _ => unreachable!("Expected I32")
            },
            user_id: match row.get("user_id").unwrap() {
                gluesql::prelude::Value::I32(user_id) => user_id.clone(),
                _ => unreachable!("Expected I32")
            },
            operation: match row.get("operation").unwrap() {
                gluesql::prelude::Value::Str(operation) => operation.clone(),
                _ => unreachable!("Expected Str")
            },
            table_name: match row.get("table_name").unwrap() {
                gluesql::prelude::Value::Str(table_name) => table_name.clone(),
                _ => unreachable!("Expected Str")
            },
            record: match row.get("record").unwrap() {
                gluesql::prelude::Value::Str(record) => record.clone(),
                _ => unreachable!("Expected Str")
            },
            read: match row.get("read").unwrap() {
                gluesql::prelude::Value::Bool(read) => read.clone(),
                _ => unreachable!("Expected Bool")
            },
        }
    }
}
#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct ObservationSubject {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub icon_id: i32,
    pub color_id: i32,
}

unsafe impl Send for ObservationSubject {}
unsafe impl Sync for ObservationSubject {}
impl Tabular for ObservationSubject {
    const TABLE: Table = Table::ObservationSubjects;
}
impl Describable for ObservationSubject {
    fn description(&self) -> Option<&str> {
        Some(self.description.as_str())
    }
}
impl Colorable for ObservationSubject {
    fn color(&self) -> Option<&str> {
        None
    }
}

impl Filtrable for ObservationSubject {
    type Filter = ObservationSubjectFilter;
}
#[cfg(feature = "frontend")]
impl ObservationSubject {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::text(self.name),
            gluesql::core::ast_builder::text(self.description),
            gluesql::core::ast_builder::num(self.icon_id),
            gluesql::core::ast_builder::num(self.color_id),
        ]
    }

    /// Insert the ObservationSubject into the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn insert<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut> (
self: crate::database::flat_variants::ObservationSubject,
connection: &mut gluesql::prelude::Glue<C>,
) -> Result<usize, gluesql::prelude::Error>    {
        use gluesql::core::ast_builder::*;
        table("observation_subjects")
            .insert()
            .columns("id, name, description, icon_id, color_id")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get ObservationSubject from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("observation_subjects")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, name, description, icon_id, color_id")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete ObservationSubject from the database.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("observation_subjects")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of ObservationSubject from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("observation_subjects")
            .update()        
.set("id", gluesql::core::ast_builder::num(self.id))        
.set("name", gluesql::core::ast_builder::text(self.name))        
.set("description", gluesql::core::ast_builder::text(self.description))        
.set("icon_id", gluesql::core::ast_builder::num(self.icon_id))        
.set("color_id", gluesql::core::ast_builder::num(self.color_id))            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all ObservationSubject from the database.
    ///
    /// # Arguments
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        filter: Option<&ObservationSubjectFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("observation_subjects")
            .select()
            .filter(filter.map_or_else(|| gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into(), |filter| filter.as_filter_expression()))
           .project("id, name, description, icon_id, color_id")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::I32(id) => id.clone(),
                _ => unreachable!("Expected I32")
            },
            name: match row.get("name").unwrap() {
                gluesql::prelude::Value::Str(name) => name.clone(),
                _ => unreachable!("Expected Str")
            },
            description: match row.get("description").unwrap() {
                gluesql::prelude::Value::Str(description) => description.clone(),
                _ => unreachable!("Expected Str")
            },
            icon_id: match row.get("icon_id").unwrap() {
                gluesql::prelude::Value::I32(icon_id) => icon_id.clone(),
                _ => unreachable!("Expected I32")
            },
            color_id: match row.get("color_id").unwrap() {
                gluesql::prelude::Value::I32(color_id) => color_id.clone(),
                _ => unreachable!("Expected I32")
            },
        }
    }
}
#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct Observation {
    pub id: uuid::Uuid,
    pub parent_observation_id: Option<uuid::Uuid>,
    pub created_by: i32,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_by: i32,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub project_id: i32,
    pub organism_id: Option<uuid::Uuid>,
    pub sample_id: Option<uuid::Uuid>,
    pub subject_id: i32,
    pub notes: Option<String>,
    pub picture: crate::types::JPEG,
}

unsafe impl Send for Observation {}
unsafe impl Sync for Observation {}
impl Tabular for Observation {
    const TABLE: Table = Table::Observations;
}
impl Describable for Observation {
    fn description(&self) -> Option<&str> {
        self.notes.as_deref()
    }
}
impl Colorable for Observation {
    fn color(&self) -> Option<&str> {
        None
    }
}

impl Filtrable for Observation {
    type Filter = ObservationFilter;
}
#[cfg(feature = "frontend")]
impl Observation {
    pub fn get_picture_as_url(&self) -> String {
        self.picture.guess_image_url().unwrap()
    }

    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::uuid(self.id.to_string()),
            match self.parent_observation_id {
                Some(parent_observation_id) => gluesql::core::ast_builder::uuid(parent_observation_id.to_string()),
                None => gluesql::core::ast_builder::null(),
            },
            gluesql::core::ast_builder::num(self.created_by),
            match self.created_at {
                Some(created_at) => gluesql::core::ast_builder::timestamp(created_at.to_string()),
                None => gluesql::core::ast_builder::null(),
            },
            gluesql::core::ast_builder::num(self.updated_by),
            match self.updated_at {
                Some(updated_at) => gluesql::core::ast_builder::timestamp(updated_at.to_string()),
                None => gluesql::core::ast_builder::null(),
            },
            gluesql::core::ast_builder::num(self.project_id),
            match self.organism_id {
                Some(organism_id) => gluesql::core::ast_builder::uuid(organism_id.to_string()),
                None => gluesql::core::ast_builder::null(),
            },
            match self.sample_id {
                Some(sample_id) => gluesql::core::ast_builder::uuid(sample_id.to_string()),
                None => gluesql::core::ast_builder::null(),
            },
            gluesql::core::ast_builder::num(self.subject_id),
            match self.notes {
                Some(notes) => gluesql::core::ast_builder::text(notes),
                None => gluesql::core::ast_builder::null(),
            },
            gluesql::core::ast_builder::bytea(self.picture),
        ]
    }

    /// Insert the Observation into the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn insert<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut> (
self: crate::database::flat_variants::Observation,
connection: &mut gluesql::prelude::Glue<C>,
) -> Result<usize, gluesql::prelude::Error>    {
        use gluesql::core::ast_builder::*;
        table("observations")
            .insert()
            .columns("id, parent_observation_id, created_by, created_at, updated_by, updated_at, project_id, organism_id, sample_id, subject_id, notes, picture")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get Observation from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: uuid::Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("observations")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, parent_observation_id, created_by, created_at, updated_by, updated_at, project_id, organism_id, sample_id, subject_id, notes, picture")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete Observation from the database.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: uuid::Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("observations")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of Observation from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let mut update_row = table("observations")
            .update()        
.set("id", gluesql::core::ast_builder::uuid(self.id.to_string()))        
.set("created_by", gluesql::core::ast_builder::num(self.created_by))        
.set("updated_by", gluesql::core::ast_builder::num(self.updated_by))        
.set("project_id", gluesql::core::ast_builder::num(self.project_id))        
.set("subject_id", gluesql::core::ast_builder::num(self.subject_id))        
.set("picture", gluesql::core::ast_builder::bytea(self.picture));
        if let Some(parent_observation_id) = self.parent_observation_id {
            update_row = update_row.set("parent_observation_id", gluesql::core::ast_builder::uuid(parent_observation_id.to_string()));
        }
        if let Some(created_at) = self.created_at {
            update_row = update_row.set("created_at", gluesql::core::ast_builder::timestamp(created_at.to_string()));
        }
        if let Some(updated_at) = self.updated_at {
            update_row = update_row.set("updated_at", gluesql::core::ast_builder::timestamp(updated_at.to_string()));
        }
        if let Some(organism_id) = self.organism_id {
            update_row = update_row.set("organism_id", gluesql::core::ast_builder::uuid(organism_id.to_string()));
        }
        if let Some(sample_id) = self.sample_id {
            update_row = update_row.set("sample_id", gluesql::core::ast_builder::uuid(sample_id.to_string()));
        }
        if let Some(notes) = self.notes {
            update_row = update_row.set("notes", gluesql::core::ast_builder::text(notes));
        }
            update_row.execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all Observation from the database.
    ///
    /// # Arguments
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        filter: Option<&ObservationFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("observations")
            .select()
            .filter(filter.map_or_else(|| gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into(), |filter| filter.as_filter_expression()))
           .project("id, parent_observation_id, created_by, created_at, updated_by, updated_at, project_id, organism_id, sample_id, subject_id, notes, picture")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    /// Get all Observation from the database ordered by the `updated_at` column.
    ///
    /// # Arguments
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all_by_updated_at<C>(
        filter: Option<&ObservationFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("observations")
            .select()
            .filter(filter.map_or_else(|| gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into(), |filter| filter.as_filter_expression()))
           .project("id, parent_observation_id, created_by, created_at, updated_by, updated_at, project_id, organism_id, sample_id, subject_id, notes, picture")
            .order_by("updated_at desc")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => uuid::Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            parent_observation_id: match row.get("parent_observation_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(parent_observation_id) => Some(uuid::Uuid::from_u128(*parent_observation_id)),
                _ => unreachable!("Expected Uuid"),
            },
            created_by: match row.get("created_by").unwrap() {
                gluesql::prelude::Value::I32(created_by) => created_by.clone(),
                _ => unreachable!("Expected I32")
            },
            created_at: match row.get("created_at").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Timestamp(created_at) => Some(created_at.clone()),
                _ => unreachable!("Expected Timestamp")
            },
            updated_by: match row.get("updated_by").unwrap() {
                gluesql::prelude::Value::I32(updated_by) => updated_by.clone(),
                _ => unreachable!("Expected I32")
            },
            updated_at: match row.get("updated_at").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Timestamp(updated_at) => Some(updated_at.clone()),
                _ => unreachable!("Expected Timestamp")
            },
            project_id: match row.get("project_id").unwrap() {
                gluesql::prelude::Value::I32(project_id) => project_id.clone(),
                _ => unreachable!("Expected I32")
            },
            organism_id: match row.get("organism_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(organism_id) => Some(uuid::Uuid::from_u128(*organism_id)),
                _ => unreachable!("Expected Uuid"),
            },
            sample_id: match row.get("sample_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(sample_id) => Some(uuid::Uuid::from_u128(*sample_id)),
                _ => unreachable!("Expected Uuid"),
            },
            subject_id: match row.get("subject_id").unwrap() {
                gluesql::prelude::Value::I32(subject_id) => subject_id.clone(),
                _ => unreachable!("Expected I32")
            },
            notes: match row.get("notes").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Str(notes) => Some(notes.clone()),
                _ => unreachable!("Expected Str")
            },
            picture: match row.get("picture").unwrap() {
                gluesql::prelude::Value::Bytea(picture) => picture.clone().into(),
                _ => unreachable!("Expected Bytea"),
            },
        }
    }
}
#[derive(Eq, PartialEq, Debug, Clone, Copy, serde::Serialize, serde::Deserialize, Default)]
pub struct OrganismBioOttTaxonItem {
    pub created_by: i32,
    pub created_at: chrono::NaiveDateTime,
    pub organism_id: uuid::Uuid,
    pub taxon_id: i32,
}

unsafe impl Send for OrganismBioOttTaxonItem {}
unsafe impl Sync for OrganismBioOttTaxonItem {}
impl Tabular for OrganismBioOttTaxonItem {
    const TABLE: Table = Table::OrganismBioOttTaxonItems;
}
impl Describable for OrganismBioOttTaxonItem {
    fn description(&self) -> Option<&str> {
        None
    }
}
impl Colorable for OrganismBioOttTaxonItem {
    fn color(&self) -> Option<&str> {
        None
    }
}

impl Filtrable for OrganismBioOttTaxonItem {
    type Filter = OrganismBioOttTaxonItemFilter;
}
#[cfg(feature = "frontend")]
impl OrganismBioOttTaxonItem {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.created_by),
            gluesql::core::ast_builder::timestamp(self.created_at.to_string()),
            gluesql::core::ast_builder::uuid(self.organism_id.to_string()),
            gluesql::core::ast_builder::num(self.taxon_id),
        ]
    }

    /// Insert the OrganismBioOttTaxonItem into the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn insert<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut> (
self: crate::database::flat_variants::OrganismBioOttTaxonItem,
connection: &mut gluesql::prelude::Glue<C>,
) -> Result<usize, gluesql::prelude::Error>    {
        use gluesql::core::ast_builder::*;
        table("organism_bio_ott_taxon_items")
            .insert()
            .columns("created_by, created_at, organism_id, taxon_id")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get OrganismBioOttTaxonItem from the database by its ID.
    ///
    /// # Arguments
    /// * `( organism_id, taxon_id )` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        ( organism_id, taxon_id ): ( uuid::Uuid, i32 ),
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("organism_bio_ott_taxon_items")
            .select()
            .filter(col("organism_id").eq(organism_id.to_string()))
            .filter(col("taxon_id").eq(taxon_id.to_string()))
            .project("created_by, created_at, organism_id, taxon_id")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete OrganismBioOttTaxonItem from the database.
    ///
    /// # Arguments
    /// * `( organism_id, taxon_id )` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        ( organism_id, taxon_id ): ( uuid::Uuid, i32 ),
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("organism_bio_ott_taxon_items")
            .delete()
            .filter(col("organism_id").eq(organism_id.to_string()))
            .filter(col("taxon_id").eq(taxon_id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of OrganismBioOttTaxonItem from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(( self.organism_id, self.taxon_id ), connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("organism_bio_ott_taxon_items")
            .update()        
.set("created_by", gluesql::core::ast_builder::num(self.created_by))        
.set("created_at", gluesql::core::ast_builder::timestamp(self.created_at.to_string()))        
.set("organism_id", gluesql::core::ast_builder::uuid(self.organism_id.to_string()))        
.set("taxon_id", gluesql::core::ast_builder::num(self.taxon_id))            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all OrganismBioOttTaxonItem from the database.
    ///
    /// # Arguments
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        filter: Option<&OrganismBioOttTaxonItemFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("organism_bio_ott_taxon_items")
            .select()
            .filter(filter.map_or_else(|| gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into(), |filter| filter.as_filter_expression()))
           .project("created_by, created_at, organism_id, taxon_id")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            created_by: match row.get("created_by").unwrap() {
                gluesql::prelude::Value::I32(created_by) => created_by.clone(),
                _ => unreachable!("Expected I32")
            },
            created_at: match row.get("created_at").unwrap() {
                gluesql::prelude::Value::Timestamp(created_at) => created_at.clone(),
                _ => unreachable!("Expected Timestamp")
            },
            organism_id: match row.get("organism_id").unwrap() {
                gluesql::prelude::Value::Uuid(organism_id) => uuid::Uuid::from_u128(*organism_id),
                _ => unreachable!("Expected Uuid"),
            },
            taxon_id: match row.get("taxon_id").unwrap() {
                gluesql::prelude::Value::I32(taxon_id) => taxon_id.clone(),
                _ => unreachable!("Expected I32")
            },
        }
    }
}
#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct Organism {
    pub id: uuid::Uuid,
    pub host_organism_id: Option<uuid::Uuid>,
    pub sample_id: Option<uuid::Uuid>,
    pub notes: Option<String>,
    pub nameplate_id: i32,
    pub project_id: i32,
    pub created_by: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_by: i32,
    pub updated_at: chrono::NaiveDateTime,
}

unsafe impl Send for Organism {}
unsafe impl Sync for Organism {}
impl Tabular for Organism {
    const TABLE: Table = Table::Organisms;
}
impl Describable for Organism {
    fn description(&self) -> Option<&str> {
        self.notes.as_deref()
    }
}
impl Colorable for Organism {
    fn color(&self) -> Option<&str> {
        None
    }
}

impl Filtrable for Organism {
    type Filter = OrganismFilter;
}
#[cfg(feature = "frontend")]
impl Organism {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::uuid(self.id.to_string()),
            match self.host_organism_id {
                Some(host_organism_id) => gluesql::core::ast_builder::uuid(host_organism_id.to_string()),
                None => gluesql::core::ast_builder::null(),
            },
            match self.sample_id {
                Some(sample_id) => gluesql::core::ast_builder::uuid(sample_id.to_string()),
                None => gluesql::core::ast_builder::null(),
            },
            match self.notes {
                Some(notes) => gluesql::core::ast_builder::text(notes),
                None => gluesql::core::ast_builder::null(),
            },
            gluesql::core::ast_builder::num(self.nameplate_id),
            gluesql::core::ast_builder::num(self.project_id),
            gluesql::core::ast_builder::num(self.created_by),
            gluesql::core::ast_builder::timestamp(self.created_at.to_string()),
            gluesql::core::ast_builder::num(self.updated_by),
            gluesql::core::ast_builder::timestamp(self.updated_at.to_string()),
        ]
    }

    /// Insert the Organism into the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn insert<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut> (
self: crate::database::flat_variants::Organism,
connection: &mut gluesql::prelude::Glue<C>,
) -> Result<usize, gluesql::prelude::Error>    {
        use gluesql::core::ast_builder::*;
        table("organisms")
            .insert()
            .columns("id, host_organism_id, sample_id, notes, nameplate_id, project_id, created_by, created_at, updated_by, updated_at")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get Organism from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: uuid::Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("organisms")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, host_organism_id, sample_id, notes, nameplate_id, project_id, created_by, created_at, updated_by, updated_at")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete Organism from the database.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: uuid::Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("organisms")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of Organism from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let mut update_row = table("organisms")
            .update()        
.set("id", gluesql::core::ast_builder::uuid(self.id.to_string()))        
.set("nameplate_id", gluesql::core::ast_builder::num(self.nameplate_id))        
.set("project_id", gluesql::core::ast_builder::num(self.project_id))        
.set("created_by", gluesql::core::ast_builder::num(self.created_by))        
.set("created_at", gluesql::core::ast_builder::timestamp(self.created_at.to_string()))        
.set("updated_by", gluesql::core::ast_builder::num(self.updated_by))        
.set("updated_at", gluesql::core::ast_builder::timestamp(self.updated_at.to_string()));
        if let Some(host_organism_id) = self.host_organism_id {
            update_row = update_row.set("host_organism_id", gluesql::core::ast_builder::uuid(host_organism_id.to_string()));
        }
        if let Some(sample_id) = self.sample_id {
            update_row = update_row.set("sample_id", gluesql::core::ast_builder::uuid(sample_id.to_string()));
        }
        if let Some(notes) = self.notes {
            update_row = update_row.set("notes", gluesql::core::ast_builder::text(notes));
        }
            update_row.execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all Organism from the database.
    ///
    /// # Arguments
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        filter: Option<&OrganismFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("organisms")
            .select()
            .filter(filter.map_or_else(|| gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into(), |filter| filter.as_filter_expression()))
           .project("id, host_organism_id, sample_id, notes, nameplate_id, project_id, created_by, created_at, updated_by, updated_at")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    /// Get all Organism from the database ordered by the `updated_at` column.
    ///
    /// # Arguments
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all_by_updated_at<C>(
        filter: Option<&OrganismFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("organisms")
            .select()
            .filter(filter.map_or_else(|| gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into(), |filter| filter.as_filter_expression()))
           .project("id, host_organism_id, sample_id, notes, nameplate_id, project_id, created_by, created_at, updated_by, updated_at")
            .order_by("updated_at desc")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => uuid::Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            host_organism_id: match row.get("host_organism_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(host_organism_id) => Some(uuid::Uuid::from_u128(*host_organism_id)),
                _ => unreachable!("Expected Uuid"),
            },
            sample_id: match row.get("sample_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(sample_id) => Some(uuid::Uuid::from_u128(*sample_id)),
                _ => unreachable!("Expected Uuid"),
            },
            notes: match row.get("notes").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Str(notes) => Some(notes.clone()),
                _ => unreachable!("Expected Str")
            },
            nameplate_id: match row.get("nameplate_id").unwrap() {
                gluesql::prelude::Value::I32(nameplate_id) => nameplate_id.clone(),
                _ => unreachable!("Expected I32")
            },
            project_id: match row.get("project_id").unwrap() {
                gluesql::prelude::Value::I32(project_id) => project_id.clone(),
                _ => unreachable!("Expected I32")
            },
            created_by: match row.get("created_by").unwrap() {
                gluesql::prelude::Value::I32(created_by) => created_by.clone(),
                _ => unreachable!("Expected I32")
            },
            created_at: match row.get("created_at").unwrap() {
                gluesql::prelude::Value::Timestamp(created_at) => created_at.clone(),
                _ => unreachable!("Expected Timestamp")
            },
            updated_by: match row.get("updated_by").unwrap() {
                gluesql::prelude::Value::I32(updated_by) => updated_by.clone(),
                _ => unreachable!("Expected I32")
            },
            updated_at: match row.get("updated_at").unwrap() {
                gluesql::prelude::Value::Timestamp(updated_at) => updated_at.clone(),
                _ => unreachable!("Expected Timestamp")
            },
        }
    }
}
#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct Organization {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub country_id: i32,
    pub state_province: Option<String>,
    pub domain: String,
}

unsafe impl Send for Organization {}
unsafe impl Sync for Organization {}
impl Tabular for Organization {
    const TABLE: Table = Table::Organizations;
}
impl Describable for Organization {
    fn description(&self) -> Option<&str> {
        None
    }
}
impl Colorable for Organization {
    fn color(&self) -> Option<&str> {
        None
    }
}

impl Filtrable for Organization {
    type Filter = OrganizationFilter;
}
#[cfg(feature = "frontend")]
impl Organization {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::text(self.name),
            gluesql::core::ast_builder::text(self.url),
            gluesql::core::ast_builder::num(self.country_id),
            match self.state_province {
                Some(state_province) => gluesql::core::ast_builder::text(state_province),
                None => gluesql::core::ast_builder::null(),
            },
            gluesql::core::ast_builder::text(self.domain),
        ]
    }

    /// Insert the Organization into the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn insert<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut> (
self: crate::database::flat_variants::Organization,
connection: &mut gluesql::prelude::Glue<C>,
) -> Result<usize, gluesql::prelude::Error>    {
        use gluesql::core::ast_builder::*;
        table("organizations")
            .insert()
            .columns("id, name, url, country_id, state_province, domain")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get Organization from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("organizations")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, name, url, country_id, state_province, domain")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete Organization from the database.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("organizations")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of Organization from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let mut update_row = table("organizations")
            .update()        
.set("id", gluesql::core::ast_builder::num(self.id))        
.set("name", gluesql::core::ast_builder::text(self.name))        
.set("url", gluesql::core::ast_builder::text(self.url))        
.set("country_id", gluesql::core::ast_builder::num(self.country_id))        
.set("domain", gluesql::core::ast_builder::text(self.domain));
        if let Some(state_province) = self.state_province {
            update_row = update_row.set("state_province", gluesql::core::ast_builder::text(state_province));
        }
            update_row.execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all Organization from the database.
    ///
    /// # Arguments
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        filter: Option<&OrganizationFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("organizations")
            .select()
            .filter(filter.map_or_else(|| gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into(), |filter| filter.as_filter_expression()))
           .project("id, name, url, country_id, state_province, domain")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::I32(id) => id.clone(),
                _ => unreachable!("Expected I32")
            },
            name: match row.get("name").unwrap() {
                gluesql::prelude::Value::Str(name) => name.clone(),
                _ => unreachable!("Expected Str")
            },
            url: match row.get("url").unwrap() {
                gluesql::prelude::Value::Str(url) => url.clone(),
                _ => unreachable!("Expected Str")
            },
            country_id: match row.get("country_id").unwrap() {
                gluesql::prelude::Value::I32(country_id) => country_id.clone(),
                _ => unreachable!("Expected I32")
            },
            state_province: match row.get("state_province").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Str(state_province) => Some(state_province.clone()),
                _ => unreachable!("Expected Str")
            },
            domain: match row.get("domain").unwrap() {
                gluesql::prelude::Value::Str(domain) => domain.clone(),
                _ => unreachable!("Expected Str")
            },
        }
    }
}
#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct PermanenceCategory {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub icon_id: i32,
    pub color_id: i32,
}

unsafe impl Send for PermanenceCategory {}
unsafe impl Sync for PermanenceCategory {}
impl Tabular for PermanenceCategory {
    const TABLE: Table = Table::PermanenceCategories;
}
impl Describable for PermanenceCategory {
    fn description(&self) -> Option<&str> {
        Some(self.description.as_str())
    }
}
impl Colorable for PermanenceCategory {
    fn color(&self) -> Option<&str> {
        None
    }
}

impl Filtrable for PermanenceCategory {
    type Filter = PermanenceCategoryFilter;
}
#[cfg(feature = "frontend")]
impl PermanenceCategory {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::text(self.name),
            gluesql::core::ast_builder::text(self.description),
            gluesql::core::ast_builder::num(self.icon_id),
            gluesql::core::ast_builder::num(self.color_id),
        ]
    }

    /// Insert the PermanenceCategory into the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn insert<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut> (
self: crate::database::flat_variants::PermanenceCategory,
connection: &mut gluesql::prelude::Glue<C>,
) -> Result<usize, gluesql::prelude::Error>    {
        use gluesql::core::ast_builder::*;
        table("permanence_categories")
            .insert()
            .columns("id, name, description, icon_id, color_id")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get PermanenceCategory from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("permanence_categories")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, name, description, icon_id, color_id")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete PermanenceCategory from the database.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("permanence_categories")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of PermanenceCategory from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("permanence_categories")
            .update()        
.set("id", gluesql::core::ast_builder::num(self.id))        
.set("name", gluesql::core::ast_builder::text(self.name))        
.set("description", gluesql::core::ast_builder::text(self.description))        
.set("icon_id", gluesql::core::ast_builder::num(self.icon_id))        
.set("color_id", gluesql::core::ast_builder::num(self.color_id))            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all PermanenceCategory from the database.
    ///
    /// # Arguments
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        filter: Option<&PermanenceCategoryFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("permanence_categories")
            .select()
            .filter(filter.map_or_else(|| gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into(), |filter| filter.as_filter_expression()))
           .project("id, name, description, icon_id, color_id")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::I32(id) => id.clone(),
                _ => unreachable!("Expected I32")
            },
            name: match row.get("name").unwrap() {
                gluesql::prelude::Value::Str(name) => name.clone(),
                _ => unreachable!("Expected Str")
            },
            description: match row.get("description").unwrap() {
                gluesql::prelude::Value::Str(description) => description.clone(),
                _ => unreachable!("Expected Str")
            },
            icon_id: match row.get("icon_id").unwrap() {
                gluesql::prelude::Value::I32(icon_id) => icon_id.clone(),
                _ => unreachable!("Expected I32")
            },
            color_id: match row.get("color_id").unwrap() {
                gluesql::prelude::Value::I32(color_id) => color_id.clone(),
                _ => unreachable!("Expected I32")
            },
        }
    }
}
#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct ProjectState {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub icon_id: i32,
    pub color_id: i32,
}

unsafe impl Send for ProjectState {}
unsafe impl Sync for ProjectState {}
impl Tabular for ProjectState {
    const TABLE: Table = Table::ProjectStates;
}
impl Describable for ProjectState {
    fn description(&self) -> Option<&str> {
        Some(self.description.as_str())
    }
}
impl Colorable for ProjectState {
    fn color(&self) -> Option<&str> {
        None
    }
}

impl Filtrable for ProjectState {
    type Filter = ProjectStateFilter;
}
#[cfg(feature = "frontend")]
impl ProjectState {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::text(self.name),
            gluesql::core::ast_builder::text(self.description),
            gluesql::core::ast_builder::num(self.icon_id),
            gluesql::core::ast_builder::num(self.color_id),
        ]
    }

    /// Insert the ProjectState into the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn insert<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut> (
self: crate::database::flat_variants::ProjectState,
connection: &mut gluesql::prelude::Glue<C>,
) -> Result<usize, gluesql::prelude::Error>    {
        use gluesql::core::ast_builder::*;
        table("project_states")
            .insert()
            .columns("id, name, description, icon_id, color_id")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get ProjectState from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("project_states")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, name, description, icon_id, color_id")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete ProjectState from the database.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("project_states")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of ProjectState from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("project_states")
            .update()        
.set("id", gluesql::core::ast_builder::num(self.id))        
.set("name", gluesql::core::ast_builder::text(self.name))        
.set("description", gluesql::core::ast_builder::text(self.description))        
.set("icon_id", gluesql::core::ast_builder::num(self.icon_id))        
.set("color_id", gluesql::core::ast_builder::num(self.color_id))            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all ProjectState from the database.
    ///
    /// # Arguments
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        filter: Option<&ProjectStateFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("project_states")
            .select()
            .filter(filter.map_or_else(|| gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into(), |filter| filter.as_filter_expression()))
           .project("id, name, description, icon_id, color_id")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::I32(id) => id.clone(),
                _ => unreachable!("Expected I32")
            },
            name: match row.get("name").unwrap() {
                gluesql::prelude::Value::Str(name) => name.clone(),
                _ => unreachable!("Expected Str")
            },
            description: match row.get("description").unwrap() {
                gluesql::prelude::Value::Str(description) => description.clone(),
                _ => unreachable!("Expected Str")
            },
            icon_id: match row.get("icon_id").unwrap() {
                gluesql::prelude::Value::I32(icon_id) => icon_id.clone(),
                _ => unreachable!("Expected I32")
            },
            color_id: match row.get("color_id").unwrap() {
                gluesql::prelude::Value::I32(color_id) => color_id.clone(),
                _ => unreachable!("Expected I32")
            },
        }
    }
}
#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub public: bool,
    pub state_id: i32,
    pub icon_id: i32,
    pub color_id: i32,
    pub parent_project_id: Option<i32>,
    pub budget: Option<f64>,
    pub expenses: Option<f64>,
    pub created_by: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_by: i32,
    pub updated_at: chrono::NaiveDateTime,
    pub expected_end_date: Option<chrono::NaiveDateTime>,
    pub end_date: Option<chrono::NaiveDateTime>,
}

unsafe impl Send for Project {}
unsafe impl Sync for Project {}
impl Tabular for Project {
    const TABLE: Table = Table::Projects;
}
impl Describable for Project {
    fn description(&self) -> Option<&str> {
        Some(self.description.as_str())
    }
}
impl Colorable for Project {
    fn color(&self) -> Option<&str> {
        None
    }
}

impl Filtrable for Project {
    type Filter = ProjectFilter;
}
#[cfg(feature = "frontend")]
impl Project {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::text(self.name),
            gluesql::core::ast_builder::text(self.description),
            (self.public.into()),
            gluesql::core::ast_builder::num(self.state_id),
            gluesql::core::ast_builder::num(self.icon_id),
            gluesql::core::ast_builder::num(self.color_id),
            match self.parent_project_id {
                Some(parent_project_id) => gluesql::core::ast_builder::num(parent_project_id),
                None => gluesql::core::ast_builder::null(),
            },
            match self.budget {
                Some(budget) => gluesql::core::ast_builder::num(budget),
                None => gluesql::core::ast_builder::null(),
            },
            match self.expenses {
                Some(expenses) => gluesql::core::ast_builder::num(expenses),
                None => gluesql::core::ast_builder::null(),
            },
            gluesql::core::ast_builder::num(self.created_by),
            gluesql::core::ast_builder::timestamp(self.created_at.to_string()),
            gluesql::core::ast_builder::num(self.updated_by),
            gluesql::core::ast_builder::timestamp(self.updated_at.to_string()),
            match self.expected_end_date {
                Some(expected_end_date) => gluesql::core::ast_builder::timestamp(expected_end_date.to_string()),
                None => gluesql::core::ast_builder::null(),
            },
            match self.end_date {
                Some(end_date) => gluesql::core::ast_builder::timestamp(end_date.to_string()),
                None => gluesql::core::ast_builder::null(),
            },
        ]
    }

    /// Insert the Project into the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn insert<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut> (
self: crate::database::flat_variants::Project,
connection: &mut gluesql::prelude::Glue<C>,
) -> Result<usize, gluesql::prelude::Error>    {
        use gluesql::core::ast_builder::*;
        table("projects")
            .insert()
            .columns("id, name, description, public, state_id, icon_id, color_id, parent_project_id, budget, expenses, created_by, created_at, updated_by, updated_at, expected_end_date, end_date")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get Project from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("projects")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, name, description, public, state_id, icon_id, color_id, parent_project_id, budget, expenses, created_by, created_at, updated_by, updated_at, expected_end_date, end_date")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete Project from the database.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("projects")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of Project from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let mut update_row = table("projects")
            .update()        
.set("id", gluesql::core::ast_builder::num(self.id))        
.set("name", gluesql::core::ast_builder::text(self.name))        
.set("description", gluesql::core::ast_builder::text(self.description))        
.set("public", self.public)        
.set("state_id", gluesql::core::ast_builder::num(self.state_id))        
.set("icon_id", gluesql::core::ast_builder::num(self.icon_id))        
.set("color_id", gluesql::core::ast_builder::num(self.color_id))        
.set("created_by", gluesql::core::ast_builder::num(self.created_by))        
.set("created_at", gluesql::core::ast_builder::timestamp(self.created_at.to_string()))        
.set("updated_by", gluesql::core::ast_builder::num(self.updated_by))        
.set("updated_at", gluesql::core::ast_builder::timestamp(self.updated_at.to_string()));
        if let Some(parent_project_id) = self.parent_project_id {
            update_row = update_row.set("parent_project_id", gluesql::core::ast_builder::num(parent_project_id));
        }
        if let Some(budget) = self.budget {
            update_row = update_row.set("budget", gluesql::core::ast_builder::num(budget));
        }
        if let Some(expenses) = self.expenses {
            update_row = update_row.set("expenses", gluesql::core::ast_builder::num(expenses));
        }
        if let Some(expected_end_date) = self.expected_end_date {
            update_row = update_row.set("expected_end_date", gluesql::core::ast_builder::timestamp(expected_end_date.to_string()));
        }
        if let Some(end_date) = self.end_date {
            update_row = update_row.set("end_date", gluesql::core::ast_builder::timestamp(end_date.to_string()));
        }
            update_row.execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all Project from the database.
    ///
    /// # Arguments
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        filter: Option<&ProjectFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("projects")
            .select()
            .filter(filter.map_or_else(|| gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into(), |filter| filter.as_filter_expression()))
           .project("id, name, description, public, state_id, icon_id, color_id, parent_project_id, budget, expenses, created_by, created_at, updated_by, updated_at, expected_end_date, end_date")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    /// Get all Project from the database ordered by the `updated_at` column.
    ///
    /// # Arguments
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all_by_updated_at<C>(
        filter: Option<&ProjectFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("projects")
            .select()
            .filter(filter.map_or_else(|| gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into(), |filter| filter.as_filter_expression()))
           .project("id, name, description, public, state_id, icon_id, color_id, parent_project_id, budget, expenses, created_by, created_at, updated_by, updated_at, expected_end_date, end_date")
            .order_by("updated_at desc")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::I32(id) => id.clone(),
                _ => unreachable!("Expected I32")
            },
            name: match row.get("name").unwrap() {
                gluesql::prelude::Value::Str(name) => name.clone(),
                _ => unreachable!("Expected Str")
            },
            description: match row.get("description").unwrap() {
                gluesql::prelude::Value::Str(description) => description.clone(),
                _ => unreachable!("Expected Str")
            },
            public: match row.get("public").unwrap() {
                gluesql::prelude::Value::Bool(public) => public.clone(),
                _ => unreachable!("Expected Bool")
            },
            state_id: match row.get("state_id").unwrap() {
                gluesql::prelude::Value::I32(state_id) => state_id.clone(),
                _ => unreachable!("Expected I32")
            },
            icon_id: match row.get("icon_id").unwrap() {
                gluesql::prelude::Value::I32(icon_id) => icon_id.clone(),
                _ => unreachable!("Expected I32")
            },
            color_id: match row.get("color_id").unwrap() {
                gluesql::prelude::Value::I32(color_id) => color_id.clone(),
                _ => unreachable!("Expected I32")
            },
            parent_project_id: match row.get("parent_project_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::I32(parent_project_id) => Some(parent_project_id.clone()),
                _ => unreachable!("Expected I32")
            },
            budget: match row.get("budget").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::F64(budget) => Some(budget.clone()),
                _ => unreachable!("Expected F64")
            },
            expenses: match row.get("expenses").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::F64(expenses) => Some(expenses.clone()),
                _ => unreachable!("Expected F64")
            },
            created_by: match row.get("created_by").unwrap() {
                gluesql::prelude::Value::I32(created_by) => created_by.clone(),
                _ => unreachable!("Expected I32")
            },
            created_at: match row.get("created_at").unwrap() {
                gluesql::prelude::Value::Timestamp(created_at) => created_at.clone(),
                _ => unreachable!("Expected Timestamp")
            },
            updated_by: match row.get("updated_by").unwrap() {
                gluesql::prelude::Value::I32(updated_by) => updated_by.clone(),
                _ => unreachable!("Expected I32")
            },
            updated_at: match row.get("updated_at").unwrap() {
                gluesql::prelude::Value::Timestamp(updated_at) => updated_at.clone(),
                _ => unreachable!("Expected Timestamp")
            },
            expected_end_date: match row.get("expected_end_date").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Timestamp(expected_end_date) => Some(expected_end_date.clone()),
                _ => unreachable!("Expected Timestamp")
            },
            end_date: match row.get("end_date").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Timestamp(end_date) => Some(end_date.clone()),
                _ => unreachable!("Expected Timestamp")
            },
        }
    }
}
#[derive(Eq, PartialEq, PartialOrd, Debug, Clone, Copy, Ord, serde::Serialize, serde::Deserialize, Default)]
pub struct ProjectsTeamsRoleInvitation {
    pub table_id: i32,
    pub team_id: i32,
    pub role_id: i32,
    pub created_by: i32,
    pub created_at: chrono::NaiveDateTime,
}

unsafe impl Send for ProjectsTeamsRoleInvitation {}
unsafe impl Sync for ProjectsTeamsRoleInvitation {}
impl Tabular for ProjectsTeamsRoleInvitation {
    const TABLE: Table = Table::ProjectsTeamsRoleInvitations;
}
impl Describable for ProjectsTeamsRoleInvitation {
    fn description(&self) -> Option<&str> {
        None
    }
}
impl Colorable for ProjectsTeamsRoleInvitation {
    fn color(&self) -> Option<&str> {
        None
    }
}

impl Filtrable for ProjectsTeamsRoleInvitation {
    type Filter = ProjectsTeamsRoleInvitationFilter;
}
#[cfg(feature = "frontend")]
impl ProjectsTeamsRoleInvitation {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.table_id),
            gluesql::core::ast_builder::num(self.team_id),
            gluesql::core::ast_builder::num(self.role_id),
            gluesql::core::ast_builder::num(self.created_by),
            gluesql::core::ast_builder::timestamp(self.created_at.to_string()),
        ]
    }

    /// Insert the ProjectsTeamsRoleInvitation into the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn insert<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut> (
self: crate::database::flat_variants::ProjectsTeamsRoleInvitation,
connection: &mut gluesql::prelude::Glue<C>,
) -> Result<usize, gluesql::prelude::Error>    {
        use gluesql::core::ast_builder::*;
        table("projects_teams_role_invitations")
            .insert()
            .columns("table_id, team_id, role_id, created_by, created_at")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get ProjectsTeamsRoleInvitation from the database by its ID.
    ///
    /// # Arguments
    /// * `( table_id, team_id )` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        ( table_id, team_id ): ( i32, i32 ),
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("projects_teams_role_invitations")
            .select()
            .filter(col("table_id").eq(table_id.to_string()))
            .filter(col("team_id").eq(team_id.to_string()))
            .project("table_id, team_id, role_id, created_by, created_at")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete ProjectsTeamsRoleInvitation from the database.
    ///
    /// # Arguments
    /// * `( table_id, team_id )` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        ( table_id, team_id ): ( i32, i32 ),
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("projects_teams_role_invitations")
            .delete()
            .filter(col("table_id").eq(table_id.to_string()))
            .filter(col("team_id").eq(team_id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of ProjectsTeamsRoleInvitation from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(( self.table_id, self.team_id ), connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("projects_teams_role_invitations")
            .update()        
.set("table_id", gluesql::core::ast_builder::num(self.table_id))        
.set("team_id", gluesql::core::ast_builder::num(self.team_id))        
.set("role_id", gluesql::core::ast_builder::num(self.role_id))        
.set("created_by", gluesql::core::ast_builder::num(self.created_by))        
.set("created_at", gluesql::core::ast_builder::timestamp(self.created_at.to_string()))            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all ProjectsTeamsRoleInvitation from the database.
    ///
    /// # Arguments
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        filter: Option<&ProjectsTeamsRoleInvitationFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("projects_teams_role_invitations")
            .select()
            .filter(filter.map_or_else(|| gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into(), |filter| filter.as_filter_expression()))
           .project("table_id, team_id, role_id, created_by, created_at")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            table_id: match row.get("table_id").unwrap() {
                gluesql::prelude::Value::I32(table_id) => table_id.clone(),
                _ => unreachable!("Expected I32")
            },
            team_id: match row.get("team_id").unwrap() {
                gluesql::prelude::Value::I32(team_id) => team_id.clone(),
                _ => unreachable!("Expected I32")
            },
            role_id: match row.get("role_id").unwrap() {
                gluesql::prelude::Value::I32(role_id) => role_id.clone(),
                _ => unreachable!("Expected I32")
            },
            created_by: match row.get("created_by").unwrap() {
                gluesql::prelude::Value::I32(created_by) => created_by.clone(),
                _ => unreachable!("Expected I32")
            },
            created_at: match row.get("created_at").unwrap() {
                gluesql::prelude::Value::Timestamp(created_at) => created_at.clone(),
                _ => unreachable!("Expected Timestamp")
            },
        }
    }
}
#[derive(Eq, PartialEq, PartialOrd, Debug, Clone, Copy, Ord, serde::Serialize, serde::Deserialize, Default)]
pub struct ProjectsTeamsRoleRequest {
    pub table_id: i32,
    pub team_id: i32,
    pub role_id: i32,
    pub created_by: i32,
    pub created_at: chrono::NaiveDateTime,
}

unsafe impl Send for ProjectsTeamsRoleRequest {}
unsafe impl Sync for ProjectsTeamsRoleRequest {}
impl Tabular for ProjectsTeamsRoleRequest {
    const TABLE: Table = Table::ProjectsTeamsRoleRequests;
}
impl Describable for ProjectsTeamsRoleRequest {
    fn description(&self) -> Option<&str> {
        None
    }
}
impl Colorable for ProjectsTeamsRoleRequest {
    fn color(&self) -> Option<&str> {
        None
    }
}

impl Filtrable for ProjectsTeamsRoleRequest {
    type Filter = ProjectsTeamsRoleRequestFilter;
}
#[cfg(feature = "frontend")]
impl ProjectsTeamsRoleRequest {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.table_id),
            gluesql::core::ast_builder::num(self.team_id),
            gluesql::core::ast_builder::num(self.role_id),
            gluesql::core::ast_builder::num(self.created_by),
            gluesql::core::ast_builder::timestamp(self.created_at.to_string()),
        ]
    }

    /// Insert the ProjectsTeamsRoleRequest into the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn insert<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut> (
self: crate::database::flat_variants::ProjectsTeamsRoleRequest,
connection: &mut gluesql::prelude::Glue<C>,
) -> Result<usize, gluesql::prelude::Error>    {
        use gluesql::core::ast_builder::*;
        table("projects_teams_role_requests")
            .insert()
            .columns("table_id, team_id, role_id, created_by, created_at")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get ProjectsTeamsRoleRequest from the database by its ID.
    ///
    /// # Arguments
    /// * `( table_id, team_id )` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        ( table_id, team_id ): ( i32, i32 ),
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("projects_teams_role_requests")
            .select()
            .filter(col("table_id").eq(table_id.to_string()))
            .filter(col("team_id").eq(team_id.to_string()))
            .project("table_id, team_id, role_id, created_by, created_at")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete ProjectsTeamsRoleRequest from the database.
    ///
    /// # Arguments
    /// * `( table_id, team_id )` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        ( table_id, team_id ): ( i32, i32 ),
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("projects_teams_role_requests")
            .delete()
            .filter(col("table_id").eq(table_id.to_string()))
            .filter(col("team_id").eq(team_id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of ProjectsTeamsRoleRequest from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(( self.table_id, self.team_id ), connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("projects_teams_role_requests")
            .update()        
.set("table_id", gluesql::core::ast_builder::num(self.table_id))        
.set("team_id", gluesql::core::ast_builder::num(self.team_id))        
.set("role_id", gluesql::core::ast_builder::num(self.role_id))        
.set("created_by", gluesql::core::ast_builder::num(self.created_by))        
.set("created_at", gluesql::core::ast_builder::timestamp(self.created_at.to_string()))            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all ProjectsTeamsRoleRequest from the database.
    ///
    /// # Arguments
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        filter: Option<&ProjectsTeamsRoleRequestFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("projects_teams_role_requests")
            .select()
            .filter(filter.map_or_else(|| gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into(), |filter| filter.as_filter_expression()))
           .project("table_id, team_id, role_id, created_by, created_at")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            table_id: match row.get("table_id").unwrap() {
                gluesql::prelude::Value::I32(table_id) => table_id.clone(),
                _ => unreachable!("Expected I32")
            },
            team_id: match row.get("team_id").unwrap() {
                gluesql::prelude::Value::I32(team_id) => team_id.clone(),
                _ => unreachable!("Expected I32")
            },
            role_id: match row.get("role_id").unwrap() {
                gluesql::prelude::Value::I32(role_id) => role_id.clone(),
                _ => unreachable!("Expected I32")
            },
            created_by: match row.get("created_by").unwrap() {
                gluesql::prelude::Value::I32(created_by) => created_by.clone(),
                _ => unreachable!("Expected I32")
            },
            created_at: match row.get("created_at").unwrap() {
                gluesql::prelude::Value::Timestamp(created_at) => created_at.clone(),
                _ => unreachable!("Expected Timestamp")
            },
        }
    }
}
#[derive(Eq, PartialEq, PartialOrd, Debug, Clone, Copy, Ord, serde::Serialize, serde::Deserialize, Default)]
pub struct ProjectsTeamsRole {
    pub table_id: i32,
    pub team_id: i32,
    pub role_id: i32,
    pub created_by: i32,
    pub created_at: chrono::NaiveDateTime,
}

unsafe impl Send for ProjectsTeamsRole {}
unsafe impl Sync for ProjectsTeamsRole {}
impl Tabular for ProjectsTeamsRole {
    const TABLE: Table = Table::ProjectsTeamsRoles;
}
impl Describable for ProjectsTeamsRole {
    fn description(&self) -> Option<&str> {
        None
    }
}
impl Colorable for ProjectsTeamsRole {
    fn color(&self) -> Option<&str> {
        None
    }
}

impl Filtrable for ProjectsTeamsRole {
    type Filter = ProjectsTeamsRoleFilter;
}
#[cfg(feature = "frontend")]
impl ProjectsTeamsRole {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.table_id),
            gluesql::core::ast_builder::num(self.team_id),
            gluesql::core::ast_builder::num(self.role_id),
            gluesql::core::ast_builder::num(self.created_by),
            gluesql::core::ast_builder::timestamp(self.created_at.to_string()),
        ]
    }

    /// Insert the ProjectsTeamsRole into the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn insert<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut> (
self: crate::database::flat_variants::ProjectsTeamsRole,
connection: &mut gluesql::prelude::Glue<C>,
) -> Result<usize, gluesql::prelude::Error>    {
        use gluesql::core::ast_builder::*;
        table("projects_teams_roles")
            .insert()
            .columns("table_id, team_id, role_id, created_by, created_at")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get ProjectsTeamsRole from the database by its ID.
    ///
    /// # Arguments
    /// * `( table_id, team_id )` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        ( table_id, team_id ): ( i32, i32 ),
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("projects_teams_roles")
            .select()
            .filter(col("table_id").eq(table_id.to_string()))
            .filter(col("team_id").eq(team_id.to_string()))
            .project("table_id, team_id, role_id, created_by, created_at")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete ProjectsTeamsRole from the database.
    ///
    /// # Arguments
    /// * `( table_id, team_id )` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        ( table_id, team_id ): ( i32, i32 ),
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("projects_teams_roles")
            .delete()
            .filter(col("table_id").eq(table_id.to_string()))
            .filter(col("team_id").eq(team_id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of ProjectsTeamsRole from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(( self.table_id, self.team_id ), connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("projects_teams_roles")
            .update()        
.set("table_id", gluesql::core::ast_builder::num(self.table_id))        
.set("team_id", gluesql::core::ast_builder::num(self.team_id))        
.set("role_id", gluesql::core::ast_builder::num(self.role_id))        
.set("created_by", gluesql::core::ast_builder::num(self.created_by))        
.set("created_at", gluesql::core::ast_builder::timestamp(self.created_at.to_string()))            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all ProjectsTeamsRole from the database.
    ///
    /// # Arguments
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        filter: Option<&ProjectsTeamsRoleFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("projects_teams_roles")
            .select()
            .filter(filter.map_or_else(|| gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into(), |filter| filter.as_filter_expression()))
           .project("table_id, team_id, role_id, created_by, created_at")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            table_id: match row.get("table_id").unwrap() {
                gluesql::prelude::Value::I32(table_id) => table_id.clone(),
                _ => unreachable!("Expected I32")
            },
            team_id: match row.get("team_id").unwrap() {
                gluesql::prelude::Value::I32(team_id) => team_id.clone(),
                _ => unreachable!("Expected I32")
            },
            role_id: match row.get("role_id").unwrap() {
                gluesql::prelude::Value::I32(role_id) => role_id.clone(),
                _ => unreachable!("Expected I32")
            },
            created_by: match row.get("created_by").unwrap() {
                gluesql::prelude::Value::I32(created_by) => created_by.clone(),
                _ => unreachable!("Expected I32")
            },
            created_at: match row.get("created_at").unwrap() {
                gluesql::prelude::Value::Timestamp(created_at) => created_at.clone(),
                _ => unreachable!("Expected Timestamp")
            },
        }
    }
}
#[derive(Eq, PartialEq, PartialOrd, Debug, Clone, Copy, Ord, serde::Serialize, serde::Deserialize, Default)]
pub struct ProjectsUsersRoleInvitation {
    pub table_id: i32,
    pub user_id: i32,
    pub role_id: i32,
    pub created_by: i32,
    pub created_at: chrono::NaiveDateTime,
}

unsafe impl Send for ProjectsUsersRoleInvitation {}
unsafe impl Sync for ProjectsUsersRoleInvitation {}
impl Tabular for ProjectsUsersRoleInvitation {
    const TABLE: Table = Table::ProjectsUsersRoleInvitations;
}
impl Describable for ProjectsUsersRoleInvitation {
    fn description(&self) -> Option<&str> {
        None
    }
}
impl Colorable for ProjectsUsersRoleInvitation {
    fn color(&self) -> Option<&str> {
        None
    }
}

impl Filtrable for ProjectsUsersRoleInvitation {
    type Filter = ProjectsUsersRoleInvitationFilter;
}
#[cfg(feature = "frontend")]
impl ProjectsUsersRoleInvitation {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.table_id),
            gluesql::core::ast_builder::num(self.user_id),
            gluesql::core::ast_builder::num(self.role_id),
            gluesql::core::ast_builder::num(self.created_by),
            gluesql::core::ast_builder::timestamp(self.created_at.to_string()),
        ]
    }

    /// Insert the ProjectsUsersRoleInvitation into the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn insert<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut> (
self: crate::database::flat_variants::ProjectsUsersRoleInvitation,
connection: &mut gluesql::prelude::Glue<C>,
) -> Result<usize, gluesql::prelude::Error>    {
        use gluesql::core::ast_builder::*;
        table("projects_users_role_invitations")
            .insert()
            .columns("table_id, user_id, role_id, created_by, created_at")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get ProjectsUsersRoleInvitation from the database by its ID.
    ///
    /// # Arguments
    /// * `( table_id, user_id )` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        ( table_id, user_id ): ( i32, i32 ),
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("projects_users_role_invitations")
            .select()
            .filter(col("table_id").eq(table_id.to_string()))
            .filter(col("user_id").eq(user_id.to_string()))
            .project("table_id, user_id, role_id, created_by, created_at")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete ProjectsUsersRoleInvitation from the database.
    ///
    /// # Arguments
    /// * `( table_id, user_id )` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        ( table_id, user_id ): ( i32, i32 ),
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("projects_users_role_invitations")
            .delete()
            .filter(col("table_id").eq(table_id.to_string()))
            .filter(col("user_id").eq(user_id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of ProjectsUsersRoleInvitation from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(( self.table_id, self.user_id ), connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("projects_users_role_invitations")
            .update()        
.set("table_id", gluesql::core::ast_builder::num(self.table_id))        
.set("user_id", gluesql::core::ast_builder::num(self.user_id))        
.set("role_id", gluesql::core::ast_builder::num(self.role_id))        
.set("created_by", gluesql::core::ast_builder::num(self.created_by))        
.set("created_at", gluesql::core::ast_builder::timestamp(self.created_at.to_string()))            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all ProjectsUsersRoleInvitation from the database.
    ///
    /// # Arguments
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        filter: Option<&ProjectsUsersRoleInvitationFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("projects_users_role_invitations")
            .select()
            .filter(filter.map_or_else(|| gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into(), |filter| filter.as_filter_expression()))
           .project("table_id, user_id, role_id, created_by, created_at")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            table_id: match row.get("table_id").unwrap() {
                gluesql::prelude::Value::I32(table_id) => table_id.clone(),
                _ => unreachable!("Expected I32")
            },
            user_id: match row.get("user_id").unwrap() {
                gluesql::prelude::Value::I32(user_id) => user_id.clone(),
                _ => unreachable!("Expected I32")
            },
            role_id: match row.get("role_id").unwrap() {
                gluesql::prelude::Value::I32(role_id) => role_id.clone(),
                _ => unreachable!("Expected I32")
            },
            created_by: match row.get("created_by").unwrap() {
                gluesql::prelude::Value::I32(created_by) => created_by.clone(),
                _ => unreachable!("Expected I32")
            },
            created_at: match row.get("created_at").unwrap() {
                gluesql::prelude::Value::Timestamp(created_at) => created_at.clone(),
                _ => unreachable!("Expected Timestamp")
            },
        }
    }
}
#[derive(Eq, PartialEq, PartialOrd, Debug, Clone, Copy, Ord, serde::Serialize, serde::Deserialize, Default)]
pub struct ProjectsUsersRoleRequest {
    pub table_id: i32,
    pub user_id: i32,
    pub role_id: i32,
    pub created_by: i32,
    pub created_at: chrono::NaiveDateTime,
}

unsafe impl Send for ProjectsUsersRoleRequest {}
unsafe impl Sync for ProjectsUsersRoleRequest {}
impl Tabular for ProjectsUsersRoleRequest {
    const TABLE: Table = Table::ProjectsUsersRoleRequests;
}
impl Describable for ProjectsUsersRoleRequest {
    fn description(&self) -> Option<&str> {
        None
    }
}
impl Colorable for ProjectsUsersRoleRequest {
    fn color(&self) -> Option<&str> {
        None
    }
}

impl Filtrable for ProjectsUsersRoleRequest {
    type Filter = ProjectsUsersRoleRequestFilter;
}
#[cfg(feature = "frontend")]
impl ProjectsUsersRoleRequest {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.table_id),
            gluesql::core::ast_builder::num(self.user_id),
            gluesql::core::ast_builder::num(self.role_id),
            gluesql::core::ast_builder::num(self.created_by),
            gluesql::core::ast_builder::timestamp(self.created_at.to_string()),
        ]
    }

    /// Insert the ProjectsUsersRoleRequest into the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn insert<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut> (
self: crate::database::flat_variants::ProjectsUsersRoleRequest,
connection: &mut gluesql::prelude::Glue<C>,
) -> Result<usize, gluesql::prelude::Error>    {
        use gluesql::core::ast_builder::*;
        table("projects_users_role_requests")
            .insert()
            .columns("table_id, user_id, role_id, created_by, created_at")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get ProjectsUsersRoleRequest from the database by its ID.
    ///
    /// # Arguments
    /// * `( table_id, user_id )` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        ( table_id, user_id ): ( i32, i32 ),
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("projects_users_role_requests")
            .select()
            .filter(col("table_id").eq(table_id.to_string()))
            .filter(col("user_id").eq(user_id.to_string()))
            .project("table_id, user_id, role_id, created_by, created_at")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete ProjectsUsersRoleRequest from the database.
    ///
    /// # Arguments
    /// * `( table_id, user_id )` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        ( table_id, user_id ): ( i32, i32 ),
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("projects_users_role_requests")
            .delete()
            .filter(col("table_id").eq(table_id.to_string()))
            .filter(col("user_id").eq(user_id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of ProjectsUsersRoleRequest from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(( self.table_id, self.user_id ), connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("projects_users_role_requests")
            .update()        
.set("table_id", gluesql::core::ast_builder::num(self.table_id))        
.set("user_id", gluesql::core::ast_builder::num(self.user_id))        
.set("role_id", gluesql::core::ast_builder::num(self.role_id))        
.set("created_by", gluesql::core::ast_builder::num(self.created_by))        
.set("created_at", gluesql::core::ast_builder::timestamp(self.created_at.to_string()))            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all ProjectsUsersRoleRequest from the database.
    ///
    /// # Arguments
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        filter: Option<&ProjectsUsersRoleRequestFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("projects_users_role_requests")
            .select()
            .filter(filter.map_or_else(|| gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into(), |filter| filter.as_filter_expression()))
           .project("table_id, user_id, role_id, created_by, created_at")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            table_id: match row.get("table_id").unwrap() {
                gluesql::prelude::Value::I32(table_id) => table_id.clone(),
                _ => unreachable!("Expected I32")
            },
            user_id: match row.get("user_id").unwrap() {
                gluesql::prelude::Value::I32(user_id) => user_id.clone(),
                _ => unreachable!("Expected I32")
            },
            role_id: match row.get("role_id").unwrap() {
                gluesql::prelude::Value::I32(role_id) => role_id.clone(),
                _ => unreachable!("Expected I32")
            },
            created_by: match row.get("created_by").unwrap() {
                gluesql::prelude::Value::I32(created_by) => created_by.clone(),
                _ => unreachable!("Expected I32")
            },
            created_at: match row.get("created_at").unwrap() {
                gluesql::prelude::Value::Timestamp(created_at) => created_at.clone(),
                _ => unreachable!("Expected Timestamp")
            },
        }
    }
}
#[derive(Eq, PartialEq, PartialOrd, Debug, Clone, Copy, Ord, serde::Serialize, serde::Deserialize, Default)]
pub struct ProjectsUsersRole {
    pub table_id: i32,
    pub user_id: i32,
    pub role_id: i32,
    pub created_by: i32,
    pub created_at: chrono::NaiveDateTime,
}

unsafe impl Send for ProjectsUsersRole {}
unsafe impl Sync for ProjectsUsersRole {}
impl Tabular for ProjectsUsersRole {
    const TABLE: Table = Table::ProjectsUsersRoles;
}
impl Describable for ProjectsUsersRole {
    fn description(&self) -> Option<&str> {
        None
    }
}
impl Colorable for ProjectsUsersRole {
    fn color(&self) -> Option<&str> {
        None
    }
}

impl Filtrable for ProjectsUsersRole {
    type Filter = ProjectsUsersRoleFilter;
}
#[cfg(feature = "frontend")]
impl ProjectsUsersRole {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.table_id),
            gluesql::core::ast_builder::num(self.user_id),
            gluesql::core::ast_builder::num(self.role_id),
            gluesql::core::ast_builder::num(self.created_by),
            gluesql::core::ast_builder::timestamp(self.created_at.to_string()),
        ]
    }

    /// Insert the ProjectsUsersRole into the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn insert<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut> (
self: crate::database::flat_variants::ProjectsUsersRole,
connection: &mut gluesql::prelude::Glue<C>,
) -> Result<usize, gluesql::prelude::Error>    {
        use gluesql::core::ast_builder::*;
        table("projects_users_roles")
            .insert()
            .columns("table_id, user_id, role_id, created_by, created_at")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get ProjectsUsersRole from the database by its ID.
    ///
    /// # Arguments
    /// * `( table_id, user_id )` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        ( table_id, user_id ): ( i32, i32 ),
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("projects_users_roles")
            .select()
            .filter(col("table_id").eq(table_id.to_string()))
            .filter(col("user_id").eq(user_id.to_string()))
            .project("table_id, user_id, role_id, created_by, created_at")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete ProjectsUsersRole from the database.
    ///
    /// # Arguments
    /// * `( table_id, user_id )` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        ( table_id, user_id ): ( i32, i32 ),
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("projects_users_roles")
            .delete()
            .filter(col("table_id").eq(table_id.to_string()))
            .filter(col("user_id").eq(user_id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of ProjectsUsersRole from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(( self.table_id, self.user_id ), connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("projects_users_roles")
            .update()        
.set("table_id", gluesql::core::ast_builder::num(self.table_id))        
.set("user_id", gluesql::core::ast_builder::num(self.user_id))        
.set("role_id", gluesql::core::ast_builder::num(self.role_id))        
.set("created_by", gluesql::core::ast_builder::num(self.created_by))        
.set("created_at", gluesql::core::ast_builder::timestamp(self.created_at.to_string()))            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all ProjectsUsersRole from the database.
    ///
    /// # Arguments
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        filter: Option<&ProjectsUsersRoleFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("projects_users_roles")
            .select()
            .filter(filter.map_or_else(|| gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into(), |filter| filter.as_filter_expression()))
           .project("table_id, user_id, role_id, created_by, created_at")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            table_id: match row.get("table_id").unwrap() {
                gluesql::prelude::Value::I32(table_id) => table_id.clone(),
                _ => unreachable!("Expected I32")
            },
            user_id: match row.get("user_id").unwrap() {
                gluesql::prelude::Value::I32(user_id) => user_id.clone(),
                _ => unreachable!("Expected I32")
            },
            role_id: match row.get("role_id").unwrap() {
                gluesql::prelude::Value::I32(role_id) => role_id.clone(),
                _ => unreachable!("Expected I32")
            },
            created_by: match row.get("created_by").unwrap() {
                gluesql::prelude::Value::I32(created_by) => created_by.clone(),
                _ => unreachable!("Expected I32")
            },
            created_at: match row.get("created_at").unwrap() {
                gluesql::prelude::Value::Timestamp(created_at) => created_at.clone(),
                _ => unreachable!("Expected Timestamp")
            },
        }
    }
}
#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct Role {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub icon_id: i32,
    pub color_id: i32,
}

unsafe impl Send for Role {}
unsafe impl Sync for Role {}
impl Tabular for Role {
    const TABLE: Table = Table::Roles;
}
impl Describable for Role {
    fn description(&self) -> Option<&str> {
        Some(self.description.as_str())
    }
}
impl Colorable for Role {
    fn color(&self) -> Option<&str> {
        None
    }
}

impl Filtrable for Role {
    type Filter = RoleFilter;
}
#[cfg(feature = "frontend")]
impl Role {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::text(self.name),
            gluesql::core::ast_builder::text(self.description),
            gluesql::core::ast_builder::num(self.icon_id),
            gluesql::core::ast_builder::num(self.color_id),
        ]
    }

    /// Insert the Role into the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn insert<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut> (
self: crate::database::flat_variants::Role,
connection: &mut gluesql::prelude::Glue<C>,
) -> Result<usize, gluesql::prelude::Error>    {
        use gluesql::core::ast_builder::*;
        table("roles")
            .insert()
            .columns("id, name, description, icon_id, color_id")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get Role from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("roles")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, name, description, icon_id, color_id")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete Role from the database.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("roles")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of Role from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("roles")
            .update()        
.set("id", gluesql::core::ast_builder::num(self.id))        
.set("name", gluesql::core::ast_builder::text(self.name))        
.set("description", gluesql::core::ast_builder::text(self.description))        
.set("icon_id", gluesql::core::ast_builder::num(self.icon_id))        
.set("color_id", gluesql::core::ast_builder::num(self.color_id))            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all Role from the database.
    ///
    /// # Arguments
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        filter: Option<&RoleFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("roles")
            .select()
            .filter(filter.map_or_else(|| gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into(), |filter| filter.as_filter_expression()))
           .project("id, name, description, icon_id, color_id")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::I32(id) => id.clone(),
                _ => unreachable!("Expected I32")
            },
            name: match row.get("name").unwrap() {
                gluesql::prelude::Value::Str(name) => name.clone(),
                _ => unreachable!("Expected Str")
            },
            description: match row.get("description").unwrap() {
                gluesql::prelude::Value::Str(description) => description.clone(),
                _ => unreachable!("Expected Str")
            },
            icon_id: match row.get("icon_id").unwrap() {
                gluesql::prelude::Value::I32(icon_id) => icon_id.clone(),
                _ => unreachable!("Expected I32")
            },
            color_id: match row.get("color_id").unwrap() {
                gluesql::prelude::Value::I32(color_id) => color_id.clone(),
                _ => unreachable!("Expected I32")
            },
        }
    }
}
#[derive(Eq, PartialEq, Debug, Clone, Copy, serde::Serialize, serde::Deserialize, Default)]
pub struct SampleBioOttTaxonItem {
    pub created_by: i32,
    pub created_at: chrono::NaiveDateTime,
    pub sample_id: uuid::Uuid,
    pub taxon_id: i32,
}

unsafe impl Send for SampleBioOttTaxonItem {}
unsafe impl Sync for SampleBioOttTaxonItem {}
impl Tabular for SampleBioOttTaxonItem {
    const TABLE: Table = Table::SampleBioOttTaxonItems;
}
impl Describable for SampleBioOttTaxonItem {
    fn description(&self) -> Option<&str> {
        None
    }
}
impl Colorable for SampleBioOttTaxonItem {
    fn color(&self) -> Option<&str> {
        None
    }
}

impl Filtrable for SampleBioOttTaxonItem {
    type Filter = SampleBioOttTaxonItemFilter;
}
#[cfg(feature = "frontend")]
impl SampleBioOttTaxonItem {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.created_by),
            gluesql::core::ast_builder::timestamp(self.created_at.to_string()),
            gluesql::core::ast_builder::uuid(self.sample_id.to_string()),
            gluesql::core::ast_builder::num(self.taxon_id),
        ]
    }

    /// Insert the SampleBioOttTaxonItem into the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn insert<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut> (
self: crate::database::flat_variants::SampleBioOttTaxonItem,
connection: &mut gluesql::prelude::Glue<C>,
) -> Result<usize, gluesql::prelude::Error>    {
        use gluesql::core::ast_builder::*;
        table("sample_bio_ott_taxon_items")
            .insert()
            .columns("created_by, created_at, sample_id, taxon_id")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get SampleBioOttTaxonItem from the database by its ID.
    ///
    /// # Arguments
    /// * `( sample_id, taxon_id )` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        ( sample_id, taxon_id ): ( uuid::Uuid, i32 ),
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("sample_bio_ott_taxon_items")
            .select()
            .filter(col("sample_id").eq(sample_id.to_string()))
            .filter(col("taxon_id").eq(taxon_id.to_string()))
            .project("created_by, created_at, sample_id, taxon_id")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete SampleBioOttTaxonItem from the database.
    ///
    /// # Arguments
    /// * `( sample_id, taxon_id )` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        ( sample_id, taxon_id ): ( uuid::Uuid, i32 ),
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("sample_bio_ott_taxon_items")
            .delete()
            .filter(col("sample_id").eq(sample_id.to_string()))
            .filter(col("taxon_id").eq(taxon_id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of SampleBioOttTaxonItem from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(( self.sample_id, self.taxon_id ), connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("sample_bio_ott_taxon_items")
            .update()        
.set("created_by", gluesql::core::ast_builder::num(self.created_by))        
.set("created_at", gluesql::core::ast_builder::timestamp(self.created_at.to_string()))        
.set("sample_id", gluesql::core::ast_builder::uuid(self.sample_id.to_string()))        
.set("taxon_id", gluesql::core::ast_builder::num(self.taxon_id))            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all SampleBioOttTaxonItem from the database.
    ///
    /// # Arguments
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        filter: Option<&SampleBioOttTaxonItemFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("sample_bio_ott_taxon_items")
            .select()
            .filter(filter.map_or_else(|| gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into(), |filter| filter.as_filter_expression()))
           .project("created_by, created_at, sample_id, taxon_id")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            created_by: match row.get("created_by").unwrap() {
                gluesql::prelude::Value::I32(created_by) => created_by.clone(),
                _ => unreachable!("Expected I32")
            },
            created_at: match row.get("created_at").unwrap() {
                gluesql::prelude::Value::Timestamp(created_at) => created_at.clone(),
                _ => unreachable!("Expected Timestamp")
            },
            sample_id: match row.get("sample_id").unwrap() {
                gluesql::prelude::Value::Uuid(sample_id) => uuid::Uuid::from_u128(*sample_id),
                _ => unreachable!("Expected Uuid"),
            },
            taxon_id: match row.get("taxon_id").unwrap() {
                gluesql::prelude::Value::I32(taxon_id) => taxon_id.clone(),
                _ => unreachable!("Expected I32")
            },
        }
    }
}
#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct SampleContainerCategory {
    pub id: i32,
    pub name: String,
    pub volume: f64,
    pub unit: String,
    pub material_id: i32,
    pub description: String,
    pub icon_id: i32,
    pub color_id: i32,
}

unsafe impl Send for SampleContainerCategory {}
unsafe impl Sync for SampleContainerCategory {}
impl Tabular for SampleContainerCategory {
    const TABLE: Table = Table::SampleContainerCategories;
}
impl Describable for SampleContainerCategory {
    fn description(&self) -> Option<&str> {
        Some(self.description.as_str())
    }
}
impl Colorable for SampleContainerCategory {
    fn color(&self) -> Option<&str> {
        None
    }
}

impl Filtrable for SampleContainerCategory {
    type Filter = SampleContainerCategoryFilter;
}
#[cfg(feature = "frontend")]
impl SampleContainerCategory {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::text(self.name),
            gluesql::core::ast_builder::num(self.volume),
            gluesql::core::ast_builder::text(self.unit),
            gluesql::core::ast_builder::num(self.material_id),
            gluesql::core::ast_builder::text(self.description),
            gluesql::core::ast_builder::num(self.icon_id),
            gluesql::core::ast_builder::num(self.color_id),
        ]
    }

    /// Insert the SampleContainerCategory into the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn insert<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut> (
self: crate::database::flat_variants::SampleContainerCategory,
connection: &mut gluesql::prelude::Glue<C>,
) -> Result<usize, gluesql::prelude::Error>    {
        use gluesql::core::ast_builder::*;
        table("sample_container_categories")
            .insert()
            .columns("id, name, volume, unit, material_id, description, icon_id, color_id")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get SampleContainerCategory from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("sample_container_categories")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, name, volume, unit, material_id, description, icon_id, color_id")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete SampleContainerCategory from the database.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("sample_container_categories")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of SampleContainerCategory from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("sample_container_categories")
            .update()        
.set("id", gluesql::core::ast_builder::num(self.id))        
.set("name", gluesql::core::ast_builder::text(self.name))        
.set("volume", gluesql::core::ast_builder::num(self.volume))        
.set("unit", gluesql::core::ast_builder::text(self.unit))        
.set("material_id", gluesql::core::ast_builder::num(self.material_id))        
.set("description", gluesql::core::ast_builder::text(self.description))        
.set("icon_id", gluesql::core::ast_builder::num(self.icon_id))        
.set("color_id", gluesql::core::ast_builder::num(self.color_id))            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all SampleContainerCategory from the database.
    ///
    /// # Arguments
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        filter: Option<&SampleContainerCategoryFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("sample_container_categories")
            .select()
            .filter(filter.map_or_else(|| gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into(), |filter| filter.as_filter_expression()))
           .project("id, name, volume, unit, material_id, description, icon_id, color_id")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::I32(id) => id.clone(),
                _ => unreachable!("Expected I32")
            },
            name: match row.get("name").unwrap() {
                gluesql::prelude::Value::Str(name) => name.clone(),
                _ => unreachable!("Expected Str")
            },
            volume: match row.get("volume").unwrap() {
                gluesql::prelude::Value::F64(volume) => volume.clone(),
                _ => unreachable!("Expected F64")
            },
            unit: match row.get("unit").unwrap() {
                gluesql::prelude::Value::Str(unit) => unit.clone(),
                _ => unreachable!("Expected Str")
            },
            material_id: match row.get("material_id").unwrap() {
                gluesql::prelude::Value::I32(material_id) => material_id.clone(),
                _ => unreachable!("Expected I32")
            },
            description: match row.get("description").unwrap() {
                gluesql::prelude::Value::Str(description) => description.clone(),
                _ => unreachable!("Expected Str")
            },
            icon_id: match row.get("icon_id").unwrap() {
                gluesql::prelude::Value::I32(icon_id) => icon_id.clone(),
                _ => unreachable!("Expected I32")
            },
            color_id: match row.get("color_id").unwrap() {
                gluesql::prelude::Value::I32(color_id) => color_id.clone(),
                _ => unreachable!("Expected I32")
            },
        }
    }
}
#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct SampleContainer {
    pub id: i32,
    pub barcode: String,
    pub project_id: i32,
    pub category_id: i32,
    pub created_by: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_by: i32,
    pub updated_at: chrono::NaiveDateTime,
}

unsafe impl Send for SampleContainer {}
unsafe impl Sync for SampleContainer {}
impl Tabular for SampleContainer {
    const TABLE: Table = Table::SampleContainers;
}
impl Describable for SampleContainer {
    fn description(&self) -> Option<&str> {
        None
    }
}
impl Colorable for SampleContainer {
    fn color(&self) -> Option<&str> {
        None
    }
}

impl Filtrable for SampleContainer {
    type Filter = SampleContainerFilter;
}
#[cfg(feature = "frontend")]
impl SampleContainer {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::text(self.barcode),
            gluesql::core::ast_builder::num(self.project_id),
            gluesql::core::ast_builder::num(self.category_id),
            gluesql::core::ast_builder::num(self.created_by),
            gluesql::core::ast_builder::timestamp(self.created_at.to_string()),
            gluesql::core::ast_builder::num(self.updated_by),
            gluesql::core::ast_builder::timestamp(self.updated_at.to_string()),
        ]
    }

    /// Insert the SampleContainer into the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn insert<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut> (
self: crate::database::flat_variants::SampleContainer,
connection: &mut gluesql::prelude::Glue<C>,
) -> Result<usize, gluesql::prelude::Error>    {
        use gluesql::core::ast_builder::*;
        table("sample_containers")
            .insert()
            .columns("id, barcode, project_id, category_id, created_by, created_at, updated_by, updated_at")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get SampleContainer from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("sample_containers")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, barcode, project_id, category_id, created_by, created_at, updated_by, updated_at")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete SampleContainer from the database.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("sample_containers")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of SampleContainer from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("sample_containers")
            .update()        
.set("id", gluesql::core::ast_builder::num(self.id))        
.set("barcode", gluesql::core::ast_builder::text(self.barcode))        
.set("project_id", gluesql::core::ast_builder::num(self.project_id))        
.set("category_id", gluesql::core::ast_builder::num(self.category_id))        
.set("created_by", gluesql::core::ast_builder::num(self.created_by))        
.set("created_at", gluesql::core::ast_builder::timestamp(self.created_at.to_string()))        
.set("updated_by", gluesql::core::ast_builder::num(self.updated_by))        
.set("updated_at", gluesql::core::ast_builder::timestamp(self.updated_at.to_string()))            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all SampleContainer from the database.
    ///
    /// # Arguments
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        filter: Option<&SampleContainerFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("sample_containers")
            .select()
            .filter(filter.map_or_else(|| gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into(), |filter| filter.as_filter_expression()))
           .project("id, barcode, project_id, category_id, created_by, created_at, updated_by, updated_at")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    /// Get all SampleContainer from the database ordered by the `updated_at` column.
    ///
    /// # Arguments
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all_by_updated_at<C>(
        filter: Option<&SampleContainerFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("sample_containers")
            .select()
            .filter(filter.map_or_else(|| gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into(), |filter| filter.as_filter_expression()))
           .project("id, barcode, project_id, category_id, created_by, created_at, updated_by, updated_at")
            .order_by("updated_at desc")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::I32(id) => id.clone(),
                _ => unreachable!("Expected I32")
            },
            barcode: match row.get("barcode").unwrap() {
                gluesql::prelude::Value::Str(barcode) => barcode.clone(),
                _ => unreachable!("Expected Str")
            },
            project_id: match row.get("project_id").unwrap() {
                gluesql::prelude::Value::I32(project_id) => project_id.clone(),
                _ => unreachable!("Expected I32")
            },
            category_id: match row.get("category_id").unwrap() {
                gluesql::prelude::Value::I32(category_id) => category_id.clone(),
                _ => unreachable!("Expected I32")
            },
            created_by: match row.get("created_by").unwrap() {
                gluesql::prelude::Value::I32(created_by) => created_by.clone(),
                _ => unreachable!("Expected I32")
            },
            created_at: match row.get("created_at").unwrap() {
                gluesql::prelude::Value::Timestamp(created_at) => created_at.clone(),
                _ => unreachable!("Expected Timestamp")
            },
            updated_by: match row.get("updated_by").unwrap() {
                gluesql::prelude::Value::I32(updated_by) => updated_by.clone(),
                _ => unreachable!("Expected I32")
            },
            updated_at: match row.get("updated_at").unwrap() {
                gluesql::prelude::Value::Timestamp(updated_at) => updated_at.clone(),
                _ => unreachable!("Expected Timestamp")
            },
        }
    }
}
#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct SampleState {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub icon_id: i32,
    pub color_id: i32,
}

unsafe impl Send for SampleState {}
unsafe impl Sync for SampleState {}
impl Tabular for SampleState {
    const TABLE: Table = Table::SampleStates;
}
impl Describable for SampleState {
    fn description(&self) -> Option<&str> {
        Some(self.description.as_str())
    }
}
impl Colorable for SampleState {
    fn color(&self) -> Option<&str> {
        None
    }
}

impl Filtrable for SampleState {
    type Filter = SampleStateFilter;
}
#[cfg(feature = "frontend")]
impl SampleState {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::text(self.name),
            gluesql::core::ast_builder::text(self.description),
            gluesql::core::ast_builder::num(self.icon_id),
            gluesql::core::ast_builder::num(self.color_id),
        ]
    }

    /// Insert the SampleState into the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn insert<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut> (
self: crate::database::flat_variants::SampleState,
connection: &mut gluesql::prelude::Glue<C>,
) -> Result<usize, gluesql::prelude::Error>    {
        use gluesql::core::ast_builder::*;
        table("sample_states")
            .insert()
            .columns("id, name, description, icon_id, color_id")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get SampleState from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("sample_states")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, name, description, icon_id, color_id")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete SampleState from the database.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("sample_states")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of SampleState from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("sample_states")
            .update()        
.set("id", gluesql::core::ast_builder::num(self.id))        
.set("name", gluesql::core::ast_builder::text(self.name))        
.set("description", gluesql::core::ast_builder::text(self.description))        
.set("icon_id", gluesql::core::ast_builder::num(self.icon_id))        
.set("color_id", gluesql::core::ast_builder::num(self.color_id))            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all SampleState from the database.
    ///
    /// # Arguments
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        filter: Option<&SampleStateFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("sample_states")
            .select()
            .filter(filter.map_or_else(|| gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into(), |filter| filter.as_filter_expression()))
           .project("id, name, description, icon_id, color_id")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::I32(id) => id.clone(),
                _ => unreachable!("Expected I32")
            },
            name: match row.get("name").unwrap() {
                gluesql::prelude::Value::Str(name) => name.clone(),
                _ => unreachable!("Expected Str")
            },
            description: match row.get("description").unwrap() {
                gluesql::prelude::Value::Str(description) => description.clone(),
                _ => unreachable!("Expected Str")
            },
            icon_id: match row.get("icon_id").unwrap() {
                gluesql::prelude::Value::I32(icon_id) => icon_id.clone(),
                _ => unreachable!("Expected I32")
            },
            color_id: match row.get("color_id").unwrap() {
                gluesql::prelude::Value::I32(color_id) => color_id.clone(),
                _ => unreachable!("Expected I32")
            },
        }
    }
}
#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct Sample {
    pub id: uuid::Uuid,
    pub container_id: i32,
    pub notes: Option<String>,
    pub project_id: i32,
    pub created_by: i32,
    pub sampled_by: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_by: i32,
    pub updated_at: chrono::NaiveDateTime,
    pub state_id: i32,
}

unsafe impl Send for Sample {}
unsafe impl Sync for Sample {}
impl Tabular for Sample {
    const TABLE: Table = Table::Samples;
}
impl Describable for Sample {
    fn description(&self) -> Option<&str> {
        self.notes.as_deref()
    }
}
impl Colorable for Sample {
    fn color(&self) -> Option<&str> {
        None
    }
}

impl Filtrable for Sample {
    type Filter = SampleFilter;
}
#[cfg(feature = "frontend")]
impl Sample {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::uuid(self.id.to_string()),
            gluesql::core::ast_builder::num(self.container_id),
            match self.notes {
                Some(notes) => gluesql::core::ast_builder::text(notes),
                None => gluesql::core::ast_builder::null(),
            },
            gluesql::core::ast_builder::num(self.project_id),
            gluesql::core::ast_builder::num(self.created_by),
            gluesql::core::ast_builder::num(self.sampled_by),
            gluesql::core::ast_builder::timestamp(self.created_at.to_string()),
            gluesql::core::ast_builder::num(self.updated_by),
            gluesql::core::ast_builder::timestamp(self.updated_at.to_string()),
            gluesql::core::ast_builder::num(self.state_id),
        ]
    }

    /// Insert the Sample into the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn insert<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut> (
self: crate::database::flat_variants::Sample,
connection: &mut gluesql::prelude::Glue<C>,
) -> Result<usize, gluesql::prelude::Error>    {
        use gluesql::core::ast_builder::*;
        table("samples")
            .insert()
            .columns("id, container_id, notes, project_id, created_by, sampled_by, created_at, updated_by, updated_at, state_id")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get Sample from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: uuid::Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("samples")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, container_id, notes, project_id, created_by, sampled_by, created_at, updated_by, updated_at, state_id")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete Sample from the database.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: uuid::Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("samples")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of Sample from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let mut update_row = table("samples")
            .update()        
.set("id", gluesql::core::ast_builder::uuid(self.id.to_string()))        
.set("container_id", gluesql::core::ast_builder::num(self.container_id))        
.set("project_id", gluesql::core::ast_builder::num(self.project_id))        
.set("created_by", gluesql::core::ast_builder::num(self.created_by))        
.set("sampled_by", gluesql::core::ast_builder::num(self.sampled_by))        
.set("created_at", gluesql::core::ast_builder::timestamp(self.created_at.to_string()))        
.set("updated_by", gluesql::core::ast_builder::num(self.updated_by))        
.set("updated_at", gluesql::core::ast_builder::timestamp(self.updated_at.to_string()))        
.set("state_id", gluesql::core::ast_builder::num(self.state_id));
        if let Some(notes) = self.notes {
            update_row = update_row.set("notes", gluesql::core::ast_builder::text(notes));
        }
            update_row.execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all Sample from the database.
    ///
    /// # Arguments
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        filter: Option<&SampleFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("samples")
            .select()
            .filter(filter.map_or_else(|| gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into(), |filter| filter.as_filter_expression()))
           .project("id, container_id, notes, project_id, created_by, sampled_by, created_at, updated_by, updated_at, state_id")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    /// Get all Sample from the database ordered by the `updated_at` column.
    ///
    /// # Arguments
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all_by_updated_at<C>(
        filter: Option<&SampleFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("samples")
            .select()
            .filter(filter.map_or_else(|| gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into(), |filter| filter.as_filter_expression()))
           .project("id, container_id, notes, project_id, created_by, sampled_by, created_at, updated_by, updated_at, state_id")
            .order_by("updated_at desc")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => uuid::Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            container_id: match row.get("container_id").unwrap() {
                gluesql::prelude::Value::I32(container_id) => container_id.clone(),
                _ => unreachable!("Expected I32")
            },
            notes: match row.get("notes").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Str(notes) => Some(notes.clone()),
                _ => unreachable!("Expected Str")
            },
            project_id: match row.get("project_id").unwrap() {
                gluesql::prelude::Value::I32(project_id) => project_id.clone(),
                _ => unreachable!("Expected I32")
            },
            created_by: match row.get("created_by").unwrap() {
                gluesql::prelude::Value::I32(created_by) => created_by.clone(),
                _ => unreachable!("Expected I32")
            },
            sampled_by: match row.get("sampled_by").unwrap() {
                gluesql::prelude::Value::I32(sampled_by) => sampled_by.clone(),
                _ => unreachable!("Expected I32")
            },
            created_at: match row.get("created_at").unwrap() {
                gluesql::prelude::Value::Timestamp(created_at) => created_at.clone(),
                _ => unreachable!("Expected Timestamp")
            },
            updated_by: match row.get("updated_by").unwrap() {
                gluesql::prelude::Value::I32(updated_by) => updated_by.clone(),
                _ => unreachable!("Expected I32")
            },
            updated_at: match row.get("updated_at").unwrap() {
                gluesql::prelude::Value::Timestamp(updated_at) => updated_at.clone(),
                _ => unreachable!("Expected Timestamp")
            },
            state_id: match row.get("state_id").unwrap() {
                gluesql::prelude::Value::I32(state_id) => state_id.clone(),
                _ => unreachable!("Expected I32")
            },
        }
    }
}
#[derive(Eq, PartialEq, PartialOrd, Debug, Clone, Copy, Ord, serde::Serialize, serde::Deserialize, Default)]
pub struct Spectra {
    pub id: i32,
    pub spectra_collection_id: i32,
}

unsafe impl Send for Spectra {}
unsafe impl Sync for Spectra {}
impl Tabular for Spectra {
    const TABLE: Table = Table::Spectra;
}
impl Describable for Spectra {
    fn description(&self) -> Option<&str> {
        None
    }
}
impl Colorable for Spectra {
    fn color(&self) -> Option<&str> {
        None
    }
}

impl Filtrable for Spectra {
    type Filter = SpectraFilter;
}
#[cfg(feature = "frontend")]
impl Spectra {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::num(self.spectra_collection_id),
        ]
    }

    /// Insert the Spectra into the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn insert<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut> (
self: crate::database::flat_variants::Spectra,
connection: &mut gluesql::prelude::Glue<C>,
) -> Result<usize, gluesql::prelude::Error>    {
        use gluesql::core::ast_builder::*;
        table("spectra")
            .insert()
            .columns("id, spectra_collection_id")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get Spectra from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("spectra")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, spectra_collection_id")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete Spectra from the database.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("spectra")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of Spectra from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("spectra")
            .update()        
.set("id", gluesql::core::ast_builder::num(self.id))        
.set("spectra_collection_id", gluesql::core::ast_builder::num(self.spectra_collection_id))            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all Spectra from the database.
    ///
    /// # Arguments
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        filter: Option<&SpectraFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("spectra")
            .select()
            .filter(filter.map_or_else(|| gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into(), |filter| filter.as_filter_expression()))
           .project("id, spectra_collection_id")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::I32(id) => id.clone(),
                _ => unreachable!("Expected I32")
            },
            spectra_collection_id: match row.get("spectra_collection_id").unwrap() {
                gluesql::prelude::Value::I32(spectra_collection_id) => spectra_collection_id.clone(),
                _ => unreachable!("Expected I32")
            },
        }
    }
}
#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct SpectraCollection {
    pub id: i32,
    pub notes: Option<String>,
    pub sample_id: uuid::Uuid,
    pub created_by: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_by: i32,
    pub updated_at: chrono::NaiveDateTime,
}

unsafe impl Send for SpectraCollection {}
unsafe impl Sync for SpectraCollection {}
impl Tabular for SpectraCollection {
    const TABLE: Table = Table::SpectraCollections;
}
impl Describable for SpectraCollection {
    fn description(&self) -> Option<&str> {
        self.notes.as_deref()
    }
}
impl Colorable for SpectraCollection {
    fn color(&self) -> Option<&str> {
        None
    }
}

impl Filtrable for SpectraCollection {
    type Filter = SpectraCollectionFilter;
}
#[cfg(feature = "frontend")]
impl SpectraCollection {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
            match self.notes {
                Some(notes) => gluesql::core::ast_builder::text(notes),
                None => gluesql::core::ast_builder::null(),
            },
            gluesql::core::ast_builder::uuid(self.sample_id.to_string()),
            gluesql::core::ast_builder::num(self.created_by),
            gluesql::core::ast_builder::timestamp(self.created_at.to_string()),
            gluesql::core::ast_builder::num(self.updated_by),
            gluesql::core::ast_builder::timestamp(self.updated_at.to_string()),
        ]
    }

    /// Insert the SpectraCollection into the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn insert<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut> (
self: crate::database::flat_variants::SpectraCollection,
connection: &mut gluesql::prelude::Glue<C>,
) -> Result<usize, gluesql::prelude::Error>    {
        use gluesql::core::ast_builder::*;
        table("spectra_collections")
            .insert()
            .columns("id, notes, sample_id, created_by, created_at, updated_by, updated_at")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get SpectraCollection from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("spectra_collections")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, notes, sample_id, created_by, created_at, updated_by, updated_at")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete SpectraCollection from the database.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("spectra_collections")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of SpectraCollection from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let mut update_row = table("spectra_collections")
            .update()        
.set("id", gluesql::core::ast_builder::num(self.id))        
.set("sample_id", gluesql::core::ast_builder::uuid(self.sample_id.to_string()))        
.set("created_by", gluesql::core::ast_builder::num(self.created_by))        
.set("created_at", gluesql::core::ast_builder::timestamp(self.created_at.to_string()))        
.set("updated_by", gluesql::core::ast_builder::num(self.updated_by))        
.set("updated_at", gluesql::core::ast_builder::timestamp(self.updated_at.to_string()));
        if let Some(notes) = self.notes {
            update_row = update_row.set("notes", gluesql::core::ast_builder::text(notes));
        }
            update_row.execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all SpectraCollection from the database.
    ///
    /// # Arguments
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        filter: Option<&SpectraCollectionFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("spectra_collections")
            .select()
            .filter(filter.map_or_else(|| gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into(), |filter| filter.as_filter_expression()))
           .project("id, notes, sample_id, created_by, created_at, updated_by, updated_at")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    /// Get all SpectraCollection from the database ordered by the `updated_at` column.
    ///
    /// # Arguments
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all_by_updated_at<C>(
        filter: Option<&SpectraCollectionFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("spectra_collections")
            .select()
            .filter(filter.map_or_else(|| gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into(), |filter| filter.as_filter_expression()))
           .project("id, notes, sample_id, created_by, created_at, updated_by, updated_at")
            .order_by("updated_at desc")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::I32(id) => id.clone(),
                _ => unreachable!("Expected I32")
            },
            notes: match row.get("notes").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Str(notes) => Some(notes.clone()),
                _ => unreachable!("Expected Str")
            },
            sample_id: match row.get("sample_id").unwrap() {
                gluesql::prelude::Value::Uuid(sample_id) => uuid::Uuid::from_u128(*sample_id),
                _ => unreachable!("Expected Uuid"),
            },
            created_by: match row.get("created_by").unwrap() {
                gluesql::prelude::Value::I32(created_by) => created_by.clone(),
                _ => unreachable!("Expected I32")
            },
            created_at: match row.get("created_at").unwrap() {
                gluesql::prelude::Value::Timestamp(created_at) => created_at.clone(),
                _ => unreachable!("Expected Timestamp")
            },
            updated_by: match row.get("updated_by").unwrap() {
                gluesql::prelude::Value::I32(updated_by) => updated_by.clone(),
                _ => unreachable!("Expected I32")
            },
            updated_at: match row.get("updated_at").unwrap() {
                gluesql::prelude::Value::Timestamp(updated_at) => updated_at.clone(),
                _ => unreachable!("Expected Timestamp")
            },
        }
    }
}
#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct TeamState {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub icon_id: i32,
    pub color_id: i32,
}

unsafe impl Send for TeamState {}
unsafe impl Sync for TeamState {}
impl Tabular for TeamState {
    const TABLE: Table = Table::TeamStates;
}
impl Describable for TeamState {
    fn description(&self) -> Option<&str> {
        Some(self.description.as_str())
    }
}
impl Colorable for TeamState {
    fn color(&self) -> Option<&str> {
        None
    }
}

impl Filtrable for TeamState {
    type Filter = TeamStateFilter;
}
#[cfg(feature = "frontend")]
impl TeamState {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::text(self.name),
            gluesql::core::ast_builder::text(self.description),
            gluesql::core::ast_builder::num(self.icon_id),
            gluesql::core::ast_builder::num(self.color_id),
        ]
    }

    /// Insert the TeamState into the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn insert<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut> (
self: crate::database::flat_variants::TeamState,
connection: &mut gluesql::prelude::Glue<C>,
) -> Result<usize, gluesql::prelude::Error>    {
        use gluesql::core::ast_builder::*;
        table("team_states")
            .insert()
            .columns("id, name, description, icon_id, color_id")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get TeamState from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("team_states")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, name, description, icon_id, color_id")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete TeamState from the database.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("team_states")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of TeamState from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("team_states")
            .update()        
.set("id", gluesql::core::ast_builder::num(self.id))        
.set("name", gluesql::core::ast_builder::text(self.name))        
.set("description", gluesql::core::ast_builder::text(self.description))        
.set("icon_id", gluesql::core::ast_builder::num(self.icon_id))        
.set("color_id", gluesql::core::ast_builder::num(self.color_id))            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all TeamState from the database.
    ///
    /// # Arguments
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        filter: Option<&TeamStateFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("team_states")
            .select()
            .filter(filter.map_or_else(|| gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into(), |filter| filter.as_filter_expression()))
           .project("id, name, description, icon_id, color_id")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::I32(id) => id.clone(),
                _ => unreachable!("Expected I32")
            },
            name: match row.get("name").unwrap() {
                gluesql::prelude::Value::Str(name) => name.clone(),
                _ => unreachable!("Expected Str")
            },
            description: match row.get("description").unwrap() {
                gluesql::prelude::Value::Str(description) => description.clone(),
                _ => unreachable!("Expected Str")
            },
            icon_id: match row.get("icon_id").unwrap() {
                gluesql::prelude::Value::I32(icon_id) => icon_id.clone(),
                _ => unreachable!("Expected I32")
            },
            color_id: match row.get("color_id").unwrap() {
                gluesql::prelude::Value::I32(color_id) => color_id.clone(),
                _ => unreachable!("Expected I32")
            },
        }
    }
}
#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct Team {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub icon_id: i32,
    pub color_id: i32,
    pub state_id: i32,
    pub parent_team_id: Option<i32>,
    pub created_by: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_by: i32,
    pub updated_at: chrono::NaiveDateTime,
}

unsafe impl Send for Team {}
unsafe impl Sync for Team {}
impl Tabular for Team {
    const TABLE: Table = Table::Teams;
}
impl Describable for Team {
    fn description(&self) -> Option<&str> {
        Some(self.description.as_str())
    }
}
impl Colorable for Team {
    fn color(&self) -> Option<&str> {
        None
    }
}

impl Filtrable for Team {
    type Filter = TeamFilter;
}
#[cfg(feature = "frontend")]
impl Team {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::text(self.name),
            gluesql::core::ast_builder::text(self.description),
            gluesql::core::ast_builder::num(self.icon_id),
            gluesql::core::ast_builder::num(self.color_id),
            gluesql::core::ast_builder::num(self.state_id),
            match self.parent_team_id {
                Some(parent_team_id) => gluesql::core::ast_builder::num(parent_team_id),
                None => gluesql::core::ast_builder::null(),
            },
            gluesql::core::ast_builder::num(self.created_by),
            gluesql::core::ast_builder::timestamp(self.created_at.to_string()),
            gluesql::core::ast_builder::num(self.updated_by),
            gluesql::core::ast_builder::timestamp(self.updated_at.to_string()),
        ]
    }

    /// Insert the Team into the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn insert<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut> (
self: crate::database::flat_variants::Team,
connection: &mut gluesql::prelude::Glue<C>,
) -> Result<usize, gluesql::prelude::Error>    {
        use gluesql::core::ast_builder::*;
        table("teams")
            .insert()
            .columns("id, name, description, icon_id, color_id, state_id, parent_team_id, created_by, created_at, updated_by, updated_at")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get Team from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("teams")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, name, description, icon_id, color_id, state_id, parent_team_id, created_by, created_at, updated_by, updated_at")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete Team from the database.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("teams")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of Team from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let mut update_row = table("teams")
            .update()        
.set("id", gluesql::core::ast_builder::num(self.id))        
.set("name", gluesql::core::ast_builder::text(self.name))        
.set("description", gluesql::core::ast_builder::text(self.description))        
.set("icon_id", gluesql::core::ast_builder::num(self.icon_id))        
.set("color_id", gluesql::core::ast_builder::num(self.color_id))        
.set("state_id", gluesql::core::ast_builder::num(self.state_id))        
.set("created_by", gluesql::core::ast_builder::num(self.created_by))        
.set("created_at", gluesql::core::ast_builder::timestamp(self.created_at.to_string()))        
.set("updated_by", gluesql::core::ast_builder::num(self.updated_by))        
.set("updated_at", gluesql::core::ast_builder::timestamp(self.updated_at.to_string()));
        if let Some(parent_team_id) = self.parent_team_id {
            update_row = update_row.set("parent_team_id", gluesql::core::ast_builder::num(parent_team_id));
        }
            update_row.execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all Team from the database.
    ///
    /// # Arguments
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        filter: Option<&TeamFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("teams")
            .select()
            .filter(filter.map_or_else(|| gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into(), |filter| filter.as_filter_expression()))
           .project("id, name, description, icon_id, color_id, state_id, parent_team_id, created_by, created_at, updated_by, updated_at")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    /// Get all Team from the database ordered by the `updated_at` column.
    ///
    /// # Arguments
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all_by_updated_at<C>(
        filter: Option<&TeamFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("teams")
            .select()
            .filter(filter.map_or_else(|| gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into(), |filter| filter.as_filter_expression()))
           .project("id, name, description, icon_id, color_id, state_id, parent_team_id, created_by, created_at, updated_by, updated_at")
            .order_by("updated_at desc")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::I32(id) => id.clone(),
                _ => unreachable!("Expected I32")
            },
            name: match row.get("name").unwrap() {
                gluesql::prelude::Value::Str(name) => name.clone(),
                _ => unreachable!("Expected Str")
            },
            description: match row.get("description").unwrap() {
                gluesql::prelude::Value::Str(description) => description.clone(),
                _ => unreachable!("Expected Str")
            },
            icon_id: match row.get("icon_id").unwrap() {
                gluesql::prelude::Value::I32(icon_id) => icon_id.clone(),
                _ => unreachable!("Expected I32")
            },
            color_id: match row.get("color_id").unwrap() {
                gluesql::prelude::Value::I32(color_id) => color_id.clone(),
                _ => unreachable!("Expected I32")
            },
            state_id: match row.get("state_id").unwrap() {
                gluesql::prelude::Value::I32(state_id) => state_id.clone(),
                _ => unreachable!("Expected I32")
            },
            parent_team_id: match row.get("parent_team_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::I32(parent_team_id) => Some(parent_team_id.clone()),
                _ => unreachable!("Expected I32")
            },
            created_by: match row.get("created_by").unwrap() {
                gluesql::prelude::Value::I32(created_by) => created_by.clone(),
                _ => unreachable!("Expected I32")
            },
            created_at: match row.get("created_at").unwrap() {
                gluesql::prelude::Value::Timestamp(created_at) => created_at.clone(),
                _ => unreachable!("Expected Timestamp")
            },
            updated_by: match row.get("updated_by").unwrap() {
                gluesql::prelude::Value::I32(updated_by) => updated_by.clone(),
                _ => unreachable!("Expected I32")
            },
            updated_at: match row.get("updated_at").unwrap() {
                gluesql::prelude::Value::Timestamp(updated_at) => updated_at.clone(),
                _ => unreachable!("Expected Timestamp")
            },
        }
    }
}
#[derive(Eq, PartialEq, PartialOrd, Debug, Clone, Copy, Ord, serde::Serialize, serde::Deserialize, Default)]
pub struct TeamsTeamsRoleInvitation {
    pub table_id: i32,
    pub team_id: i32,
    pub role_id: i32,
    pub created_by: i32,
    pub created_at: chrono::NaiveDateTime,
}

unsafe impl Send for TeamsTeamsRoleInvitation {}
unsafe impl Sync for TeamsTeamsRoleInvitation {}
impl Tabular for TeamsTeamsRoleInvitation {
    const TABLE: Table = Table::TeamsTeamsRoleInvitations;
}
impl Describable for TeamsTeamsRoleInvitation {
    fn description(&self) -> Option<&str> {
        None
    }
}
impl Colorable for TeamsTeamsRoleInvitation {
    fn color(&self) -> Option<&str> {
        None
    }
}

impl Filtrable for TeamsTeamsRoleInvitation {
    type Filter = TeamsTeamsRoleInvitationFilter;
}
#[cfg(feature = "frontend")]
impl TeamsTeamsRoleInvitation {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.table_id),
            gluesql::core::ast_builder::num(self.team_id),
            gluesql::core::ast_builder::num(self.role_id),
            gluesql::core::ast_builder::num(self.created_by),
            gluesql::core::ast_builder::timestamp(self.created_at.to_string()),
        ]
    }

    /// Insert the TeamsTeamsRoleInvitation into the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn insert<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut> (
self: crate::database::flat_variants::TeamsTeamsRoleInvitation,
connection: &mut gluesql::prelude::Glue<C>,
) -> Result<usize, gluesql::prelude::Error>    {
        use gluesql::core::ast_builder::*;
        table("teams_teams_role_invitations")
            .insert()
            .columns("table_id, team_id, role_id, created_by, created_at")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get TeamsTeamsRoleInvitation from the database by its ID.
    ///
    /// # Arguments
    /// * `( table_id, team_id )` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        ( table_id, team_id ): ( i32, i32 ),
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("teams_teams_role_invitations")
            .select()
            .filter(col("table_id").eq(table_id.to_string()))
            .filter(col("team_id").eq(team_id.to_string()))
            .project("table_id, team_id, role_id, created_by, created_at")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete TeamsTeamsRoleInvitation from the database.
    ///
    /// # Arguments
    /// * `( table_id, team_id )` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        ( table_id, team_id ): ( i32, i32 ),
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("teams_teams_role_invitations")
            .delete()
            .filter(col("table_id").eq(table_id.to_string()))
            .filter(col("team_id").eq(team_id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of TeamsTeamsRoleInvitation from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(( self.table_id, self.team_id ), connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("teams_teams_role_invitations")
            .update()        
.set("table_id", gluesql::core::ast_builder::num(self.table_id))        
.set("team_id", gluesql::core::ast_builder::num(self.team_id))        
.set("role_id", gluesql::core::ast_builder::num(self.role_id))        
.set("created_by", gluesql::core::ast_builder::num(self.created_by))        
.set("created_at", gluesql::core::ast_builder::timestamp(self.created_at.to_string()))            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all TeamsTeamsRoleInvitation from the database.
    ///
    /// # Arguments
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        filter: Option<&TeamsTeamsRoleInvitationFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("teams_teams_role_invitations")
            .select()
            .filter(filter.map_or_else(|| gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into(), |filter| filter.as_filter_expression()))
           .project("table_id, team_id, role_id, created_by, created_at")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            table_id: match row.get("table_id").unwrap() {
                gluesql::prelude::Value::I32(table_id) => table_id.clone(),
                _ => unreachable!("Expected I32")
            },
            team_id: match row.get("team_id").unwrap() {
                gluesql::prelude::Value::I32(team_id) => team_id.clone(),
                _ => unreachable!("Expected I32")
            },
            role_id: match row.get("role_id").unwrap() {
                gluesql::prelude::Value::I32(role_id) => role_id.clone(),
                _ => unreachable!("Expected I32")
            },
            created_by: match row.get("created_by").unwrap() {
                gluesql::prelude::Value::I32(created_by) => created_by.clone(),
                _ => unreachable!("Expected I32")
            },
            created_at: match row.get("created_at").unwrap() {
                gluesql::prelude::Value::Timestamp(created_at) => created_at.clone(),
                _ => unreachable!("Expected Timestamp")
            },
        }
    }
}
#[derive(Eq, PartialEq, PartialOrd, Debug, Clone, Copy, Ord, serde::Serialize, serde::Deserialize, Default)]
pub struct TeamsUsersRoleInvitation {
    pub table_id: i32,
    pub user_id: i32,
    pub role_id: i32,
    pub created_by: i32,
    pub created_at: chrono::NaiveDateTime,
}

unsafe impl Send for TeamsUsersRoleInvitation {}
unsafe impl Sync for TeamsUsersRoleInvitation {}
impl Tabular for TeamsUsersRoleInvitation {
    const TABLE: Table = Table::TeamsUsersRoleInvitations;
}
impl Describable for TeamsUsersRoleInvitation {
    fn description(&self) -> Option<&str> {
        None
    }
}
impl Colorable for TeamsUsersRoleInvitation {
    fn color(&self) -> Option<&str> {
        None
    }
}

impl Filtrable for TeamsUsersRoleInvitation {
    type Filter = TeamsUsersRoleInvitationFilter;
}
#[cfg(feature = "frontend")]
impl TeamsUsersRoleInvitation {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.table_id),
            gluesql::core::ast_builder::num(self.user_id),
            gluesql::core::ast_builder::num(self.role_id),
            gluesql::core::ast_builder::num(self.created_by),
            gluesql::core::ast_builder::timestamp(self.created_at.to_string()),
        ]
    }

    /// Insert the TeamsUsersRoleInvitation into the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn insert<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut> (
self: crate::database::flat_variants::TeamsUsersRoleInvitation,
connection: &mut gluesql::prelude::Glue<C>,
) -> Result<usize, gluesql::prelude::Error>    {
        use gluesql::core::ast_builder::*;
        table("teams_users_role_invitations")
            .insert()
            .columns("table_id, user_id, role_id, created_by, created_at")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get TeamsUsersRoleInvitation from the database by its ID.
    ///
    /// # Arguments
    /// * `( table_id, user_id )` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        ( table_id, user_id ): ( i32, i32 ),
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("teams_users_role_invitations")
            .select()
            .filter(col("table_id").eq(table_id.to_string()))
            .filter(col("user_id").eq(user_id.to_string()))
            .project("table_id, user_id, role_id, created_by, created_at")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete TeamsUsersRoleInvitation from the database.
    ///
    /// # Arguments
    /// * `( table_id, user_id )` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        ( table_id, user_id ): ( i32, i32 ),
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("teams_users_role_invitations")
            .delete()
            .filter(col("table_id").eq(table_id.to_string()))
            .filter(col("user_id").eq(user_id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of TeamsUsersRoleInvitation from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(( self.table_id, self.user_id ), connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("teams_users_role_invitations")
            .update()        
.set("table_id", gluesql::core::ast_builder::num(self.table_id))        
.set("user_id", gluesql::core::ast_builder::num(self.user_id))        
.set("role_id", gluesql::core::ast_builder::num(self.role_id))        
.set("created_by", gluesql::core::ast_builder::num(self.created_by))        
.set("created_at", gluesql::core::ast_builder::timestamp(self.created_at.to_string()))            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all TeamsUsersRoleInvitation from the database.
    ///
    /// # Arguments
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        filter: Option<&TeamsUsersRoleInvitationFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("teams_users_role_invitations")
            .select()
            .filter(filter.map_or_else(|| gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into(), |filter| filter.as_filter_expression()))
           .project("table_id, user_id, role_id, created_by, created_at")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            table_id: match row.get("table_id").unwrap() {
                gluesql::prelude::Value::I32(table_id) => table_id.clone(),
                _ => unreachable!("Expected I32")
            },
            user_id: match row.get("user_id").unwrap() {
                gluesql::prelude::Value::I32(user_id) => user_id.clone(),
                _ => unreachable!("Expected I32")
            },
            role_id: match row.get("role_id").unwrap() {
                gluesql::prelude::Value::I32(role_id) => role_id.clone(),
                _ => unreachable!("Expected I32")
            },
            created_by: match row.get("created_by").unwrap() {
                gluesql::prelude::Value::I32(created_by) => created_by.clone(),
                _ => unreachable!("Expected I32")
            },
            created_at: match row.get("created_at").unwrap() {
                gluesql::prelude::Value::Timestamp(created_at) => created_at.clone(),
                _ => unreachable!("Expected Timestamp")
            },
        }
    }
}
#[derive(Eq, PartialEq, PartialOrd, Debug, Clone, Copy, Ord, serde::Serialize, serde::Deserialize, Default)]
pub struct TeamsUsersRoleRequest {
    pub table_id: i32,
    pub user_id: i32,
    pub role_id: i32,
    pub created_by: i32,
    pub created_at: chrono::NaiveDateTime,
}

unsafe impl Send for TeamsUsersRoleRequest {}
unsafe impl Sync for TeamsUsersRoleRequest {}
impl Tabular for TeamsUsersRoleRequest {
    const TABLE: Table = Table::TeamsUsersRoleRequests;
}
impl Describable for TeamsUsersRoleRequest {
    fn description(&self) -> Option<&str> {
        None
    }
}
impl Colorable for TeamsUsersRoleRequest {
    fn color(&self) -> Option<&str> {
        None
    }
}

impl Filtrable for TeamsUsersRoleRequest {
    type Filter = TeamsUsersRoleRequestFilter;
}
#[cfg(feature = "frontend")]
impl TeamsUsersRoleRequest {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.table_id),
            gluesql::core::ast_builder::num(self.user_id),
            gluesql::core::ast_builder::num(self.role_id),
            gluesql::core::ast_builder::num(self.created_by),
            gluesql::core::ast_builder::timestamp(self.created_at.to_string()),
        ]
    }

    /// Insert the TeamsUsersRoleRequest into the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn insert<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut> (
self: crate::database::flat_variants::TeamsUsersRoleRequest,
connection: &mut gluesql::prelude::Glue<C>,
) -> Result<usize, gluesql::prelude::Error>    {
        use gluesql::core::ast_builder::*;
        table("teams_users_role_requests")
            .insert()
            .columns("table_id, user_id, role_id, created_by, created_at")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get TeamsUsersRoleRequest from the database by its ID.
    ///
    /// # Arguments
    /// * `( table_id, user_id )` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        ( table_id, user_id ): ( i32, i32 ),
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("teams_users_role_requests")
            .select()
            .filter(col("table_id").eq(table_id.to_string()))
            .filter(col("user_id").eq(user_id.to_string()))
            .project("table_id, user_id, role_id, created_by, created_at")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete TeamsUsersRoleRequest from the database.
    ///
    /// # Arguments
    /// * `( table_id, user_id )` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        ( table_id, user_id ): ( i32, i32 ),
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("teams_users_role_requests")
            .delete()
            .filter(col("table_id").eq(table_id.to_string()))
            .filter(col("user_id").eq(user_id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of TeamsUsersRoleRequest from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(( self.table_id, self.user_id ), connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("teams_users_role_requests")
            .update()        
.set("table_id", gluesql::core::ast_builder::num(self.table_id))        
.set("user_id", gluesql::core::ast_builder::num(self.user_id))        
.set("role_id", gluesql::core::ast_builder::num(self.role_id))        
.set("created_by", gluesql::core::ast_builder::num(self.created_by))        
.set("created_at", gluesql::core::ast_builder::timestamp(self.created_at.to_string()))            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all TeamsUsersRoleRequest from the database.
    ///
    /// # Arguments
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        filter: Option<&TeamsUsersRoleRequestFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("teams_users_role_requests")
            .select()
            .filter(filter.map_or_else(|| gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into(), |filter| filter.as_filter_expression()))
           .project("table_id, user_id, role_id, created_by, created_at")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            table_id: match row.get("table_id").unwrap() {
                gluesql::prelude::Value::I32(table_id) => table_id.clone(),
                _ => unreachable!("Expected I32")
            },
            user_id: match row.get("user_id").unwrap() {
                gluesql::prelude::Value::I32(user_id) => user_id.clone(),
                _ => unreachable!("Expected I32")
            },
            role_id: match row.get("role_id").unwrap() {
                gluesql::prelude::Value::I32(role_id) => role_id.clone(),
                _ => unreachable!("Expected I32")
            },
            created_by: match row.get("created_by").unwrap() {
                gluesql::prelude::Value::I32(created_by) => created_by.clone(),
                _ => unreachable!("Expected I32")
            },
            created_at: match row.get("created_at").unwrap() {
                gluesql::prelude::Value::Timestamp(created_at) => created_at.clone(),
                _ => unreachable!("Expected Timestamp")
            },
        }
    }
}
#[derive(Eq, PartialEq, PartialOrd, Debug, Clone, Copy, Ord, serde::Serialize, serde::Deserialize, Default)]
pub struct TeamsUsersRole {
    pub table_id: i32,
    pub user_id: i32,
    pub role_id: i32,
    pub created_by: i32,
    pub created_at: chrono::NaiveDateTime,
}

unsafe impl Send for TeamsUsersRole {}
unsafe impl Sync for TeamsUsersRole {}
impl Tabular for TeamsUsersRole {
    const TABLE: Table = Table::TeamsUsersRoles;
}
impl Describable for TeamsUsersRole {
    fn description(&self) -> Option<&str> {
        None
    }
}
impl Colorable for TeamsUsersRole {
    fn color(&self) -> Option<&str> {
        None
    }
}

impl Filtrable for TeamsUsersRole {
    type Filter = TeamsUsersRoleFilter;
}
#[cfg(feature = "frontend")]
impl TeamsUsersRole {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.table_id),
            gluesql::core::ast_builder::num(self.user_id),
            gluesql::core::ast_builder::num(self.role_id),
            gluesql::core::ast_builder::num(self.created_by),
            gluesql::core::ast_builder::timestamp(self.created_at.to_string()),
        ]
    }

    /// Insert the TeamsUsersRole into the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn insert<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut> (
self: crate::database::flat_variants::TeamsUsersRole,
connection: &mut gluesql::prelude::Glue<C>,
) -> Result<usize, gluesql::prelude::Error>    {
        use gluesql::core::ast_builder::*;
        table("teams_users_roles")
            .insert()
            .columns("table_id, user_id, role_id, created_by, created_at")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get TeamsUsersRole from the database by its ID.
    ///
    /// # Arguments
    /// * `( table_id, user_id )` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        ( table_id, user_id ): ( i32, i32 ),
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("teams_users_roles")
            .select()
            .filter(col("table_id").eq(table_id.to_string()))
            .filter(col("user_id").eq(user_id.to_string()))
            .project("table_id, user_id, role_id, created_by, created_at")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete TeamsUsersRole from the database.
    ///
    /// # Arguments
    /// * `( table_id, user_id )` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        ( table_id, user_id ): ( i32, i32 ),
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("teams_users_roles")
            .delete()
            .filter(col("table_id").eq(table_id.to_string()))
            .filter(col("user_id").eq(user_id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of TeamsUsersRole from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(( self.table_id, self.user_id ), connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("teams_users_roles")
            .update()        
.set("table_id", gluesql::core::ast_builder::num(self.table_id))        
.set("user_id", gluesql::core::ast_builder::num(self.user_id))        
.set("role_id", gluesql::core::ast_builder::num(self.role_id))        
.set("created_by", gluesql::core::ast_builder::num(self.created_by))        
.set("created_at", gluesql::core::ast_builder::timestamp(self.created_at.to_string()))            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all TeamsUsersRole from the database.
    ///
    /// # Arguments
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        filter: Option<&TeamsUsersRoleFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("teams_users_roles")
            .select()
            .filter(filter.map_or_else(|| gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into(), |filter| filter.as_filter_expression()))
           .project("table_id, user_id, role_id, created_by, created_at")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            table_id: match row.get("table_id").unwrap() {
                gluesql::prelude::Value::I32(table_id) => table_id.clone(),
                _ => unreachable!("Expected I32")
            },
            user_id: match row.get("user_id").unwrap() {
                gluesql::prelude::Value::I32(user_id) => user_id.clone(),
                _ => unreachable!("Expected I32")
            },
            role_id: match row.get("role_id").unwrap() {
                gluesql::prelude::Value::I32(role_id) => role_id.clone(),
                _ => unreachable!("Expected I32")
            },
            created_by: match row.get("created_by").unwrap() {
                gluesql::prelude::Value::I32(created_by) => created_by.clone(),
                _ => unreachable!("Expected I32")
            },
            created_at: match row.get("created_at").unwrap() {
                gluesql::prelude::Value::Timestamp(created_at) => created_at.clone(),
                _ => unreachable!("Expected Timestamp")
            },
        }
    }
}
#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct Unit {
    pub id: i32,
    pub name: String,
    pub unit: String,
    pub icon_id: i32,
    pub color_id: i32,
}

unsafe impl Send for Unit {}
unsafe impl Sync for Unit {}
impl Tabular for Unit {
    const TABLE: Table = Table::Units;
}
impl Describable for Unit {
    fn description(&self) -> Option<&str> {
        None
    }
}
impl Colorable for Unit {
    fn color(&self) -> Option<&str> {
        None
    }
}

impl Filtrable for Unit {
    type Filter = UnitFilter;
}
#[cfg(feature = "frontend")]
impl Unit {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::text(self.name),
            gluesql::core::ast_builder::text(self.unit),
            gluesql::core::ast_builder::num(self.icon_id),
            gluesql::core::ast_builder::num(self.color_id),
        ]
    }

    /// Insert the Unit into the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn insert<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut> (
self: crate::database::flat_variants::Unit,
connection: &mut gluesql::prelude::Glue<C>,
) -> Result<usize, gluesql::prelude::Error>    {
        use gluesql::core::ast_builder::*;
        table("units")
            .insert()
            .columns("id, name, unit, icon_id, color_id")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get Unit from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("units")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, name, unit, icon_id, color_id")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete Unit from the database.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("units")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of Unit from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("units")
            .update()        
.set("id", gluesql::core::ast_builder::num(self.id))        
.set("name", gluesql::core::ast_builder::text(self.name))        
.set("unit", gluesql::core::ast_builder::text(self.unit))        
.set("icon_id", gluesql::core::ast_builder::num(self.icon_id))        
.set("color_id", gluesql::core::ast_builder::num(self.color_id))            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all Unit from the database.
    ///
    /// # Arguments
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        filter: Option<&UnitFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("units")
            .select()
            .filter(filter.map_or_else(|| gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into(), |filter| filter.as_filter_expression()))
           .project("id, name, unit, icon_id, color_id")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::I32(id) => id.clone(),
                _ => unreachable!("Expected I32")
            },
            name: match row.get("name").unwrap() {
                gluesql::prelude::Value::Str(name) => name.clone(),
                _ => unreachable!("Expected Str")
            },
            unit: match row.get("unit").unwrap() {
                gluesql::prelude::Value::Str(unit) => unit.clone(),
                _ => unreachable!("Expected Str")
            },
            icon_id: match row.get("icon_id").unwrap() {
                gluesql::prelude::Value::I32(icon_id) => icon_id.clone(),
                _ => unreachable!("Expected I32")
            },
            color_id: match row.get("color_id").unwrap() {
                gluesql::prelude::Value::I32(color_id) => color_id.clone(),
                _ => unreachable!("Expected I32")
            },
        }
    }
}
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
    const TABLE: Table = Table::UserEmails;
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

impl Filtrable for UserEmail {
    type Filter = UserEmailFilter;
}
#[cfg(feature = "frontend")]
impl UserEmail {
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
    pub async fn insert<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut> (
self: crate::database::flat_variants::UserEmail,
connection: &mut gluesql::prelude::Glue<C>,
) -> Result<usize, gluesql::prelude::Error>    {
        use gluesql::core::ast_builder::*;
        table("user_emails")
            .insert()
            .columns("id, email, created_by, created_at, login_provider_id, primary_email")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get UserEmail from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("user_emails")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, email, created_by, created_at, login_provider_id, primary_email")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete UserEmail from the database.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("user_emails")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of UserEmail from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("user_emails")
            .update()        
.set("id", gluesql::core::ast_builder::num(self.id))        
.set("email", gluesql::core::ast_builder::text(self.email))        
.set("created_by", gluesql::core::ast_builder::num(self.created_by))        
.set("created_at", gluesql::core::ast_builder::timestamp(self.created_at.to_string()))        
.set("login_provider_id", gluesql::core::ast_builder::num(self.login_provider_id))        
.set("primary_email", self.primary_email)            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all UserEmail from the database.
    ///
    /// # Arguments
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        filter: Option<&UserEmailFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("user_emails")
            .select()
            .filter(filter.map_or_else(|| gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into(), |filter| filter.as_filter_expression()))
           .project("id, email, created_by, created_at, login_provider_id, primary_email")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::I32(id) => id.clone(),
                _ => unreachable!("Expected I32")
            },
            email: match row.get("email").unwrap() {
                gluesql::prelude::Value::Str(email) => email.clone(),
                _ => unreachable!("Expected Str")
            },
            created_by: match row.get("created_by").unwrap() {
                gluesql::prelude::Value::I32(created_by) => created_by.clone(),
                _ => unreachable!("Expected I32")
            },
            created_at: match row.get("created_at").unwrap() {
                gluesql::prelude::Value::Timestamp(created_at) => created_at.clone(),
                _ => unreachable!("Expected Timestamp")
            },
            login_provider_id: match row.get("login_provider_id").unwrap() {
                gluesql::prelude::Value::I32(login_provider_id) => login_provider_id.clone(),
                _ => unreachable!("Expected I32")
            },
            primary_email: match row.get("primary_email").unwrap() {
                gluesql::prelude::Value::Bool(primary_email) => primary_email.clone(),
                _ => unreachable!("Expected Bool")
            },
        }
    }
}
#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub description: Option<String>,
    pub organization_id: Option<i32>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub picture: crate::types::JPEG,
}

unsafe impl Send for User {}
unsafe impl Sync for User {}
impl Tabular for User {
    const TABLE: Table = Table::Users;
}
impl Describable for User {
    fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}
impl Colorable for User {
    fn color(&self) -> Option<&str> {
        None
    }
}

impl Filtrable for User {
    type Filter = UserFilter;
}
#[cfg(feature = "frontend")]
impl User {
    pub fn get_picture_as_url(&self) -> String {
        self.picture.guess_image_url().unwrap()
    }

    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::text(self.first_name),
            match self.middle_name {
                Some(middle_name) => gluesql::core::ast_builder::text(middle_name),
                None => gluesql::core::ast_builder::null(),
            },
            gluesql::core::ast_builder::text(self.last_name),
            match self.description {
                Some(description) => gluesql::core::ast_builder::text(description),
                None => gluesql::core::ast_builder::null(),
            },
            match self.organization_id {
                Some(organization_id) => gluesql::core::ast_builder::num(organization_id),
                None => gluesql::core::ast_builder::null(),
            },
            gluesql::core::ast_builder::timestamp(self.created_at.to_string()),
            gluesql::core::ast_builder::timestamp(self.updated_at.to_string()),
            gluesql::core::ast_builder::bytea(self.picture),
        ]
    }

    /// Insert the User into the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn insert<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut> (
self: crate::database::flat_variants::User,
connection: &mut gluesql::prelude::Glue<C>,
) -> Result<usize, gluesql::prelude::Error>    {
        use gluesql::core::ast_builder::*;
        table("users")
            .insert()
            .columns("id, first_name, middle_name, last_name, description, organization_id, created_at, updated_at, picture")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get User from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("users")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, first_name, middle_name, last_name, description, organization_id, created_at, updated_at, picture")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete User from the database.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("users")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of User from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let mut update_row = table("users")
            .update()        
.set("id", gluesql::core::ast_builder::num(self.id))        
.set("first_name", gluesql::core::ast_builder::text(self.first_name))        
.set("last_name", gluesql::core::ast_builder::text(self.last_name))        
.set("created_at", gluesql::core::ast_builder::timestamp(self.created_at.to_string()))        
.set("updated_at", gluesql::core::ast_builder::timestamp(self.updated_at.to_string()))        
.set("picture", gluesql::core::ast_builder::bytea(self.picture));
        if let Some(middle_name) = self.middle_name {
            update_row = update_row.set("middle_name", gluesql::core::ast_builder::text(middle_name));
        }
        if let Some(description) = self.description {
            update_row = update_row.set("description", gluesql::core::ast_builder::text(description));
        }
        if let Some(organization_id) = self.organization_id {
            update_row = update_row.set("organization_id", gluesql::core::ast_builder::num(organization_id));
        }
            update_row.execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all User from the database.
    ///
    /// # Arguments
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        filter: Option<&UserFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("users")
            .select()
            .filter(filter.map_or_else(|| gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into(), |filter| filter.as_filter_expression()))
           .project("id, first_name, middle_name, last_name, description, organization_id, created_at, updated_at, picture")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    /// Get all User from the database ordered by the `updated_at` column.
    ///
    /// # Arguments
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all_by_updated_at<C>(
        filter: Option<&UserFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("users")
            .select()
            .filter(filter.map_or_else(|| gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into(), |filter| filter.as_filter_expression()))
           .project("id, first_name, middle_name, last_name, description, organization_id, created_at, updated_at, picture")
            .order_by("updated_at desc")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::I32(id) => id.clone(),
                _ => unreachable!("Expected I32")
            },
            first_name: match row.get("first_name").unwrap() {
                gluesql::prelude::Value::Str(first_name) => first_name.clone(),
                _ => unreachable!("Expected Str")
            },
            middle_name: match row.get("middle_name").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Str(middle_name) => Some(middle_name.clone()),
                _ => unreachable!("Expected Str")
            },
            last_name: match row.get("last_name").unwrap() {
                gluesql::prelude::Value::Str(last_name) => last_name.clone(),
                _ => unreachable!("Expected Str")
            },
            description: match row.get("description").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Str(description) => Some(description.clone()),
                _ => unreachable!("Expected Str")
            },
            organization_id: match row.get("organization_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::I32(organization_id) => Some(organization_id.clone()),
                _ => unreachable!("Expected I32")
            },
            created_at: match row.get("created_at").unwrap() {
                gluesql::prelude::Value::Timestamp(created_at) => created_at.clone(),
                _ => unreachable!("Expected Timestamp")
            },
            updated_at: match row.get("updated_at").unwrap() {
                gluesql::prelude::Value::Timestamp(updated_at) => updated_at.clone(),
                _ => unreachable!("Expected Timestamp")
            },
            picture: match row.get("picture").unwrap() {
                gluesql::prelude::Value::Bytea(picture) => picture.clone().into(),
                _ => unreachable!("Expected Bytea"),
            },
        }
    }
}
#[derive(Eq, PartialEq, PartialOrd, Debug, Clone, Copy, Ord, serde::Serialize, serde::Deserialize, Default)]
pub struct UsersUsersRoleInvitation {
    pub table_id: i32,
    pub user_id: i32,
    pub role_id: i32,
    pub created_by: i32,
    pub created_at: chrono::NaiveDateTime,
}

unsafe impl Send for UsersUsersRoleInvitation {}
unsafe impl Sync for UsersUsersRoleInvitation {}
impl Tabular for UsersUsersRoleInvitation {
    const TABLE: Table = Table::UsersUsersRoleInvitations;
}
impl Describable for UsersUsersRoleInvitation {
    fn description(&self) -> Option<&str> {
        None
    }
}
impl Colorable for UsersUsersRoleInvitation {
    fn color(&self) -> Option<&str> {
        None
    }
}

impl Filtrable for UsersUsersRoleInvitation {
    type Filter = UsersUsersRoleInvitationFilter;
}
#[cfg(feature = "frontend")]
impl UsersUsersRoleInvitation {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.table_id),
            gluesql::core::ast_builder::num(self.user_id),
            gluesql::core::ast_builder::num(self.role_id),
            gluesql::core::ast_builder::num(self.created_by),
            gluesql::core::ast_builder::timestamp(self.created_at.to_string()),
        ]
    }

    /// Insert the UsersUsersRoleInvitation into the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn insert<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut> (
self: crate::database::flat_variants::UsersUsersRoleInvitation,
connection: &mut gluesql::prelude::Glue<C>,
) -> Result<usize, gluesql::prelude::Error>    {
        use gluesql::core::ast_builder::*;
        table("users_users_role_invitations")
            .insert()
            .columns("table_id, user_id, role_id, created_by, created_at")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get UsersUsersRoleInvitation from the database by its ID.
    ///
    /// # Arguments
    /// * `( table_id, user_id )` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        ( table_id, user_id ): ( i32, i32 ),
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("users_users_role_invitations")
            .select()
            .filter(col("table_id").eq(table_id.to_string()))
            .filter(col("user_id").eq(user_id.to_string()))
            .project("table_id, user_id, role_id, created_by, created_at")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete UsersUsersRoleInvitation from the database.
    ///
    /// # Arguments
    /// * `( table_id, user_id )` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        ( table_id, user_id ): ( i32, i32 ),
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("users_users_role_invitations")
            .delete()
            .filter(col("table_id").eq(table_id.to_string()))
            .filter(col("user_id").eq(user_id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of UsersUsersRoleInvitation from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(( self.table_id, self.user_id ), connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("users_users_role_invitations")
            .update()        
.set("table_id", gluesql::core::ast_builder::num(self.table_id))        
.set("user_id", gluesql::core::ast_builder::num(self.user_id))        
.set("role_id", gluesql::core::ast_builder::num(self.role_id))        
.set("created_by", gluesql::core::ast_builder::num(self.created_by))        
.set("created_at", gluesql::core::ast_builder::timestamp(self.created_at.to_string()))            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all UsersUsersRoleInvitation from the database.
    ///
    /// # Arguments
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        filter: Option<&UsersUsersRoleInvitationFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("users_users_role_invitations")
            .select()
            .filter(filter.map_or_else(|| gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into(), |filter| filter.as_filter_expression()))
           .project("table_id, user_id, role_id, created_by, created_at")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            table_id: match row.get("table_id").unwrap() {
                gluesql::prelude::Value::I32(table_id) => table_id.clone(),
                _ => unreachable!("Expected I32")
            },
            user_id: match row.get("user_id").unwrap() {
                gluesql::prelude::Value::I32(user_id) => user_id.clone(),
                _ => unreachable!("Expected I32")
            },
            role_id: match row.get("role_id").unwrap() {
                gluesql::prelude::Value::I32(role_id) => role_id.clone(),
                _ => unreachable!("Expected I32")
            },
            created_by: match row.get("created_by").unwrap() {
                gluesql::prelude::Value::I32(created_by) => created_by.clone(),
                _ => unreachable!("Expected I32")
            },
            created_at: match row.get("created_at").unwrap() {
                gluesql::prelude::Value::Timestamp(created_at) => created_at.clone(),
                _ => unreachable!("Expected Timestamp")
            },
        }
    }
}
#[derive(Eq, PartialEq, PartialOrd, Debug, Clone, Copy, Ord, serde::Serialize, serde::Deserialize, Default)]
pub struct UsersUsersRoleRequest {
    pub table_id: i32,
    pub user_id: i32,
    pub role_id: i32,
    pub created_by: i32,
    pub created_at: chrono::NaiveDateTime,
}

unsafe impl Send for UsersUsersRoleRequest {}
unsafe impl Sync for UsersUsersRoleRequest {}
impl Tabular for UsersUsersRoleRequest {
    const TABLE: Table = Table::UsersUsersRoleRequests;
}
impl Describable for UsersUsersRoleRequest {
    fn description(&self) -> Option<&str> {
        None
    }
}
impl Colorable for UsersUsersRoleRequest {
    fn color(&self) -> Option<&str> {
        None
    }
}

impl Filtrable for UsersUsersRoleRequest {
    type Filter = UsersUsersRoleRequestFilter;
}
#[cfg(feature = "frontend")]
impl UsersUsersRoleRequest {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.table_id),
            gluesql::core::ast_builder::num(self.user_id),
            gluesql::core::ast_builder::num(self.role_id),
            gluesql::core::ast_builder::num(self.created_by),
            gluesql::core::ast_builder::timestamp(self.created_at.to_string()),
        ]
    }

    /// Insert the UsersUsersRoleRequest into the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn insert<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut> (
self: crate::database::flat_variants::UsersUsersRoleRequest,
connection: &mut gluesql::prelude::Glue<C>,
) -> Result<usize, gluesql::prelude::Error>    {
        use gluesql::core::ast_builder::*;
        table("users_users_role_requests")
            .insert()
            .columns("table_id, user_id, role_id, created_by, created_at")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get UsersUsersRoleRequest from the database by its ID.
    ///
    /// # Arguments
    /// * `( table_id, user_id )` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        ( table_id, user_id ): ( i32, i32 ),
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("users_users_role_requests")
            .select()
            .filter(col("table_id").eq(table_id.to_string()))
            .filter(col("user_id").eq(user_id.to_string()))
            .project("table_id, user_id, role_id, created_by, created_at")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete UsersUsersRoleRequest from the database.
    ///
    /// # Arguments
    /// * `( table_id, user_id )` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        ( table_id, user_id ): ( i32, i32 ),
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("users_users_role_requests")
            .delete()
            .filter(col("table_id").eq(table_id.to_string()))
            .filter(col("user_id").eq(user_id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of UsersUsersRoleRequest from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(( self.table_id, self.user_id ), connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("users_users_role_requests")
            .update()        
.set("table_id", gluesql::core::ast_builder::num(self.table_id))        
.set("user_id", gluesql::core::ast_builder::num(self.user_id))        
.set("role_id", gluesql::core::ast_builder::num(self.role_id))        
.set("created_by", gluesql::core::ast_builder::num(self.created_by))        
.set("created_at", gluesql::core::ast_builder::timestamp(self.created_at.to_string()))            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all UsersUsersRoleRequest from the database.
    ///
    /// # Arguments
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        filter: Option<&UsersUsersRoleRequestFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("users_users_role_requests")
            .select()
            .filter(filter.map_or_else(|| gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into(), |filter| filter.as_filter_expression()))
           .project("table_id, user_id, role_id, created_by, created_at")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            table_id: match row.get("table_id").unwrap() {
                gluesql::prelude::Value::I32(table_id) => table_id.clone(),
                _ => unreachable!("Expected I32")
            },
            user_id: match row.get("user_id").unwrap() {
                gluesql::prelude::Value::I32(user_id) => user_id.clone(),
                _ => unreachable!("Expected I32")
            },
            role_id: match row.get("role_id").unwrap() {
                gluesql::prelude::Value::I32(role_id) => role_id.clone(),
                _ => unreachable!("Expected I32")
            },
            created_by: match row.get("created_by").unwrap() {
                gluesql::prelude::Value::I32(created_by) => created_by.clone(),
                _ => unreachable!("Expected I32")
            },
            created_at: match row.get("created_at").unwrap() {
                gluesql::prelude::Value::Timestamp(created_at) => created_at.clone(),
                _ => unreachable!("Expected Timestamp")
            },
        }
    }
}
#[derive(Eq, PartialEq, PartialOrd, Debug, Clone, Copy, Ord, serde::Serialize, serde::Deserialize, Default)]
pub struct UsersUsersRole {
    pub table_id: i32,
    pub user_id: i32,
    pub role_id: i32,
    pub created_by: i32,
    pub created_at: chrono::NaiveDateTime,
}

unsafe impl Send for UsersUsersRole {}
unsafe impl Sync for UsersUsersRole {}
impl Tabular for UsersUsersRole {
    const TABLE: Table = Table::UsersUsersRoles;
}
impl Describable for UsersUsersRole {
    fn description(&self) -> Option<&str> {
        None
    }
}
impl Colorable for UsersUsersRole {
    fn color(&self) -> Option<&str> {
        None
    }
}

impl Filtrable for UsersUsersRole {
    type Filter = UsersUsersRoleFilter;
}
#[cfg(feature = "frontend")]
impl UsersUsersRole {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.table_id),
            gluesql::core::ast_builder::num(self.user_id),
            gluesql::core::ast_builder::num(self.role_id),
            gluesql::core::ast_builder::num(self.created_by),
            gluesql::core::ast_builder::timestamp(self.created_at.to_string()),
        ]
    }

    /// Insert the UsersUsersRole into the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn insert<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut> (
self: crate::database::flat_variants::UsersUsersRole,
connection: &mut gluesql::prelude::Glue<C>,
) -> Result<usize, gluesql::prelude::Error>    {
        use gluesql::core::ast_builder::*;
        table("users_users_roles")
            .insert()
            .columns("table_id, user_id, role_id, created_by, created_at")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get UsersUsersRole from the database by its ID.
    ///
    /// # Arguments
    /// * `( table_id, user_id )` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        ( table_id, user_id ): ( i32, i32 ),
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("users_users_roles")
            .select()
            .filter(col("table_id").eq(table_id.to_string()))
            .filter(col("user_id").eq(user_id.to_string()))
            .project("table_id, user_id, role_id, created_by, created_at")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete UsersUsersRole from the database.
    ///
    /// # Arguments
    /// * `( table_id, user_id )` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        ( table_id, user_id ): ( i32, i32 ),
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("users_users_roles")
            .delete()
            .filter(col("table_id").eq(table_id.to_string()))
            .filter(col("user_id").eq(user_id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of UsersUsersRole from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(( self.table_id, self.user_id ), connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("users_users_roles")
            .update()        
.set("table_id", gluesql::core::ast_builder::num(self.table_id))        
.set("user_id", gluesql::core::ast_builder::num(self.user_id))        
.set("role_id", gluesql::core::ast_builder::num(self.role_id))        
.set("created_by", gluesql::core::ast_builder::num(self.created_by))        
.set("created_at", gluesql::core::ast_builder::timestamp(self.created_at.to_string()))            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all UsersUsersRole from the database.
    ///
    /// # Arguments
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        filter: Option<&UsersUsersRoleFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("users_users_roles")
            .select()
            .filter(filter.map_or_else(|| gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into(), |filter| filter.as_filter_expression()))
           .project("table_id, user_id, role_id, created_by, created_at")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            table_id: match row.get("table_id").unwrap() {
                gluesql::prelude::Value::I32(table_id) => table_id.clone(),
                _ => unreachable!("Expected I32")
            },
            user_id: match row.get("user_id").unwrap() {
                gluesql::prelude::Value::I32(user_id) => user_id.clone(),
                _ => unreachable!("Expected I32")
            },
            role_id: match row.get("role_id").unwrap() {
                gluesql::prelude::Value::I32(role_id) => role_id.clone(),
                _ => unreachable!("Expected I32")
            },
            created_by: match row.get("created_by").unwrap() {
                gluesql::prelude::Value::I32(created_by) => created_by.clone(),
                _ => unreachable!("Expected I32")
            },
            created_at: match row.get("created_at").unwrap() {
                gluesql::prelude::Value::Timestamp(created_at) => created_at.clone(),
                _ => unreachable!("Expected Timestamp")
            },
        }
    }
}
