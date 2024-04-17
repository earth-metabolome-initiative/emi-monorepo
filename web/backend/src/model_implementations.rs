use diesel::connection::SimpleConnection;
use crate::models::*;
use crate::schema::*;
use crate::views::DocumentsView;
use crate::DieselConn;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::r2d2::PooledConnection;
use email_address::*;
use image::DynamicImage;
use image::ImageFormat;
use uuid::Uuid;
use validator::Validate;
use web_common::api::ApiError;
use web_common::custom_validators::Image;
use web_common::custom_validators::ImageSize;
use web_common::database::updates::CompleteProfile;

impl DocumentFormat {
    pub fn from_extension(
        extension: &str,
        conn: &mut PooledConnection<ConnectionManager<diesel::PgConnection>>,
    ) -> Result<DocumentFormat, diesel::result::Error> {
        use crate::schema::describables::dsl::*;
        use crate::schema::document_formats::dsl::*;
        // The extension of the format is stored as the name of the describable.
        document_formats
            .inner_join(describables)
            .filter(name.eq(extension))
            .select(DocumentFormat::as_select())
            .first::<DocumentFormat>(conn)
    }
}

impl Document {
    pub fn from_path(
        path: &str,
        conn: &mut PooledConnection<ConnectionManager<diesel::PgConnection>>,
    ) -> Result<Document, diesel::result::Error> {
        documents::dsl::documents
            .filter(documents::dsl::path.eq(path))
            .first::<Document>(conn)
    }
}

#[derive(Queryable, Insertable, Debug)]
#[diesel(table_name = user_emails)]
pub(crate) struct NewUserEmail {
    email: String,
    user_id: Uuid,
    login_provider_id: Uuid,
}

impl NewUserEmail {
    pub(crate) fn new(email: &str, user_id: Uuid, login_provider_id: Uuid) -> NewUserEmail {
        assert!(!email.is_empty());
        assert!(EmailAddress::is_valid(email));

        NewUserEmail {
            email: email.to_string(),
            user_id,
            login_provider_id,
        }
    }

    pub fn insert(
        &self,
        conn: &mut PooledConnection<ConnectionManager<diesel::PgConnection>>,
    ) -> Result<UserEmail, diesel::result::Error> {
        use crate::schema::user_emails::dsl::*;
        let result = diesel::insert_into(user_emails).values(self).execute(conn);
        match result {
            Ok(_) => user_emails
                .filter(email.eq(&self.email))
                .filter(user_id.eq(self.user_id))
                .filter(login_provider_id.eq(self.login_provider_id))
                .first::<UserEmail>(conn),
            Err(e) => Err(e),
        }
    }
}

#[derive(Queryable, Insertable, Debug)]
#[diesel(table_name = primary_user_emails)]
pub(crate) struct NewPrimaryUserEmail {
    id: Uuid,
}

impl NewPrimaryUserEmail {
    pub(crate) fn new(id: Uuid) -> NewPrimaryUserEmail {
        NewPrimaryUserEmail { id }
    }

    pub(crate) fn insert(
        &self,
        conn: &mut PooledConnection<ConnectionManager<diesel::PgConnection>>,
    ) -> QueryResult<usize> {
        use crate::schema::primary_user_emails::dsl::*;
        diesel::insert_into(primary_user_emails)
            .values(self)
            .execute(conn)
    }
}

impl UserEmail {
    pub(crate) fn provider_id(&self) -> Uuid {
        self.login_provider_id
    }
}

impl LoginProvider {
    pub fn from_provider_name(
        provider_name: &str,
        pool: &Pool<ConnectionManager<PgConnection>>,
    ) -> Result<LoginProvider, String> {
        use crate::schema::login_providers::dsl::*;
        let mut conn = pool.get().unwrap();
        let provider = login_providers
            .filter(name.eq(provider_name))
            .first::<LoginProvider>(&mut conn);
        match provider {
            Ok(provider) => Ok(provider),
            Err(_) => Err(format!("No provider with name {} found", provider_name)),
        }
    }
}

#[derive(Queryable, Insertable, Debug, Default)]
#[diesel(table_name = users)]
pub(crate) struct NewUser {
    first_name: Option<String>,
    middle_name: Option<String>,
    last_name: Option<String>,
}

impl User {
    /// Returns the UserMail associated to the provided email if it is associated to the current user.
    ///
    /// # Arguments
    /// * `user_email` - The email of the user.
    /// * `conn` - The database connection pool.
    pub fn get_user_email_from_email(
        &self,
        user_email: &str,
        conn: &mut PooledConnection<ConnectionManager<diesel::PgConnection>>,
    ) -> QueryResult<UserEmail> {
        use crate::schema::user_emails::dsl::*;
        user_emails
            .filter(email.eq(user_email))
            .filter(user_id.eq(self.id))
            .first::<UserEmail>(conn)
    }

    /// Returns the user's id.
    pub fn id(&self) -> Uuid {
        self.id
    }

    /// Inserts the user new profile and thumbnail pictures in the database.
    ///
    /// # Arguments
    /// * `profile_picture` - The profile picture to insert.
    /// * `thumbnail` - The thumbnail to insert.
    /// * `conn` - The database connection pool.
    pub fn insert_profile_pictures(
        &self,
        profile_picture: DynamicImage,
        conn: &mut PooledConnection<ConnectionManager<diesel::PgConnection>>,
    ) -> Result<(), ApiError> {
        // First, we create the thumbnail.
        let thumbnail =
            profile_picture.thumbnail(ImageSize::Thumbnail.width(), ImageSize::Thumbnail.height());

        let png_format = DocumentFormat::from_extension("png", conn)?;

        conn.transaction::<_, ApiError, _>(|conn| {
            // First, we create the document for the profile picture.
            let profile_picture_document = NewDocument::new(
                self.standard_profile_picture_path(),
                png_format.id,
                profile_picture.as_bytes().len() as i32,
            );
            let new_editable = NewEditable::new(self.id);
            let profile_picture_document = profile_picture_document.insert(
                conn,
                &new_editable,
                NewDescribable::new("Profile Picture", None),
            )?;
            // Similarly, we create the document for the thumbnail.
            let thumbnail_document = NewDocument::new(
                self.thumbnail_path(),
                png_format.id,
                thumbnail.as_bytes().len() as i32,
            );
            let thumbnail_document = thumbnail_document.insert(
                conn,
                &new_editable,
                NewDescribable::new("Profile Picture Thumbnail", None),
            )?;
            // We attempt to save the profile picture and thumbnail
            let profile_picture_path =
                DocumentsView::get( profile_picture_document.id, conn,)?.internal_path();
            profile_picture.save_with_format(profile_picture_path, ImageFormat::Png)?;
            let thumbnail_path = DocumentsView::get(thumbnail_document.id, conn)?.internal_path();
            thumbnail.save_with_format(thumbnail_path, ImageFormat::Png)?;
            Ok(())
        })
    }

    pub fn thumbnail_path(&self) -> String {
        web_common::database::views::PublicUser::profile_picture_path(
            self.id,
            &ImageSize::Thumbnail,
        )
    }

    pub fn standard_profile_picture_path(&self) -> String {
        web_common::database::views::PublicUser::profile_picture_path(self.id, &ImageSize::Standard)
    }

    pub fn has_profile_picture(&self, conn: &mut crate::DieselConn) -> bool {
        // In order to determine whether a user has a profile picture, we need to check whether
        // the user is the author, in the field created_by from the editables table, of any
        // document from the documents table as determined by the path column.
        let profile_picture_path = self.standard_profile_picture_path();
        use crate::schema::documents::dsl::*;
        use crate::schema::editables::dsl::*;
        editables
            .inner_join(documents)
            .filter(created_by.eq(self.id))
            .filter(path.eq(profile_picture_path))
            .first::<(Editable, Document)>(conn)
            .is_ok()
    }

    /// Method to update a user's name.
    ///
    /// # Arguments
    /// * `conn` - A connection to the database.
    /// * `profile` - The data to use to complete the profile.
    pub fn update_profile(
        &self,
        conn: &mut DieselConn,
        new_profile: CompleteProfile,
    ) -> Result<(), ApiError> {
        new_profile.validate()?;

        let picture: Image = new_profile.picture.into();
        let squared_profile_picture = picture.to_face_square().map_err(|errors| {
            log::error!("Failed to square profile picture: {}", errors.join(", "));
            ApiError::internal_server_error()
        })?;

        conn.batch_execute("SET TRANSACTION ISOLATION LEVEL SERIALIZABLE;")
            .expect("Failed to set transaction isolation level");

        // We need to execute multiple queries in a single transaction so to
        // avoid that the user is left with a profile picture but no name or vice versa.
        conn.transaction::<_, ApiError, _>(|conn| {
            use crate::schema::users::dsl::*;
            diesel::update(users.filter(id.eq(self.id)))
                .set((
                    first_name.eq(new_profile.first_name.to_string()),
                    middle_name.eq(new_profile.middle_name.map(|s| s.to_string())),
                    last_name.eq(new_profile.last_name.to_string()),
                ))
                .execute(conn)?;

            self.insert_profile_pictures(squared_profile_picture, conn)?;
            Ok(())
        })
    }
}

impl LoginProvider {
    /// Returns list of available login providers.
    ///
    /// # Arguments
    /// * `pool` - The database connection pool.
    pub fn get_all(
        pool: &Pool<ConnectionManager<PgConnection>>,
    ) -> Result<Vec<LoginProvider>, String> {
        use crate::schema::login_providers::dsl::*;
        let mut conn = pool.get().unwrap();
        let providers = login_providers.load::<LoginProvider>(&mut conn);
        match providers {
            Ok(providers) => Ok(providers),
            Err(_) => Err("Failed to retrieve login providers".to_string()),
        }
    }
}

#[derive(Queryable, Insertable, Debug)]
#[diesel(table_name = editables)]
pub struct NewEditable {
    pub created_by: Uuid,
}

impl NewEditable {
    pub fn new(created_by: Uuid) -> NewEditable {
        NewEditable { created_by }
    }

    /// Insert the editable into the database.
    pub fn insert(
        &self,
        conn: &mut PooledConnection<ConnectionManager<diesel::PgConnection>>,
    ) -> Result<Editable, diesel::result::Error> {
        diesel::insert_into(editables::table)
            .values(self)
            .get_result::<Editable>(conn)
    }
}

impl Describable {
    pub fn insert(
        &self,
        conn: &mut PooledConnection<ConnectionManager<diesel::PgConnection>>,
    ) -> Result<Describable, diesel::result::Error> {
        diesel::insert_into(describables::table)
            .values(self)
            .get_result::<Describable>(conn)
    }
}

#[derive(Queryable, Insertable, Debug, Clone)]
#[diesel(table_name = describables)]
pub struct NewDescribable {
    pub name: String,
    pub description: Option<String>,
}

impl NewDescribable {
    pub fn new(name: &str, description: Option<&str>) -> NewDescribable {
        NewDescribable {
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
        }
    }

    pub fn into_describable(self, editable: Editable) -> Describable {
        Describable {
            id: editable.id,
            name: self.name,
            description: self.description,
        }
    }
}

#[derive(Debug)]
pub struct NewDocument {
    pub path: String,
    pub format_id: Uuid,
    pub bytes: i32,
}

impl NewDocument {
    pub fn new(path: String, format_id: Uuid, bytes: i32) -> NewDocument {
        NewDocument {
            path,
            format_id,
            bytes,
        }
    }

    fn into_document(self, editable: Editable) -> Document {
        Document {
            id: editable.id,
            path: self.path,
            format_id: self.format_id,
            bytes: self.bytes,
        }
    }

    /// Insert the document into the database.
    pub fn insert(
        self,
        conn: &mut PooledConnection<ConnectionManager<diesel::PgConnection>>,
        new_editable: &NewEditable,
        new_describable: NewDescribable,
    ) -> Result<Document, diesel::result::Error> {
        conn.transaction::<_, diesel::result::Error, _>(|conn| {
            let editable = diesel::insert_into(editables::table)
                .values(new_editable)
                .get_result::<Editable>(conn)?;

            let new_document = self.into_document(editable.clone());
            let new_describable = new_describable.into_describable(editable);

            // We insert the description of the document.
            new_describable.insert(conn)?;

            diesel::insert_into(documents::table)
                .values(&new_document)
                .get_result::<Document>(conn)
        })
    }
}
