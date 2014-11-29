use collection::CassCollection;
use error::CassError;
use types::CassDecimal;
use types::CassInet;
use types::CassBytes;
use types::CassValue;
use session::Session;
use result::CResult;
use consistency::CassConsistency;
use types::internal as types_internal;
use session::internal as session_internal;
use statement::internal as statement_internal;
use future::internal as future_internal;
use std::fmt;

use uuid::Uuid;

use future::Future as CassFuture;
use std::fmt::Show;
use std::fmt::Formatter;


pub struct Statement {
   pub cass_statement:*mut statement_internal::CassStatement,
   pub last_error:CassError
}

impl Show for Statement {
   fn fmt(&self, f: &mut Formatter) -> fmt::Result {
     write!(f, "(Statement:{})", self)
    }
}

impl Drop for Statement {
  fn drop(&mut self) {unsafe{
  internal::cass_statement_free(self.cass_statement)
  }}
}

#[allow(dead_code)]
impl Statement {
  pub fn new(statement_string: &str, parameter_count: types_internal::CassSizeType) ->  Statement {unsafe{
    let cass_string = CassValue::str_to_cass_string(statement_string);
    let statement = statement_internal::cass_statement_new(cass_string,parameter_count);
    Statement{cass_statement:statement,last_error:CassError::new(0)}
  }}

  pub fn build_from_string(statement_string:&String, parameter_count: types_internal::CassSizeType) -> Statement {unsafe{
    let query_cstring = statement_string.to_c_str();
    let query = types_internal::cass_string_init(query_cstring.as_ptr());
    Statement{cass_statement:statement_internal::cass_statement_new(query,parameter_count),last_error:CassError::new(0)}
  }}

  pub fn build_from_str(statement_string:&str, parameter_count: types_internal::CassSizeType) -> Statement {unsafe{
    let query_cstring = statement_string.to_c_str();
    let query = types_internal::cass_string_init(query_cstring.as_ptr());
    Statement{cass_statement:statement_internal::cass_statement_new(query,parameter_count),last_error:CassError::new(0)}
  }}

  pub fn set_paging_size( &mut self, page_size: ::libc::c_int) -> Option<CassError> {unsafe{
    let error = CassError{cass_error:statement_internal::cass_statement_set_paging_size(self.cass_statement,page_size)};
    if error.is_error() {return Some(error)} else {return None}
  }}

  pub fn set_paging_state(&mut self, result: &mut CResult) -> Option<CassError> {unsafe{
    match result.clone().ok() {
      Some(r) => {
        let resp = statement_internal::cass_statement_set_paging_state(self.cass_statement,r.cass_result);
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
  
  pub fn set_serial_consistency(&mut self) -> Result<&mut Statement,CassError> {unsafe{
    self.last_error = CassError{cass_error:statement_internal::cass_statement_set_serial_consistency(self.cass_statement,CassConsistency::SERIAL)};
    Ok(self)
  }}

  pub fn add_key_index(&mut self, index:u64) -> Result<&mut Statement,CassError> {unsafe{
    self.last_error = CassError{cass_error:statement_internal::cass_statement_add_key_index(self.cass_statement,index)};
    Ok(self)
  }}

  pub fn set_consistency(&mut self,consistency:CassConsistency) -> Result<&mut Statement,CassError> {unsafe{
    self.last_error = CassError{cass_error:statement_internal::cass_statement_set_consistency(self.cass_statement,consistency)};
    Ok(self)
  }}

  pub fn set_keyspace(&mut self, keyspace:&str) -> Result<&mut Statement,CassError> {unsafe{
    self.last_error = CassError{cass_error:statement_internal::cass_statement_set_keyspace(self.cass_statement,keyspace.as_ptr() as *const i8)};
    Ok(self)
  }}
 
  pub fn bind_null(&mut self, index: types_internal::CassSizeType) -> Result<&mut Statement,CassError> {unsafe{
    self.last_error = CassError{cass_error:statement_internal::cass_statement_bind_null(self.cass_statement,index)};
    Ok(self)
  }}

  pub fn bind_int32(&mut self, index: types_internal::CassSizeType, value: i32) -> Result<&mut Statement,CassError> {unsafe{
    self.last_error = CassError{cass_error:statement_internal::cass_statement_bind_int32(self.cass_statement,index,value)};
    Ok(self)
  }}

  pub fn bind_int32_by_name(&mut self, name: &str, value: i32) -> Result<&mut Statement,CassError> {unsafe{
    self.last_error = CassError{cass_error:internal::cass_statement_bind_int32_by_name(self.cass_statement,name.as_ptr() as *const i8,value)};
    Ok(self)
  }}

  pub fn bind_int64_by_name(&mut self, name: &str, value: i64) -> Result<&mut Statement,CassError> {unsafe{
    self.last_error = CassError{cass_error:internal::cass_statement_bind_int64_by_name(self.cass_statement,name.as_ptr() as *const i8,value)};
    Ok(self)
  }}

  pub fn bind_float_by_name(&mut self, name: &str, value: f32) -> Result<&mut Statement,CassError> {unsafe{
    self.last_error = CassError{cass_error:internal::cass_statement_bind_float_by_name(self.cass_statement,name.as_ptr() as *const i8,value)};
    Ok(self)
  }}

  pub fn bind_double_by_name(&mut self, name: &str, value: f64) -> Result<&mut Statement,CassError> {unsafe{
    self.last_error = CassError{cass_error:internal::cass_statement_bind_double_by_name(self.cass_statement,name.as_ptr() as *const i8,value)};
    Ok(self)
  }}

  pub fn bind_bool_by_name(&mut self, name: &str, value: bool) -> Result<&mut Statement,CassError> {unsafe{
    self.last_error = CassError{cass_error:internal::cass_statement_bind_bool_by_name(self.cass_statement,name.as_ptr() as *const i8,match value {true=>1,false=>0})};
    Ok(self)
  }}

  pub fn bind_string_by_name(&mut self, name: &str, value: &str) -> Result<&mut Statement,CassError> {unsafe{
    self.last_error = CassError{cass_error:internal::cass_statement_bind_string_by_name(self.cass_statement,name.as_ptr() as *const i8,CassValue::str_to_cass_string(value))};
    Ok(self)
  }}

  pub fn bind_bytes_by_name(&mut self, name: &str, value: CassBytes) -> Result<&mut Statement,CassError> {unsafe{
    self.last_error = CassError{cass_error:internal::cass_statement_bind_bytes_by_name(self.cass_statement,name.as_ptr() as *const i8,value.cass_bytes)};
    Ok(self)
  }}

  pub fn bind_uuid_by_name(&mut self, name: &str, value: Uuid) -> Result<&mut Statement,CassError> {unsafe{
	  let bytes = value.as_bytes();
	  let my_uuid:[u8,..16]=
	  [bytes[0],bytes[1],bytes[2],bytes[3],bytes[4],bytes[5],bytes[6],bytes[7],
	   bytes[8],bytes[9],bytes[10],bytes[11],bytes[12],bytes[13],bytes[14],bytes[15]];

    self.last_error = CassError{cass_error:statement_internal::cass_statement_bind_uuid_by_name(self.cass_statement,name.as_ptr() as *const i8,my_uuid)};
    Ok(self)
  }}

  pub fn bind_inet_by_name(&mut self, name: &str, value: CassInet) -> Result<&mut Statement,CassError> {unsafe{
    self.last_error = CassError{cass_error:statement_internal::cass_statement_bind_inet_by_name(self.cass_statement,name.as_ptr() as *const i8,value.cass_inet)};
    Ok(self)
  }}

  pub fn bind_decimal_by_name(&mut self, name: &str, value: CassDecimal) -> Result<&mut Statement,CassError> {unsafe{
    self.last_error = CassError{cass_error:statement_internal::cass_statement_bind_decimal_by_name(self.cass_statement,name.as_ptr() as *const i8,value.cass_decimal)};
    Ok(self)
  }}

  pub fn bind_collection_by_name(&mut self,name: &str, collection: CassCollection) -> Result<&mut Statement,CassError> {unsafe{
    self.last_error = CassError{cass_error:statement_internal::cass_statement_bind_collection_by_name(self.cass_statement,name.as_ptr() as *const i8,&*collection.cass_collection)};
    Ok(self)
  }}


  pub fn bind_custom_by_name(&mut self, name: &str, size: types_internal::CassSizeType, output: *mut *mut u8) -> Result<&mut Statement,CassError> {unsafe{
    self.last_error = CassError{cass_error:statement_internal::cass_statement_bind_custom_by_name(self.cass_statement,name.as_ptr() as *const i8,size,output)};
    Ok(self)
  }}

  pub fn bind_int64(&mut self, index: types_internal::CassSizeType, value: i64) -> Result<&mut Statement,CassError> {unsafe{
    self.last_error = CassError{cass_error:statement_internal::cass_statement_bind_int64(self.cass_statement,index,value)};
    Ok(self)
  }}

  pub fn bind_float(&mut self, index: types_internal::CassSizeType, value: f32) -> Result<&mut Statement,CassError> {unsafe{
    self.last_error = CassError{cass_error:statement_internal::cass_statement_bind_float(self.cass_statement,index,value)};
    Ok(self)
  }}

  pub fn bind_double(&mut self, index: types_internal::CassSizeType, value: f64) -> Result<&mut Statement,CassError> {unsafe{
    self.last_error = CassError{cass_error:statement_internal::cass_statement_bind_double(self.cass_statement,index,value)};
    Ok(self)
  }}

  pub fn bind_bool(&mut self, index: types_internal::CassSizeType, value: types_internal::CassBoolType) -> Result<&mut Statement,CassError> {unsafe{
    self.last_error = CassError{cass_error:statement_internal::cass_statement_bind_bool(self.cass_statement,index,value)};
    Ok(self)
  }}

  pub fn bind_string(&mut self, index: types_internal::CassSizeType, value: &str) -> Result<&mut Statement,CassError> {unsafe{
    let cass_string = CassValue::str_to_cass_string(value);
    self.last_error = CassError{cass_error:statement_internal::cass_statement_bind_string(self.cass_statement,index,cass_string)};
    Ok(self)
  }}

  pub fn bind_str(&mut self, index: types_internal::CassSizeType, value: &str) -> Result<&mut Statement,CassError> {
    self.bind_string(index, value)
  }


  pub fn bind_bytes(&mut self, index: types_internal::CassSizeType, value: CassBytes) -> Result<&mut Statement,CassError> {unsafe{
    self.last_error = CassError{cass_error:statement_internal::cass_statement_bind_bytes(self.cass_statement,index,value.cass_bytes)};
    Ok(self)
  }}

  pub fn bind_uuid(&mut self, index: types_internal::CassSizeType, value: Uuid) -> Result<&mut Statement,CassError> {unsafe{
	  let bytes = value.as_bytes();
	  let my_uuid:[u8,..16]=
	  [bytes[0],bytes[1],bytes[2],bytes[3],bytes[4],bytes[5],bytes[6],bytes[7],
	   bytes[8],bytes[9],bytes[10],bytes[11],bytes[12],bytes[13],bytes[14],bytes[15]];

    self.last_error = CassError{cass_error:statement_internal::cass_statement_bind_uuid(self.cass_statement,index,my_uuid)};
    Ok(self)
  }}

  pub fn bind_inet(&mut self, index: types_internal::CassSizeType, value: CassInet) -> Result<&mut Statement,CassError> {unsafe{
    self.last_error = CassError{cass_error:statement_internal::cass_statement_bind_inet(self.cass_statement,index,value.cass_inet)};
    Ok(self)
  }}

  pub fn bind_decimal(&mut self, index: types_internal::CassSizeType, value: CassDecimal) -> Result<&mut Statement,CassError> {unsafe{
    self.last_error = CassError{cass_error:statement_internal::cass_statement_bind_decimal(self.cass_statement,index,value.cass_decimal)};
    Ok(self)
  }}

  pub fn bind_custom(&mut self, index: types_internal::CassSizeType, size: types_internal::CassSizeType, output: *mut *mut u8) -> Result<&mut Statement,CassError> {unsafe{
    self.last_error = CassError{cass_error:statement_internal::cass_statement_bind_custom(self.cass_statement,index,size,output)};
    Ok(self)
  }}

  pub fn bind_collection(&mut self, index: types_internal::CassSizeType, collection: CassCollection) -> Result<&mut Statement,CassError> {unsafe{
    self.last_error = CassError{cass_error:statement_internal::cass_statement_bind_collection(self.cass_statement,index,&*collection.cass_collection)};
    Ok(self)
  }}
}

#[allow(dead_code)]
pub struct Prepared {
  pub cass_prepared:*const statement_internal::CassPrepared,
}

#[allow(dead_code)]
impl Prepared {
  pub fn new(statement_string:&str, session:Session) -> CassFuture {unsafe{
    let cass_string = CassValue::str_to_cass_string(statement_string);
    let state:*mut future_internal::CassFuture = session_internal::cass_session_prepare(session.cass_session,cass_string);
    let prepared = CassFuture{cass_future:state};
    prepared
  }}

  pub fn free(&mut self) {unsafe{
     statement_internal::cass_prepared_free(self.cass_prepared);
  }}
  
  pub fn cass_prepared_bind(&mut self) -> Statement {unsafe{
	 Statement{cass_statement:statement_internal::cass_prepared_bind(self.cass_prepared),last_error:CassError::new(0)}
  }}

#[allow(unused_variables)]
  pub fn bind(&mut self, parameter_count: types_internal::CassSizeType) -> Statement {unsafe{
    Statement{cass_statement:statement_internal::cass_prepared_bind(self.cass_prepared),last_error:CassError::new(0)}
  }}
}

impl Drop for Prepared {
  fn drop(&mut self) {
    self.free();
  }
}

pub mod internal {
  use types::internal as types_internal;
  use error::internal as error_internal;
  use result::internal as result_internal;
  use collection::internal as collection_internal;
  use consistency;
  
  pub enum CassPrepared { }
  pub enum CassStatement { }

  #[link(name = "cassandra")]
  extern "C" {
    pub fn cass_statement_new(query: types_internal::CassString, parameter_count: types_internal::CassSizeType) -> *mut CassStatement;
    pub fn cass_statement_free(statement: *mut CassStatement);
    pub fn cass_statement_add_key_index(statement: *mut CassStatement, index: types_internal::CassSizeType) -> error_internal::CassError;
    pub fn cass_statement_set_keyspace(statement: *mut CassStatement, keyspace: *const ::libc::c_char) -> error_internal::CassError;
    pub fn cass_statement_set_consistency(statement: *mut CassStatement, consistency: consistency::CassConsistency) -> error_internal::CassError;
    pub fn cass_statement_set_serial_consistency(statement: *mut CassStatement, serial_consistency: consistency::CassConsistency) -> error_internal::CassError;
    pub fn cass_statement_set_paging_size(statement: *mut CassStatement, page_size: ::libc::c_int) -> error_internal::CassError;
    pub fn cass_statement_set_paging_state(statement: *mut CassStatement, result: *const result_internal::CassResult) -> error_internal::CassError;
    pub fn cass_statement_bind_null(statement: *mut CassStatement, index: types_internal::CassSizeType) -> error_internal::CassError;
    pub fn cass_statement_bind_int32(statement: *mut CassStatement, index:types_internal:: CassSizeType, value: i32) -> error_internal::CassError;
    pub fn cass_statement_bind_int64(statement: *mut CassStatement, index: types_internal::CassSizeType, value: i64) -> error_internal::CassError;
    pub fn cass_statement_bind_float(statement: *mut CassStatement, index: types_internal::CassSizeType, value: f32) -> error_internal::CassError;
    pub fn cass_statement_bind_double(statement: *mut CassStatement, index: types_internal::CassSizeType, value: f64) -> error_internal::CassError;
    pub fn cass_statement_bind_bool(statement: *mut CassStatement, index: types_internal::CassSizeType, value: types_internal::CassBoolType) -> error_internal::CassError;
    pub fn cass_statement_bind_string(statement: *mut CassStatement, index: types_internal::CassSizeType, value: types_internal::CassString) -> error_internal::CassError;
    pub fn cass_statement_bind_bytes(statement: *mut CassStatement, index: types_internal::CassSizeType, value: types_internal::CassBytes) -> error_internal::CassError;
    pub fn cass_statement_bind_uuid(statement: *mut CassStatement, index: types_internal::CassSizeType, value: types_internal::CassUuid) -> error_internal::CassError;
    pub fn cass_statement_bind_inet(statement: *mut CassStatement, index: types_internal::CassSizeType, value: types_internal::CassInet) -> error_internal::CassError;
    pub fn cass_statement_bind_decimal(statement: *mut CassStatement, index: types_internal::CassSizeType, value: types_internal::CassDecimal) -> error_internal::CassError;
    pub fn cass_statement_bind_custom(statement: *mut CassStatement, index: types_internal::CassSizeType, size: types_internal::CassSizeType, output: *mut *mut u8) -> error_internal::CassError;
    pub fn cass_statement_bind_collection(statement: *mut CassStatement, index: types_internal::CassSizeType, collection: *const collection_internal::CassCollection) -> error_internal::CassError;
    pub fn cass_statement_bind_int32_by_name(statement: *mut CassStatement, name: *const ::libc::c_char, value: i32) -> error_internal::CassError;
    pub fn cass_statement_bind_int64_by_name(statement: *mut CassStatement, name: *const ::libc::c_char, value: i64) -> error_internal::CassError;
    pub fn cass_statement_bind_float_by_name(statement: *mut CassStatement, name: *const ::libc::c_char, value: f32) -> error_internal::CassError;
    pub fn cass_statement_bind_double_by_name(statement: *mut CassStatement, name: *const ::libc::c_char, value: f64) -> error_internal::CassError;
    pub fn cass_statement_bind_bool_by_name(statement: *mut CassStatement, name: *const ::libc::c_char, value: types_internal::CassBoolType) -> error_internal::CassError;
    pub fn cass_statement_bind_string_by_name(statement: *mut CassStatement, name: *const ::libc::c_char, value: types_internal::CassString) -> error_internal::CassError;
    pub fn cass_statement_bind_bytes_by_name(statement: *mut CassStatement, name: *const ::libc::c_char, value: types_internal::CassBytes) -> error_internal::CassError;
    pub fn cass_statement_bind_uuid_by_name(statement: *mut CassStatement, name: *const ::libc::c_char, value: types_internal::CassUuid) -> error_internal::CassError;
    pub fn cass_statement_bind_inet_by_name(statement: *mut CassStatement, name: *const ::libc::c_char, value: types_internal::CassInet) -> error_internal::CassError;
    pub fn cass_statement_bind_decimal_by_name(statement: *mut CassStatement, name: *const ::libc::c_char, value: types_internal::CassDecimal) -> error_internal::CassError;
    pub fn cass_statement_bind_custom_by_name(statement: *mut CassStatement, name: *const ::libc::c_char, size: types_internal::CassSizeType, output: *mut *mut u8) -> error_internal::CassError;
    pub fn cass_statement_bind_collection_by_name(statement: *mut CassStatement, name: *const ::libc::c_char, collection: *const collection_internal::CassCollection) -> error_internal::CassError;

    pub fn cass_prepared_free(prepared: *const CassPrepared);
    pub fn cass_prepared_bind(prepared: *const CassPrepared) -> *mut CassStatement;

  }
}
