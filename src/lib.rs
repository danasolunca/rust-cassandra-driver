#![feature(globs)]
#![feature(unsafe_destructor)]

extern crate libc;
//extern crate uuid;

pub use batch::Batch;
pub use cluster::Cluster;
pub use collection::CassCollection;
pub use consistency::CassConsistency;
pub use error::Error;
pub use future::Future;
pub use iterator::RowIterator;
pub use iterator::CollectionIterator;
pub use iterator::ResultIterator;
pub use result::CResult;
pub use result::CassResult;	
pub use row::Row;
pub use session::Session;
pub use statement::Statement;
pub use statement::Prepared;
pub use log::CassLogLevelType;

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
