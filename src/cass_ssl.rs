use error::CassError;
use error::Error;
use types::Value;
use types::CassString;

use libc::c_char;
use libc::c_int;

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum VerifyType {
  NONE = 0,
  PEER_CERT = 1,
  PEER_IDENTITY=2,
}

#[allow(dead_code)]
impl CassSsl {
  pub fn new() -> &'static CassSsl {unsafe{
    &*cass_ssl_new()
  }}

  pub fn add_trusted_cert(&mut self, cert: &str) -> Error {unsafe{
    Error{cass_error:cass_ssl_add_trusted_cert(self,Value::str_to_cass_string(cert))}
  }}

  pub fn set_verify_flags(&mut self, flags: i32) {unsafe{
    cass_ssl_set_verify_flags(self,flags);
  }}

  pub fn set_cert(&mut self, cert: &str) -> Error {unsafe{
    Error{cass_error:cass_ssl_set_cert(self,Value::str_to_cass_string(cert))}
  }}

  pub fn set_private_key(&mut self, key: &str, password: &str) -> Error {unsafe{
    Error{cass_error:cass_ssl_set_private_key(self,Value::str_to_cass_string(key),Value::str_to_cass_string(password).data)}
  }}  
}

impl Drop for CassSsl {
  fn drop(&mut self) {unsafe{
    cass_ssl_free(self);
  }}
}

pub enum CassSsl { }

#[link(name = "cassandra")]
extern "C" {
  fn cass_ssl_new() -> *mut CassSsl;    
  fn cass_ssl_free(ssl: *mut CassSsl);
  fn cass_ssl_add_trusted_cert(ssl: *mut CassSsl, cert: CassString) -> CassError;
  fn cass_ssl_set_verify_flags(ssl: *mut CassSsl, flags: c_int);
  fn cass_ssl_set_cert(ssl: *mut CassSsl, cert: CassString) -> CassError;
  fn cass_ssl_set_private_key(ssl: *mut CassSsl, key: CassString, password: *const c_char) -> CassError;
  }

