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

#[allow(dead_code)]
pub struct Batch {
  pub cass_batch:*mut CassBatch,
}

#[allow(dead_code)]
impl Batch {
  pub fn new(batch_type: BatchType) -> Batch {unsafe{
    Batch{cass_batch:cass_batch_new(batch_type as u32)}
  }}

  pub fn free(&self) {unsafe{
    cass_batch_free(self.cass_batch);
  }}

  pub fn add_statement(&self, statement: Statement) -> Error {unsafe{
    Error{cass_error:cass_batch_add_statement(self.cass_batch,statement.cass_statement)}
  }}

  pub fn set_consistency(&self, consistency: CassConsistency) -> Error {unsafe{
    Error{cass_error:cass_batch_set_consistency(self.cass_batch,consistency)}
  }}

}

//~ impl Drop for Batch {
  //~ fn drop(&mut self) {
    //~ self.free();
  //~ }
//~ }

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
    super::Batch::new(BatchType::LOGGED);
    super::Batch::new(BatchType::UNLOGGED);
    super::Batch::new(BatchType::COUNTER);
  }
}

