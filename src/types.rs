extern crate libc;

use std::string::raw;

use cass_internal_api;
use error::Error as CassError;
use libc::c_char;
use std::vec::Vec;
use std::num::Int;

use std::io::net::ip::IpAddr;
use std::io::net::ip::Ipv4Addr;
use std::io::net::ip::Ipv6Addr;

//use cassandra_internal_api::CASS_COLLECTION_TYPE_LIST as TYPE_LIST;
//use cassandra_internal_api::CASS_COLLECTION_TYPE_MAP as TYPE_MAP;
//use cassandra_internal_api::CASS_COLLECTION_TYPE_SET as TYPE_SET;

use CollectionIterator;


#[allow(dead_code)]
pub struct CassValue {
  pub cass_value:*const cass_internal_api::CassValue
}

#[allow(dead_code)]
pub struct CassUuid {
  pub cass_uuid:cass_internal_api::CassUuid,
}

#[allow(dead_code)]
pub struct CassInet {
  pub cass_inet:cass_internal_api::CassInet,
}


#[allow(dead_code)]
pub struct CassBytes {
  pub cass_bytes:cass_internal_api::CassBytes,
}

#[allow(dead_code)]
pub struct CassDecimal {
  pub cass_decimal:cass_internal_api::CassDecimal,
}

#[allow(dead_code)]
pub struct CassValueType {
  cass_value_type:cass_internal_api::CassValueType,
}

#[allow(dead_code)]
impl CassValue {

  pub fn get_collection_iterator(&self) -> CollectionIterator {unsafe{
    CollectionIterator{cass_iterator:cass_internal_api::cass_iterator_from_collection(self.cass_value)}
  }}

  pub fn get_string(self) -> Result<String,CassError> {unsafe{
    let ref mut output:cass_internal_api::Struct_CassString_=cass_internal_api::cass_string_init(self.cass_value as *const i8);
    let ref mut output = *output;
    let err_string = cass_internal_api::cass_value_get_string(self.cass_value,&mut*output);
    let err = CassError{cass_error:err_string};
    let ref mut output = *output;
    if err.cass_error == cass_internal_api::CASS_OK {
      let length=output.length as uint;
      println!("item length: {}", length);
      println!("raw: {}", raw::from_parts(output.data as *mut u8, length, length));
      Ok(raw::from_parts(output.data as *mut u8, length, length))
    } else {Err(err)}
  }}

  pub fn get_int32(self) ->  Result<cass_internal_api::cass_int32_t,CassError> {unsafe{
    let ref mut output:cass_internal_api::cass_int32_t=0;
    let err = CassError{cass_error:cass_internal_api::cass_value_get_int32(self.cass_value,output)};
    if err.cass_error == cass_internal_api::CASS_OK {return Ok(*output)} else {return Err(err)}
  }}

  pub fn get_int64(self) ->  Result<cass_internal_api::cass_int64_t,CassError> {unsafe{
    let ref mut output:cass_internal_api::cass_int64_t=0;
    let err = CassError{cass_error:cass_internal_api::cass_value_get_int64(self.cass_value,output)};
    if err.cass_error == cass_internal_api::CASS_OK {return Ok(*output)} else {return Err(err)}
    }}

  pub fn get_float(self) ->  Result<cass_internal_api::cass_float_t,CassError> {unsafe{
    let ref mut output:cass_internal_api::cass_float_t=0.0;
    let err = CassError{cass_error:cass_internal_api::cass_value_get_float(self.cass_value,output)};
    if err.cass_error == cass_internal_api::CASS_OK {return Ok(*output)} else {return Err(err)}
  }}

  pub fn get_double(self) -> Result<f64,CassError> {unsafe{
    let ref mut output:f64=0.0;
    let err = CassError{cass_error:cass_internal_api::cass_value_get_double(self.cass_value,output)};
    if err.cass_error == cass_internal_api::CASS_OK {return Ok(*output)} else {return Err(err)}
  }}

  pub fn get_bool(self) -> Result<bool,CassError> {unsafe{
    let ref mut output:u32=0;
    let err = CassError{cass_error:cass_internal_api::cass_value_get_bool(self.cass_value,output)};
    if err.cass_error == cass_internal_api::CASS_OK {return Ok(*output> 0)} else {return Err(err)}
  }}

  pub fn get_uuid(self, output: CassUuid) -> CassError {unsafe{
    CassError{cass_error:cass_internal_api::cass_value_get_uuid(self.cass_value,output.cass_uuid)}
  }}

  pub fn get_inet(self, mut output: CassInet) -> CassError {unsafe{
    let ref mut cass_inet = output.cass_inet;
    CassError{cass_error:cass_internal_api::cass_value_get_inet(self.cass_value,cass_inet)}
  }}

  pub fn get_bytes(self, mut output: CassBytes) -> CassError {unsafe{
    let ref mut my_bytes = output.cass_bytes;
    CassError{cass_error:cass_internal_api::cass_value_get_bytes(self.cass_value,my_bytes)}
  }}

  pub fn get_decimal(self, mut output: CassDecimal) -> CassError {unsafe{
    let ref mut my_decimal = output.cass_decimal;
    CassError{cass_error:cass_internal_api::cass_value_get_decimal(self.cass_value,my_decimal)}
  }}

  pub fn is_null(self) -> bool {unsafe{
    !cass_internal_api::cass_value_is_null(self.cass_value) == Int::zero()
  }}

  pub fn is_collection(self) -> bool {unsafe{
    !cass_internal_api::cass_value_is_collection(self.cass_value) == Int::zero()
  }}

  pub fn item_count(self) -> u64 {unsafe{
    cass_internal_api::cass_value_item_count(self.cass_value)
  }}

  pub fn primary_sub_type(self) -> CassValueType {unsafe{
    CassValueType{cass_value_type:cass_internal_api::cass_value_primary_sub_type(self.cass_value)}
  }}

  pub fn secondary_sub_type(self) -> CassValueType {unsafe{
    CassValueType{cass_value_type:cass_internal_api::cass_value_secondary_sub_type(self.cass_value)}
  }}

  //FIXME segfaults
  pub fn generate_timeuuid() -> CassUuid {unsafe{
    let output:cass_internal_api::CassUuid = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
    cass_internal_api::cass_uuid_generate_time(output);
    CassUuid{cass_uuid:output}
  }}

  fn cass_uuid_from_time(time: cass_internal_api::cass_uint64_t, output: CassUuid) {unsafe{
    cass_internal_api::cass_uuid_from_time(time,output.cass_uuid);
  }}

  pub fn cass_uuid_min_from_time(time: cass_internal_api::cass_uint64_t, output: CassUuid) {unsafe{
    cass_internal_api::cass_uuid_min_from_time(time,output.cass_uuid)
  }}

  pub fn cass_uuid_max_from_time(time: cass_internal_api::cass_uint64_t, output: CassUuid) {unsafe{
    cass_internal_api::cass_uuid_max_from_time(time,output.cass_uuid)
  }}

  pub fn cass_uuid_generate_random(output: CassUuid) {unsafe{
    cass_internal_api::cass_uuid_generate_random(output.cass_uuid)
  }}

  pub fn cass_uuid_timestamp(uuid: CassUuid) -> cass_internal_api::cass_uint64_t {unsafe{
    cass_internal_api::cass_uuid_timestamp(uuid.cass_uuid)
  }}

  pub fn cass_uuid_version(uuid: CassUuid) -> cass_internal_api::cass_uint8_t {unsafe{
    cass_internal_api::cass_uuid_version(uuid.cass_uuid)
  }}

  pub fn cass_uuid_string(uuid: CassUuid, output: *mut c_char) {unsafe{
    cass_internal_api::cass_uuid_string(uuid.cass_uuid,output)
  }}

  fn cass_inet_init_v4(address: *const cass_internal_api::cass_uint8_t) -> CassInet {unsafe{
    CassInet{cass_inet:cass_internal_api::cass_inet_init_v4(address)}
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

  pub fn str_to_cass_string(string:&String) -> cass_internal_api::CassString {unsafe{
     cass_internal_api::cass_string_init2(string.as_bytes().as_ptr() as *const i8,string.as_bytes().len() as u64)
  }}

  pub fn cass_string_to_str(cass_str:cass_internal_api::CassString) -> String {unsafe{
    raw::from_buf_len(cass_str.data as *const u8,cass_str.length as uint)
  }}

  //PRIVATE METHODS
  fn cass_inet_init_v6(address: *const cass_internal_api::cass_uint8_t) -> CassInet {unsafe{
    CassInet{cass_inet:cass_internal_api::cass_inet_init_v6(address)}
  }}

  fn cass_decimal_init(scale: cass_internal_api::cass_int32_t, varint: CassBytes) -> CassDecimal {unsafe{
    CassDecimal{cass_decimal:cass_internal_api::cass_decimal_init(scale,varint.cass_bytes)}
  }}

  fn cass_bytes_init(data: *const cass_internal_api::cass_byte_t, size: cass_internal_api::cass_size_t) -> CassBytes {unsafe{
    CassBytes{cass_bytes:cass_internal_api::cass_bytes_init(data,size)}
  }}
}

#[cfg(test)]
  mod tests {
    use cass_internal_api;
    #[test]
    fn string_wrapping() {
      let test_string = "test_string2345678".to_string();
      let cass_string:cassandra_internal_api::CassString = super::CassValue::str_to_cass_string(&test_string);
      //println!("cassstr: {}", cass_string);
      let reconstituted:String = super::CassValue::cass_string_to_str(cass_string);
      println!("reconstituted: {}", reconstituted);
      assert!(test_string == reconstituted);
    }
}
