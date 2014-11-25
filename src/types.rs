extern crate libc;

use std::string::raw;

use collection::internal as collection_internal;
use types::internal as types_internal;
use error::internal as error_internal;

use error::Error as CassError;
use libc::c_char;
use std::vec::Vec;
use std::num::Int;

use std::io::net::ip::IpAddr;
use std::io::net::ip::Ipv4Addr;
use std::io::net::ip::Ipv6Addr;


use CollectionIterator;



pub enum ValueType {
  UNKNOWN = 65535,
  CUSTOM = 0,
  ASCII = 1,
  BIGINT = 2,
  BLOB = 3,
  BOOLEAN = 4,
  COUNTER = 5,
  DECIMAL = 6,
  DOUBLE = 7,
  FLOAT = 8,
  INT = 9,
  TEXT = 10,
  TIMESTAMP = 11,
  UUID = 12,
  VARCHAR = 13,
  VARINT = 14,
  TIMEUUID = 15,
  INET = 16,
  LIST = 32,
  MAP = 33,
  SET = 34,
}

#[allow(dead_code)]
pub struct CassValue {
  pub cass_value:*const internal::CassValue
}

#[allow(dead_code)]
pub struct CassUuid {
  pub cass_uuid:internal::CassUuid,
}

#[allow(dead_code)]
pub struct CassInet {
  pub cass_inet:internal::CassInet,
}


#[allow(dead_code)]
pub struct CassBytes {
  pub cass_bytes:internal::CassBytes,
}

#[allow(dead_code)]
pub struct CassDecimal {
  pub cass_decimal:internal::CassDecimal,
}

#[allow(dead_code)]
pub struct CassValueType {
  cass_value_type:internal::CassValueType,
}

#[allow(dead_code)]
impl CassValue {

  pub fn get_collection_iterator(&self) -> CollectionIterator {unsafe{
    CollectionIterator{cass_iterator:collection_internal::cass_iterator_from_collection(self.cass_value)}
  }}

  pub fn get_string(self) -> Result<String,CassError> {unsafe{
    let ref mut output:internal::Struct_CassString_=internal::cass_string_init(self.cass_value as *const i8);
    let ref mut output = *output;
    let err_string = types_internal::cass_value_get_string(self.cass_value,&mut*output);
    let err = CassError{cass_error:err_string};
    let ref mut output = *output;
    if err.cass_error == error_internal::CASS_OK {
      let length=output.length as uint;
      println!("item length: {}", length);
      println!("raw: {}", raw::from_parts(output.data as *mut u8, length, length));
      Ok(raw::from_parts(output.data as *mut u8, length, length))
    } else {Err(err)}
  }}

  pub fn get_int32(self) ->  Result<i32,CassError> {unsafe{
    let ref mut output:i32=0;
    let err = CassError{cass_error:types_internal::cass_value_get_int32(self.cass_value,output)};
    if err.cass_error == error_internal::CASS_OK {return Ok(*output)} else {return Err(err)}
  }}

  pub fn get_int64(self) ->  Result<i64,CassError> {unsafe{
    let ref mut output:i64=0;
    let err = CassError{cass_error:types_internal::cass_value_get_int64(self.cass_value,output)};
    if err.cass_error == error_internal::CASS_OK {return Ok(*output)} else {return Err(err)}
    }}

  pub fn get_float(self) ->  Result<f32,CassError> {unsafe{
    let ref mut output:f32=0.0;
    let err = CassError{cass_error:types_internal::cass_value_get_float(self.cass_value,output)};
    if err.cass_error == error_internal::CASS_OK {return Ok(*output)} else {return Err(err)}
  }}

  pub fn get_double(self) -> Result<f64,CassError> {unsafe{
    let ref mut output:f64=0.0;
    let err = CassError{cass_error:types_internal::cass_value_get_double(self.cass_value,output)};
    if err.cass_error == error_internal::CASS_OK {return Ok(*output)} else {return Err(err)}
  }}

  pub fn get_bool(self) -> Result<bool,CassError> {unsafe{
    let ref mut output:u32=0;
    let err = CassError{cass_error:types_internal::cass_value_get_bool(self.cass_value,output)};
    if err.cass_error == error_internal::CASS_OK {return Ok(*output> 0)} else {return Err(err)}
  }}

  pub fn get_uuid(self, output: CassUuid) -> CassError {unsafe{
    CassError{cass_error:types_internal::cass_value_get_uuid(self.cass_value,output.cass_uuid)}
  }}

  pub fn get_inet(self, mut output: CassInet) -> CassError {unsafe{
    let ref mut cass_inet = output.cass_inet;
    CassError{cass_error:types_internal::cass_value_get_inet(self.cass_value,cass_inet)}
  }}

  pub fn get_bytes(self, mut output: CassBytes) -> CassError {unsafe{
    let ref mut my_bytes = output.cass_bytes;
    CassError{cass_error:types_internal::cass_value_get_bytes(self.cass_value,my_bytes)}
  }}

  pub fn get_decimal(self, mut output: CassDecimal) -> CassError {unsafe{
    let ref mut my_decimal = output.cass_decimal;
    CassError{cass_error:types_internal::cass_value_get_decimal(self.cass_value,my_decimal)}
  }}

  pub fn is_null(self) -> bool {unsafe{
    !types_internal::cass_value_is_null(self.cass_value) == Int::zero()
  }}

  pub fn is_collection(self) -> bool {unsafe{
    !types_internal::cass_value_is_collection(self.cass_value) == Int::zero()
  }}

  pub fn item_count(self) -> u64 {unsafe{
    types_internal::cass_value_item_count(self.cass_value)
  }}

  pub fn primary_sub_type(self) -> CassValueType {unsafe{
    CassValueType{cass_value_type:types_internal::cass_value_primary_sub_type(self.cass_value)}
  }}

  pub fn secondary_sub_type(self) -> CassValueType {unsafe{
    CassValueType{cass_value_type:types_internal::cass_value_secondary_sub_type(self.cass_value)}
  }}

  //FIXME segfaults
  pub fn generate_timeuuid() -> CassUuid {unsafe{
    let output:types_internal::CassUuid = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
    types_internal::cass_uuid_generate_time(output);
    CassUuid{cass_uuid:output}
  }}

  fn cass_uuid_from_time(time: u64, output: CassUuid) {unsafe{
    internal::cass_uuid_from_time(time,output.cass_uuid);
  }}

  pub fn cass_uuid_min_from_time(time: u64, output: CassUuid) {unsafe{
    internal::cass_uuid_min_from_time(time,output.cass_uuid)
  }}

  pub fn cass_uuid_max_from_time(time: u64, output: CassUuid) {unsafe{
    internal::cass_uuid_max_from_time(time,output.cass_uuid)
  }}

  pub fn cass_uuid_generate_random(output: CassUuid) {unsafe{
    internal::cass_uuid_generate_random(output.cass_uuid)
  }}

  pub fn cass_uuid_timestamp(uuid: CassUuid) -> u64 {unsafe{
    internal::cass_uuid_timestamp(uuid.cass_uuid)
  }}

  pub fn cass_uuid_version(uuid: CassUuid) -> u8 {unsafe{
    internal::cass_uuid_version(uuid.cass_uuid)
  }}

  pub fn cass_uuid_string(uuid: CassUuid, output: *mut c_char) {unsafe{
    internal::cass_uuid_string(uuid.cass_uuid,output)
  }}

  fn cass_inet_init_v4(address: *const u8) -> CassInet {unsafe{
    CassInet{cass_inet:internal::cass_inet_init_v4(address)}
  }}

  pub fn build_cass_inet(addr: IpAddr) -> CassInet {
    match addr {
      Ipv4Addr(oct1,oct2,oct3,oct4) => {
        let mut v:Vec<u8> = Vec::with_capacity(4);
        v.push(oct1);v.push(oct2);v.push(oct3);v.push(oct4);
        CassValue::cass_inet_init_v4(v.as_ptr())
      },
      Ipv6Addr(_,_,_,_,_,_,_,_) => {
        //let v:Vec<u16> = Vec::with_capacity(8);
        panic!("FIXME: can't handle v6 addresses yet");
        //~ v.push(seg1);v.push(seg2);v.push(seg3);v.push(seg4);
        //~ v.push(seg5);v.push(seg6);v.push(seg7);v.push(seg8);
        //~ CassValue::cass_inet_init_v4(v.as_ptr())
      },
    }
  }

  pub fn build_cass_bytes(data: Vec<u8>) -> CassBytes {
    CassValue::cass_bytes_init(data.as_slice().as_ptr(), data.len() as u64)
  }
  
  pub fn build_cass_decimal(int_value: i32, scale:i32) -> CassDecimal {
    let varint = CassValue::build_cass_bytes(int_value.to_string().into_bytes());
    CassValue::cass_decimal_init(scale,varint)
  }

  pub fn str_to_cass_string(string:&String) -> internal::CassString {unsafe{
     internal::cass_string_init2(string.as_bytes().as_ptr() as *const i8,string.as_bytes().len() as u64)
  }}

  pub fn cass_string_to_str(cass_str:internal::CassString) -> String {unsafe{
    raw::from_buf_len(cass_str.data as *const u8,cass_str.length as uint)
  }}

  //PRIVATE METHODS
  fn cass_inet_init_v6(address: *const u8) -> CassInet {unsafe{
    CassInet{cass_inet:internal::cass_inet_init_v6(address)}
  }}

  fn cass_decimal_init(scale: i32, varint: CassBytes) -> CassDecimal {unsafe{
    CassDecimal{cass_decimal:internal::cass_decimal_init(scale,varint.cass_bytes)}
  }}

  fn cass_bytes_init(data: *const u8, size: internal::cass_size_t) -> CassBytes {unsafe{
    CassBytes{cass_bytes:internal::cass_bytes_init(data,size)}
  }}
}

pub mod internal {
  use error::internal as error_internal;
  
  use std::fmt::Show;
  use std::fmt;
  use std::string::raw;


  impl Show for Struct_CassString_ {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {unsafe{
      let raw = self.data as *const u8;
      let length = self.length as uint;
      write!(f, "{}", raw::from_buf_len(raw, length))
    }
  }}

  
  pub type Enum_CassValueType_ = ::libc::c_uint;
  pub type CassValueType = Enum_CassValueType_;
  
  pub enum Struct_CassValue_ { }
  pub type CassValue = Struct_CassValue_;

  #[repr(C)]
  pub struct Struct_CassDecimal_ {
    pub scale: i32,
    pub varint: CassBytes,
  }
  pub type CassDecimal = Struct_CassDecimal_;
  
  #[repr(C)]
  pub struct Struct_CassInet_ {
    pub address: [u8, ..16u],
    pub address_length: u8,
  }
  pub type CassInet = Struct_CassInet_;

  pub type CassUuid = [u8, ..16u];

  #[repr(C)]
  pub struct Struct_CassBytes_ {
    pub data: *const u8,
    pub size: cass_size_t,
  }
  pub type CassBytes = Struct_CassBytes_;

  pub type size_t = ::libc::c_ulong;
  pub type cass_size_t = size_t;
  pub type Enum_Unnamed1 = ::libc::c_uint;
  pub type cass_bool_t = Enum_Unnamed1;
  #[repr(C)]
  pub struct Struct_CassString_ {
    pub data: *const ::libc::c_char,
    pub length: cass_size_t,
  }
  pub type CassString = Struct_CassString_;
  pub type cass_duration_t = u64;


  #[link(name = "cassandra")]
  extern "C" {
    pub fn cass_uuid_generate_time(output: CassUuid);
    pub fn cass_uuid_from_time(time: u64, output: CassUuid);
    pub fn cass_uuid_min_from_time(time: u64, output: CassUuid);
    pub fn cass_uuid_max_from_time(time: u64, output: CassUuid);
    pub fn cass_uuid_generate_random(output: CassUuid);
    pub fn cass_uuid_timestamp(uuid: CassUuid) -> u64;
    pub fn cass_uuid_version(uuid: CassUuid) -> u8;
    pub fn cass_uuid_string(uuid: CassUuid, output: *mut ::libc::c_char);

    pub fn cass_value_get_int32(value: *const CassValue, output: *mut i32) -> error_internal::CassError;
    pub fn cass_value_get_int64(value: *const CassValue, output: *mut i64) -> error_internal::CassError;
    pub fn cass_value_get_float(value: *const CassValue, output: *mut f32) -> error_internal::CassError;
    pub fn cass_value_get_double(value: *const CassValue, output: *mut f64) -> error_internal::CassError;
    pub fn cass_value_get_bool(value: *const CassValue, output: *mut cass_bool_t) -> error_internal::CassError;
    pub fn cass_value_get_uuid(value: *const CassValue, output: CassUuid) -> error_internal::CassError;
    pub fn cass_value_get_inet(value: *const CassValue, output: *mut CassInet) -> error_internal::CassError;
    pub fn cass_value_get_string(value: *const CassValue, output: *mut CassString) -> error_internal::CassError;
    pub fn cass_value_get_bytes(value: *const CassValue, output: *mut CassBytes) -> error_internal::CassError;
    pub fn cass_value_get_decimal(value: *const CassValue, output: *mut CassDecimal) -> error_internal::CassError;
    pub fn cass_value_type(value: *const CassValue) -> CassValueType;
    pub fn cass_value_is_null(value: *const CassValue) -> cass_bool_t;
    pub fn cass_value_is_collection(value: *const CassValue) -> cass_bool_t;
    pub fn cass_value_item_count(collection: *const CassValue) -> cass_size_t;
    pub fn cass_value_primary_sub_type(collection: *const CassValue) -> CassValueType;
    pub fn cass_value_secondary_sub_type(collection: *const CassValue) -> CassValueType;
    
    pub fn cass_string_init(null_terminated: *const ::libc::c_char) -> CassString;
    pub fn cass_string_init2(data: *const ::libc::c_char, length: cass_size_t) -> CassString;

    pub fn cass_inet_init_v4(address: *const u8) -> CassInet;
    pub fn cass_inet_init_v6(address: *const u8) -> CassInet;
    pub fn cass_decimal_init(scale: i32, varint: CassBytes) -> CassDecimal;
    pub fn cass_bytes_init(data: *const u8, size: cass_size_t) -> CassBytes;

  }
}

#[cfg(test)]
  mod tests {
    use cass_internal_api;
    #[test]
    fn string_wrapping() {
      let test_string = "test_string2345678".to_string();
      let cass_string:cass_internal_api::CassString = super::CassValue::str_to_cass_string(&test_string);
      //println!("cassstr: {}", cass_string);
      let reconstituted:String = super::CassValue::cass_string_to_str(cass_string);
      println!("reconstituted: {}", reconstituted);
      assert!(test_string == reconstituted);
    }
}
