//! This file is generated automatically and should not be modified.
//!
//! Any edits you may apply to this document will be overwritten next time the
//! backend is generated. Refrain from making any changes to this file.

//! If you need to make changes to the backend, please modify the `generate_models`
//! document in the `migrations` folder.

#![allow(unused)]
#![allow(clippy::all)]

use diesel::Queryable;
use diesel::QueryableByName;
use diesel::Identifiable;
use diesel::Insertable;
use crate::schema::*;
use crate::sql_function_bindings::*;
use diesel::Selectable;
use serde::Deserialize;
use serde::Serialize;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::PooledConnection;
use diesel::prelude::*;
use web_common::database::filter_structs::*;
use uuid::Uuid;
use chrono::NaiveDateTime;
use crate::views::schema::*;

