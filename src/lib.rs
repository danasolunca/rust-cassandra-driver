#![feature(globs)]
#![feature(unsafe_destructor)]

extern crate libc;
extern crate uuid;

pub use batch::Batch as CassBatch;
pub use cluster::Cluster as CassCluster;
pub use collection::Collection as CassCollection;
pub use consistency::CassConsistency;
pub use error::Error as CassError;
pub use future::Future as CassFuture;
pub use iterator::RowIterator;
pub use iterator::CollectionIterator;
pub use iterator::ResultIterator;
pub use iterator::CassIterator;
pub use result::Result as CassResult;	
pub use row::Row as CassRow;
pub use session::Session as CassSession;
pub use statement::Statement as CassStatement;
pub use statement::Prepared as CassPrepared;
pub use log::LogLevelType;
pub use log::CassLogLevel;
pub use cass_ssl::Ssl as CassSsl;
pub use schema::Schema;
pub use schema::SchemaMeta;
pub use schema::CassSchemaMetaField;
pub use log::LogLevelType as CassLogLevelType;

pub use types::Value;
pub use types::Uuid;
pub use types::Bytes;
pub use types::Inet;
pub use batch::BatchType;

mod session;
mod cass_ssl;
mod log;
mod cluster;
mod option;
mod error;
mod future;
mod statement;
mod row;
mod batch;
mod result;
mod compression;
mod consistency;
mod schema;
mod types;
mod collection;
mod iterator;
