pub enum VerifyType {
  NONE = 0,
  PEER_CERT = 1,
  PEER_IDENTITY=2,
}

pub mod internal {
  use types::internal as types_internal;
  use error::internal as error_internal;


  
  pub enum Struct_CassSsl_ { }
  pub type CassSsl = Struct_CassSsl_;

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
