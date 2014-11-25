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

pub mod internal {
  use types::internal as types_internal;
  pub type CassLogLevel = ::libc::c_uint;
  pub type CassLogCallback = ::std::option::Option<extern "C" fn
                              (arg1: u64, arg2: CassLogLevel,
                               arg3: types_internal::CassString, arg4: *mut ::libc::c_void)>;
  #[link(name = "cassandra")]
  extern "C" {
    pub fn cass_log_level_string(log_level: CassLogLevel) -> *const ::libc::c_char;
  }
}
