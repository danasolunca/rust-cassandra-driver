use collection;
use collection::_CassCollection;
use collection::CassCollection;
use consistency;
use consistency::CassConsistency;
use error::_CassError;
use error::CassError;
use future::CassFuture;
use result;
use result::CassResult;
use session;
use session::CassSession;
use types::_CassBoolType;
use types::_CassSizeType;
use types::Decimal;
use types::_CassInet;
use types::_CassBytes;
use types::_CassString;
use types::CassValue;
use uuid::Uuid;
use types::_CassUuid;
use types::_Bytes;

use std::io::net::ip::IpAddr;


use libc::c_char;
use libc::c_int;

use std::fmt;
use std::fmt::Show;
use std::fmt::Formatter;
use std::result::Result as RustResult;

impl Show for CassStatement {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "(Statement:{})", self)
  }
}

impl Drop for CassStatement {
  fn drop(&mut self) {unsafe{
    cass_statement_free(self)
  }}
}

  pub trait CassBindable {
    fn bind_by_name(&self, name: &str, statement: &mut CassStatement) -> u32;
    fn bind(&self, index: _CassSizeType, statement: &mut CassStatement) -> u32;
  }

  impl CassBindable for Uuid {
    fn bind(&self, index: _CassSizeType, _statement: &mut CassStatement)-> u32 {unsafe{
        cass_statement_bind_uuid(_statement,index, CassValue::uuid_to_cassuuid(self))
    }}

    fn bind_by_name(&self, name: &str, _statement: &mut CassStatement)-> u32 {unsafe{
      cass_statement_bind_uuid_by_name(_statement,name.as_ptr() as *const i8, CassValue::uuid_to_cassuuid(self))
    }}
  }

  impl CassBindable for String {
    fn bind(&self, index: _CassSizeType, _statement: &mut CassStatement)-> u32 {unsafe{
        cass_statement_bind_string(_statement,index,CassValue::string_to_cass_string(self))
    }}

    fn bind_by_name(&self, name: &str, _statement: &mut CassStatement)-> u32 {unsafe{
      cass_statement_bind_string_by_name(_statement,name.as_ptr() as *const i8, CassValue::string_to_cass_string(self))
    }}
  }

  impl CassBindable for &'static str {
      fn bind(&self, index: _CassSizeType, _statement: &mut CassStatement)-> u32 {unsafe{
        cass_statement_bind_string(_statement,index,CassValue::string_to_cass_string(&self.to_string()))  
    }}
    
    fn bind_by_name(&self, name: &str, _statement: &mut CassStatement)-> u32 {unsafe{
      cass_statement_bind_string_by_name(_statement,name.as_ptr() as *const i8, CassValue::str_to_cass_string(*self))
    }}
  }

  impl CassBindable for bool {
    fn bind(&self, index: _CassSizeType, _statement: &mut CassStatement)-> u32 {unsafe{
      cass_statement_bind_bool(_statement,index,if *self == true {1} else {0})
    }}
    
    fn bind_by_name(&self, name: &str, _statement: &mut CassStatement)-> u32 {unsafe{
      cass_statement_bind_bool_by_name(_statement,name.as_ptr() as *const i8,if *self == true {1} else {0})
    }}
  }
    
  impl CassBindable for f32 {
    fn bind(&self, index: _CassSizeType, _statement: &mut CassStatement)-> u32 {unsafe{
      cass_statement_bind_float(_statement,index,*self)
    }}
    
    fn bind_by_name(&self, name: &str, _statement: &mut CassStatement)-> u32 {unsafe{
      cass_statement_bind_float_by_name(_statement,name.as_ptr() as *const i8,*self)
    }}
  }

  impl CassBindable for f64 {
    fn bind(&self, index: _CassSizeType, _statement: &mut CassStatement)-> u32 {unsafe{
      cass_statement_bind_double(_statement,index,*self)
    }}
    
    fn bind_by_name(&self, name: &str, _statement: &mut CassStatement)-> u32 {unsafe{
      cass_statement_bind_double_by_name(_statement,name.as_ptr() as *const i8,*self)
    }}
  }

  impl CassBindable for i32 {
    fn bind(&self, index: _CassSizeType, _statement: &mut CassStatement)-> u32 {unsafe{
      cass_statement_bind_int32(_statement,index,*self)
    }}
    
    fn bind_by_name(&self, name: &str, _statement: &mut CassStatement)-> u32 {unsafe{
      cass_statement_bind_int32_by_name(_statement,name.as_ptr() as *const i8,*self)
    }}
  }

  impl CassBindable for i64 {
    fn bind(&self, index: _CassSizeType, _statement: &mut CassStatement)-> u32 {unsafe{
      cass_statement_bind_int64(_statement,index,*self)
    }}
    
    fn bind_by_name(&self, name: &str, _statement: &mut CassStatement)-> u32 {unsafe{
      cass_statement_bind_int64_by_name(_statement,name.as_ptr() as *const i8,*self)
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
    fn bind(&self, index: _CassSizeType, _statement: &mut CassStatement)-> u32 {unsafe{
      cass_statement_bind_inet(_statement,index,CassValue::ipaddr2cassinet(*self))
    }}

    fn bind_by_name(&self, name: &str, _statement: &mut CassStatement)-> u32 {unsafe{
      cass_statement_bind_inet_by_name(_statement,name.as_ptr() as *const i8,CassValue::ipaddr2cassinet(*self))
    }}
  }

  impl CassBindable for _Bytes {
    fn bind(&self, index: _CassSizeType, _statement: &mut CassStatement)-> u32 {unsafe{
      cass_statement_bind_bytes(_statement,index,CassValue::bytes2cassbytes(self))
    }}

    fn bind_by_name(&self, name: &str, _statement: &mut CassStatement)-> u32 {unsafe{
      cass_statement_bind_bytes_by_name(_statement,name.as_ptr() as *const i8, CassValue::bytes2cassbytes(self))
    }}
  }

  impl CassBindable for collection::_CassCollection {
    fn bind(&self, index: _CassSizeType, _statement: &mut CassStatement)-> u32 {unsafe{
      cass_statement_bind_collection(_statement,index,self)
    }}

    fn bind_by_name(&self, name: &str, _statement: &mut CassStatement)-> u32 {unsafe{
      cass_statement_bind_collection_by_name(_statement,name.as_ptr() as *const i8,self)
    }}
  }

#[allow(dead_code)]
impl CassStatement {
  pub fn bind_by_idx<T: CassBindable>(&mut self, index: _CassSizeType, value: T) -> RustResult<&mut CassStatement,CassError> {
    let result:u32 = value.bind(index, self);
    Ok(self)
  }

  pub fn bind_by_name<T: CassBindable>(&mut self, name: &str, value: T) -> RustResult<&mut CassStatement,CassError> {
    let result:u32=value.bind_by_name(name, self);
    Ok(self)
  }

  //~ pub fn bind<T: CassBindable>(&mut self, value: T) -> RustResult<&mut CassStatement,Error> {
    //~ self.last_error = Error{cass_error:value.bind(self.bound_index, self)};
    //~ self.bound_index+=1;
    //~ Ok(self)
  //~ }

  pub fn bind_all(&mut self, values: &[&CassBindable]) -> RustResult<&mut CassStatement,CassError> {
    let mut index = 0u64;
    for value in (values.iter()) {
      //FIXME
      let result:u32=value.bind(index, self);
      index+=1;
    }
    Ok(self)
  }
  
  pub fn new(statement_string: &str, parameter_count: _CassSizeType) ->  &mut CassStatement {unsafe{
    let cass_string = CassValue::str_to_cass_string(statement_string);
    &mut*cass_statement_new(cass_string,parameter_count)
  }}

  pub fn build_from_string(statement_string:&String, parameter_count: _CassSizeType) -> &mut CassStatement {
    CassStatement::new(statement_string.as_slice(),parameter_count)
  }

  pub fn build_from_str(statement_string:&str, parameter_count: _CassSizeType) -> &mut CassStatement {
    CassStatement::new(statement_string.as_slice(),parameter_count)
  }

  pub fn set_paging_size( &mut self, page_size: ::libc::c_int) -> Option<CassError> {unsafe{
    cass_statement_set_paging_size(self,page_size);
    None
    //if error.is_error() {return Some(error)} else {return None}
  }}

  pub fn set_paging_state(&mut self, result: &mut RustResult<&CassResult,CassError>) -> Option<CassError> {unsafe{
    match result.ok() {
      Some(r) => {
        let resp = cass_statement_set_paging_state(self,r);
        if resp > 0 {
	      Some(CassError{err:resp})
	    } 
	    else {
	      None
	    }
	  },
      //FIXME can this happen?
    None => Some(CassError{err:1u32}),
    }
  }}

  pub fn set_serial_consistency(&mut self) -> RustResult<&mut CassStatement,CassError> {unsafe{
    cass_statement_set_serial_consistency(self,CassConsistency::SERIAL);
    Ok(self)
  }}

  pub fn add_key_index(&mut self, index:u64) -> RustResult<&mut CassStatement,CassError> {unsafe{
    cass_statement_add_key_index(self,index);
    Ok(self)
  }}

  pub fn set_consistency(&mut self,consistency:CassConsistency) -> RustResult<&mut CassStatement,CassError> {unsafe{
    cass_statement_set_consistency(self,consistency);
    Ok(self)
  }}

  pub fn set_keyspace(&mut self, keyspace:&str) -> RustResult<&mut CassStatement,_CassError> {unsafe{
    cass_statement_set_keyspace(self,keyspace.as_ptr() as *const i8);
    Ok(self)
  }}

  pub fn bind_collection(&mut self, index: _CassSizeType, collection: CassCollection) -> RustResult<&mut CassStatement,CassError> {unsafe{
    cass_statement_bind_collection(self,index,&*collection.collection);
    Ok(self)
  }}
}


impl CassPrepared {
  pub fn new(statement_string:String, session:&mut CassSession) -> &CassFuture {unsafe{
    let cass_string = CassValue::str_to_cass_string(statement_string.as_slice());
    &*session::cass_session_prepare(session,cass_string)
  }}

  fn free(&self) {unsafe{
     cass_prepared_free(self);
  }}
  
  pub fn cass_prepared_bind(&self) -> &CassStatement {unsafe{
	 &*cass_prepared_bind(self)
  }}

#[allow(unused_variables)]
  pub fn bind(&self, parameter_count: _CassSizeType) -> &CassStatement {unsafe{
    &*cass_prepared_bind(self)
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
  fn cass_statement_new(query: _CassString, parameter_count: _CassSizeType) -> *mut CassStatement;
  fn cass_statement_free(statement: *mut CassStatement);
  fn cass_statement_add_key_index(statement: *mut CassStatement, index: _CassSizeType) -> _CassError;
  fn cass_statement_set_keyspace(statement: *mut CassStatement, keyspace: *const c_char) -> _CassError;
  fn cass_statement_set_consistency(statement: *mut CassStatement, consistency: consistency::CassConsistency) -> _CassError;
  fn cass_statement_set_serial_consistency(statement: *mut CassStatement, serial_consistency: consistency::CassConsistency) -> _CassError;
  fn cass_statement_set_paging_size(statement: *mut CassStatement, page_size: c_int) -> _CassError;
  fn cass_statement_set_paging_state(statement: *mut CassStatement, result: *const result::CassResult) -> _CassError;
  fn cass_statement_bind_null(statement: *mut CassStatement, index: _CassSizeType) -> _CassError;
  fn cass_statement_bind_int32(statement: *mut CassStatement, index: _CassSizeType, value: i32) -> _CassError;
  fn cass_statement_bind_int64(statement: *mut CassStatement, index: _CassSizeType, value: i64) -> _CassError;
  fn cass_statement_bind_float(statement: *mut CassStatement, index: _CassSizeType, value: f32) -> _CassError;
  fn cass_statement_bind_double(statement: *mut CassStatement, index: _CassSizeType, value: f64) -> _CassError;
  fn cass_statement_bind_bool(statement: *mut CassStatement, index: _CassSizeType, value: _CassBoolType) -> _CassError;
  fn cass_statement_bind_string(statement: *mut CassStatement, index: _CassSizeType, value: _CassString) -> _CassError;
  fn cass_statement_bind_bytes(statement: *mut CassStatement, index: _CassSizeType, value: _CassBytes) -> _CassError;
  fn cass_statement_bind_uuid(statement: *mut CassStatement, index: _CassSizeType, value: _CassUuid) -> _CassError;
  fn cass_statement_bind_inet(statement: *mut CassStatement, index: _CassSizeType, value: _CassInet) -> _CassError;
  fn cass_statement_bind_decimal(statement: *mut CassStatement, index: _CassSizeType, value: Decimal) -> _CassError;
  fn cass_statement_bind_custom(statement: *mut CassStatement, index: _CassSizeType, size: _CassSizeType, output: *mut *mut u8) -> _CassError;
  fn cass_statement_bind_collection(statement: *mut CassStatement, index: _CassSizeType, collection: *const collection::_CassCollection) -> _CassError;
  fn cass_statement_bind_int32_by_name(statement: *mut CassStatement, name: *const c_char, value: i32) -> _CassError;
  fn cass_statement_bind_int64_by_name(statement: *mut CassStatement, name: *const c_char, value: i64) -> _CassError;
  fn cass_statement_bind_float_by_name(statement: *mut CassStatement, name: *const c_char, value: f32) -> _CassError;
  fn cass_statement_bind_double_by_name(statement: *mut CassStatement, name: *const c_char, value: f64) -> _CassError;
  fn cass_statement_bind_bool_by_name(statement: *mut CassStatement, name: *const c_char, value: _CassBoolType) -> _CassError;
  fn cass_statement_bind_string_by_name(statement: *mut CassStatement, name: *const c_char, value: _CassString) -> _CassError;
  fn cass_statement_bind_bytes_by_name(statement: *mut CassStatement, name: *const c_char, value: _CassBytes) -> _CassError;
  fn cass_statement_bind_uuid_by_name(statement: *mut CassStatement, name: *const c_char, value: _CassUuid) -> _CassError;
  fn cass_statement_bind_inet_by_name(statement: *mut CassStatement, name: *const c_char, value: _CassInet) -> _CassError;
  fn cass_statement_bind_decimal_by_name(statement: *mut CassStatement, name: *const c_char, value: Decimal) -> _CassError;
  fn cass_statement_bind_custom_by_name(statement: *mut CassStatement, name: *const c_char, size: _CassSizeType, output: *mut *mut u8) -> _CassError;
  fn cass_statement_bind_collection_by_name(statement: *mut CassStatement, name: *const c_char, collection: *const collection::_CassCollection) -> _CassError;
  fn cass_prepared_free(prepared: *const CassPrepared);
  fn cass_prepared_bind(prepared: *const CassPrepared) -> *mut CassStatement;
}

