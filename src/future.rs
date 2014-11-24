use cass_internal_api;

use std::string::raw;

use session::Session;
use error::Error as CassError;
use statement::Prepared;
use result::CassResult;

mod cassandra {
  #[path="../error.rs"] pub mod error;
}

#[allow(dead_code)]
pub struct Future {
  pub cass_future:*mut  cass_internal_api::Struct_CassFuture_,
}


impl Drop for Future {
  fn drop(&mut self) {unsafe{
      cass_internal_api::cass_future_free(self.cass_future)
  }
}}

#[allow(dead_code)]
impl Future {


  pub fn ready(&self) -> cass_internal_api::cass_bool_t {unsafe{
    cass_internal_api::cass_future_ready(self.cass_future)
  }}

  pub fn wait(&mut self) {unsafe{
    cass_internal_api::cass_future_wait(self.cass_future)
  }}

  pub fn timed(&mut self, timeout: cass_internal_api::cass_duration_t) -> cass_internal_api::cass_bool_t {unsafe{
    cass_internal_api::cass_future_wait_timed(self.cass_future,timeout)
  }}

  pub fn get_session(&mut self) -> Session {unsafe{
    Session{cass_session:cass_internal_api::cass_future_get_session(self.cass_future)}
  }}

  pub fn get_result(&mut self) -> CassResult {unsafe{
    CassResult{cass_result:cass_internal_api::cass_future_get_result(self.cass_future)}
  }}

  pub fn get_prepared(&mut self) -> Prepared {unsafe{
    Prepared{cass_prepared:cass_internal_api::cass_future_get_prepared(self.cass_future)}
  }}

  pub fn error_code(&mut self) -> CassError {unsafe{
    CassError{cass_error:cass_internal_api::cass_future_error_code(self.cass_future)}
  }}

  pub fn error_message(&mut self) -> String {unsafe{
    let cstr = cass_internal_api::cass_future_error_message(self.cass_future);
    let (raw,length) = (cstr.data as *mut u8,cstr.length as uint);
    raw::from_parts(raw, length, length)
  }}

  pub fn print_error(&mut self) {
    println!("Error: {}", "self");
  }
}
