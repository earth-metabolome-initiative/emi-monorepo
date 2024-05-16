use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;
use chrono::NaiveDateTime;
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct BioOttRank {
    pub id: i32,
    pub name: String,
    pub font_awesome_icon_id: i32,
}
#[cfg(feature = "frontend")]
impl BioOttRank {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::text(self.name),
            gluesql::core::ast_builder::num(self.font_awesome_icon_id),
        ]
    }

    /// Insert the BioOttRank into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table BioOttRank
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("bio_ott_ranks")
            .insert()
            .columns("id, name, font_awesome_icon_id")
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
            .project("id, name, font_awesome_icon_id")
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
.set("font_awesome_icon_id", gluesql::core::ast_builder::num(self.font_awesome_icon_id))            .execute(connection)
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
        let select_row = table("bio_ott_ranks")
            .select()
            .project("id, name, font_awesome_icon_id")
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
            font_awesome_icon_id: match row.get("font_awesome_icon_id").unwrap() {
                gluesql::prelude::Value::I32(font_awesome_icon_id) => font_awesome_icon_id.clone(),
                _ => unreachable!("Expected I32")
            },
        }
    }
}
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
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
    pub font_awesome_icon_id: i32,
    pub color_id: i32,
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
            gluesql::core::ast_builder::num(self.font_awesome_icon_id),
            gluesql::core::ast_builder::num(self.color_id),
        ]
    }

    /// Insert the BioOttTaxonItem into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table BioOttTaxonItem
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("bio_ott_taxon_items")
            .insert()
            .columns("id, name, ott_id, ott_rank_id, wikidata_id, ncbi_id, gbif_id, irmng_id, worms_id, domain_id, kingdom_id, phylum_id, class_id, order_id, family_id, genus_id, parent_id, font_awesome_icon_id, color_id")
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
            .project("id, name, ott_id, ott_rank_id, wikidata_id, ncbi_id, gbif_id, irmng_id, worms_id, domain_id, kingdom_id, phylum_id, class_id, order_id, family_id, genus_id, parent_id, font_awesome_icon_id, color_id")
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
.set("font_awesome_icon_id", gluesql::core::ast_builder::num(self.font_awesome_icon_id))        
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
        let select_row = table("bio_ott_taxon_items")
            .select()
            .project("id, name, ott_id, ott_rank_id, wikidata_id, ncbi_id, gbif_id, irmng_id, worms_id, domain_id, kingdom_id, phylum_id, class_id, order_id, family_id, genus_id, parent_id, font_awesome_icon_id, color_id")
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
            font_awesome_icon_id: match row.get("font_awesome_icon_id").unwrap() {
                gluesql::prelude::Value::I32(font_awesome_icon_id) => font_awesome_icon_id.clone(),
                _ => unreachable!("Expected I32")
            },
            color_id: match row.get("color_id").unwrap() {
                gluesql::prelude::Value::I32(color_id) => color_id.clone(),
                _ => unreachable!("Expected I32")
            },
        }
    }
}
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct Color {
    pub id: i32,
    pub name: String,
    pub hexadecimal_value: String,
}
#[cfg(feature = "frontend")]
impl Color {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::text(self.name),
            gluesql::core::ast_builder::text(self.hexadecimal_value),
        ]
    }

    /// Insert the Color into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table Color
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("colors")
            .insert()
            .columns("id, name, hexadecimal_value")
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
            .project("id, name, hexadecimal_value")
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
.set("hexadecimal_value", gluesql::core::ast_builder::text(self.hexadecimal_value))            .execute(connection)
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
            .project("id, name, hexadecimal_value")
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
        }
    }
}
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct Country {
    pub id: i32,
    pub iso: String,
    pub emoji: String,
    pub unicode: String,
    pub name: String,
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
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table Country
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
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
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct DerivedSample {
    pub created_by: i32,
    pub created_at: NaiveDateTime,
    pub updated_by: i32,
    pub updated_at: NaiveDateTime,
    pub parent_sample_id: Uuid,
    pub child_sample_id: Uuid,
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
        ]
    }

    /// Insert the DerivedSample into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table DerivedSample
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("derived_samples")
            .insert()
            .columns("created_by, created_at, updated_by, updated_at, parent_sample_id, child_sample_id")
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
        ( parent_sample_id, child_sample_id ): ( Uuid, Uuid ),
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("derived_samples")
            .select()
            .filter(col("parent_sample_id").eq(parent_sample_id.to_string()))
            .filter(col("child_sample_id").eq(child_sample_id.to_string()))
            .project("created_by, created_at, updated_by, updated_at, parent_sample_id, child_sample_id")
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
        ( parent_sample_id, child_sample_id ): ( Uuid, Uuid ),
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
.set("child_sample_id", gluesql::core::ast_builder::uuid(self.child_sample_id.to_string()))            .execute(connection)
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
        let select_row = table("derived_samples")
            .select()
            .project("created_by, created_at, updated_by, updated_at, parent_sample_id, child_sample_id")
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
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all_by_updated_at<C>(
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("derived_samples")
            .select()
            .project("created_by, created_at, updated_by, updated_at, parent_sample_id, child_sample_id")
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
                gluesql::prelude::Value::Uuid(parent_sample_id) => Uuid::from_u128(*parent_sample_id),
                _ => unreachable!("Expected Uuid"),
            },
            child_sample_id: match row.get("child_sample_id").unwrap() {
                gluesql::prelude::Value::Uuid(child_sample_id) => Uuid::from_u128(*child_sample_id),
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct DocumentFormat {
    pub id: i32,
    pub extension: String,
    pub mime_type: String,
}
#[cfg(feature = "frontend")]
impl DocumentFormat {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::text(self.extension),
            gluesql::core::ast_builder::text(self.mime_type),
        ]
    }

    /// Insert the DocumentFormat into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table DocumentFormat
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("document_formats")
            .insert()
            .columns("id, extension, mime_type")
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
            .project("id, extension, mime_type")
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
.set("mime_type", gluesql::core::ast_builder::text(self.mime_type))            .execute(connection)
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
        let select_row = table("document_formats")
            .select()
            .project("id, extension, mime_type")
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
        }
    }
}
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct FontAwesomeIcon {
    pub id: i32,
    pub name: String,
}
#[cfg(feature = "frontend")]
impl FontAwesomeIcon {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::text(self.name),
        ]
    }

    /// Insert the FontAwesomeIcon into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table FontAwesomeIcon
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("font_awesome_icons")
            .insert()
            .columns("id, name")
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
            .project("id, name")
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
            .project("id, name")
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
        }
    }
}
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct LoginProvider {
    pub id: i32,
    pub name: String,
    pub font_awesome_icon_id: i32,
    pub color_id: i32,
    pub client_id_var_name: String,
    pub redirect_uri_var_name: String,
    pub oauth_url: String,
    pub scope: String,
}
#[cfg(feature = "frontend")]
impl LoginProvider {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::text(self.name),
            gluesql::core::ast_builder::num(self.font_awesome_icon_id),
            gluesql::core::ast_builder::num(self.color_id),
            gluesql::core::ast_builder::text(self.client_id_var_name),
            gluesql::core::ast_builder::text(self.redirect_uri_var_name),
            gluesql::core::ast_builder::text(self.oauth_url),
            gluesql::core::ast_builder::text(self.scope),
        ]
    }

    /// Insert the LoginProvider into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table LoginProvider
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("login_providers")
            .insert()
            .columns("id, name, font_awesome_icon_id, color_id, client_id_var_name, redirect_uri_var_name, oauth_url, scope")
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
            .project("id, name, font_awesome_icon_id, color_id, client_id_var_name, redirect_uri_var_name, oauth_url, scope")
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
.set("font_awesome_icon_id", gluesql::core::ast_builder::num(self.font_awesome_icon_id))        
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
        let select_row = table("login_providers")
            .select()
            .project("id, name, font_awesome_icon_id, color_id, client_id_var_name, redirect_uri_var_name, oauth_url, scope")
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
            font_awesome_icon_id: match row.get("font_awesome_icon_id").unwrap() {
                gluesql::prelude::Value::I32(font_awesome_icon_id) => font_awesome_icon_id.clone(),
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
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct Notification {
    pub id: i32,
    pub user_id: i32,
    pub operation: String,
    pub table_name: String,
    pub record: String,
    pub read: bool,
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
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table Notification
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
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
        let select_row = table("notifications")
            .select()
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
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct Organization {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub country_id: i32,
    pub state_province: Option<String>,
    pub domain: String,
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
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table Organization
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
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
        let select_row = table("organizations")
            .select()
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
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct ProjectState {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub font_awesome_icon_id: i32,
    pub color_id: i32,
}
#[cfg(feature = "frontend")]
impl ProjectState {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::text(self.name),
            gluesql::core::ast_builder::text(self.description),
            gluesql::core::ast_builder::num(self.font_awesome_icon_id),
            gluesql::core::ast_builder::num(self.color_id),
        ]
    }

    /// Insert the ProjectState into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table ProjectState
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("project_states")
            .insert()
            .columns("id, name, description, font_awesome_icon_id, color_id")
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
            .project("id, name, description, font_awesome_icon_id, color_id")
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
.set("font_awesome_icon_id", gluesql::core::ast_builder::num(self.font_awesome_icon_id))        
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
        let select_row = table("project_states")
            .select()
            .project("id, name, description, font_awesome_icon_id, color_id")
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
            font_awesome_icon_id: match row.get("font_awesome_icon_id").unwrap() {
                gluesql::prelude::Value::I32(font_awesome_icon_id) => font_awesome_icon_id.clone(),
                _ => unreachable!("Expected I32")
            },
            color_id: match row.get("color_id").unwrap() {
                gluesql::prelude::Value::I32(color_id) => color_id.clone(),
                _ => unreachable!("Expected I32")
            },
        }
    }
}
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub public: bool,
    pub state_id: i32,
    pub parent_project_id: Option<i32>,
    pub budget: Option<f64>,
    pub expenses: Option<f64>,
    pub created_by: i32,
    pub created_at: NaiveDateTime,
    pub updated_by: i32,
    pub updated_at: NaiveDateTime,
    pub expected_end_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
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
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table Project
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("projects")
            .insert()
            .columns("id, name, description, public, state_id, parent_project_id, budget, expenses, created_by, created_at, updated_by, updated_at, expected_end_date, end_date")
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
            .project("id, name, description, public, state_id, parent_project_id, budget, expenses, created_by, created_at, updated_by, updated_at, expected_end_date, end_date")
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
        let select_row = table("projects")
            .select()
            .project("id, name, description, public, state_id, parent_project_id, budget, expenses, created_by, created_at, updated_by, updated_at, expected_end_date, end_date")
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
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all_by_updated_at<C>(
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("projects")
            .select()
            .project("id, name, description, public, state_id, parent_project_id, budget, expenses, created_by, created_at, updated_by, updated_at, expected_end_date, end_date")
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
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct ProjectsTeamsRole {
    pub table_id: i32,
    pub team_id: i32,
    pub role_id: i32,
    pub created_by: i32,
    pub created_at: NaiveDateTime,
    pub updated_by: i32,
    pub updated_at: NaiveDateTime,
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
            gluesql::core::ast_builder::num(self.updated_by),
            gluesql::core::ast_builder::timestamp(self.updated_at.to_string()),
        ]
    }

    /// Insert the ProjectsTeamsRole into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table ProjectsTeamsRole
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("projects_teams_roles")
            .insert()
            .columns("table_id, team_id, role_id, created_by, created_at, updated_by, updated_at")
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
    /// * `( table_id, team_id, role_id )` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        ( table_id, team_id, role_id ): ( i32, i32, i32 ),
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("projects_teams_roles")
            .select()
            .filter(col("table_id").eq(table_id.to_string()))
            .filter(col("team_id").eq(team_id.to_string()))
            .filter(col("role_id").eq(role_id.to_string()))
            .project("table_id, team_id, role_id, created_by, created_at, updated_by, updated_at")
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
    /// * `( table_id, team_id, role_id )` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        ( table_id, team_id, role_id ): ( i32, i32, i32 ),
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("projects_teams_roles")
            .delete()
            .filter(col("table_id").eq(table_id.to_string()))
            .filter(col("team_id").eq(team_id.to_string()))
            .filter(col("role_id").eq(role_id.to_string()))
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
        Self::delete_from_id(( self.table_id, self.team_id, self.role_id ), connection).await
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
    /// Get all ProjectsTeamsRole from the database.
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
        let select_row = table("projects_teams_roles")
            .select()
            .project("table_id, team_id, role_id, created_by, created_at, updated_by, updated_at")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    /// Get all ProjectsTeamsRole from the database ordered by the `updated_at` column.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all_by_updated_at<C>(
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("projects_teams_roles")
            .select()
            .project("table_id, team_id, role_id, created_by, created_at, updated_by, updated_at")
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
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct ProjectsUsersRole {
    pub table_id: i32,
    pub user_id: i32,
    pub role_id: i32,
    pub created_by: i32,
    pub created_at: Option<NaiveDateTime>,
    pub updated_by: i32,
    pub updated_at: Option<NaiveDateTime>,
}
#[cfg(feature = "frontend")]
impl ProjectsUsersRole {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.table_id),
            gluesql::core::ast_builder::num(self.user_id),
            gluesql::core::ast_builder::num(self.role_id),
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
        ]
    }

    /// Insert the ProjectsUsersRole into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table ProjectsUsersRole
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("projects_users_roles")
            .insert()
            .columns("table_id, user_id, role_id, created_by, created_at, updated_by, updated_at")
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
    /// * `( table_id, user_id, role_id )` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        ( table_id, user_id, role_id ): ( i32, i32, i32 ),
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("projects_users_roles")
            .select()
            .filter(col("table_id").eq(table_id.to_string()))
            .filter(col("user_id").eq(user_id.to_string()))
            .filter(col("role_id").eq(role_id.to_string()))
            .project("table_id, user_id, role_id, created_by, created_at, updated_by, updated_at")
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
    /// * `( table_id, user_id, role_id )` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        ( table_id, user_id, role_id ): ( i32, i32, i32 ),
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("projects_users_roles")
            .delete()
            .filter(col("table_id").eq(table_id.to_string()))
            .filter(col("user_id").eq(user_id.to_string()))
            .filter(col("role_id").eq(role_id.to_string()))
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
        Self::delete_from_id(( self.table_id, self.user_id, self.role_id ), connection).await
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
        let mut update_row = table("projects_users_roles")
            .update()        
.set("table_id", gluesql::core::ast_builder::num(self.table_id))        
.set("user_id", gluesql::core::ast_builder::num(self.user_id))        
.set("role_id", gluesql::core::ast_builder::num(self.role_id))        
.set("created_by", gluesql::core::ast_builder::num(self.created_by))        
.set("updated_by", gluesql::core::ast_builder::num(self.updated_by));
        if let Some(created_at) = self.created_at {
            update_row = update_row.set("created_at", gluesql::core::ast_builder::timestamp(created_at.to_string()));
        }
        if let Some(updated_at) = self.updated_at {
            update_row = update_row.set("updated_at", gluesql::core::ast_builder::timestamp(updated_at.to_string()));
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
    /// Get all ProjectsUsersRole from the database.
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
        let select_row = table("projects_users_roles")
            .select()
            .project("table_id, user_id, role_id, created_by, created_at, updated_by, updated_at")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    /// Get all ProjectsUsersRole from the database ordered by the `updated_at` column.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all_by_updated_at<C>(
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("projects_users_roles")
            .select()
            .project("table_id, user_id, role_id, created_by, created_at, updated_by, updated_at")
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
        }
    }
}
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct Role {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub font_awesome_icon_id: i32,
    pub color_id: i32,
}
#[cfg(feature = "frontend")]
impl Role {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::text(self.name),
            gluesql::core::ast_builder::text(self.description),
            gluesql::core::ast_builder::num(self.font_awesome_icon_id),
            gluesql::core::ast_builder::num(self.color_id),
        ]
    }

    /// Insert the Role into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table Role
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("roles")
            .insert()
            .columns("id, name, description, font_awesome_icon_id, color_id")
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
            .project("id, name, description, font_awesome_icon_id, color_id")
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
.set("font_awesome_icon_id", gluesql::core::ast_builder::num(self.font_awesome_icon_id))        
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
        let select_row = table("roles")
            .select()
            .project("id, name, description, font_awesome_icon_id, color_id")
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
            font_awesome_icon_id: match row.get("font_awesome_icon_id").unwrap() {
                gluesql::prelude::Value::I32(font_awesome_icon_id) => font_awesome_icon_id.clone(),
                _ => unreachable!("Expected I32")
            },
            color_id: match row.get("color_id").unwrap() {
                gluesql::prelude::Value::I32(color_id) => color_id.clone(),
                _ => unreachable!("Expected I32")
            },
        }
    }
}
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct SampleBioOttTaxonItem {
    pub created_by: i32,
    pub created_at: NaiveDateTime,
    pub updated_by: i32,
    pub updated_at: NaiveDateTime,
    pub sample_id: Uuid,
    pub taxon_id: i32,
}
#[cfg(feature = "frontend")]
impl SampleBioOttTaxonItem {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.created_by),
            gluesql::core::ast_builder::timestamp(self.created_at.to_string()),
            gluesql::core::ast_builder::num(self.updated_by),
            gluesql::core::ast_builder::timestamp(self.updated_at.to_string()),
            gluesql::core::ast_builder::uuid(self.sample_id.to_string()),
            gluesql::core::ast_builder::num(self.taxon_id),
        ]
    }

    /// Insert the SampleBioOttTaxonItem into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table SampleBioOttTaxonItem
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("sample_bio_ott_taxon_items")
            .insert()
            .columns("created_by, created_at, updated_by, updated_at, sample_id, taxon_id")
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
        ( sample_id, taxon_id ): ( Uuid, i32 ),
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("sample_bio_ott_taxon_items")
            .select()
            .filter(col("sample_id").eq(sample_id.to_string()))
            .filter(col("taxon_id").eq(taxon_id.to_string()))
            .project("created_by, created_at, updated_by, updated_at, sample_id, taxon_id")
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
        ( sample_id, taxon_id ): ( Uuid, i32 ),
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
.set("updated_by", gluesql::core::ast_builder::num(self.updated_by))        
.set("updated_at", gluesql::core::ast_builder::timestamp(self.updated_at.to_string()))        
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
        let select_row = table("sample_bio_ott_taxon_items")
            .select()
            .project("created_by, created_at, updated_by, updated_at, sample_id, taxon_id")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    /// Get all SampleBioOttTaxonItem from the database ordered by the `updated_at` column.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all_by_updated_at<C>(
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("sample_bio_ott_taxon_items")
            .select()
            .project("created_by, created_at, updated_by, updated_at, sample_id, taxon_id")
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
            sample_id: match row.get("sample_id").unwrap() {
                gluesql::prelude::Value::Uuid(sample_id) => Uuid::from_u128(*sample_id),
                _ => unreachable!("Expected Uuid"),
            },
            taxon_id: match row.get("taxon_id").unwrap() {
                gluesql::prelude::Value::I32(taxon_id) => taxon_id.clone(),
                _ => unreachable!("Expected I32")
            },
        }
    }
}
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct SampleState {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub font_awesome_icon_id: i32,
    pub color_id: i32,
}
#[cfg(feature = "frontend")]
impl SampleState {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::text(self.name),
            gluesql::core::ast_builder::text(self.description),
            gluesql::core::ast_builder::num(self.font_awesome_icon_id),
            gluesql::core::ast_builder::num(self.color_id),
        ]
    }

    /// Insert the SampleState into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table SampleState
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("sample_states")
            .insert()
            .columns("id, name, description, font_awesome_icon_id, color_id")
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
            .project("id, name, description, font_awesome_icon_id, color_id")
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
.set("font_awesome_icon_id", gluesql::core::ast_builder::num(self.font_awesome_icon_id))        
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
        let select_row = table("sample_states")
            .select()
            .project("id, name, description, font_awesome_icon_id, color_id")
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
            font_awesome_icon_id: match row.get("font_awesome_icon_id").unwrap() {
                gluesql::prelude::Value::I32(font_awesome_icon_id) => font_awesome_icon_id.clone(),
                _ => unreachable!("Expected I32")
            },
            color_id: match row.get("color_id").unwrap() {
                gluesql::prelude::Value::I32(color_id) => color_id.clone(),
                _ => unreachable!("Expected I32")
            },
        }
    }
}
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct SampledIndividualBioOttTaxonItem {
    pub created_by: i32,
    pub created_at: NaiveDateTime,
    pub updated_by: i32,
    pub updated_at: NaiveDateTime,
    pub sampled_individual_id: Uuid,
    pub taxon_id: i32,
}
#[cfg(feature = "frontend")]
impl SampledIndividualBioOttTaxonItem {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.created_by),
            gluesql::core::ast_builder::timestamp(self.created_at.to_string()),
            gluesql::core::ast_builder::num(self.updated_by),
            gluesql::core::ast_builder::timestamp(self.updated_at.to_string()),
            gluesql::core::ast_builder::uuid(self.sampled_individual_id.to_string()),
            gluesql::core::ast_builder::num(self.taxon_id),
        ]
    }

    /// Insert the SampledIndividualBioOttTaxonItem into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table SampledIndividualBioOttTaxonItem
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("sampled_individual_bio_ott_taxon_items")
            .insert()
            .columns("created_by, created_at, updated_by, updated_at, sampled_individual_id, taxon_id")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get SampledIndividualBioOttTaxonItem from the database by its ID.
    ///
    /// # Arguments
    /// * `( sampled_individual_id, taxon_id )` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        ( sampled_individual_id, taxon_id ): ( Uuid, i32 ),
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("sampled_individual_bio_ott_taxon_items")
            .select()
            .filter(col("sampled_individual_id").eq(sampled_individual_id.to_string()))
            .filter(col("taxon_id").eq(taxon_id.to_string()))
            .project("created_by, created_at, updated_by, updated_at, sampled_individual_id, taxon_id")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete SampledIndividualBioOttTaxonItem from the database.
    ///
    /// # Arguments
    /// * `( sampled_individual_id, taxon_id )` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        ( sampled_individual_id, taxon_id ): ( Uuid, i32 ),
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("sampled_individual_bio_ott_taxon_items")
            .delete()
            .filter(col("sampled_individual_id").eq(sampled_individual_id.to_string()))
            .filter(col("taxon_id").eq(taxon_id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of SampledIndividualBioOttTaxonItem from the database.
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
        Self::delete_from_id(( self.sampled_individual_id, self.taxon_id ), connection).await
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
        table("sampled_individual_bio_ott_taxon_items")
            .update()        
.set("created_by", gluesql::core::ast_builder::num(self.created_by))        
.set("created_at", gluesql::core::ast_builder::timestamp(self.created_at.to_string()))        
.set("updated_by", gluesql::core::ast_builder::num(self.updated_by))        
.set("updated_at", gluesql::core::ast_builder::timestamp(self.updated_at.to_string()))        
.set("sampled_individual_id", gluesql::core::ast_builder::uuid(self.sampled_individual_id.to_string()))        
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
    /// Get all SampledIndividualBioOttTaxonItem from the database.
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
        let select_row = table("sampled_individual_bio_ott_taxon_items")
            .select()
            .project("created_by, created_at, updated_by, updated_at, sampled_individual_id, taxon_id")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    /// Get all SampledIndividualBioOttTaxonItem from the database ordered by the `updated_at` column.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all_by_updated_at<C>(
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("sampled_individual_bio_ott_taxon_items")
            .select()
            .project("created_by, created_at, updated_by, updated_at, sampled_individual_id, taxon_id")
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
            sampled_individual_id: match row.get("sampled_individual_id").unwrap() {
                gluesql::prelude::Value::Uuid(sampled_individual_id) => Uuid::from_u128(*sampled_individual_id),
                _ => unreachable!("Expected Uuid"),
            },
            taxon_id: match row.get("taxon_id").unwrap() {
                gluesql::prelude::Value::I32(taxon_id) => taxon_id.clone(),
                _ => unreachable!("Expected I32")
            },
        }
    }
}
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct SampledIndividual {
    pub id: Uuid,
    pub created_by: i32,
    pub created_at: NaiveDateTime,
    pub updated_by: i32,
    pub updated_at: NaiveDateTime,
    pub tagged: bool,
}
#[cfg(feature = "frontend")]
impl SampledIndividual {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::uuid(self.id.to_string()),
            gluesql::core::ast_builder::num(self.created_by),
            gluesql::core::ast_builder::timestamp(self.created_at.to_string()),
            gluesql::core::ast_builder::num(self.updated_by),
            gluesql::core::ast_builder::timestamp(self.updated_at.to_string()),
            (self.tagged.into()),
        ]
    }

    /// Insert the SampledIndividual into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table SampledIndividual
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("sampled_individuals")
            .insert()
            .columns("id, created_by, created_at, updated_by, updated_at, tagged")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get SampledIndividual from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("sampled_individuals")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, created_by, created_at, updated_by, updated_at, tagged")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete SampledIndividual from the database.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("sampled_individuals")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of SampledIndividual from the database.
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
        table("sampled_individuals")
            .update()        
.set("id", gluesql::core::ast_builder::uuid(self.id.to_string()))        
.set("created_by", gluesql::core::ast_builder::num(self.created_by))        
.set("created_at", gluesql::core::ast_builder::timestamp(self.created_at.to_string()))        
.set("updated_by", gluesql::core::ast_builder::num(self.updated_by))        
.set("updated_at", gluesql::core::ast_builder::timestamp(self.updated_at.to_string()))        
.set("tagged", self.tagged)            .execute(connection)
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
    /// Get all SampledIndividual from the database.
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
        let select_row = table("sampled_individuals")
            .select()
            .project("id, created_by, created_at, updated_by, updated_at, tagged")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    /// Get all SampledIndividual from the database ordered by the `updated_at` column.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all_by_updated_at<C>(
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("sampled_individuals")
            .select()
            .project("id, created_by, created_at, updated_by, updated_at, tagged")
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
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
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
            tagged: match row.get("tagged").unwrap() {
                gluesql::prelude::Value::Bool(tagged) => tagged.clone(),
                _ => unreachable!("Expected Bool")
            },
        }
    }
}
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct SampledIndividualsTeamsRole {
    pub table_id: Uuid,
    pub team_id: i32,
    pub role_id: i32,
    pub created_by: i32,
    pub created_at: NaiveDateTime,
    pub updated_by: i32,
    pub updated_at: NaiveDateTime,
}
#[cfg(feature = "frontend")]
impl SampledIndividualsTeamsRole {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::uuid(self.table_id.to_string()),
            gluesql::core::ast_builder::num(self.team_id),
            gluesql::core::ast_builder::num(self.role_id),
            gluesql::core::ast_builder::num(self.created_by),
            gluesql::core::ast_builder::timestamp(self.created_at.to_string()),
            gluesql::core::ast_builder::num(self.updated_by),
            gluesql::core::ast_builder::timestamp(self.updated_at.to_string()),
        ]
    }

    /// Insert the SampledIndividualsTeamsRole into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table SampledIndividualsTeamsRole
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("sampled_individuals_teams_roles")
            .insert()
            .columns("table_id, team_id, role_id, created_by, created_at, updated_by, updated_at")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get SampledIndividualsTeamsRole from the database by its ID.
    ///
    /// # Arguments
    /// * `( table_id, team_id, role_id )` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        ( table_id, team_id, role_id ): ( Uuid, i32, i32 ),
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("sampled_individuals_teams_roles")
            .select()
            .filter(col("table_id").eq(table_id.to_string()))
            .filter(col("team_id").eq(team_id.to_string()))
            .filter(col("role_id").eq(role_id.to_string()))
            .project("table_id, team_id, role_id, created_by, created_at, updated_by, updated_at")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete SampledIndividualsTeamsRole from the database.
    ///
    /// # Arguments
    /// * `( table_id, team_id, role_id )` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        ( table_id, team_id, role_id ): ( Uuid, i32, i32 ),
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("sampled_individuals_teams_roles")
            .delete()
            .filter(col("table_id").eq(table_id.to_string()))
            .filter(col("team_id").eq(team_id.to_string()))
            .filter(col("role_id").eq(role_id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of SampledIndividualsTeamsRole from the database.
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
        Self::delete_from_id(( self.table_id, self.team_id, self.role_id ), connection).await
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
        table("sampled_individuals_teams_roles")
            .update()        
.set("table_id", gluesql::core::ast_builder::uuid(self.table_id.to_string()))        
.set("team_id", gluesql::core::ast_builder::num(self.team_id))        
.set("role_id", gluesql::core::ast_builder::num(self.role_id))        
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
    /// Get all SampledIndividualsTeamsRole from the database.
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
        let select_row = table("sampled_individuals_teams_roles")
            .select()
            .project("table_id, team_id, role_id, created_by, created_at, updated_by, updated_at")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    /// Get all SampledIndividualsTeamsRole from the database ordered by the `updated_at` column.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all_by_updated_at<C>(
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("sampled_individuals_teams_roles")
            .select()
            .project("table_id, team_id, role_id, created_by, created_at, updated_by, updated_at")
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
            table_id: match row.get("table_id").unwrap() {
                gluesql::prelude::Value::Uuid(table_id) => Uuid::from_u128(*table_id),
                _ => unreachable!("Expected Uuid"),
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
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct SampledIndividualsUsersRole {
    pub table_id: Uuid,
    pub user_id: i32,
    pub role_id: i32,
    pub created_by: i32,
    pub created_at: NaiveDateTime,
    pub updated_by: i32,
    pub updated_at: NaiveDateTime,
}
#[cfg(feature = "frontend")]
impl SampledIndividualsUsersRole {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::uuid(self.table_id.to_string()),
            gluesql::core::ast_builder::num(self.user_id),
            gluesql::core::ast_builder::num(self.role_id),
            gluesql::core::ast_builder::num(self.created_by),
            gluesql::core::ast_builder::timestamp(self.created_at.to_string()),
            gluesql::core::ast_builder::num(self.updated_by),
            gluesql::core::ast_builder::timestamp(self.updated_at.to_string()),
        ]
    }

    /// Insert the SampledIndividualsUsersRole into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table SampledIndividualsUsersRole
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("sampled_individuals_users_roles")
            .insert()
            .columns("table_id, user_id, role_id, created_by, created_at, updated_by, updated_at")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get SampledIndividualsUsersRole from the database by its ID.
    ///
    /// # Arguments
    /// * `( table_id, user_id, role_id )` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        ( table_id, user_id, role_id ): ( Uuid, i32, i32 ),
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("sampled_individuals_users_roles")
            .select()
            .filter(col("table_id").eq(table_id.to_string()))
            .filter(col("user_id").eq(user_id.to_string()))
            .filter(col("role_id").eq(role_id.to_string()))
            .project("table_id, user_id, role_id, created_by, created_at, updated_by, updated_at")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete SampledIndividualsUsersRole from the database.
    ///
    /// # Arguments
    /// * `( table_id, user_id, role_id )` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        ( table_id, user_id, role_id ): ( Uuid, i32, i32 ),
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("sampled_individuals_users_roles")
            .delete()
            .filter(col("table_id").eq(table_id.to_string()))
            .filter(col("user_id").eq(user_id.to_string()))
            .filter(col("role_id").eq(role_id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of SampledIndividualsUsersRole from the database.
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
        Self::delete_from_id(( self.table_id, self.user_id, self.role_id ), connection).await
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
        table("sampled_individuals_users_roles")
            .update()        
.set("table_id", gluesql::core::ast_builder::uuid(self.table_id.to_string()))        
.set("user_id", gluesql::core::ast_builder::num(self.user_id))        
.set("role_id", gluesql::core::ast_builder::num(self.role_id))        
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
    /// Get all SampledIndividualsUsersRole from the database.
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
        let select_row = table("sampled_individuals_users_roles")
            .select()
            .project("table_id, user_id, role_id, created_by, created_at, updated_by, updated_at")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    /// Get all SampledIndividualsUsersRole from the database ordered by the `updated_at` column.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all_by_updated_at<C>(
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("sampled_individuals_users_roles")
            .select()
            .project("table_id, user_id, role_id, created_by, created_at, updated_by, updated_at")
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
            table_id: match row.get("table_id").unwrap() {
                gluesql::prelude::Value::Uuid(table_id) => Uuid::from_u128(*table_id),
                _ => unreachable!("Expected Uuid"),
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
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct Sample {
    pub id: Uuid,
    pub created_by: i32,
    pub sampled_by: i32,
    pub created_at: NaiveDateTime,
    pub updated_by: i32,
    pub updated_at: NaiveDateTime,
    pub state: i32,
}
#[cfg(feature = "frontend")]
impl Sample {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::uuid(self.id.to_string()),
            gluesql::core::ast_builder::num(self.created_by),
            gluesql::core::ast_builder::num(self.sampled_by),
            gluesql::core::ast_builder::timestamp(self.created_at.to_string()),
            gluesql::core::ast_builder::num(self.updated_by),
            gluesql::core::ast_builder::timestamp(self.updated_at.to_string()),
            gluesql::core::ast_builder::num(self.state),
        ]
    }

    /// Insert the Sample into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table Sample
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("samples")
            .insert()
            .columns("id, created_by, sampled_by, created_at, updated_by, updated_at, state")
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
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("samples")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, created_by, sampled_by, created_at, updated_by, updated_at, state")
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
        id: Uuid,
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
        table("samples")
            .update()        
.set("id", gluesql::core::ast_builder::uuid(self.id.to_string()))        
.set("created_by", gluesql::core::ast_builder::num(self.created_by))        
.set("sampled_by", gluesql::core::ast_builder::num(self.sampled_by))        
.set("created_at", gluesql::core::ast_builder::timestamp(self.created_at.to_string()))        
.set("updated_by", gluesql::core::ast_builder::num(self.updated_by))        
.set("updated_at", gluesql::core::ast_builder::timestamp(self.updated_at.to_string()))        
.set("state", gluesql::core::ast_builder::num(self.state))            .execute(connection)
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
        let select_row = table("samples")
            .select()
            .project("id, created_by, sampled_by, created_at, updated_by, updated_at, state")
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
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all_by_updated_at<C>(
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("samples")
            .select()
            .project("id, created_by, sampled_by, created_at, updated_by, updated_at, state")
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
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
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
            state: match row.get("state").unwrap() {
                gluesql::prelude::Value::I32(state) => state.clone(),
                _ => unreachable!("Expected I32")
            },
        }
    }
}
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct SamplesTeamsRole {
    pub table_id: Uuid,
    pub team_id: i32,
    pub role_id: i32,
    pub created_by: i32,
    pub created_at: NaiveDateTime,
    pub updated_by: i32,
    pub updated_at: NaiveDateTime,
}
#[cfg(feature = "frontend")]
impl SamplesTeamsRole {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::uuid(self.table_id.to_string()),
            gluesql::core::ast_builder::num(self.team_id),
            gluesql::core::ast_builder::num(self.role_id),
            gluesql::core::ast_builder::num(self.created_by),
            gluesql::core::ast_builder::timestamp(self.created_at.to_string()),
            gluesql::core::ast_builder::num(self.updated_by),
            gluesql::core::ast_builder::timestamp(self.updated_at.to_string()),
        ]
    }

    /// Insert the SamplesTeamsRole into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table SamplesTeamsRole
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("samples_teams_roles")
            .insert()
            .columns("table_id, team_id, role_id, created_by, created_at, updated_by, updated_at")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get SamplesTeamsRole from the database by its ID.
    ///
    /// # Arguments
    /// * `( table_id, team_id, role_id )` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        ( table_id, team_id, role_id ): ( Uuid, i32, i32 ),
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("samples_teams_roles")
            .select()
            .filter(col("table_id").eq(table_id.to_string()))
            .filter(col("team_id").eq(team_id.to_string()))
            .filter(col("role_id").eq(role_id.to_string()))
            .project("table_id, team_id, role_id, created_by, created_at, updated_by, updated_at")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete SamplesTeamsRole from the database.
    ///
    /// # Arguments
    /// * `( table_id, team_id, role_id )` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        ( table_id, team_id, role_id ): ( Uuid, i32, i32 ),
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("samples_teams_roles")
            .delete()
            .filter(col("table_id").eq(table_id.to_string()))
            .filter(col("team_id").eq(team_id.to_string()))
            .filter(col("role_id").eq(role_id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of SamplesTeamsRole from the database.
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
        Self::delete_from_id(( self.table_id, self.team_id, self.role_id ), connection).await
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
        table("samples_teams_roles")
            .update()        
.set("table_id", gluesql::core::ast_builder::uuid(self.table_id.to_string()))        
.set("team_id", gluesql::core::ast_builder::num(self.team_id))        
.set("role_id", gluesql::core::ast_builder::num(self.role_id))        
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
    /// Get all SamplesTeamsRole from the database.
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
        let select_row = table("samples_teams_roles")
            .select()
            .project("table_id, team_id, role_id, created_by, created_at, updated_by, updated_at")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    /// Get all SamplesTeamsRole from the database ordered by the `updated_at` column.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all_by_updated_at<C>(
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("samples_teams_roles")
            .select()
            .project("table_id, team_id, role_id, created_by, created_at, updated_by, updated_at")
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
            table_id: match row.get("table_id").unwrap() {
                gluesql::prelude::Value::Uuid(table_id) => Uuid::from_u128(*table_id),
                _ => unreachable!("Expected Uuid"),
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
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct SamplesUsersRole {
    pub table_id: Uuid,
    pub user_id: i32,
    pub role_id: i32,
    pub created_by: i32,
    pub created_at: NaiveDateTime,
    pub updated_by: i32,
    pub updated_at: NaiveDateTime,
}
#[cfg(feature = "frontend")]
impl SamplesUsersRole {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::uuid(self.table_id.to_string()),
            gluesql::core::ast_builder::num(self.user_id),
            gluesql::core::ast_builder::num(self.role_id),
            gluesql::core::ast_builder::num(self.created_by),
            gluesql::core::ast_builder::timestamp(self.created_at.to_string()),
            gluesql::core::ast_builder::num(self.updated_by),
            gluesql::core::ast_builder::timestamp(self.updated_at.to_string()),
        ]
    }

    /// Insert the SamplesUsersRole into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table SamplesUsersRole
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("samples_users_roles")
            .insert()
            .columns("table_id, user_id, role_id, created_by, created_at, updated_by, updated_at")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get SamplesUsersRole from the database by its ID.
    ///
    /// # Arguments
    /// * `( table_id, user_id, role_id )` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        ( table_id, user_id, role_id ): ( Uuid, i32, i32 ),
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("samples_users_roles")
            .select()
            .filter(col("table_id").eq(table_id.to_string()))
            .filter(col("user_id").eq(user_id.to_string()))
            .filter(col("role_id").eq(role_id.to_string()))
            .project("table_id, user_id, role_id, created_by, created_at, updated_by, updated_at")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete SamplesUsersRole from the database.
    ///
    /// # Arguments
    /// * `( table_id, user_id, role_id )` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        ( table_id, user_id, role_id ): ( Uuid, i32, i32 ),
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("samples_users_roles")
            .delete()
            .filter(col("table_id").eq(table_id.to_string()))
            .filter(col("user_id").eq(user_id.to_string()))
            .filter(col("role_id").eq(role_id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of SamplesUsersRole from the database.
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
        Self::delete_from_id(( self.table_id, self.user_id, self.role_id ), connection).await
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
        table("samples_users_roles")
            .update()        
.set("table_id", gluesql::core::ast_builder::uuid(self.table_id.to_string()))        
.set("user_id", gluesql::core::ast_builder::num(self.user_id))        
.set("role_id", gluesql::core::ast_builder::num(self.role_id))        
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
    /// Get all SamplesUsersRole from the database.
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
        let select_row = table("samples_users_roles")
            .select()
            .project("table_id, user_id, role_id, created_by, created_at, updated_by, updated_at")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    /// Get all SamplesUsersRole from the database ordered by the `updated_at` column.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all_by_updated_at<C>(
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("samples_users_roles")
            .select()
            .project("table_id, user_id, role_id, created_by, created_at, updated_by, updated_at")
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
            table_id: match row.get("table_id").unwrap() {
                gluesql::prelude::Value::Uuid(table_id) => Uuid::from_u128(*table_id),
                _ => unreachable!("Expected Uuid"),
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
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct Spectra {
    pub id: i32,
    pub spectra_collection_id: i32,
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
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table Spectra
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
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
        let select_row = table("spectra")
            .select()
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
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct SpectraCollection {
    pub id: i32,
    pub sample_id: Uuid,
    pub created_by: i32,
    pub created_at: NaiveDateTime,
    pub updated_by: i32,
    pub updated_at: NaiveDateTime,
}
#[cfg(feature = "frontend")]
impl SpectraCollection {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::uuid(self.sample_id.to_string()),
            gluesql::core::ast_builder::num(self.created_by),
            gluesql::core::ast_builder::timestamp(self.created_at.to_string()),
            gluesql::core::ast_builder::num(self.updated_by),
            gluesql::core::ast_builder::timestamp(self.updated_at.to_string()),
        ]
    }

    /// Insert the SpectraCollection into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table SpectraCollection
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("spectra_collections")
            .insert()
            .columns("id, sample_id, created_by, created_at, updated_by, updated_at")
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
            .project("id, sample_id, created_by, created_at, updated_by, updated_at")
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
        table("spectra_collections")
            .update()        
.set("id", gluesql::core::ast_builder::num(self.id))        
.set("sample_id", gluesql::core::ast_builder::uuid(self.sample_id.to_string()))        
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
    /// Get all SpectraCollection from the database.
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
        let select_row = table("spectra_collections")
            .select()
            .project("id, sample_id, created_by, created_at, updated_by, updated_at")
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
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all_by_updated_at<C>(
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("spectra_collections")
            .select()
            .project("id, sample_id, created_by, created_at, updated_by, updated_at")
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
            sample_id: match row.get("sample_id").unwrap() {
                gluesql::prelude::Value::Uuid(sample_id) => Uuid::from_u128(*sample_id),
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
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct SpectraCollectionsTeamsRole {
    pub table_id: i32,
    pub team_id: i32,
    pub role_id: i32,
    pub created_by: i32,
    pub created_at: NaiveDateTime,
    pub updated_by: i32,
    pub updated_at: NaiveDateTime,
}
#[cfg(feature = "frontend")]
impl SpectraCollectionsTeamsRole {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.table_id),
            gluesql::core::ast_builder::num(self.team_id),
            gluesql::core::ast_builder::num(self.role_id),
            gluesql::core::ast_builder::num(self.created_by),
            gluesql::core::ast_builder::timestamp(self.created_at.to_string()),
            gluesql::core::ast_builder::num(self.updated_by),
            gluesql::core::ast_builder::timestamp(self.updated_at.to_string()),
        ]
    }

    /// Insert the SpectraCollectionsTeamsRole into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table SpectraCollectionsTeamsRole
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("spectra_collections_teams_roles")
            .insert()
            .columns("table_id, team_id, role_id, created_by, created_at, updated_by, updated_at")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get SpectraCollectionsTeamsRole from the database by its ID.
    ///
    /// # Arguments
    /// * `( table_id, team_id, role_id )` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        ( table_id, team_id, role_id ): ( i32, i32, i32 ),
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("spectra_collections_teams_roles")
            .select()
            .filter(col("table_id").eq(table_id.to_string()))
            .filter(col("team_id").eq(team_id.to_string()))
            .filter(col("role_id").eq(role_id.to_string()))
            .project("table_id, team_id, role_id, created_by, created_at, updated_by, updated_at")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete SpectraCollectionsTeamsRole from the database.
    ///
    /// # Arguments
    /// * `( table_id, team_id, role_id )` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        ( table_id, team_id, role_id ): ( i32, i32, i32 ),
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("spectra_collections_teams_roles")
            .delete()
            .filter(col("table_id").eq(table_id.to_string()))
            .filter(col("team_id").eq(team_id.to_string()))
            .filter(col("role_id").eq(role_id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of SpectraCollectionsTeamsRole from the database.
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
        Self::delete_from_id(( self.table_id, self.team_id, self.role_id ), connection).await
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
        table("spectra_collections_teams_roles")
            .update()        
.set("table_id", gluesql::core::ast_builder::num(self.table_id))        
.set("team_id", gluesql::core::ast_builder::num(self.team_id))        
.set("role_id", gluesql::core::ast_builder::num(self.role_id))        
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
    /// Get all SpectraCollectionsTeamsRole from the database.
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
        let select_row = table("spectra_collections_teams_roles")
            .select()
            .project("table_id, team_id, role_id, created_by, created_at, updated_by, updated_at")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    /// Get all SpectraCollectionsTeamsRole from the database ordered by the `updated_at` column.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all_by_updated_at<C>(
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("spectra_collections_teams_roles")
            .select()
            .project("table_id, team_id, role_id, created_by, created_at, updated_by, updated_at")
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
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct SpectraCollectionsUsersRole {
    pub table_id: i32,
    pub user_id: i32,
    pub role_id: i32,
    pub created_by: i32,
    pub created_at: NaiveDateTime,
    pub updated_by: i32,
    pub updated_at: NaiveDateTime,
}
#[cfg(feature = "frontend")]
impl SpectraCollectionsUsersRole {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.table_id),
            gluesql::core::ast_builder::num(self.user_id),
            gluesql::core::ast_builder::num(self.role_id),
            gluesql::core::ast_builder::num(self.created_by),
            gluesql::core::ast_builder::timestamp(self.created_at.to_string()),
            gluesql::core::ast_builder::num(self.updated_by),
            gluesql::core::ast_builder::timestamp(self.updated_at.to_string()),
        ]
    }

    /// Insert the SpectraCollectionsUsersRole into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table SpectraCollectionsUsersRole
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("spectra_collections_users_roles")
            .insert()
            .columns("table_id, user_id, role_id, created_by, created_at, updated_by, updated_at")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get SpectraCollectionsUsersRole from the database by its ID.
    ///
    /// # Arguments
    /// * `( table_id, user_id, role_id )` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        ( table_id, user_id, role_id ): ( i32, i32, i32 ),
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("spectra_collections_users_roles")
            .select()
            .filter(col("table_id").eq(table_id.to_string()))
            .filter(col("user_id").eq(user_id.to_string()))
            .filter(col("role_id").eq(role_id.to_string()))
            .project("table_id, user_id, role_id, created_by, created_at, updated_by, updated_at")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete SpectraCollectionsUsersRole from the database.
    ///
    /// # Arguments
    /// * `( table_id, user_id, role_id )` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        ( table_id, user_id, role_id ): ( i32, i32, i32 ),
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("spectra_collections_users_roles")
            .delete()
            .filter(col("table_id").eq(table_id.to_string()))
            .filter(col("user_id").eq(user_id.to_string()))
            .filter(col("role_id").eq(role_id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of SpectraCollectionsUsersRole from the database.
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
        Self::delete_from_id(( self.table_id, self.user_id, self.role_id ), connection).await
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
        table("spectra_collections_users_roles")
            .update()        
.set("table_id", gluesql::core::ast_builder::num(self.table_id))        
.set("user_id", gluesql::core::ast_builder::num(self.user_id))        
.set("role_id", gluesql::core::ast_builder::num(self.role_id))        
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
    /// Get all SpectraCollectionsUsersRole from the database.
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
        let select_row = table("spectra_collections_users_roles")
            .select()
            .project("table_id, user_id, role_id, created_by, created_at, updated_by, updated_at")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    /// Get all SpectraCollectionsUsersRole from the database ordered by the `updated_at` column.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all_by_updated_at<C>(
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("spectra_collections_users_roles")
            .select()
            .project("table_id, user_id, role_id, created_by, created_at, updated_by, updated_at")
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
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct TeamState {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub font_awesome_icon_id: i32,
    pub color_id: i32,
}
#[cfg(feature = "frontend")]
impl TeamState {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::text(self.name),
            gluesql::core::ast_builder::text(self.description),
            gluesql::core::ast_builder::num(self.font_awesome_icon_id),
            gluesql::core::ast_builder::num(self.color_id),
        ]
    }

    /// Insert the TeamState into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table TeamState
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("team_states")
            .insert()
            .columns("id, name, description, font_awesome_icon_id, color_id")
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
            .project("id, name, description, font_awesome_icon_id, color_id")
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
.set("font_awesome_icon_id", gluesql::core::ast_builder::num(self.font_awesome_icon_id))        
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
        let select_row = table("team_states")
            .select()
            .project("id, name, description, font_awesome_icon_id, color_id")
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
            font_awesome_icon_id: match row.get("font_awesome_icon_id").unwrap() {
                gluesql::prelude::Value::I32(font_awesome_icon_id) => font_awesome_icon_id.clone(),
                _ => unreachable!("Expected I32")
            },
            color_id: match row.get("color_id").unwrap() {
                gluesql::prelude::Value::I32(color_id) => color_id.clone(),
                _ => unreachable!("Expected I32")
            },
        }
    }
}
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct Team {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub parent_team_id: Option<i32>,
    pub created_by: i32,
    pub created_at: NaiveDateTime,
    pub updated_by: i32,
    pub updated_at: NaiveDateTime,
}
#[cfg(feature = "frontend")]
impl Team {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::text(self.name),
            gluesql::core::ast_builder::text(self.description),
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
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table Team
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("teams")
            .insert()
            .columns("id, name, description, parent_team_id, created_by, created_at, updated_by, updated_at")
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
            .project("id, name, description, parent_team_id, created_by, created_at, updated_by, updated_at")
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
        let select_row = table("teams")
            .select()
            .project("id, name, description, parent_team_id, created_by, created_at, updated_by, updated_at")
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
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all_by_updated_at<C>(
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("teams")
            .select()
            .project("id, name, description, parent_team_id, created_by, created_at, updated_by, updated_at")
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
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct TeamsUsersRole {
    pub table_id: i32,
    pub user_id: i32,
    pub role_id: i32,
    pub created_by: i32,
    pub created_at: NaiveDateTime,
    pub updated_by: i32,
    pub updated_at: NaiveDateTime,
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
            gluesql::core::ast_builder::num(self.updated_by),
            gluesql::core::ast_builder::timestamp(self.updated_at.to_string()),
        ]
    }

    /// Insert the TeamsUsersRole into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table TeamsUsersRole
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("teams_users_roles")
            .insert()
            .columns("table_id, user_id, role_id, created_by, created_at, updated_by, updated_at")
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
    /// * `( table_id, user_id, role_id )` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        ( table_id, user_id, role_id ): ( i32, i32, i32 ),
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("teams_users_roles")
            .select()
            .filter(col("table_id").eq(table_id.to_string()))
            .filter(col("user_id").eq(user_id.to_string()))
            .filter(col("role_id").eq(role_id.to_string()))
            .project("table_id, user_id, role_id, created_by, created_at, updated_by, updated_at")
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
    /// * `( table_id, user_id, role_id )` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        ( table_id, user_id, role_id ): ( i32, i32, i32 ),
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("teams_users_roles")
            .delete()
            .filter(col("table_id").eq(table_id.to_string()))
            .filter(col("user_id").eq(user_id.to_string()))
            .filter(col("role_id").eq(role_id.to_string()))
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
        Self::delete_from_id(( self.table_id, self.user_id, self.role_id ), connection).await
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
    /// Get all TeamsUsersRole from the database.
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
        let select_row = table("teams_users_roles")
            .select()
            .project("table_id, user_id, role_id, created_by, created_at, updated_by, updated_at")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    /// Get all TeamsUsersRole from the database ordered by the `updated_at` column.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all_by_updated_at<C>(
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("teams_users_roles")
            .select()
            .project("table_id, user_id, role_id, created_by, created_at, updated_by, updated_at")
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
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct Unit {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub symbol: String,
}
#[cfg(feature = "frontend")]
impl Unit {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::text(self.name),
            gluesql::core::ast_builder::text(self.description),
            gluesql::core::ast_builder::text(self.symbol),
        ]
    }

    /// Insert the Unit into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table Unit
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("units")
            .insert()
            .columns("id, name, description, symbol")
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
            .project("id, name, description, symbol")
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
.set("description", gluesql::core::ast_builder::text(self.description))        
.set("symbol", gluesql::core::ast_builder::text(self.symbol))            .execute(connection)
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
        let select_row = table("units")
            .select()
            .project("id, name, description, symbol")
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
            symbol: match row.get("symbol").unwrap() {
                gluesql::prelude::Value::Str(symbol) => symbol.clone(),
                _ => unreachable!("Expected Str")
            },
        }
    }
}
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct UserEmail {
    pub id: i32,
    pub email: String,
    pub created_by: i32,
    pub created_at: NaiveDateTime,
    pub login_provider_id: i32,
    pub primary_email: bool,
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
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table UserEmail
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
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
        let select_row = table("user_emails")
            .select()
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
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub profile_picture: Vec<u8>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
#[cfg(feature = "frontend")]
impl User {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::text(self.first_name),
            match self.middle_name {
                Some(middle_name) => gluesql::core::ast_builder::text(middle_name),
                None => gluesql::core::ast_builder::null(),
            },
            gluesql::core::ast_builder::text(self.last_name),
            gluesql::core::ast_builder::bytea(self.profile_picture),
            gluesql::core::ast_builder::timestamp(self.created_at.to_string()),
            gluesql::core::ast_builder::timestamp(self.updated_at.to_string()),
        ]
    }

    /// Insert the User into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table User
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("users")
            .insert()
            .columns("id, first_name, middle_name, last_name, profile_picture, created_at, updated_at")
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
            .project("id, first_name, middle_name, last_name, profile_picture, created_at, updated_at")
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
.set("profile_picture", gluesql::core::ast_builder::bytea(self.profile_picture))        
.set("created_at", gluesql::core::ast_builder::timestamp(self.created_at.to_string()))        
.set("updated_at", gluesql::core::ast_builder::timestamp(self.updated_at.to_string()));
        if let Some(middle_name) = self.middle_name {
            update_row = update_row.set("middle_name", gluesql::core::ast_builder::text(middle_name));
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
        let select_row = table("users")
            .select()
            .project("id, first_name, middle_name, last_name, profile_picture, created_at, updated_at")
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
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    ///
    pub async fn all_by_updated_at<C>(
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("users")
            .select()
            .project("id, first_name, middle_name, last_name, profile_picture, created_at, updated_at")
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
            profile_picture: match row.get("profile_picture").unwrap() {
                gluesql::prelude::Value::Bytea(profile_picture) => profile_picture.clone(),
                _ => unreachable!("Expected Bytea")
            },
            created_at: match row.get("created_at").unwrap() {
                gluesql::prelude::Value::Timestamp(created_at) => created_at.clone(),
                _ => unreachable!("Expected Timestamp")
            },
            updated_at: match row.get("updated_at").unwrap() {
                gluesql::prelude::Value::Timestamp(updated_at) => updated_at.clone(),
                _ => unreachable!("Expected Timestamp")
            },
        }
    }
}
