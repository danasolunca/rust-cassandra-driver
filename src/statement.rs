use collection;
use collection::CassCollection;
use collection::Collection;
use consistency;
use consistency::CassConsistency;
use error::CassError;
use error::Error;
use future::Future;
use future::CassFuture;
use result;
use result::Result;
use session;
use session::Session;
use types::CassBoolType;
use types::CassSizeType;
use types::CassDecimal;
use types::CassInet;
use types::CassUuid;
use types::CassBytes;
use types::CassString;
use types::Decimal;
use types::Inet;
use types::Bytes;
use types::Value;
use uuid::Uuid;

use libc::c_char;
use libc::c_int;

use std::fmt;
use std::fmt::Show;
use std::fmt::Formatter;
use std::result::Result as RustResult;


pub struct Statement {
  pub cass_statement:*mut CassStatement,
  pub last_error:Error,
  bound_index:u64
}

impl Show for Statement {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "(Statement:{})", self)
  }
}

impl Drop for Statement {
  fn drop(&mut self) {unsafe{
    cass_statement_free(self.cass_statement)
  }}
}

  pub trait CassBindable {
    fn bind(&self, index: CassSizeType, statement: &mut Statement) -> u32;
  }

  impl CassBindable for String {
      fn bind(&self, index: CassSizeType, _statement: &mut Statement)-> u32 {unsafe{
        cass_statement_bind_string(_statement.cass_statement,index,Value::string_to_cass_string(self))
    }}
  }

  impl CassBindable for &'static str {
      fn bind(&self, index: CassSizeType, _statement: &mut Statement)-> u32 {unsafe{
        cass_statement_bind_string(_statement.cass_statement,index,Value::string_to_cass_string(&self.to_string()))
  
    }}
  }

  impl CassBindable for bool {
    fn bind(&self, index: CassSizeType, _statement: &mut Statement)-> u32 {unsafe{
      cass_statement_bind_bool(_statement.cass_statement,index,if *self == true {1} else {0})
    }}
  }
    
  impl CassBindable for f32 {
    fn bind(&self, index: CassSizeType, _statement: &mut Statement)-> u32 {unsafe{
      cass_statement_bind_float(_statement.cass_statement,index,*self)
    }}
  }

  impl CassBindable for f64 {
    fn bind(&self, index: CassSizeType, _statement: &mut Statement)-> u32 {unsafe{
      cass_statement_bind_double(_statement.cass_statement,index,*self)
    }}
  }

  impl CassBindable for i32 {
    fn bind(&self, index: CassSizeType, _statement: &mut Statement)-> u32 {unsafe{
      cass_statement_bind_int32(_statement.cass_statement,index,*self)
    }}
  }

  impl CassBindable for i64 {
    fn bind(&self, index: CassSizeType, _statement: &mut Statement)-> u32 {unsafe{
      cass_statement_bind_int64(_statement.cass_statement,index,*self)
    }}
  }

  impl CassBindable for Decimal {
    fn bind(&self, index: CassSizeType, _statement: &mut Statement)-> u32 {unsafe{
      cass_statement_bind_decimal(_statement.cass_statement,index,self.cass_decimal)
    }}
  }

  impl CassBindable for Inet {
    fn bind(&self, index: CassSizeType, _statement: &mut Statement)-> u32 {unsafe{
      cass_statement_bind_inet(_statement.cass_statement,index,self.cass_inet)
    }}
  }

  impl CassBindable for Bytes {
    fn bind(&self, index: CassSizeType, _statement: &mut Statement)-> u32 {unsafe{
      cass_statement_bind_bytes(_statement.cass_statement,index,self.cass_bytes)
    }}
  }

  impl CassBindable for collection::CassCollection {
    fn bind(&self, index: CassSizeType, _statement: &mut Statement)-> u32 {unsafe{
      cass_statement_bind_collection(_statement.cass_statement,index,self)
    }}
  }

#[allow(dead_code)]
impl Statement {
  pub fn bind_by_idx<T: CassBindable>(&mut self, index: CassSizeType, value: T) -> RustResult<&mut Statement,Error> {
    self.last_error = Error{cass_error:value.bind(index, self)};
    Ok(self)
  }

  pub fn bind<T: CassBindable>(&mut self, value: T) -> RustResult<&mut Statement,Error> {
    self.last_error = Error{cass_error:value.bind(self.bound_index, self)};
    self.bound_index+=1;
    Ok(self)
  }

  pub fn bind_all(&mut self, values: &[&CassBindable]) -> RustResult<&mut Statement,Error> {
    let mut index = 0u64;
    for value in (values.iter()) {
      self.last_error = Error{cass_error:value.bind(index, self)};
      index+=1;
    }
    Ok(self)
  }
  
  pub fn new(statement_string: &str, parameter_count: CassSizeType) ->  Statement {unsafe{
    let cass_string = Value::str_to_cass_string(statement_string);
    let statement = cass_statement_new(cass_string,parameter_count);
    Statement{cass_statement:statement,last_error:Error::new(0), bound_index:0}
  }}

  pub fn build_from_string(statement_string:&String, parameter_count: CassSizeType) -> Statement {
    Statement::new(statement_string.as_slice(),parameter_count)
  }

  pub fn build_from_str(statement_string:&str, parameter_count: CassSizeType) -> Statement {
    Statement::new(statement_string.as_slice(),parameter_count)
  }

  pub fn set_paging_size( &mut self, page_size: ::libc::c_int) -> Option<Error> {unsafe{
    let error = Error{cass_error:cass_statement_set_paging_size(self.cass_statement,page_size)};
    if error.is_error() {return Some(error)} else {return None}
  }}

  pub fn set_paging_state(&mut self, result: &mut RustResult<Result,Error>) -> Option<Error> {unsafe{
    match result.clone().ok() {
      Some(r) => {
        let resp = cass_statement_set_paging_state(self.cass_statement,r.cass_result);
        if resp > 0 {
	      Some(Error{cass_error:resp})
	    } 
	    else {
	      None
	    }
	  },
      //FIXME can this happen?
      None => Some(Error{cass_error:1u32}),
    }
  }}

  
  pub fn set_serial_consistency(&mut self) -> RustResult<&mut Statement,Error> {unsafe{
    self.last_error = Error{cass_error:cass_statement_set_serial_consistency(self.cass_statement,CassConsistency::SERIAL)};
    Ok(self)
  }}

  pub fn add_key_index(&mut self, index:u64) -> RustResult<&mut Statement,Error> {unsafe{
    self.last_error = Error{cass_error:cass_statement_add_key_index(self.cass_statement,index)};
    Ok(self)
  }}

  pub fn set_consistency(&mut self,consistency:CassConsistency) -> RustResult<&mut Statement,Error> {unsafe{
    self.last_error = Error{cass_error:cass_statement_set_consistency(self.cass_statement,consistency)};
    Ok(self)
  }}

  pub fn set_keyspace(&mut self, keyspace:&str) -> RustResult<&mut Statement,CassError> {unsafe{
    self.last_error = Error{cass_error:cass_statement_set_keyspace(self.cass_statement,keyspace.as_ptr() as *const i8)};
    Ok(self)
  }}
 
  fn bind_null(&mut self, index: CassSizeType) -> RustResult<&mut Statement,Error> {unsafe{
    self.last_error = Error{cass_error:cass_statement_bind_null(self.cass_statement,index)};
    Ok(self)
  }}

  fn bind_int32(&mut self, index: CassSizeType, value: i32) -> RustResult<&mut Statement,Error> {unsafe{
    self.last_error = Error{cass_error:cass_statement_bind_int32(self.cass_statement,index,value)};
    Ok(self)
  }}

  pub fn bind_int32_by_name(&mut self, name: &str, value: i32) -> RustResult<&mut Statement,CassError> {unsafe{
    self.last_error = Error{cass_error:cass_statement_bind_int32_by_name(self.cass_statement,name.as_ptr() as *const i8,value)};
    Ok(self)
  }}

  pub fn bind_int64_by_name(&mut self, name: &str, value: i64) -> RustResult<&mut Statement,Error> {unsafe{
    self.last_error = Error{cass_error:cass_statement_bind_int64_by_name(self.cass_statement,name.as_ptr() as *const i8,value)};
    Ok(self)
  }}

  pub fn bind_float_by_name(&mut self, name: &str, value: f32) -> RustResult<&mut Statement,Error> {unsafe{
    self.last_error = Error{cass_error:cass_statement_bind_float_by_name(self.cass_statement,name.as_ptr() as *const i8,value)};
    Ok(self)
  }}

  pub fn bind_double_by_name(&mut self, name: &str, value: f64) -> RustResult<&mut Statement,Error> {unsafe{
    self.last_error = Error{cass_error:cass_statement_bind_double_by_name(self.cass_statement,name.as_ptr() as *const i8,value)};
    Ok(self)
  }}

  pub fn bind_bool_by_name(&mut self, name: &str, value: bool) -> RustResult<&mut Statement,Error> {unsafe{
    self.last_error = Error{cass_error:cass_statement_bind_bool_by_name(self.cass_statement,name.as_ptr() as *const i8,match value {true=>1,false=>0})};
    Ok(self)
  }}

  pub fn bind_string_by_name(&mut self, name: &str, value: &str) -> RustResult<&mut Statement,Error> {unsafe{
    self.last_error = Error{cass_error:cass_statement_bind_string_by_name(self.cass_statement,name.as_ptr() as *const i8,Value::str_to_cass_string(value))};
    Ok(self)
  }}

  pub fn bind_bytes_by_name(&mut self, name: &str, value: Bytes) -> RustResult<&mut Statement,Error> {unsafe{
    self.last_error = Error{cass_error:cass_statement_bind_bytes_by_name(self.cass_statement,name.as_ptr() as *const i8,value.cass_bytes)};
    Ok(self)
  }}

  pub fn bind_uuid_by_name(&mut self, name: &str, value: Uuid) -> RustResult<&mut Statement,Error> {unsafe{
	  let bytes = value.as_bytes();
	  let my_uuid:[u8,..16]=
	  [bytes[0],bytes[1],bytes[2],bytes[3],bytes[4],bytes[5],bytes[6],bytes[7],
	   bytes[8],bytes[9],bytes[10],bytes[11],bytes[12],bytes[13],bytes[14],bytes[15]];

    self.last_error = Error{cass_error:cass_statement_bind_uuid_by_name(self.cass_statement,name.as_ptr() as *const i8,my_uuid)};
    Ok(self)
  }}

  fn bind_inet_by_name(&mut self, name: &str, value: Inet) -> RustResult<&mut Statement,Error> {unsafe{
    self.last_error = Error{cass_error:cass_statement_bind_inet_by_name(self.cass_statement,name.as_ptr() as *const i8,value.cass_inet)};
    Ok(self)
  }}

  fn bind_decimal_by_name(&mut self, name: &str, value: Decimal) -> RustResult<&mut Statement,Error> {unsafe{
    self.last_error = Error{cass_error:cass_statement_bind_decimal_by_name(self.cass_statement,name.as_ptr() as *const i8,value.cass_decimal)};
    Ok(self)
  }}

  fn bind_collection_by_name(&mut self,name: &str, collection: Collection) -> RustResult<&mut Statement,Error> {unsafe{
    self.last_error = Error{cass_error:cass_statement_bind_collection_by_name(self.cass_statement,name.as_ptr() as *const i8,&*collection.cass_collection)};
    Ok(self)
  }}


  fn bind_custom_by_name(&mut self, name: &str, size: CassSizeType, output: *mut *mut u8) -> RustResult<&mut Statement,Error> {unsafe{
    self.last_error = Error{cass_error:cass_statement_bind_custom_by_name(self.cass_statement,name.as_ptr() as *const i8,size,output)};
    Ok(self)
  }}

  fn bind_int64(&mut self, index: CassSizeType, value: i64) -> RustResult<&mut Statement,Error> {unsafe{
    self.last_error = Error{cass_error:cass_statement_bind_int64(self.cass_statement,index,value)};
    Ok(self)
  }}

  fn bind_float(&mut self, index: CassSizeType, value: f32) -> RustResult<&mut Statement,Error> {unsafe{
    self.last_error = Error{cass_error:cass_statement_bind_float(self.cass_statement,index,value)};
    Ok(self)
  }}

  fn bind_double(&mut self, index: CassSizeType, value: f64) -> RustResult<&mut Statement,Error> {unsafe{
    self.last_error = Error{cass_error:cass_statement_bind_double(self.cass_statement,index,value)};
    Ok(self)
  }}

  fn bind_bool(&mut self, index: CassSizeType, value: bool) -> RustResult<&mut Statement,Error> {unsafe{
    self.last_error = Error{cass_error:cass_statement_bind_bool(self.cass_statement,index,if value == true {1} else {0})};
    Ok(self)
  }}

  fn bind_string(&mut self, index: CassSizeType, value: String) -> RustResult<&mut Statement,Error> {unsafe{
    let cass_string = Value::string_to_cass_string(&value);
    self.last_error = Error{cass_error:cass_statement_bind_string(self.cass_statement,index,cass_string)};
    Ok(self)
  }}

  fn bind_str(&mut self, index: CassSizeType, value: &str) -> RustResult<&mut Statement,Error> {
    self.bind_str(index, value)
  }


  fn bind_bytes(&mut self, index: CassSizeType, value: Bytes) -> RustResult<&mut Statement,Error> {unsafe{
    self.last_error = Error{cass_error:cass_statement_bind_bytes(self.cass_statement,index,value.cass_bytes)};
    Ok(self)
  }}

  pub fn bind_uuid(&mut self, index: CassSizeType, value: Uuid) -> RustResult<&mut Statement,Error> {unsafe{
	  let bytes = value.as_bytes();
	  let my_uuid:[u8,..16]=
	  [bytes[0],bytes[1],bytes[2],bytes[3],bytes[4],bytes[5],bytes[6],bytes[7],
	   bytes[8],bytes[9],bytes[10],bytes[11],bytes[12],bytes[13],bytes[14],bytes[15]];

    self.last_error = Error{cass_error:cass_statement_bind_uuid(self.cass_statement,index,my_uuid)};
    Ok(self)
  }}

  fn bind_inet(&mut self, index: CassSizeType, value: Inet) -> RustResult<&mut Statement,Error> {unsafe{
    self.last_error = Error{cass_error:cass_statement_bind_inet(self.cass_statement,index,value.cass_inet)};
    Ok(self)
  }}

  fn bind_decimal(&mut self, index: CassSizeType, value: Decimal) -> RustResult<&mut Statement,Error> {unsafe{
    self.last_error = Error{cass_error:cass_statement_bind_decimal(self.cass_statement,index,value.cass_decimal)};
    Ok(self)
  }}

   fn bind_custom(&mut self, index: CassSizeType, size: CassSizeType, output: *mut *mut u8) -> RustResult<&mut Statement,Error> {unsafe{
    self.last_error = Error{cass_error:cass_statement_bind_custom(self.cass_statement,index,size,output)};
    Ok(self)
  }}

  pub fn bind_collection(&mut self, index: CassSizeType, collection: Collection) -> RustResult<&mut Statement,Error> {unsafe{
    self.last_error = Error{cass_error:cass_statement_bind_collection(self.cass_statement,index,&*collection.cass_collection)};
    Ok(self)
  }}
}

#[allow(dead_code)]
pub struct Prepared {
  pub cass_prepared:*const CassPrepared,
}

#[allow(dead_code)]
impl Prepared {
  pub fn new(statement_string:&str, session:Session) -> Future {unsafe{
    let cass_string = Value::str_to_cass_string(statement_string);
    let state:*mut CassFuture = session::cass_session_prepare(session.cass_session,cass_string);
    let prepared = Future{cass_future:state};
    prepared
  }}

  pub fn free(&mut self) {unsafe{
     cass_prepared_free(self.cass_prepared);
  }}
  
  pub fn cass_prepared_bind(&mut self) -> Statement {unsafe{
	 Statement{cass_statement:cass_prepared_bind(self.cass_prepared),last_error:Error::new(0),bound_index:0}
  }}

#[allow(unused_variables)]
  pub fn bind(&mut self, parameter_count: CassSizeType) -> Statement {unsafe{
    Statement{cass_statement:cass_prepared_bind(self.cass_prepared),last_error:Error::new(0),bound_index:0}
  }}
}

impl Drop for Prepared {
  fn drop(&mut self) {
    self.free();
  }
}

pub enum CassPrepared { }
pub enum CassStatement { }

#[link(name = "cassandra")]
extern "C" {
  fn cass_statement_new(query: CassString, parameter_count: CassSizeType) -> *mut CassStatement;
  fn cass_statement_free(statement: *mut CassStatement);
  fn cass_statement_add_key_index(statement: *mut CassStatement, index: CassSizeType) -> CassError;
  fn cass_statement_set_keyspace(statement: *mut CassStatement, keyspace: *const c_char) -> CassError;
  fn cass_statement_set_consistency(statement: *mut CassStatement, consistency: consistency::CassConsistency) -> CassError;
  fn cass_statement_set_serial_consistency(statement: *mut CassStatement, serial_consistency: consistency::CassConsistency) -> CassError;
  fn cass_statement_set_paging_size(statement: *mut CassStatement, page_size: c_int) -> CassError;
  fn cass_statement_set_paging_state(statement: *mut CassStatement, result: *const result::CassResult) -> CassError;
  fn cass_statement_bind_null(statement: *mut CassStatement, index: CassSizeType) -> CassError;
  fn cass_statement_bind_int32(statement: *mut CassStatement, index: CassSizeType, value: i32) -> CassError;
  fn cass_statement_bind_int64(statement: *mut CassStatement, index: CassSizeType, value: i64) -> CassError;
  fn cass_statement_bind_float(statement: *mut CassStatement, index: CassSizeType, value: f32) -> CassError;
  fn cass_statement_bind_double(statement: *mut CassStatement, index: CassSizeType, value: f64) -> CassError;
  fn cass_statement_bind_bool(statement: *mut CassStatement, index: CassSizeType, value: CassBoolType) -> CassError;
  fn cass_statement_bind_string(statement: *mut CassStatement, index: CassSizeType, value: CassString) -> CassError;
  fn cass_statement_bind_bytes(statement: *mut CassStatement, index: CassSizeType, value: CassBytes) -> CassError;
  fn cass_statement_bind_uuid(statement: *mut CassStatement, index: CassSizeType, value: CassUuid) -> CassError;
  fn cass_statement_bind_inet(statement: *mut CassStatement, index: CassSizeType, value: CassInet) -> CassError;
  fn cass_statement_bind_decimal(statement: *mut CassStatement, index: CassSizeType, value: CassDecimal) -> CassError;
  fn cass_statement_bind_custom(statement: *mut CassStatement, index: CassSizeType, size: CassSizeType, output: *mut *mut u8) -> CassError;
  fn cass_statement_bind_collection(statement: *mut CassStatement, index: CassSizeType, collection: *const collection::CassCollection) -> CassError;
  fn cass_statement_bind_int32_by_name(statement: *mut CassStatement, name: *const c_char, value: i32) -> CassError;
  fn cass_statement_bind_int64_by_name(statement: *mut CassStatement, name: *const c_char, value: i64) -> CassError;
  fn cass_statement_bind_float_by_name(statement: *mut CassStatement, name: *const c_char, value: f32) -> CassError;
  fn cass_statement_bind_double_by_name(statement: *mut CassStatement, name: *const c_char, value: f64) -> CassError;
  fn cass_statement_bind_bool_by_name(statement: *mut CassStatement, name: *const c_char, value: CassBoolType) -> CassError;
  fn cass_statement_bind_string_by_name(statement: *mut CassStatement, name: *const c_char, value: CassString) -> CassError;
  fn cass_statement_bind_bytes_by_name(statement: *mut CassStatement, name: *const c_char, value: CassBytes) -> CassError;
  fn cass_statement_bind_uuid_by_name(statement: *mut CassStatement, name: *const c_char, value: CassUuid) -> CassError;
  fn cass_statement_bind_inet_by_name(statement: *mut CassStatement, name: *const c_char, value: CassInet) -> CassError;
  fn cass_statement_bind_decimal_by_name(statement: *mut CassStatement, name: *const c_char, value: CassDecimal) -> CassError;
  fn cass_statement_bind_custom_by_name(statement: *mut CassStatement, name: *const c_char, size: CassSizeType, output: *mut *mut u8) -> CassError;
  fn cass_statement_bind_collection_by_name(statement: *mut CassStatement, name: *const c_char, collection: *const collection::CassCollection) -> CassError;
  fn cass_prepared_free(prepared: *const CassPrepared);
  fn cass_prepared_bind(prepared: *const CassPrepared) -> *mut CassStatement;
}

