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
use std::vec::Vec;
use std::str::FromStr;

use uuid::Uuid;

use CollectionIterator;


#[allow(dead_code)]
#[repr(C)]
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

impl Copy for Value {}

#[allow(dead_code)]
pub type Bytes = Vec<u8>;

pub trait CassUuid {

  fn generate_timeuuid() -> _CassUuid {unsafe{
    let output:_CassUuid = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
    cass_uuid_generate_time(output);
    output
  }}

  fn build_from_time(time:u64) -> _CassUuid {unsafe{
    let output:_CassUuid = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
    cass_uuid_from_time(time,output);
    output
  }}

  fn min_from_time(time:u64) -> _CassUuid {unsafe{
    let output:_CassUuid = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
    cass_uuid_min_from_time(time,output);
    output
  }}

  fn max_from_time(time:u64) -> _CassUuid {unsafe{
    let output:_CassUuid = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
    cass_uuid_max_from_time(time,output);
    output
  }}

  fn generate_uuid() -> _CassUuid {unsafe{
    let output:_CassUuid = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
    cass_uuid_generate_random(output);
    output
  }}

  //~ pub fn as_string(&self) -> u8 {unsafe{
    //~ cass_uuid_string(self.cass_uuid,
    //~pub fn cass_uuid_string(uuid: CassUuid, output: *mut ::libc::c_char);
  //~ }}
}


#[allow(dead_code)]
pub struct ValueType {
  cass_value_type:CassValueType,
}

#[allow(dead_code)]
impl Value {

  pub fn bytes2cassbytes(bytes:&Bytes) -> CassBytes {unsafe{
    cass_bytes_init(bytes.as_slice().as_ptr(), bytes.len() as u64)
  }}

  pub fn cassinet2ipaddr(inet:CassInet) -> IpAddr {
    match inet.address_length {
      4 =>   FromStr::from_str(inet.to_string().as_slice()),
      16 => FromStr::from_str(inet.to_string().as_slice()),
      _ => panic!("invalid ipaddr length")
    }.unwrap()
  }

  pub fn ipaddr2cassinet(addr:IpAddr) -> CassInet {
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

  pub fn uuid_to_cassuuid(uuid:&Uuid) -> _CassUuid {
    Value::arr2b(uuid.as_bytes())
  }

  pub fn get_collection_iterator(&self) -> CollectionIterator {unsafe{
    CollectionIterator{cass_iterator:collection::cass_iterator_from_collection(self.cass_value)}
  }}

  pub fn get_string(self) -> Result<String,Error> {unsafe{
    let mut output:CassString=cass_string_init(self.cass_value as *const i8);
    let err_string = cass_value_get_string(self.cass_value,&mut output);
    let err = Error{cass_error:err_string};
    if err.cass_error == CASS_OK {
      let length=output.length as uint;
      println!("item length: {}", length);
      println!("raw: {}", String::from_raw_parts(output.data as *mut u8, length, length));
      Ok(String::from_raw_buf_len(output.data as *const u8, length))
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

  pub fn get_uuid(self) -> Uuid {unsafe{
    let output:_CassUuid = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
    cass_value_get_uuid(self.cass_value,output);
    Uuid::from_bytes(output.as_slice()).unwrap()
  }}

  pub fn get_inet(&self, addr: IpAddr) -> Error {unsafe{
    let mut inet = Value::ipaddr2cassinet(addr);
    let ref mut inet = inet;
    Error{cass_error:cass_value_get_inet(self.cass_value,inet)}
  }}

  //~ pub fn get_bytes(&self, mut output: Vec<u8>) -> Error {unsafe{
    //~ let ref mut my_bytes = output;
    //~ Error{cass_error:cass_value_get_bytes(self.cass_value,my_bytes)}
  //~ }}

  //~ pub fn get_decimal(self, mut output: f64) -> Error {unsafe{
    //~ let ref mut my_decimal = output;
    //~ Error{cass_error:cass_value_get_decimal(self.cass_value,output)}
  //~ }}

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


  fn uuid_from_time(time: u64) -> Option<Uuid> {unsafe{
    let output:_CassUuid = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
    cass_uuid_from_time(time,output);
    Uuid::from_bytes(output.as_slice())
  }}

  pub fn cass_uuid_min_from_time(time: u64) -> Option<Uuid> {unsafe{
    let output:_CassUuid = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
    cass_uuid_min_from_time(time,output);
    Uuid::from_bytes(output.as_slice())
  }}

  pub fn cass_uuid_max_from_time(time: u64, ) -> Option<Uuid> {unsafe{
    let output:_CassUuid = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
    cass_uuid_max_from_time(time,output);
    Uuid::from_bytes(output.as_slice())
  }}

  pub fn cass_uuid_generate_random(output: Uuid) {unsafe{
    cass_uuid_generate_random(Value::arr2b(output.as_bytes()))
  }}

  pub fn cass_uuid_timestamp(uuid: Uuid) -> u64 {unsafe{
    cass_uuid_timestamp(Value::arr2b(uuid.as_bytes()))
  }}

  pub fn arr2b(bytes:&[u8]) -> [u8, ..16] {
    [
      bytes[0],bytes[1],bytes[2],bytes[3],
      bytes[4],bytes[5],bytes[6],bytes[7],
      bytes[8],bytes[9],bytes[10],bytes[11],
      bytes[12],bytes[13],bytes[14],bytes[15]
    ]
  }

  fn generate_uuid() -> _CassUuid {unsafe{
    let output:_CassUuid = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
    cass_uuid_generate_random(output);
    output
  }}

  pub fn cass_uuid_version(uuid: Uuid) -> u8 {unsafe{
    let uuid = uuid.as_bytes();
    let bytes = [
    uuid[0],uuid[1],uuid[2],uuid[3],
    uuid[4],uuid[5],uuid[6],uuid[7],
    uuid[8],uuid[9],uuid[10],uuid[11],
    uuid[12],uuid[13],uuid[14],uuid[15],
    ];
    cass_uuid_version(bytes)
  }}

  pub fn cass_uuid_string(uuid: Uuid, output: *mut c_char) {unsafe{
    cass_uuid_string(Value::arr2b(uuid.as_bytes()),output)
  }}

  fn cass_inet_init_v4(address: *const u8) -> CassInet {unsafe{
    cass_inet_init_v4(address)
  }}
  
  //~ pub fn build_cass_decimal(int_value: i32, scale:i32) -> f64 {
    //~ Value::cass_decimal_init(scale,Value::build_cass_bytes(int_value.to_string().into_bytes()))
  //~ }

  pub fn string_to_cass_string(string:&String) -> CassString {unsafe{
     cass_string_init2(string.as_bytes().as_ptr() as *const i8,string.as_bytes().len() as u64)
  }}

  pub fn str_to_cass_string(string:&str) -> CassString {unsafe{
     cass_string_init2(string.as_bytes().as_ptr() as *const i8,string.as_bytes().len() as u64)
  }}

  pub fn cass_string_to_string(cass_str:CassString) -> String {unsafe{
    String::from_raw_buf_len(cass_str.data as *const u8,cass_str.length as uint)
  }}

  pub fn cass_inet_init_v6(address: *const u8) -> CassInet {unsafe{
    cass_inet_init_v6(address)
  }}

  pub fn get_type(&self) -> u32 {unsafe{
    cass_value_type(self.cass_value)
  }}
}

impl Show for CassString {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {unsafe{
    write!(f, "{}", String::from_raw_buf_len(self.data as *const u8, self.length as uint))
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

impl CassInet {
  pub fn to_string(&self) -> String {
    //FIXME
    "1.1.1.1".to_string()
  }
}

pub type _CassUuid = [u8, ..16u];

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
  fn cass_uuid_generate_time(output: _CassUuid);
  fn cass_uuid_from_time(time: u64, output: _CassUuid);
  fn cass_uuid_min_from_time(time: u64, output: _CassUuid);
  fn cass_uuid_max_from_time(time: u64, output: _CassUuid);
  fn cass_uuid_generate_random(output: _CassUuid);
  fn cass_uuid_timestamp(uuid: _CassUuid) -> u64;
  fn cass_uuid_version(uuid: _CassUuid) -> u8;
  fn cass_uuid_string(uuid: _CassUuid, output: *mut c_char);
  fn cass_value_get_int32(value: *const CassValue, output: *mut i32) -> CassError;
  fn cass_value_get_int64(value: *const CassValue, output: *mut i64) -> CassError;
  fn cass_value_get_float(value: *const CassValue, output: *mut f32) -> CassError;
  fn cass_value_get_double(value: *const CassValue, output: *mut f64) -> CassError;
  fn cass_value_get_bool(value: *const CassValue, output: *mut CassBoolType) -> CassError;
  fn cass_value_get_uuid(value: *const CassValue, output: _CassUuid) -> CassError;
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
  pub fn cass_string_init(null_terminated: *const c_char) -> CassString;
  fn cass_string_init2(data: *const c_char, length: CassSizeType) -> CassString;
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
