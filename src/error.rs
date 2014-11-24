extern crate libc;

use cass_internal_api;

use libc::c_char;


#[allow(dead_code)]
#[deriving(Clone,Show)]
pub struct Error {
  pub cass_error:cass_internal_api::CassError,
}

#[allow(dead_code)]

impl Error {
  pub fn new(err:u32) -> Error {
    Error{cass_error:err}
  }

  pub fn is_error(self) -> bool {
    if self.cass_error != cass_internal_api::CASS_OK {true} else {false}
  }


  pub fn cass_error_desc(&self) -> *const c_char {unsafe{
    cass_internal_api::cass_error_desc(self.cass_error)
  }}
}
