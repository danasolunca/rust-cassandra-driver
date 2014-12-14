use statement::Statement;
use statement::CassStatement;
use future::CassFuture;
use result::Result;
use error::Error;
use batch::CassBatch;
use schema::CassSchema;
use types;
use types::CassString;
use std::result::Result as RustResult;

//~ #[allow(dead_code)]
//~ pub struct Session {
  //~ pub cass_session:*mut CassSession
//~ }
//~ impl Copy for Session {}


#[allow(dead_code)]
impl CassSession {
  pub fn close_async(&mut self) -> &mut CassFuture {unsafe{
    &mut*cass_session_close(self)
  }}

  fn build(&mut self, statement: CassString) -> *mut CassFuture {unsafe{
    cass_session_prepare(self,statement)
  }}

  pub fn prepare(&mut self, statement: &str) -> &CassFuture {unsafe{
    &*cass_session_prepare(self,types::cass_string_init(statement.to_c_str().as_ptr()))
  }}

  pub fn execute_string(&mut self, statement:&String) -> RustResult<Result,Error> {
	let statement = Statement::build_from_string(statement, 0);
	self.execute_async(&statement);
    let future = self.execute_async(&statement);
    future.wait();
    let rc = future.error_code();
    if rc.is_error() {
      return Err(rc);
    }
    return Ok(future.get_result());
  }

  pub fn execute_str(&mut self, statement:&str) -> RustResult<Result,Error> {
    self.execute_string(&statement.to_string())
  }

  pub fn execute(&mut self, statement:&Statement) -> RustResult<Result,Error> {

    let future = self.execute_async(statement);
    future.wait();
    let rc = future.error_code();
    if rc.is_error() {
      return Err(rc);
    }
    return Ok(future.get_result());
  }

  pub fn execute_async(&mut self, statement: &Statement) -> &mut CassFuture {unsafe{
    &mut*cass_session_execute(self,&*statement.cass_statement)
  }}

  pub fn execute_batch(&mut self, batch: &CassBatch) -> &CassFuture {unsafe{
    &*cass_session_execute_batch(self,&*batch)
  }}

  pub fn get_schema(&mut self) -> &CassSchema {unsafe{
    &*cass_session_get_schema(self)
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

