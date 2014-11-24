extern crate cassandra_internal_api;

pub fn cass_log_level_string(log_level: CassLogLevel) -> *const ::libc::c_char {unsafe{
  cassandra_internal_api::cass_log_level_string(log_level)
}}
