extern crate libc;
#[allow(dead_code)]
use statement::Statement;
use error::CassError;
use consistency::CassConsistency;

pub enum BatchType {
  LOGGED=0,
  UNLOGGED=1,
  COUNTER=2,
}

#[allow(dead_code)]
pub struct Batch {
  pub cass_batch:*mut internal::CassBatch,
}

#[allow(dead_code)]
impl Batch {
  pub fn new(batch_type: BatchType) -> Batch {unsafe{
    Batch{cass_batch:internal::cass_batch_new(batch_type as u32)}
  }}

  pub fn free(&mut self) {unsafe{
    internal::cass_batch_free(self.cass_batch);
  }}

  pub fn add_statement(&mut self, statement: Statement) -> CassError {unsafe{
    CassError{cass_error:internal::cass_batch_add_statement(self.cass_batch,statement.cass_statement)}
  }}

  pub fn set_consistency(&mut self, consistency: CassConsistency) -> CassError {unsafe{
    CassError{cass_error:internal::cass_batch_set_consistency(self.cass_batch,consistency)}
  }}

}

impl Drop for Batch {
  fn drop(&mut self) {
    self.free();
  }
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

pub mod internal {
  use consistency;
  use error::internal as error_internal;
  use statement::internal as statement_internal;

  pub enum CassBatch { }
  pub type CassBatchType = ::libc::c_uint;

  #[link(name = "cassandra")]
  extern "C" {
    pub fn cass_batch_new(_type: CassBatchType) -> *mut CassBatch;
    pub fn cass_batch_free(batch: *mut CassBatch);
    pub fn cass_batch_set_consistency(batch: *mut CassBatch, consistency: consistency::CassConsistency) -> error_internal::CassError;
    pub fn cass_batch_add_statement(batch: *mut CassBatch, statement: *mut statement_internal::CassStatement) -> error_internal::CassError;
  }
}
