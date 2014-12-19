#![feature(globs)]
#![feature(unsafe_destructor)]
#![feature(default_type_params)]

extern crate libc;
extern crate uuid;

// Tools for dealing with universes (this is a doc comment, and is shown on
// the crate index page. The ! makes it apply to the parent of the comment,
// rather than what follows).
pub use batch::CassBatch;
pub use cass_ssl::CassSsl;
pub use cluster::CassCluster;
pub use collection::CassCollection;
pub use consistency::CassConsistency;
pub use error::CassError;
pub use future::CassFuture;
pub use iterator::RowIterator;
pub use iterator::CollectionIterator;
pub use iterator::ResultIterator;
pub use iterator::CassIterator;
pub use log::CassLogLevelType;
pub use result::CassResult;	
pub use row::CassRow;
pub use schema::CassSchema;
pub use schema::CassSchemaMeta;
pub use schema::CassSchemaMetaField;
pub use session::CassSession;
pub use statement::CassStatement;
pub use statement::CassPrepared;
pub use statement::CassBindable;

pub use types::CassValue;
pub use types::CassValueType;
pub use types::CassUuid;
pub use batch::BatchType;
pub use types::Decimal;

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
