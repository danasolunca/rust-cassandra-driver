extern crate libc;

use collection;
use error::CASS_OK;
use error::Error;
use error::CassError;

use libc::c_char;
use std::fmt::Show;
use std::fmt;
use std::num::Int;
use std::io::net::ip::IpAddr;
use std::io::net::ip::Ipv4Addr;
use std::io::net::ip::Ipv6Addr;
use std::string::raw;
use std::vec::Vec;

use CollectionIterator;


#[allow(dead_code)]
pub enum _ValueType {
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
pub struct Value {
  pub cass_value:*const CassValue
}

#[allow(dead_code)]
pub struct Uuid {
  pub cass_uuid:CassUuid,
}

impl Uuid {

  pub fn generate_timeuuid() -> Uuid {unsafe{
    let output:CassUuid = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
    cass_uuid_generate_time(output);
    Uuid{cass_uuid:output}
  }}

  pub fn build_from_time(time:u64) -> Uuid {unsafe{
    let output:CassUuid = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
    cass_uuid_from_time(time,output);
    Uuid{cass_uuid:output}
  }}

  pub fn min_from_time(time:u64) -> Uuid {unsafe{
    let output:CassUuid = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
    cass_uuid_min_from_time(time,output);
    Uuid{cass_uuid:output}
  }}

  pub fn max_from_time(time:u64) -> Uuid {unsafe{
    let output:CassUuid = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
    cass_uuid_max_from_time(time,output);
    Uuid{cass_uuid:output}
  }}

  pub fn generate_uuid() -> Uuid {unsafe{
    let output:CassUuid = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
    cass_uuid_generate_random(output);
    Uuid{cass_uuid:output}
  }}

  pub fn get_timestamp(&self) -> u64 {unsafe{
    cass_uuid_timestamp(self.cass_uuid)
  }}

  pub fn get_version(&self) -> u8 {unsafe{
    cass_uuid_version(self.cass_uuid)
  }}

  //~ pub fn as_string(&self) -> u8 {unsafe{
    //~ cass_uuid_string(self.cass_uuid,
    //~pub fn cass_uuid_string(uuid: CassUuid, output: *mut ::libc::c_char);
  //~ }}
}

#[allow(dead_code)]
pub struct Inet {
  pub cass_inet:CassInet,
}

#[allow(dead_code)]
pub struct Bytes {
  pub cass_bytes:CassBytes,
}

#[allow(dead_code)]
pub struct Decimal {
  pub cass_decimal:CassDecimal,
}

#[allow(dead_code)]
pub struct ValueType {
  cass_value_type:CassValueType,
}

#[allow(dead_code)]
impl Value {

  pub fn get_collection_iterator(&self) -> CollectionIterator {unsafe{
    CollectionIterator{cass_iterator:collection::cass_iterator_from_collection(self.cass_value)}
  }}

  pub fn get_string(self) -> Result<String,Error> {unsafe{
    let ref mut output:CassString=cass_string_init(self.cass_value as *const i8);
    let ref mut output = *output;
    let err_string = cass_value_get_string(self.cass_value,&mut*output);
    let err = Error{cass_error:err_string};
    let ref mut output = *output;
    if err.cass_error == CASS_OK {
      let length=output.length as uint;
      println!("item length: {}", length);
      println!("raw: {}", raw::from_parts(output.data as *mut u8, length, length));
      Ok(raw::from_parts(output.data as *mut u8, length, length))
    } else {Err(err)}
  }}

  pub fn get_int32(self) ->  Result<i32,Error> {unsafe{
    let ref mut output:i32=0;
    let err = Error{cass_error:cass_value_get_int32(self.cass_value,output)};
    if err.cass_error == CASS_OK {return Ok(*output)} else {return Err(err)}
  }}

  pub fn get_int64(self) ->  Result<i64,Error> {unsafe{
    let ref mut output:i64=0;
    let err = Error{cass_error:cass_value_get_int64(self.cass_value,output)};
    if err.cass_error == CASS_OK {return Ok(*output)} else {return Err(err)}
    }}

  pub fn get_float(self) ->  Result<f32,Error> {unsafe{
    let ref mut output:f32=0.0;
    let err = Error{cass_error:cass_value_get_float(self.cass_value,output)};
    if err.cass_error == CASS_OK {return Ok(*output)} else {return Err(err)}
  }}

  pub fn get_double(self) -> Result<f64,Error> {unsafe{
    let ref mut output:f64=0.0;
    let err = Error{cass_error:cass_value_get_double(self.cass_value,output)};
    if err.cass_error == CASS_OK {return Ok(*output)} else {return Err(err)}
  }}

  pub fn get_bool(self) -> Result<bool,Error> {unsafe{
    let ref mut output:u32=0;
    let err = Error{cass_error:cass_value_get_bool(self.cass_value,output)};
    if err.cass_error == CASS_OK {return Ok(*output> 0)} else {return Err(err)}
  }}

  pub fn get_uuid(self, output: Uuid) -> Error {unsafe{
    Error{cass_error:cass_value_get_uuid(self.cass_value,output.cass_uuid)}
  }}

  pub fn get_inet(self, mut output: Inet) -> Error {unsafe{
    let ref mut cass_inet = output.cass_inet;
    Error{cass_error:cass_value_get_inet(self.cass_value,cass_inet)}
  }}

  pub fn get_bytes(self, mut output: Bytes) -> Error {unsafe{
    let ref mut my_bytes = output.cass_bytes;
    Error{cass_error:cass_value_get_bytes(self.cass_value,my_bytes)}
  }}

  pub fn get_decimal(self, mut output: Decimal) -> Error {unsafe{
    let ref mut my_decimal = output.cass_decimal;
    Error{cass_error:cass_value_get_decimal(self.cass_value,my_decimal)}
  }}

  pub fn is_null(self) -> bool {unsafe{
    !cass_value_is_null(self.cass_value) == Int::zero()
  }}

  pub fn is_collection(self) -> bool {unsafe{
    !cass_value_is_collection(self.cass_value) == Int::zero()
  }}

  pub fn item_count(self) -> u64 {unsafe{
    cass_value_item_count(self.cass_value)
  }}

  pub fn primary_sub_type(self) -> ValueType {unsafe{
    ValueType{cass_value_type:cass_value_primary_sub_type(self.cass_value)}
  }}

  pub fn secondary_sub_type(self) -> ValueType {unsafe{
    ValueType{cass_value_type:cass_value_secondary_sub_type(self.cass_value)}
  }}


  fn cass_uuid_from_time(time: u64, output: Uuid) {unsafe{
    cass_uuid_from_time(time,output.cass_uuid);
  }}

  pub fn cass_uuid_min_from_time(time: u64, output: Uuid) {unsafe{
    cass_uuid_min_from_time(time,output.cass_uuid)
  }}

  pub fn cass_uuid_max_from_time(time: u64, output: Uuid) {unsafe{
    cass_uuid_max_from_time(time,output.cass_uuid)
  }}

  pub fn cass_uuid_generate_random(output: Uuid) {unsafe{
    cass_uuid_generate_random(output.cass_uuid)
  }}

  pub fn cass_uuid_timestamp(uuid: Uuid) -> u64 {unsafe{
    cass_uuid_timestamp(uuid.cass_uuid)
  }}

  pub fn cass_uuid_version(uuid: Uuid) -> u8 {unsafe{
    cass_uuid_version(uuid.cass_uuid)
  }}

  pub fn cass_uuid_string(uuid: Uuid, output: *mut c_char) {unsafe{
    cass_uuid_string(uuid.cass_uuid,output)
  }}

  fn cass_inet_init_v4(address: *const u8) -> Inet {unsafe{
    Inet{cass_inet:cass_inet_init_v4(address)}
  }}

  pub fn build_cass_inet(addr: IpAddr) -> Inet {
    match addr {
      Ipv4Addr(oct1,oct2,oct3,oct4) => {
        let mut v:Vec<u8> = Vec::with_capacity(4);
        v.push(oct1);v.push(oct2);v.push(oct3);v.push(oct4);
        Value::cass_inet_init_v4(v.as_ptr())
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

  pub fn build_cass_bytes(data: Vec<u8>) -> Bytes {
    Value::cass_bytes_init(data.as_slice().as_ptr(), data.len() as u64)
  }
  
  pub fn build_cass_decimal(int_value: i32, scale:i32) -> Decimal {
    let varint = Value::build_cass_bytes(int_value.to_string().into_bytes());
    Value::cass_decimal_init(scale,varint)
  }

  pub fn string_to_cass_string(string:&String) -> CassString {unsafe{
     cass_string_init2(string.as_bytes().as_ptr() as *const i8,string.as_bytes().len() as u64)
  }}

  pub fn str_to_cass_string(string:&str) -> CassString {unsafe{
     cass_string_init2(string.as_bytes().as_ptr() as *const i8,string.as_bytes().len() as u64)
  }}

  pub fn cass_string_to_string(cass_str:CassString) -> String {unsafe{
    raw::from_buf_len(cass_str.data as *const u8,cass_str.length as uint)
  }}

  pub fn cass_inet_init_v6(address: *const u8) -> Inet {unsafe{
    Inet{cass_inet:cass_inet_init_v6(address)}
  }}

  fn cass_decimal_init(scale: i32, varint: Bytes) -> Decimal {unsafe{
    Decimal{cass_decimal:cass_decimal_init(scale,varint.cass_bytes)}
  }}

  fn cass_bytes_init(data: *const u8, size: CassSizeType) -> Bytes {unsafe{
    Bytes{cass_bytes:cass_bytes_init(data,size)}
  }}

  pub fn get_type(&self) -> u32 {unsafe{
    cass_value_type(self.cass_value)
  }}
}

impl Show for CassString {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {unsafe{
    let raw = self.data as *const u8;
    let length = self.length as uint;
    write!(f, "{}", raw::from_buf_len(raw, length))
  }
}}

pub type CassValueType = u32;
#[repr(C)]
#[allow(dead_code)]
pub enum CassValue {
  CassDecimal,
  CassBytes,
  CassInet,
  CassUuid,
}

#[repr(C)]
pub struct CassDecimal {
  pub scale: i32,
  pub varint: CassBytes,
}
  
#[repr(C)]
pub struct CassInet {
  pub address: [u8, ..16u],
  pub address_length: u8,
}

pub type CassUuid = [u8, ..16u];

#[repr(C)]
pub struct CassBytes {
  pub data: *const u8,
  pub size: CassSizeType,
}

pub type CassSizeType = u64;
pub type CassBoolType = u32;
#[repr(C)]
pub struct CassString {
  pub data: *const i8,
  pub length: CassSizeType,
}

pub type CassDurationType = u64;

#[link(name = "cassandra")]
extern "C" {
  fn cass_uuid_generate_time(output: CassUuid);
  fn cass_uuid_from_time(time: u64, output: CassUuid);
  fn cass_uuid_min_from_time(time: u64, output: CassUuid);
  fn cass_uuid_max_from_time(time: u64, output: CassUuid);
  fn cass_uuid_generate_random(output: CassUuid);
  fn cass_uuid_timestamp(uuid: CassUuid) -> u64;
  fn cass_uuid_version(uuid: CassUuid) -> u8;
  fn cass_uuid_string(uuid: CassUuid, output: *mut ::libc::c_char);
  fn cass_value_get_int32(value: *const CassValue, output: *mut i32) -> CassError;
  fn cass_value_get_int64(value: *const CassValue, output: *mut i64) -> CassError;
  fn cass_value_get_float(value: *const CassValue, output: *mut f32) -> CassError;
  fn cass_value_get_double(value: *const CassValue, output: *mut f64) -> CassError;
  fn cass_value_get_bool(value: *const CassValue, output: *mut CassBoolType) -> CassError;
  fn cass_value_get_uuid(value: *const CassValue, output: CassUuid) -> CassError;
  fn cass_value_get_inet(value: *const CassValue, output: *mut CassInet) -> CassError;
  fn cass_value_get_string(value: *const CassValue, output: *mut CassString) -> CassError;
  fn cass_value_get_bytes(value: *const CassValue, output: *mut CassBytes) -> CassError;
  fn cass_value_get_decimal(value: *const CassValue, output: *mut CassDecimal) -> CassError;
  fn cass_value_type(value: *const CassValue) -> CassValueType;
  fn cass_value_is_null(value: *const CassValue) -> CassBoolType;
  fn cass_value_is_collection(value: *const CassValue) -> CassBoolType;
  fn cass_value_item_count(collection: *const CassValue) -> CassSizeType;
  fn cass_value_primary_sub_type(collection: *const CassValue) -> CassValueType;
  fn cass_value_secondary_sub_type(collection: *const CassValue) -> CassValueType;   
  pub fn cass_string_init(null_terminated: *const ::libc::c_char) -> CassString;
  fn cass_string_init2(data: *const ::libc::c_char, length: CassSizeType) -> CassString;
  fn cass_inet_init_v4(address: *const u8) -> CassInet;
  fn cass_inet_init_v6(address: *const u8) -> CassInet;
  fn cass_decimal_init(scale: i32, varint: CassBytes) -> CassDecimal;
  fn cass_bytes_init(data: *const u8, size: CassSizeType) -> CassBytes;
}

#[cfg(test)]
mod tests {
  use super::CassString;
  use super::Value;
  #[test]
  fn string_wrapping() {
    let test_string = "test_string2345678";
    let cass_string:CassString = Value::str_to_cass_string(test_string);
    //println!("cassstr: {}", cass_string);
    let reconstituted = Value::cass_string_to_string(cass_string);
    println!("reconstituted: {}", reconstituted);
    assert!(test_string == reconstituted.as_slice());
  }
}
