//use uuid::Uuid;

use cass_internal_api;

use collection::CassCollection;
use error::Error as CassError;
use types::CassDecimal;
use types::CassInet;
use types::CassBytes;
use types::CassValue;
use session::Session;
use result::CResult;
use consistency::CONSISTENCY_SERIAL;

use future::Future as CassFuture;
use std::fmt::Show;
use std::fmt::Formatter;
use std::fmt::Result;


pub struct Statement {
   pub cass_statement:*mut cass_internal_api::CassStatement,
   pub last_error:CassError
}

impl Show for Statement {
   fn fmt(&self, f: &mut Formatter) -> Result {
     write!(f, "(Statement:{})", self)
    }
}

impl Drop for Statement {
  fn drop(&mut self) {unsafe{
  cass_internal_api::cass_statement_free(self.cass_statement)
  }}
}

#[allow(dead_code)]
impl Statement {
  pub fn new(statement_string: &String, parameter_count: cass_internal_api::cass_size_t) ->  Statement {unsafe{
    let cass_string = CassValue::str_to_cass_string(statement_string);
    let statement = cass_internal_api::cass_statement_new(cass_string,parameter_count);
    Statement{cass_statement:statement,last_error:CassError::new(0)}
  }}

  pub fn build_from_string(statement_string:&String, parameter_count: cass_internal_api::cass_size_t) -> Statement {unsafe{
    let query_cstring = statement_string.to_c_str();
    let query = cass_internal_api::cass_string_init(query_cstring.as_ptr());
    Statement{cass_statement:cass_internal_api::cass_statement_new(query,parameter_count),last_error:CassError::new(0)}
  }}

  pub fn build_from_str(statement_string:&str, parameter_count: cass_internal_api::cass_size_t) -> Statement {unsafe{
    let query_cstring = statement_string.to_c_str();
    let query = cass_internal_api::cass_string_init(query_cstring.as_ptr());
    Statement{cass_statement:cass_internal_api::cass_statement_new(query,parameter_count),last_error:CassError::new(0)}
  }}

  pub fn set_paging_size( &mut self, page_size: ::libc::c_int) -> Option<CassError> {unsafe{
    let error = CassError{cass_error:cass_internal_api::cass_statement_set_paging_size(self.cass_statement,page_size)};
    if error.is_error() {return Some(error)} else {return None}
  }}

  pub fn set_paging_state(&mut self, result: &mut CResult) -> Option<CassError> {unsafe{
    match result.clone().ok() {
      Some(r) => {
        let resp = cass_internal_api::cass_statement_set_paging_state(self.cass_statement,r.cass_result);
        if resp > 0 {
	      Some(CassError{cass_error:resp})
	    } 
	    else {
	      None
	    }
	  },
      //FIXME can this happen?
      None => Some(CassError{cass_error:1u32}),
    }
  }}
  
  pub fn set_serial_consistency(&mut self) -> &mut Statement {unsafe{
    self.last_error = CassError{cass_error:cass_internal_api::cass_statement_set_serial_consistency(self.cass_statement,CONSISTENCY_SERIAL)};
    self
  }}
   
  pub fn bind_null(&mut self, index: cass_internal_api::cass_size_t) -> &mut Statement {unsafe{
    self.last_error = CassError{cass_error:cass_internal_api::cass_statement_bind_null(self.cass_statement,index)};
    self
  }}

  pub fn bind_int32(&mut self, index: cass_internal_api::cass_size_t, value: cass_internal_api::cass_int32_t) -> &mut Statement {unsafe{
    self.last_error = CassError{cass_error:cass_internal_api::cass_statement_bind_int32(self.cass_statement,index,value)};
    self
  }}

  pub fn bind_int64(&mut self, index: cass_internal_api::cass_size_t, value: cass_internal_api::cass_int64_t) -> &mut Statement {unsafe{
    self.last_error = CassError{cass_error:cass_internal_api::cass_statement_bind_int64(self.cass_statement,index,value)};
    self
  }}

  pub fn bind_float(&mut self, index: cass_internal_api::cass_size_t, value: cass_internal_api::cass_float_t) -> &mut Statement {unsafe{
    self.last_error = CassError{cass_error:cass_internal_api::cass_statement_bind_float(self.cass_statement,index,value)};
    self
  }}

  pub fn bind_double(&mut self, index: cass_internal_api::cass_size_t, value: cass_internal_api::cass_double_t) -> &mut Statement {unsafe{
    self.last_error = CassError{cass_error:cass_internal_api::cass_statement_bind_double(self.cass_statement,index,value)};
    self
  }}

  pub fn bind_bool(&mut self, index: cass_internal_api::cass_size_t, value: cass_internal_api::cass_bool_t) -> &mut Statement {unsafe{
    self.last_error = CassError{cass_error:cass_internal_api::cass_statement_bind_bool(self.cass_statement,index,value)};
    self
  }}

  pub fn bind_string(&mut self, index: cass_internal_api::cass_size_t, value: &String) -> &mut Statement {unsafe{
    let cass_string = CassValue::str_to_cass_string(value);
      println!("cass_string={}",cass_string);
    self.last_error = CassError{cass_error:cass_internal_api::cass_statement_bind_string(self.cass_statement,index,cass_string)};
    self
  }}

  pub fn bind_str(&mut self, index: cass_internal_api::cass_size_t, value: &str) -> &mut Statement {
    self.bind_string(index, &value.to_string())
  }


  pub fn bind_bytes(&mut self, index: cass_internal_api::cass_size_t, value: CassBytes) -> &mut Statement {unsafe{
    self.last_error = CassError{cass_error:cass_internal_api::cass_statement_bind_bytes(self.cass_statement,index,value.cass_bytes)};
    self
  }}

  //~ pub fn bind_uuid(&mut self, index: cass_internal_api::cass_size_t, value: Uuid) -> &mut Statement {unsafe{
	  //~ let bytes = value.as_bytes();
	  //~ let my_uuid:[u8,..16]=
	  //~ [bytes[0],bytes[1],bytes[2],bytes[3],bytes[4],bytes[5],bytes[6],bytes[7],
	   //~ bytes[8],bytes[9],bytes[10],bytes[11],bytes[12],bytes[13],bytes[14],bytes[15]];

    //~ self.last_error = CassError{cass_error:cass_internal_api::cass_statement_bind_uuid(self.cass_statement,index,my_uuid)};
    //~ self
  //~ }}

  pub fn bind_inet(&mut self, index: cass_internal_api::cass_size_t, value: CassInet) -> &mut Statement {unsafe{
    self.last_error = CassError{cass_error:cass_internal_api::cass_statement_bind_inet(self.cass_statement,index,value.cass_inet)};
    self
  }}

  pub fn bind_decimal(&mut self, index: cass_internal_api::cass_size_t, value: CassDecimal) -> &mut Statement {unsafe{
    self.last_error = CassError{cass_error:cass_internal_api::cass_statement_bind_decimal(self.cass_statement,index,value.cass_decimal)};
    self
  }}

  pub fn bind_custom(&mut self, index: cass_internal_api::cass_size_t, size: cass_internal_api::cass_size_t, output: *mut *mut cass_internal_api::cass_byte_t) -> &mut Statement {unsafe{
    self.last_error = CassError{cass_error:cass_internal_api::cass_statement_bind_custom(self.cass_statement,index,size,output)};
    self
  }}

  pub fn bind_collection(&mut self, index: cass_internal_api::cass_size_t, collection: CassCollection) -> &mut Statement {unsafe{
    self.last_error = CassError{cass_error:cass_internal_api::cass_statement_bind_collection(self.cass_statement,index,&*collection.cass_collection)};
    self
  }}
}

#[allow(dead_code)]
pub struct Prepared {
  pub cass_prepared:*const cass_internal_api::CassPrepared,
}

#[allow(dead_code)]
impl Prepared {
  pub fn new(statement_string:&String, session:Session) -> CassFuture {unsafe{
    let cass_string = CassValue::str_to_cass_string(statement_string);
    let state:*mut cass_internal_api::Struct_CassFuture_ = cass_internal_api::cass_session_prepare(session.cass_session,cass_string);
    let prepared = CassFuture{cass_future:state};
    prepared
  }}

  pub fn free(&mut self) {unsafe{
     cass_internal_api::cass_prepared_free(self.cass_prepared);
  }}
  
  pub fn cass_prepared_bind(&mut self) -> Statement {unsafe{
	 Statement{cass_statement:cass_internal_api::cass_prepared_bind(self.cass_prepared),last_error:CassError::new(0)}
  }}

#[allow(unused_variables)]
  pub fn bind(&mut self, parameter_count: cass_internal_api::cass_size_t) -> Statement {unsafe{
    Statement{cass_statement:cass_internal_api::cass_prepared_bind(self.cass_prepared),last_error:CassError::new(0)}
  }}
}

impl Drop for Prepared {
  fn drop(&mut self) {
    self.free();
  }
}
