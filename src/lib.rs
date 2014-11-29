#![feature(globs)]
#![feature(unsafe_destructor)]

extern crate libc;
extern crate uuid;

pub use batch::Batch as CassBatch;
pub use cluster::Cluster as CassCluster;
pub use collection::CassCollection;
pub use consistency::CassConsistency;
pub use error::CassError;
pub use future::Future as CassFuture;
pub use iterator::RowIterator;
pub use iterator::CollectionIterator;
pub use iterator::ResultIterator;
pub use result::CResult;
pub use result::CassResult;	
pub use row::Row as CassRow;
pub use session::Session as CassSession;
pub use statement::Statement as CassStatement;
pub use statement::Prepared as CassPrepared;
pub use log::CassLogLevelType;
pub use log::CassLogLevel;
pub use cass_ssl::CassSsl;
pub use schema::CassSchema;
pub use schema::CassSchemaMeta;
pub use schema::CassSchemaMetaField;

pub use types::CassValue;
pub use types::CassUuid;
pub use types::CassBytes;
pub use types::CassInet;
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
