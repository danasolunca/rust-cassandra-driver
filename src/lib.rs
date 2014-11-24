#![feature(globs)]

#![crate_name = "cassandra"]
#![crate_type = "lib"]
extern crate libc;
//extern crate uuid;

pub use cluster::Cluster;
pub use batch::Batch;
pub use error::Error;
pub use future::Future;
pub use row::Row;
pub use result::CResult;
pub use result::CassResult;	
pub use session::Session;
pub use statement::Statement;
pub use statement::Prepared;
pub use collection::CassCollection;
pub use iterator::RowIterator;
pub use iterator::CollectionIterator;
pub use iterator::ResultIterator;
pub use consistency::CassConsistency;
pub use consistency::CONSISTENCY_ONE;

pub use cass_internal_api::CASS_BATCH_TYPE_LOGGED;

pub use types::CassValue;
pub use types::CassUuid;
pub use types::CassBytes;
pub use types::CassInet;

mod session;
mod cluster;
mod option;
mod error;
mod future;
mod statement;
mod row;
mod batch;
mod result;
mod consistency;
mod types;
mod collection;
mod iterator;
mod cass_internal_api;
