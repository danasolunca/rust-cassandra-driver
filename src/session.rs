use statement::Statement;
use future::Future as CassFuture;
use result::CResult;
use batch::Batch;
use schema::CassSchema;

use types::internal as types_internal;
use session::internal as session_internal;
use future::internal as future_internal;

#[allow(dead_code)]
pub struct Session {
  pub cass_session:*mut session_internal::CassSession
}

#[allow(dead_code)]
impl Session {
  pub fn close_async(&self) -> CassFuture {unsafe{
    CassFuture{cass_future:internal::cass_session_close(self.cass_session)}
  }}

  fn build(&self, statement: types_internal::CassString) -> *mut future_internal::CassFuture {unsafe{
    internal::cass_session_prepare(self.cass_session,statement)
  }}

  pub fn prepare(&self, statement: String) -> CassFuture {unsafe{
    CassFuture{cass_future:internal::cass_session_prepare(
      self.cass_session,types_internal::cass_string_init(statement.to_c_str().as_ptr())
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
    let future = internal::cass_session_execute(self.cass_session,&*statement.cass_statement);
    CassFuture{cass_future:future}
  }}

  pub fn execute_batch(&self, batch: &Batch) -> CassFuture {unsafe{
    CassFuture{cass_future:internal::cass_session_execute_batch(self.cass_session,&*batch.cass_batch)}
  }}

  pub fn get_schema(&self) -> CassSchema {unsafe{
    CassSchema{cass_schema:internal::cass_session_get_schema(self.cass_session)}
  }}
  
}

pub mod internal {
  use future::internal as future_internal;
  use types::internal as types_internal;
  use statement::internal as statement_internal;
  use batch::internal as batch_internal;
  use schema::internal as schema_internal;
  
  pub enum CassSession { }

 #[link(name = "cassandra")]
  extern "C" {
    pub fn cass_session_close(session: *mut CassSession) -> *mut future_internal::CassFuture;
    pub fn cass_session_prepare(session: *mut CassSession, query: types_internal::CassString) -> *mut future_internal::CassFuture;
    pub fn cass_session_execute(session: *mut CassSession, statement: *const statement_internal::CassStatement) -> *mut future_internal::CassFuture;
    pub fn cass_session_execute_batch(session: *mut CassSession, batch: *const batch_internal::CassBatch) -> *mut future_internal::CassFuture;
    pub fn cass_session_get_schema(session: *mut CassSession) -> *const schema_internal::CassSchema;
  }
}
