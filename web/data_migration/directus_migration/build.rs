//! Builder which executes the webcodegen on the 'directus' database
//! in order to generate the structs used throught the 'directus_migration' crate.

use webcodegen::Table;
use diesel::connection::SimpleConnection;
use diesel::pg::PgConnection;
use diesel::{Connection, RunQueryDsl};
use std::io::Write;
use std::env;
use std::fs;
use std::path::Path;

use webcodegen::errors::WebCodeGenError;
use webcodegen::*;

use config::{Config, File};

fn establish_connection_to_postgres() -> PgConnection {
	// Load the configuration
    let settings = Config::builder()
        .add_source(File::with_name("config.ini"))
        .build()
        .expect("Failed to load configuration file");

    // Extract the database settings
    let database_name: String = settings.get("database.name").expect("Missing database.name");
    let database_password: String = settings.get("database.password").expect("Missing database.password");
    let database_user: String = settings.get("database.user").expect("Missing database.user");
    let database_port: u16 = settings.get("database.port").expect("Missing database.port");
    let database_host: String = settings.get("database.host").expect("Missing database.host");

    let database_url = format!(
        "postgres://{database_user}:{database_password}@{database_host}:{database_port}/{database_name}",
    );

    let mut number_of_attempts = 0;

    while let Err(e) = PgConnection::establish(&database_url) {
        eprintln!("Failed to establish connection: {:?}", e);
        std::thread::sleep(std::time::Duration::from_secs(1));
        if number_of_attempts > 10 {
            eprintln!("Failed to establish connection after 10 attempts");
            std::process::exit(1);
        }
        number_of_attempts += 1;
    }

    PgConnection::establish(&database_url).unwrap()
}

/// Main function
pub fn main() {
	let mut conn = establish_connection_to_postgres();
	
	// We write to the target directory the generated structs

	// Get the output directory
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");

    // Path to the file to create
    let path = Path::new(&out_dir).join("directus_structs.rs");

	Table::write_all(
		&mut conn,
		path.as_ref(),
		"directus",
		None
	).expect("Failed to write the structs to the file");
}