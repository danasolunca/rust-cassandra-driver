use error::_CassError;
use error::CassError;
use types::CassValue;
use types::_CassString;

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

  pub fn add_trusted_cert(&mut self, cert: &str) -> CassError {unsafe{
    CassError{err:cass_ssl_add_trusted_cert(self,CassValue::str_to_cass_string(cert))}
  }}

  pub fn set_verify_flags(&mut self, flags: i32) {unsafe{
    cass_ssl_set_verify_flags(self,flags);
  }}

  pub fn set_cert(&mut self, cert: &str) -> CassError {unsafe{
    CassError{err:cass_ssl_set_cert(
      self,
      CassValue::str_to_cass_string(cert))
    }
  }}

  pub fn set_private_key(&mut self, key: &str, password: &str) -> CassError {unsafe{
    CassError{err:cass_ssl_set_private_key(
      self,CassValue::str_to_cass_string(key),
      CassValue::str_to_cass_string(password).data)
    }
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
  fn cass_ssl_add_trusted_cert(ssl: *mut CassSsl, cert: _CassString) -> _CassError;
  fn cass_ssl_set_verify_flags(ssl: *mut CassSsl, flags: c_int);
  fn cass_ssl_set_cert(ssl: *mut CassSsl, cert: _CassString) -> _CassError;
  fn cass_ssl_set_private_key(ssl: *mut CassSsl, key: _CassString, password: *const c_char) -> _CassError;
  }

