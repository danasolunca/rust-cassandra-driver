use std::string::raw;

use session::Session;
use error::Error as CassError;
use statement::Prepared;
use result::CassResult;
use types::internal as types_internal;

mod cassandra {
  #[path="../error.rs"] pub mod error;
}

#[allow(dead_code)]
pub struct Future {
  pub cass_future:*mut  internal::Struct_CassFuture_,
}


impl Drop for Future {
  fn drop(&mut self) {unsafe{
      internal::cass_future_free(self.cass_future)
  }
}}

#[allow(dead_code)]
impl Future {


  pub fn ready(&self) -> types_internal::cass_bool_t {unsafe{
    internal::cass_future_ready(self.cass_future)
  }}

  pub fn wait(&mut self) {unsafe{
    internal::cass_future_wait(self.cass_future)
  }}

  pub fn timed(&mut self, timeout: types_internal::cass_duration_t) -> types_internal::cass_bool_t {unsafe{
    internal::cass_future_wait_timed(self.cass_future,timeout)
  }}

  pub fn get_session(&mut self) -> Session {unsafe{
    Session{cass_session:internal::cass_future_get_session(self.cass_future)}
  }}

  pub fn get_result(&mut self) -> CassResult {unsafe{
    CassResult{cass_result:internal::cass_future_get_result(self.cass_future)}
  }}

  pub fn get_prepared(&mut self) -> Prepared {unsafe{
    Prepared{cass_prepared:internal::cass_future_get_prepared(self.cass_future)}
  }}

  pub fn error_code(&mut self) -> CassError {unsafe{
    CassError{cass_error:internal::cass_future_error_code(self.cass_future)}
  }}

  pub fn error_message(&mut self) -> String {unsafe{
    let cstr = internal::cass_future_error_message(self.cass_future);
    let (raw,length) = (cstr.data as *mut u8,cstr.length as uint);
    raw::from_parts(raw, length, length)
  }}

  pub fn print_error(&mut self) {
    println!("Error: {}", "self");
  }
}

pub mod internal {
  use types::internal as types_internal;
  use session::internal as session_internal;
  use result::internal as result_internal;
  use statement::internal as statement_internal;
  use error::internal as error_internal;

  pub type CassFutureCallback =
    ::std::option::Option<extern "C" fn
                              (arg1: *mut CassFuture,
                               arg2: *mut ::libc::c_void)>;
  
  pub enum Struct_CassFuture_ { }
  pub type CassFuture = Struct_CassFuture_;
  #[link(name = "cassandra")]
  extern "C" {
    pub fn cass_future_free(future: *mut CassFuture);
    //pub fn cass_future_set_callback(future: *mut CassFuture, callback: CassFutureCallback, data: *mut ::libc::c_void) -> CassError;
    pub fn cass_future_ready(future: *mut CassFuture) -> types_internal::cass_bool_t;
    pub fn cass_future_wait(future: *mut CassFuture);
    pub fn cass_future_wait_timed(future: *mut CassFuture, timeout_us: types_internal::cass_duration_t) -> types_internal::cass_bool_t;
    pub fn cass_future_get_session(future: *mut CassFuture) -> *mut session_internal::CassSession;
    pub fn cass_future_get_result(future: *mut CassFuture) -> *const result_internal::CassResult;
    pub fn cass_future_get_prepared(future: *mut CassFuture) -> *const statement_internal::CassPrepared;
    pub fn cass_future_error_code(future: *mut CassFuture) -> error_internal::CassError;
    pub fn cass_future_error_message(future: *mut CassFuture) -> types_internal::CassString;

  }
}
