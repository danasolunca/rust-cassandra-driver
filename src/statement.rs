use collection;
use collection::CassCollection;
use collection::Collection;
use consistency;
use consistency::CassConsistency;
use error::CassError;
use error::Error;
use future::CassFuture;
use result;
use result::CassResult;
use session;
use session::CassSession;
use types::CassBoolType;
use types::CassSizeType;
use types::CassDecimal;
use types::CassInet;
use types::CassBytes;
use types::CassString;
use types::Value;
use uuid::Uuid;
use types::_CassUuid;
use types::Bytes;

use std::io::net::ip::IpAddr;


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
    fn bind_by_name(&self, name: &str, statement: &mut Statement) -> u32;
    fn bind(&self, index: CassSizeType, statement: &mut Statement) -> u32;
  }

  impl CassBindable for Uuid {
    fn bind(&self, index: CassSizeType, _statement: &mut Statement)-> u32 {unsafe{
        cass_statement_bind_uuid(_statement.cass_statement,index, Value::uuid_to_cassuuid(self))
    }}

    fn bind_by_name(&self, name: &str, _statement: &mut Statement)-> u32 {unsafe{
      cass_statement_bind_uuid_by_name(_statement.cass_statement,name.as_ptr() as *const i8, Value::uuid_to_cassuuid(self))
    }}
  }

  impl CassBindable for String {
    fn bind(&self, index: CassSizeType, _statement: &mut Statement)-> u32 {unsafe{
        cass_statement_bind_string(_statement.cass_statement,index,Value::string_to_cass_string(self))
    }}

    fn bind_by_name(&self, name: &str, _statement: &mut Statement)-> u32 {unsafe{
      cass_statement_bind_string_by_name(_statement.cass_statement,name.as_ptr() as *const i8, Value::string_to_cass_string(self))
    }}
  }

  impl CassBindable for &'static str {
      fn bind(&self, index: CassSizeType, _statement: &mut Statement)-> u32 {unsafe{
        cass_statement_bind_string(_statement.cass_statement,index,Value::string_to_cass_string(&self.to_string()))  
    }}
    
    fn bind_by_name(&self, name: &str, _statement: &mut Statement)-> u32 {unsafe{
      cass_statement_bind_string_by_name(_statement.cass_statement,name.as_ptr() as *const i8, Value::str_to_cass_string(*self))
    }}
  }

  impl CassBindable for bool {
    fn bind(&self, index: CassSizeType, _statement: &mut Statement)-> u32 {unsafe{
      cass_statement_bind_bool(_statement.cass_statement,index,if *self == true {1} else {0})
    }}
    
    fn bind_by_name(&self, name: &str, _statement: &mut Statement)-> u32 {unsafe{
      cass_statement_bind_bool_by_name(_statement.cass_statement,name.as_ptr() as *const i8,if *self == true {1} else {0})
    }}
  }
    
  impl CassBindable for f32 {
    fn bind(&self, index: CassSizeType, _statement: &mut Statement)-> u32 {unsafe{
      cass_statement_bind_float(_statement.cass_statement,index,*self)
    }}
    
    fn bind_by_name(&self, name: &str, _statement: &mut Statement)-> u32 {unsafe{
      cass_statement_bind_float_by_name(_statement.cass_statement,name.as_ptr() as *const i8,*self)
    }}
  }

  impl CassBindable for f64 {
    fn bind(&self, index: CassSizeType, _statement: &mut Statement)-> u32 {unsafe{
      cass_statement_bind_double(_statement.cass_statement,index,*self)
    }}
    
    fn bind_by_name(&self, name: &str, _statement: &mut Statement)-> u32 {unsafe{
      cass_statement_bind_double_by_name(_statement.cass_statement,name.as_ptr() as *const i8,*self)
    }}
  }

  impl CassBindable for i32 {
    fn bind(&self, index: CassSizeType, _statement: &mut Statement)-> u32 {unsafe{
      cass_statement_bind_int32(_statement.cass_statement,index,*self)
    }}
    
    fn bind_by_name(&self, name: &str, _statement: &mut Statement)-> u32 {unsafe{
      cass_statement_bind_int32_by_name(_statement.cass_statement,name.as_ptr() as *const i8,*self)
    }}
  }

  impl CassBindable for i64 {
    fn bind(&self, index: CassSizeType, _statement: &mut Statement)-> u32 {unsafe{
      cass_statement_bind_int64(_statement.cass_statement,index,*self)
    }}
    
    fn bind_by_name(&self, name: &str, _statement: &mut Statement)-> u32 {unsafe{
      cass_statement_bind_int64_by_name(_statement.cass_statement,name.as_ptr() as *const i8,*self)
    }}
  }

  //~ impl CassBindable for BigDecimal {
    //~ fn bind(&self, index: CassSizeType, _statement: &mut Statement)-> u32 {unsafe{
      //~ let foo=self;
      //~ cass_statement_bind_decimal(_statement.cass_statement,index,*foo)
    //~ }}

    //~ fn bind_by_name(&self, name: &str, _statement: &mut Statement)-> u32 {unsafe{
      //~ cass_statement_bind_decimal_by_name(_statement.cass_statement,name.as_ptr() as *const i8,self.cass_decimal)
    //~ }}
  //~ }

  impl CassBindable for IpAddr {
    fn bind(&self, index: CassSizeType, _statement: &mut Statement)-> u32 {unsafe{
      cass_statement_bind_inet(_statement.cass_statement,index,Value::ipaddr2cassinet(*self))
    }}

    fn bind_by_name(&self, name: &str, _statement: &mut Statement)-> u32 {unsafe{
      cass_statement_bind_inet_by_name(_statement.cass_statement,name.as_ptr() as *const i8,Value::ipaddr2cassinet(*self))
    }}
  }

  impl CassBindable for Bytes {
    fn bind(&self, index: CassSizeType, _statement: &mut Statement)-> u32 {unsafe{
      cass_statement_bind_bytes(_statement.cass_statement,index,Value::bytes2cassbytes(self))
    }}

    fn bind_by_name(&self, name: &str, _statement: &mut Statement)-> u32 {unsafe{
      cass_statement_bind_bytes_by_name(_statement.cass_statement,name.as_ptr() as *const i8, Value::bytes2cassbytes(self))
    }}
  }

  impl CassBindable for collection::CassCollection {
    fn bind(&self, index: CassSizeType, _statement: &mut Statement)-> u32 {unsafe{
      cass_statement_bind_collection(_statement.cass_statement,index,self)
    }}

    fn bind_by_name(&self, name: &str, _statement: &mut Statement)-> u32 {unsafe{
      cass_statement_bind_collection_by_name(_statement.cass_statement,name.as_ptr() as *const i8,self)
    }}
  }

#[allow(dead_code)]
impl Statement {
  pub fn bind_by_idx<T: CassBindable>(&mut self, index: CassSizeType, value: T) -> RustResult<&mut Statement,Error> {
    self.last_error = Error{cass_error:value.bind(index, self)};
    Ok(self)
  }

  pub fn bind_by_name<T: CassBindable>(&mut self, name: &str, value: T) -> RustResult<&mut Statement,Error> {
    self.last_error = Error{cass_error:value.bind_by_name(name, self)};
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

  pub fn set_paging_state(&mut self, result: &mut RustResult<&CassResult,Error>) -> Option<Error> {unsafe{
    match result.ok() {
      Some(r) => {
        let resp = cass_statement_set_paging_state(self.cass_statement,r);
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

  pub fn bind_collection(&mut self, index: CassSizeType, collection: Collection) -> RustResult<&mut Statement,Error> {unsafe{
    self.last_error = Error{cass_error:cass_statement_bind_collection(self.cass_statement,index,&*collection.cass_collection)};
    Ok(self)
  }}
}


impl CassPrepared {
  pub fn new(statement_string:String, session:&mut CassSession) -> &CassFuture {unsafe{
    let cass_string = Value::str_to_cass_string(statement_string.as_slice());
    &*session::cass_session_prepare(session,cass_string)
  }}

  fn free(&self) {unsafe{
     cass_prepared_free(self);
  }}
  
  pub fn cass_prepared_bind(&self) -> Statement {unsafe{
	 Statement{cass_statement:cass_prepared_bind(self),last_error:Error::new(0),bound_index:0}
  }}

#[allow(unused_variables)]
  pub fn bind(&self, parameter_count: CassSizeType) -> Statement {unsafe{
    Statement{cass_statement:cass_prepared_bind(self),last_error:Error::new(0),bound_index:0}
  }}
}

impl Drop for CassPrepared {
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
  fn cass_statement_bind_uuid(statement: *mut CassStatement, index: CassSizeType, value: _CassUuid) -> CassError;
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
  fn cass_statement_bind_uuid_by_name(statement: *mut CassStatement, name: *const c_char, value: _CassUuid) -> CassError;
  fn cass_statement_bind_inet_by_name(statement: *mut CassStatement, name: *const c_char, value: CassInet) -> CassError;
  fn cass_statement_bind_decimal_by_name(statement: *mut CassStatement, name: *const c_char, value: CassDecimal) -> CassError;
  fn cass_statement_bind_custom_by_name(statement: *mut CassStatement, name: *const c_char, size: CassSizeType, output: *mut *mut u8) -> CassError;
  fn cass_statement_bind_collection_by_name(statement: *mut CassStatement, name: *const c_char, collection: *const collection::CassCollection) -> CassError;
  fn cass_prepared_free(prepared: *const CassPrepared);
  fn cass_prepared_bind(prepared: *const CassPrepared) -> *mut CassStatement;
}

