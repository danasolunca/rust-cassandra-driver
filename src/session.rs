use statement::Statement;
use future::Future as CassFuture;
use result::CResult;
use batch::Batch;
use cass_internal_api;

#[allow(dead_code)]
pub struct Session {
  pub cass_session:*mut cass_internal_api::CassSession
}

#[allow(dead_code)]
impl Session {
  pub fn close_async(&self) -> CassFuture {unsafe{
    CassFuture{cass_future:cass_internal_api::cass_session_close(self.cass_session)}
  }}

  fn build(&self, statement: cass_internal_api::CassString) -> *mut cass_internal_api::CassFuture {unsafe{
    cass_internal_api::cass_session_prepare(self.cass_session,statement)
  }}

  pub fn prepare(&self, statement: String) -> CassFuture {unsafe{
    CassFuture{cass_future:cass_internal_api::cass_session_prepare(
      self.cass_session,cass_internal_api::cass_string_init(
        statement.to_c_str().as_ptr()
      )
    )}
  }}

  pub fn execute_string(&self, statement:&String) -> CResult {
	let statement = Statement::build_from_string(statement, 0);
	self.execute_async(&statement);
    let mut future:CassFuture = self.execute_async(&statement);
    future.wait();
    let rc = future.error_code();
    if rc.is_error() {
      return Err(rc);
    }
    return Ok(future.get_result());
  }

  pub fn execute_str(&self, statement:&str) -> CResult {
    self.execute_string(&statement.to_string())
  }

  pub fn execute(&self, statement:&Statement) -> CResult {

    let mut future:CassFuture = self.execute_async(statement);
    future.wait();
    let rc = future.error_code();
    if rc.is_error() {
      return Err(rc);
    }
    return Ok(future.get_result());
  }

  pub fn execute_async(&self, statement: &Statement) -> CassFuture {unsafe{
    let future = cass_internal_api::cass_session_execute(self.cass_session,&*statement.cass_statement);
    CassFuture{cass_future:future}
  }}

  pub fn execute_batch(&self, batch: &Batch) -> CassFuture {unsafe{
    CassFuture{cass_future:cass_internal_api::cass_session_execute_batch(self.cass_session,&*batch.cass_batch)}
  }}
}
