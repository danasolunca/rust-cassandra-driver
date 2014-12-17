extern crate libc;

use collection;
use error::CASS_OK;
use error::CassError;
use error::_CassError;

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
pub enum CassValueType {
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
  pub val:*const _CassValue
}

impl Show for CassValue {

  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {{
    match self.get_type() {
      CassValueType::UNKNOWN => write!(f, "UNKNOWN: "),
      CassValueType::CUSTOM => write!( f, "CUSTOM: "),
      CassValueType::ASCII => write!(f, "ASCII: "),
      CassValueType::BIGINT => write!(f, "BIGINT: "),
      CassValueType::BLOB => write!(f, "BLOB: "),
      CassValueType::BOOLEAN => write!(f, "BOOLEAN: "),
      CassValueType::COUNTER => write!(f, "COUNTER: "),
      CassValueType::DECIMAL => write!(f, "DECIMAL: "),
      CassValueType::DOUBLE => write!(f, "DOUBLE: "),
      CassValueType::FLOAT => write!(f, "FLOAT: "),
      CassValueType::INT => write!(f, "INT: "),
      CassValueType::TEXT => write!(f, "TEXT: "),
      CassValueType::TIMESTAMP => write!(f, "TIMESTAMP: "),
      CassValueType::UUID => write!(f, "UUID: "),
      CassValueType::VARCHAR => write!(f, "VARCHAR: "),
      CassValueType::VARINT => write!(f, "VARINT: "),
      CassValueType::TIMEUUID => write!(f, "TIMEUUID: "),
      CassValueType::INET => write!(f, "INET: "),
      CassValueType::LIST => write!(f, "LIST: "),
      CassValueType::MAP => write!(f, "MAP: "),
      CassValueType::SET => write!(f, "SET: ")
    }
  }}
}

impl Show for CassValueType {

  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {{
    match self {
      &CassValueType::UNKNOWN => write!(f, "UNKNOWN: "),
      &CassValueType::CUSTOM => write!( f, "CUSTOM: "),
      &CassValueType::ASCII => write!(f, "ASCII: "),
      &CassValueType::BIGINT => write!(f, "BIGINT: "),
      &CassValueType::BLOB => write!(f, "BLOB: "),
      &CassValueType::BOOLEAN => write!(f, "BOOLEAN: "),
      &CassValueType::COUNTER => write!(f, "COUNTER: "),
      &CassValueType::DECIMAL => write!(f, "DECIMAL: "),
      &CassValueType::DOUBLE => write!(f, "DOUBLE: "),
      &CassValueType::FLOAT => write!(f, "FLOAT: "),
      &CassValueType::INT => write!(f, "INT: "),
      &CassValueType::TEXT => write!(f, "TEXT: "),
      &CassValueType::TIMESTAMP => write!(f, "TIMESTAMP: "),
      &CassValueType::UUID => write!(f, "UUID: "),
      &CassValueType::VARCHAR => write!(f, "VARCHAR: "),
      &CassValueType::VARINT => write!(f, "VARINT: "),
      &CassValueType::TIMEUUID => write!(f, "TIMEUUID: "),
      &CassValueType::INET => write!(f, "INET: "),
      &CassValueType::LIST => write!(f, "LIST: "),
      &CassValueType::MAP => write!(f, "MAP: "),
      &CassValueType::SET => write!(f, "SET: ")
    }
  }}
}


impl Copy for CassValue {}


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
impl CassValue {

  pub fn bytes2cassbytes(bytes:&Vec<u8>) -> _CassBytes {unsafe{
    cass_bytes_init(bytes.as_slice().as_ptr(), bytes.len() as u64)
  }}

  pub fn cassbytes2bytes(bytes:&_CassBytes) -> Vec<u8> {unsafe{
    Vec::from_raw_buf(bytes.data, bytes.size as uint)
  }}

  pub fn cassinet2ipaddr(inet:_CassInet) -> IpAddr {
    match inet.address_length {
      4 =>   FromStr::from_str(inet.to_string().as_slice()),
      16 => FromStr::from_str(inet.to_string().as_slice()),
      _ => panic!("invalid ipaddr length")
    }.unwrap()
  }

  pub fn ipaddr2cassinet(addr:IpAddr) -> _CassInet {
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

  pub fn uuid_to_cassuuid(uuid:&Uuid) -> _CassUuid {
    CassValue::arr2b(uuid.as_bytes())
  }

  pub fn get_collection_iterator(&self) -> Result<CollectionIterator,CassError> {unsafe{
    match self.is_collection() {
      true => {
        let coll = collection::cass_iterator_from_collection(self.val);
        let iter = CollectionIterator{iter:coll};
        Ok(iter)
        },
      false => {
        panic!("Error: called get_collection_iterator on a non-iterator: {}",self); 
        //Err(CassError{err:8675309})
      } //FIXME not a collection
    }
  }}

  pub fn get_string(self) -> Result<String,CassError> {unsafe{
    let mut output:_CassString=cass_string_init(self.val as *const i8);
    let err_string = cass_value_get_string(self.val,&mut output);
    let err = CassError{err:err_string};
    if err.err == CASS_OK {
      let length=output.length as uint;
      println!("item length: {}", length);
      println!("raw: {}", String::from_raw_parts(output.data as *mut u8, length, length));
      Ok(String::from_raw_buf_len(output.data as *const u8, length))
    } else {Err(err)}
  }}

  pub fn get_int32(self) ->  Result<i32,CassError> {unsafe{
    let ref mut output:i32=0;
    let err = CassError{err:cass_value_get_int32(self.val,output)};
    if err.err == CASS_OK {return Ok(*output)} else {return Err(err)}
  }}

  pub fn get_int64(self) ->  Result<i64,CassError> {unsafe{
    let ref mut output:i64=0;
    let err = CassError{err:cass_value_get_int64(self.val,output)};
    if err.err == CASS_OK {return Ok(*output)} else {return Err(err)}
    }}

  pub fn get_float(self) ->  Result<f32,CassError> {unsafe{
    let ref mut output:f32=0.0;
    let err = CassError{err:cass_value_get_float(self.val,output)};
    if err.err == CASS_OK {return Ok(*output)} else {return Err(err)}
  }}

  pub fn get_double(self) -> Result<f64,CassError> {unsafe{
    let ref mut output:f64=0.0;
    let err = CassError{err:cass_value_get_double(self.val,output)};
    if err.err == CASS_OK {return Ok(*output)} else {return Err(err)}
  }}

  pub fn get_bool(self) -> Result<bool,CassError> {unsafe{
    let ref mut output:u32=0;
    let err = CassError{err:cass_value_get_bool(self.val,output)};
    if err.err == CASS_OK {return Ok(*output> 0)} else {return Err(err)}
  }}

  pub fn get_uuid(self) -> Uuid {unsafe{
    let output:_CassUuid = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
    cass_value_get_uuid(self.val,output);
    Uuid::from_bytes(output.as_slice()).unwrap()
  }}

  pub fn get_inet(&self, addr: IpAddr) -> CassError {unsafe{
    let mut inet = CassValue::ipaddr2cassinet(addr);
    let ref mut inet = inet;
    CassError{err:cass_value_get_inet(self.val,inet)}
  }}

  //~ pub fn get_bytes(&self) -> &Vec<u8> {unsafe{
    //~ let my_ptr: *const _CassValue = self.val;
    //~ let bytes = cass_bytes_init2(my_ptr as *const u8,);
    //~ CassValue::cassbytes2bytes(bytes)
  //~ }}

  //~ pub fn get_decimal(&self) -> Result<&Decimal,CassError> {unsafe{
    //~ let ref output = Decimal{scale:1,varint:2};
    //~ let err = CassError{err:cass_value_get_decimal(self.val,output)};
    //~ match err.is_error() {
      //~ true => Ok(output),
      //~ false => err
    //~ }
  //~ }}

  pub fn is_null(self) -> bool {unsafe{
    cass_value_is_null(self.val) != Int::zero()
  }}

  pub fn is_collection(self) -> bool {unsafe{
    cass_value_is_collection(self.val) != Int::zero()
  }}

  pub fn item_count(self) -> u64 {unsafe{
    cass_value_item_count(self.val)
  }}

  pub fn primary_sub_type(self) -> CassValueType {unsafe{
    cass_value_primary_sub_type(self.val)
  }}

  pub fn secondary_sub_type(self) -> CassValueType {unsafe{
    cass_value_secondary_sub_type(self.val)
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
    cass_uuid_generate_random(CassValue::arr2b(output.as_bytes()))
  }}

  pub fn cass_uuid_timestamp(uuid: Uuid) -> u64 {unsafe{
    cass_uuid_timestamp(CassValue::arr2b(uuid.as_bytes()))
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
    cass_uuid_string(CassValue::arr2b(uuid.as_bytes()),output)
  }}

  fn cass_inet_init_v4(address: *const u8) -> _CassInet {unsafe{
    cass_inet_init_v4(address)
  }}
  
  //~ pub fn build_cass_decimal(int_value: i32, scale:i32) -> f64 {
    //~ Value::cass_decimal_init(scale,Value::build_cass_bytes(int_value.to_string().into_bytes()))
  //~ }

  pub fn string_to_cass_string(string:&String) -> _CassString {unsafe{
     cass_string_init2(string.as_bytes().as_ptr() as *const i8,string.as_bytes().len() as u64)
  }}

  pub fn str_to_cass_string(string:&str) -> _CassString {unsafe{
     cass_string_init2(string.as_bytes().as_ptr() as *const i8,string.as_bytes().len() as u64)
  }}

  pub fn cass_string_to_string(cass_str:_CassString) -> String {unsafe{
    String::from_raw_buf_len(cass_str.data as *const u8,cass_str.length as uint)
  }}

  pub fn cass_inet_init_v6(address: *const u8) -> _CassInet {unsafe{
    cass_inet_init_v6(address)
  }}

  pub fn get_type(&self) -> CassValueType {unsafe{
    cass_value_type(self.val)
  }}
}

impl Show for _CassString {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {unsafe{
    write!(f, "{}", String::from_raw_buf_len(self.data as *const u8, self.length as uint))
  }
}}

#[repr(C)]
pub enum _CassValue {
  CassDecimal,
  CassBytes,
  CassInet,
  CassUuid,
}

#[repr(C)]
pub struct Decimal {
  pub scale: i32,
  pub varint: _CassBytes,
}
  
#[repr(C)]
pub struct _CassInet {
  pub address: [u8, ..16u],
  pub address_length: u8,
}

impl _CassInet {
  pub fn to_string(&self) -> String {
    //FIXME
    "1.1.1.1".to_string()
  }
}

pub type _CassUuid = [u8, ..16u];

#[repr(C)]
pub struct _CassBytes {
  pub data: *const u8,
  pub size: _CassSizeType,
}

pub type _CassSizeType = u64;
pub type _CassBoolType = u32;

#[repr(C)]
pub struct _CassString {
  pub data: *const i8,
  pub length: _CassSizeType,
}

//pub type Bytes = Vec<u8>;

pub type _CassDurationType = u64;

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
  fn cass_value_get_int32(value: *const _CassValue, output: *mut i32) -> _CassError;
  fn cass_value_get_int64(value: *const _CassValue, output: *mut i64) -> _CassError;
  fn cass_value_get_float(value: *const _CassValue, output: *mut f32) -> _CassError;
  fn cass_value_get_double(value: *const _CassValue, output: *mut f64) -> _CassError;
  fn cass_value_get_bool(value: *const _CassValue, output: *mut _CassBoolType) -> _CassError;
  fn cass_value_get_uuid(value: *const _CassValue, output: _CassUuid) -> _CassError;
  fn cass_value_get_inet(value: *const _CassValue, output: *mut _CassInet) -> _CassError;
  fn cass_value_get_string(value: *const _CassValue, output: *mut _CassString) -> _CassError;
  fn cass_value_get_bytes(value: *const _CassValue, output: *mut _CassBytes) -> _CassError;
  fn cass_value_get_decimal(value: *const _CassValue, output: *mut Decimal) -> _CassError;
  fn cass_value_type(value: *const _CassValue) -> CassValueType;
  fn cass_value_is_null(value: *const _CassValue) -> _CassBoolType;
  fn cass_value_is_collection(value: *const _CassValue) -> _CassBoolType;
  fn cass_value_item_count(collection: *const _CassValue) -> _CassSizeType;
  fn cass_value_primary_sub_type(collection: *const _CassValue) -> CassValueType;
  fn cass_value_secondary_sub_type(collection: *const _CassValue) -> CassValueType;   
  pub fn cass_string_init(null_terminated: *const c_char) -> _CassString;
  fn cass_string_init2(data: *const c_char, length: _CassSizeType) -> _CassString;
  fn cass_inet_init_v4(address: *const u8) -> _CassInet;
  fn cass_inet_init_v6(address: *const u8) -> _CassInet;
  fn cass_decimal_init(scale: i32, varint: _CassBytes) -> Decimal;
  fn cass_bytes_init(data: *const u8, size: _CassSizeType) -> _CassBytes;
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
