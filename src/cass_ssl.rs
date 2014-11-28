use error::Error as CassError;
use types::CassValue;

#[allow(non_camel_case_types)] pub enum VerifyType {
  NONE = 0,
  PEER_CERT = 1,
  PEER_IDENTITY=2,
}

pub struct CassSsl {
  cass_ssl:*mut internal::CassSsl
}

#[allow(dead_code)]
impl CassSsl {
  pub fn new() -> CassSsl {unsafe{
    CassSsl{cass_ssl:internal::cass_ssl_new()}
  }}

  pub fn add_trusted_cert(&self, cert: &String) -> CassError {unsafe{
    CassError{cass_error:internal::cass_ssl_add_trusted_cert(self.cass_ssl,CassValue::str_to_cass_string(cert))}
  }}

  pub fn set_verify_flags(&self, flags: i32) {unsafe{
    internal::cass_ssl_set_verify_flags(self.cass_ssl,flags);
  }}

  pub fn set_cert(&self, cert: &String) -> CassError {unsafe{
    CassError{cass_error:internal::cass_ssl_set_cert(self.cass_ssl,CassValue::str_to_cass_string(cert))}
  }}

  pub fn set_private_key(&self, key: &String, password: &String) -> CassError {unsafe{
    CassError{cass_error:internal::cass_ssl_set_private_key(self.cass_ssl,CassValue::str_to_cass_string(key),CassValue::str_to_cass_string(password).data)}
  }}  
}

impl Drop for CassSsl {
  fn drop(&mut self) {unsafe{
    internal::cass_ssl_free(self.cass_ssl);
  }}
}

pub mod internal {
  use types::internal as types_internal;
  use error::internal as error_internal;


  
  pub enum CassSsl { }

  #[link(name = "cassandra")]
  extern "C" {
    pub fn cass_ssl_new() -> *mut CassSsl;    
    pub fn cass_ssl_free(ssl: *mut CassSsl);
    pub fn cass_ssl_add_trusted_cert(ssl: *mut CassSsl, cert: types_internal::CassString) -> error_internal::CassError;
    pub fn cass_ssl_set_verify_flags(ssl: *mut CassSsl, flags: ::libc::c_int);
    pub fn cass_ssl_set_cert(ssl: *mut CassSsl, cert: types_internal::CassString) -> error_internal::CassError;
    pub fn cass_ssl_set_private_key(ssl: *mut CassSsl, key: types_internal::CassString, password: *const ::libc::c_char) -> error_internal::CassError;
  }
}
