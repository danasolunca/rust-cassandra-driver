use types::_CassString;

use libc::c_char;
use libc::c_void;

#[allow(non_camel_case_types)]
pub enum CassLogLevelType {
  DISABLED=0,
  CRITICAL=1,
  ERROR=2,
  WARN=3,
  INFO=4,
  DEBUG=5,
  TRACE=6,
  LAST_ENTRY=7,
}
impl Copy for CassLogLevelType {}

pub type _CassLogLevel = u32;
//~ impl CassLogLevel {
  //~ pub fn get_level(&self) -> *const ::libc::c_char {unsafe{
    //~ cass_log_level_string(self.cass_log_level)
  //~ }}
//~ }

  pub type CassLogCallback = ::std::option::Option<extern "C" fn
                              (arg1: u64, arg2: _CassLogLevel,
                               arg3: _CassString, arg4: *mut c_void)>;
  #[link(name = "cassandra")]
  extern "C" {
    pub fn cass_log_level_string(log_level: _CassLogLevel) -> *const c_char;
  }

