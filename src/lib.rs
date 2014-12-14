#![feature(globs)]
#![feature(unsafe_destructor)]

extern crate libc;
extern crate uuid;

//pub use statement::CassBindable;
pub use batch::Batch as CassBatch;
pub use cass_ssl::Ssl as CassSsl;
pub use cluster::CassCluster;
pub use collection::Collection as CassCollection;
pub use consistency::CassConsistency;
pub use error::Error as CassError;
pub use future::Future as CassFuture;
pub use iterator::RowIterator;
pub use iterator::CollectionIterator;
pub use iterator::ResultIterator;
pub use iterator::CIterator as CassIterator;
pub use log::LogLevelType as CassLogLevelType;
pub use log::CassLogLevel;
pub use result::Result as CassResult;	
pub use row::Row as CassRow;
pub use schema::Schema as CassSchema;
pub use schema::SchemaMeta as CassSchemaMeta;
pub use schema::CassSchemaMetaField;
pub use session::Session as CassSession;
pub use statement::Statement as CassStatement;
pub use statement::Prepared as CassPrepared;
pub use statement::CassBindable;

pub use types::Value;
pub use types::CassUuid;
pub use batch::BatchType;

mod batch;
mod cass_ssl;
mod cluster;
mod collection;
mod compression;
mod consistency;
mod error;
mod future;
mod iterator;
mod log;
mod option;
mod result;
mod row;
mod session;
mod statement;
mod schema;
mod types;
