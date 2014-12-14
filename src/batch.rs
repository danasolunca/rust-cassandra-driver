extern crate libc;
#[allow(dead_code)]
use statement::Statement;
use statement::CassStatement;
use error::CassError;
use error::Error;
use consistency::CassConsistency;

use libc::c_uint;

pub enum BatchType {
  LOGGED=0,
  UNLOGGED=1,
  COUNTER=2,
}
impl Copy for BatchType {}

#[allow(dead_code)]
impl CassBatch {
  pub fn new(batch_type: BatchType) -> &'static CassBatch {unsafe{
    &*cass_batch_new(batch_type as u32)
  }}

  pub fn free(&mut self) {unsafe{
    cass_batch_free(self);
  }}

  pub fn add_statement(&mut self, statement: &Statement) -> Error {unsafe{
    Error{cass_error:cass_batch_add_statement(self,statement.cass_statement)}
  }}

  pub fn set_consistency(&mut self, consistency: CassConsistency) -> Error {unsafe{
    Error{cass_error:cass_batch_set_consistency(self,consistency)}
  }}
}

impl Drop for CassBatch {
  fn drop(&mut self) {
    self.free();
  }
}

  pub enum CassBatch { }
type CassBatchType = c_uint;

#[link(name = "cassandra")]
extern "C" {
  fn cass_batch_new(_type: CassBatchType) -> *mut CassBatch;
  fn cass_batch_free(batch: *mut CassBatch);
  fn cass_batch_set_consistency(batch: *mut CassBatch, consistency: CassConsistency) -> CassError;
  fn cass_batch_add_statement(batch: *mut CassBatch, statement: *mut CassStatement) -> CassError;
}

#[cfg(test)]
mod tests {
  pub use batch::BatchType;

  #[test]
  fn new() {
    super::CassBatch::new(BatchType::LOGGED);
    super::CassBatch::new(BatchType::UNLOGGED);
    super::CassBatch::new(BatchType::COUNTER);
  }
}

