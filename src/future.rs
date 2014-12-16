use session::CassSession;
use error::_CassError;
use error::CassError;
use result::CassResult;
use result;
use types::_CassString;
use types::_CassBoolType;
use types::_CassDurationType;
use types::_CassSizeType;
use statement::CassPrepared;
use libc::c_void;

impl Drop for CassFuture {
  fn drop(&mut self) {unsafe{
      cass_future_free(self)
  }
}}

impl CassFuture {
  pub fn ready(&mut self) -> _CassSizeType {unsafe{
    cass_future_ready(self)
  }}

  pub fn wait(&mut self) {unsafe{
    cass_future_wait(self);
  }}

  pub fn timed(&mut self, timeout: _CassDurationType) -> _CassBoolType {unsafe{
    cass_future_wait_timed(self,timeout)
  }}

  pub fn get_session(&mut self) -> &mut CassSession {unsafe{
    &mut*cass_future_get_session(self)
  }}

  pub fn get_result(&mut self) -> &CassResult {unsafe{
    &*cass_future_get_result(self)
  }}

  //~ pub fn set_callback(&mut self,callback: CassFutureCallback, data: *mut ::libc::c_void) -> CassResult {unsafe{
    //~ CassResult{cass_result:internal::cass_future_set_callback(self.cass_future)}
  //~ }}


  pub fn get_prepared(&mut self) -> &CassPrepared {unsafe{
    &*cass_future_get_prepared(self)
  }}

  pub fn error_code(&mut self) -> CassError {unsafe{
    CassError{err:cass_future_error_code(self)}
  }}

  pub fn error_message(&mut self) -> String {unsafe{
    let cstr = cass_future_error_message(self);
    let (raw,length) = (cstr.data as *mut u8,cstr.length as uint);
    String::from_raw_parts(raw, length, length)
  }}

  pub fn print_error(&self) {
    println!("Error: {}", "self");
  }
}

pub type CassFutureCallback = Option<extern "C" fn (arg1: *mut CassFuture, arg2: *mut c_void)>;
  
  pub enum CassFuture { }
  #[link(name = "cassandra")]
  extern "C" {
    pub fn cass_future_free(future: *mut CassFuture);
    pub fn cass_future_set_callback(future: *mut CassFuture, callback: CassFutureCallback, data: *mut c_void) -> _CassError;
    pub fn cass_future_ready(future: *mut CassFuture) -> _CassSizeType;
    pub fn cass_future_wait(future: *mut CassFuture);
    pub fn cass_future_wait_timed(future: *mut CassFuture, timeout_us: _CassDurationType) -> _CassBoolType;
    pub fn cass_future_get_session(future: *mut CassFuture) -> *mut CassSession;
    pub fn cass_future_get_result(future: *mut CassFuture) -> *const result::CassResult;
    pub fn cass_future_get_prepared(future: *mut CassFuture) -> *const CassPrepared;
    pub fn cass_future_error_code(future: *mut CassFuture) -> _CassError;
    pub fn cass_future_error_message(future: *mut CassFuture) -> _CassString;
  }

