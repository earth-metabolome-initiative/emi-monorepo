use crate::models::*;
use crate::nested_models::NestedDocument;
use crate::DieselConn;
use diesel::connection::SimpleConnection;
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

impl NestedDocument {
    pub fn internal_path(&self) -> String {
        format!(
            "{}/{}.{}",
            std::env::var("DOCUMENTS_DIRECTORY").unwrap(),
            self.inner.id,
            self.format.extension
        )
    }
}

#[derive(Queryable, Insertable, Debug)]
#[diesel(table_name = crate::schema::user_emails)]
pub(crate) struct NewUserEmail<'a> {
    email: &'a str,
    user_id: i32,
    login_provider_id: i32,
}

impl<'a> NewUserEmail<'a> {
    pub(crate) fn new(email: &str, user_id: i32, login_provider_id: i32) -> NewUserEmail<'_> {
        assert!(!email.is_empty());
        assert!(EmailAddress::is_valid(email));

        NewUserEmail {
            email,
            user_id,
            login_provider_id,
        }
    }

    pub fn insert(
        &self,
        conn: &mut PooledConnection<ConnectionManager<diesel::PgConnection>>,
    ) -> Result<UserEmail, diesel::result::Error> {
        use crate::schema::user_emails;
        let result = diesel::insert_into(user_emails::dsl::user_emails)
            .values(self)
            .execute(conn);
        match result {
            Ok(_) => user_emails::dsl::user_emails
                .filter(user_emails::dsl::email.eq(&self.email))
                .filter(user_emails::dsl::user_id.eq(self.user_id))
                .filter(user_emails::dsl::login_provider_id.eq(self.login_provider_id))
                .first::<UserEmail>(conn),
            Err(e) => Err(e),
        }
    }
}

#[derive(Queryable, Insertable, Debug)]
#[diesel(table_name = crate::schema::primary_user_emails)]
pub(crate) struct NewPrimaryUserEmail {
    id: i32,
}

impl NewPrimaryUserEmail {
    pub(crate) fn new(id: i32) -> NewPrimaryUserEmail {
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
    pub(crate) fn provider_id(&self) -> i32 {
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
#[diesel(table_name = crate::schema::users)]
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
        use crate::schema::user_emails;
        user_emails::dsl::user_emails
            .filter(user_emails::dsl::email.eq(user_email))
            .filter(user_emails::dsl::user_id.eq(self.id))
            .first::<UserEmail>(conn)
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
            let profile_picture_document = Document::new(
                None,
                self.id,
                self.standard_profile_picture_path(),
                png_format.id,
                profile_picture.as_bytes().len() as i32,
            )
            .insert(conn)?;
            // Similarly, we create the document for the thumbnail.
            let thumbnail_document = Document::new(
                None,
                self.id,
                self.thumbnail_path(),
                png_format.id,
                thumbnail.as_bytes().len() as i32,
            )
            .insert(conn)?;
            // We attempt to save the profile picture and thumbnail
            let profile_picture_path =
                NestedDocument::get(profile_picture_document.id, conn)?.internal_path();
            profile_picture
                .save_with_format(&profile_picture_path, ImageFormat::Png)
                .map_err(|err| {
                    log::error!(
                        "Failed to save profile picture: {}, {}",
                        err,
                        profile_picture_path
                    );
                    err
                })?;
            let thumbnail_path = NestedDocument::get(thumbnail_document.id, conn)?.internal_path();
            thumbnail
                .save_with_format(&thumbnail_path, ImageFormat::Png)
                .map_err(|err| {
                    log::error!("Failed to save thumbnail: {}, {}", err, thumbnail_path);
                    err
                })?;
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

impl Document {
    pub fn new(id: Option<Uuid>, author_id: i32, path: String, format_id: i32, bytes: i32) -> Self {
        Document {
            id: id.unwrap_or_else(Uuid::new_v4),
            author_id,
            path,
            format_id,
            bytes,
        }
    }

    /// Insert the document into the database.
    pub fn insert(
        self,
        conn: &mut PooledConnection<ConnectionManager<diesel::PgConnection>>,
    ) -> Result<Document, diesel::result::Error> {
        conn.transaction::<_, diesel::result::Error, _>(|conn| {
            use crate::schema::documents;
            diesel::insert_into(documents::table)
                .values(&self)
                .get_result::<Document>(conn)
        })
    }
}
