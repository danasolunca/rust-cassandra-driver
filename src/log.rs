use types::CassString;

use libc::c_char;
use libc::c_void;

#[allow(non_camel_case_types)]
pub enum LogLevelType {
  DISABLED=0,
  CRITICAL=1,
  ERROR=2,
  WARN=3,
  INFO=4,
  DEBUG=5,
  TRACE=6,
  LAST_ENTRY=7,
}

pub struct LogLevel {
  cass_log_level:u32
}

impl LogLevel {
  pub fn get_level(&self) -> *const ::libc::c_char {unsafe{
    cass_log_level_string(self.cass_log_level)
  }}
}

  pub type CassLogLevel = u32;
  pub type CassLogCallback = ::std::option::Option<extern "C" fn
                              (arg1: u64, arg2: CassLogLevel,
                               arg3: CassString, arg4: *mut c_void)>;
  #[link(name = "cassandra")]
  extern "C" {
    pub fn cass_log_level_string(log_level: CassLogLevel) -> *const c_char;
  }

