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
pub struct Ssl {
  pub cass_ssl:*mut CassSsl
}

#[allow(dead_code)]
impl Ssl {
  pub fn new() -> Ssl {unsafe{
    Ssl{cass_ssl:cass_ssl_new()}
  }}

  pub fn add_trusted_cert(&self, cert: &str) -> Error {unsafe{
    Error{cass_error:cass_ssl_add_trusted_cert(self.cass_ssl,Value::str_to_cass_string(cert))}
  }}

  pub fn set_verify_flags(&self, flags: i32) {unsafe{
    cass_ssl_set_verify_flags(self.cass_ssl,flags);
  }}

  pub fn set_cert(&self, cert: &str) -> Error {unsafe{
    Error{cass_error:cass_ssl_set_cert(self.cass_ssl,Value::str_to_cass_string(cert))}
  }}

  pub fn set_private_key(&self, key: &str, password: &str) -> Error {unsafe{
    Error{cass_error:cass_ssl_set_private_key(self.cass_ssl,Value::str_to_cass_string(key),Value::str_to_cass_string(password).data)}
  }}  
}

impl Drop for Ssl {
  fn drop(&mut self) {unsafe{
    cass_ssl_free(self.cass_ssl);
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

