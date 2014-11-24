extern crate libc;
#[allow(dead_code)]
use statement::Statement;
use error::Error as CassError;
use cass_internal_api;

pub type BatchType = u32;
#[allow(dead_code)]pub static LOGGED:BatchType = 0;
#[allow(dead_code)]pub static UNLOGGED:BatchType = 1;
#[allow(dead_code)]pub static COUNTER:BatchType = 2;

#[allow(dead_code)]
pub struct Batch {
  pub cass_batch:*mut cass_internal_api::CassBatch,
}

#[allow(dead_code)]
impl Batch {
  pub fn new(batch_type: BatchType) -> Batch {unsafe{
    Batch{cass_batch:cass_internal_api::cass_batch_new(batch_type)}
  }}

  pub fn free(&mut self) {unsafe{
    cass_internal_api::cass_batch_free(self.cass_batch);
  }}

  pub fn add_statement(&mut self, statement: Statement) -> CassError {unsafe{
    CassError{cass_error:cass_internal_api::cass_batch_add_statement(self.cass_batch,statement.cass_statement)}
  }}
}

impl Drop for Batch {
  fn drop(&mut self) {
    self.free();
  }
}


#[cfg(test)]
mod tests {
    #[test]
    fn new() {
      super::Batch::new(super::LOGGED);
      super::Batch::new(super::UNLOGGED);
      super::Batch::new(super::COUNTER);
    }
}
