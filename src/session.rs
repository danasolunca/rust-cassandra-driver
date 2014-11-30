use statement::Statement;
use statement::CassStatement;
use future::Future;
use future::CassFuture;
use batch::Batch;
use result::Result;
use error::Error;
use batch::CassBatch;
use schema::Schema;
use schema::CassSchema;
use types;
use types::CassString;
use std::result::Result as RustResult;

#[allow(dead_code)]
pub struct Session {
  pub cass_session:*mut CassSession
}

#[allow(dead_code)]
impl Session {
  pub fn close_async(&self) -> Future {unsafe{
    Future{cass_future:cass_session_close(self.cass_session)}
  }}

  fn build(&self, statement: CassString) -> *mut CassFuture {unsafe{
    cass_session_prepare(self.cass_session,statement)
  }}

  pub fn prepare(&self, statement: String) -> Future {unsafe{
    Future{cass_future:cass_session_prepare(
      self.cass_session,types::cass_string_init(statement.to_c_str().as_ptr())
    )}
  }}

  pub fn execute_string(&self, statement:&String) -> RustResult<Result,Error> {
	let statement = Statement::build_from_string(statement, 0);
	self.execute_async(&statement);
    let mut future:Future = self.execute_async(&statement);
    future.wait();
    let rc = future.error_code();
    if rc.is_error() {
      return Err(rc);
    }
    return Ok(future.get_result());
  }

  pub fn execute_str(&self, statement:&str) -> RustResult<Result,Error> {
    self.execute_string(&statement.to_string())
  }

  pub fn execute(&self, statement:&Statement) -> RustResult<Result,Error> {

    let mut future:Future = self.execute_async(statement);
    future.wait();
    let rc = future.error_code();
    if rc.is_error() {
      return Err(rc);
    }
    return Ok(future.get_result());
  }

  pub fn execute_async(&self, statement: &Statement) -> Future {unsafe{
    let future = cass_session_execute(self.cass_session,&*statement.cass_statement);
    Future{cass_future:future}
  }}

  pub fn execute_batch(&self, batch: &Batch) -> Future {unsafe{
    Future{cass_future:cass_session_execute_batch(self.cass_session,&*batch.cass_batch)}
  }}

  pub fn get_schema(&self) -> Schema {unsafe{
    Schema{cass_schema:cass_session_get_schema(self.cass_session)}
  }}
  
}

  
  pub enum CassSession { }

 #[link(name = "cassandra")]
  extern "C" {
    pub fn cass_session_close(session: *mut CassSession) -> *mut CassFuture;
    pub fn cass_session_prepare(session: *mut CassSession, query: CassString) -> *mut CassFuture;
    pub fn cass_session_execute(session: *mut CassSession, statement: *const CassStatement) -> *mut CassFuture;
    pub fn cass_session_execute_batch(session: *mut CassSession, batch: *const CassBatch) -> *mut CassFuture;
    pub fn cass_session_get_schema(session: *mut CassSession) -> *const CassSchema;
  }

