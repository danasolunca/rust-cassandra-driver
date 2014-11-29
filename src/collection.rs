use error::CassError;
use error::Error;
use types::Decimal;
use types::Inet;
use types::Uuid;
use types::Bytes;
use types::Value;
use types::CassValue;
use types::CassDecimal;
use types::CassInet;
use types::CassUuid;
use types::CassBytes;
use types::CassString;
use types::CassBoolType;
use types::CassSizeType;

use types::ValueType;
use types::_ValueType;
use result::CassResult;
use row::Row;
use row;
use iterator::internal as iterator_internal;


use result::internal as result_internal;

use iterator::CollectionIterator;

#[allow(dead_code)]
pub enum CollectionType {
  LIST=32 as int,
  MAP=33 as int,
  SET=34 as int,
}

#[allow(dead_code)]
pub struct Collection {
  pub cass_collection:*mut CassCollection,
}

#[allow(dead_code)]
impl Collection {
  pub fn new_list(item_count: u64) -> Collection {unsafe{
    Collection{cass_collection:cass_collection_new(CollectionType::LIST as u32,item_count)}
  }}

  pub fn new_map(item_count: u64) -> Collection {unsafe{
    Collection{cass_collection:cass_collection_new(_ValueType::MAP as u32,item_count)}
  }}

  pub fn new_set(item_count: u64) -> Collection {unsafe{
    Collection{cass_collection:cass_collection_new(_ValueType::SET as u32,item_count)}
  }}

  pub fn free(&mut self) {unsafe{
    cass_collection_free(self.cass_collection)
  }}

  pub fn append_int32(&mut self, value: i32) -> Error {unsafe{
    Error{cass_error:cass_collection_append_int32(self.cass_collection,value)}
  }}

  pub fn append_int64(&mut self, value: i64) -> Error {unsafe{
    Error{cass_error:cass_collection_append_int64(self.cass_collection,value)}
  }}

  pub fn append_float(&mut self, value: f32) -> Error {unsafe{
    Error{cass_error:cass_collection_append_float(self.cass_collection,value)}
  }}

  pub fn append_double(&mut self, value: f64) -> Error {unsafe{
    Error{cass_error:cass_collection_append_double(self.cass_collection,value)}
  }}

  pub fn append_bool(&mut self, value: bool) -> Error {unsafe{
    Error{cass_error:cass_collection_append_bool(self.cass_collection,match value {true=>1,false=>0})}
  }}

  pub fn append_string(&mut self, value: &String) -> Error {unsafe{
    let cass_string = Value::string_to_cass_string(value);
   Error{cass_error:cass_collection_append_string(self.cass_collection,cass_string)}
  }}

  pub fn append_str(&mut self, value: &str) -> Error {unsafe{
    let cass_string = Value::string_to_cass_string(&value.to_string());
    Error{cass_error:cass_collection_append_string(self.cass_collection,cass_string)}
  }}

  pub fn append_bytes(&mut self, value: Bytes) -> Error {unsafe{
    Error{cass_error:cass_collection_append_bytes(self.cass_collection,value.cass_bytes)}
  }}

  pub fn append_uuid(&mut self, value: Uuid) -> Error {unsafe{
    Error{cass_error:cass_collection_append_uuid(self.cass_collection,value.cass_uuid)}
  }}

  pub fn append_inet(&mut self, value: Inet) -> Error {unsafe{
    Error{cass_error:cass_collection_append_inet(self.cass_collection,value.cass_inet)}
    }}

  pub fn append_decimal(&mut self, value: Decimal) -> Error {unsafe{
    Error::new(cass_collection_append_decimal(self.cass_collection,value.cass_decimal))
  }}

  pub fn collection_iterator_from_collection(collection:Value) -> CollectionIterator {unsafe{
    CollectionIterator{cass_iterator:cass_iterator_from_collection(collection.cass_value)}
  }}

  pub fn collection_iterator_from_map(collection:Value) -> CollectionIterator {unsafe{
    CollectionIterator{cass_iterator:cass_iterator_from_map(collection.cass_value)}
  }}

  pub fn collection_iterator_from_result(result:CassResult) -> CollectionIterator {unsafe{
    CollectionIterator{cass_iterator:result_internal::cass_iterator_from_result(result.cass_result)}
  }}

  pub fn collection_iterator_from_row(row:Row) -> CollectionIterator {unsafe{
    CollectionIterator{cass_iterator:row::cass_iterator_from_row(row.cass_row)}
  }}
}

impl Drop for Collection {
  fn drop(&mut self) {
    self.free();
  }
}


  pub enum CassCollection { }
  type CassCollectionType = u32;

  #[link(name = "cassandra")]
  extern "C" {  
    fn cass_collection_new(_type: CassCollectionType, item_count: CassSizeType) -> *mut CassCollection;
    fn cass_collection_free(collection: *mut CassCollection);
    fn cass_collection_append_int32(collection: *mut CassCollection, value: i32) -> CassError;
    fn cass_collection_append_int64(collection: *mut CassCollection, value: i64) -> CassError;
    fn cass_collection_append_float(collection: *mut CassCollection, value: f32) -> CassError;
    fn cass_collection_append_double(collection: *mut CassCollection, value: f64) -> CassError;
    fn cass_collection_append_bool(collection: *mut CassCollection, value: CassBoolType) -> CassError;
    fn cass_collection_append_string(collection: *mut CassCollection, value: CassString) -> CassError;
    fn cass_collection_append_bytes(collection: *mut CassCollection, value: CassBytes) -> CassError;
    fn cass_collection_append_uuid(collection: *mut CassCollection, value: CassUuid) -> CassError;
    fn cass_collection_append_inet(collection: *mut CassCollection, value: CassInet) -> CassError;
    fn cass_collection_append_decimal(collection: *mut CassCollection, value: CassDecimal) -> CassError;
    pub fn cass_iterator_from_collection(value: *const CassValue) -> *mut iterator_internal::CassIterator;
    fn cass_iterator_from_map(value: *const CassValue) -> *mut iterator_internal::CassIterator;
  }



#[cfg(test)]
mod tests {
use std::str::FromStr;
use std::io::net::ip::IpAddr;
use std::io::net::ip::Ipv4Addr;
use std::io::net::ip::Ipv6Addr;
use types::CassBytes;
use super::CassCollection;
use types::Value;
use types::CassDecimal;
  #[test]
  fn new() {
    super::Collection::new_list(4);
    super::Collection::new_map(5);
    super::Collection::new_set(6);
  }

  #[test]
  fn append_list() {
    let mut list = super::Collection::new_list(10);
    list.append_bool(true);
    list.append_bytes(Value::build_cass_bytes("cass_bytes".to_string().into_bytes()));
    list.append_decimal(Value::build_cass_decimal(1234567890,3));
    list.append_double(1234.392832f64);
    list.append_float(1234.39232f32);
    list.append_inet(Value::build_cass_inet(match from_str("127.0.0.1") {
      Some(ip) => ip,
      None => panic!("failed to parse inet address")
    }));
    list.append_int32(42i32);
    list.append_int64(42i64);
    list.append_str("abcdefg");
    //FIXME most uuid stuff either doesn't work, or needs a better wrapper
    //list.append_uuid(CassValue::cass_uuid_from_time());
  }

  #[test]
  fn append_map() {
    let mut map = super::Collection::new_map(10);
    map.append_bool(true);
    map.append_bytes(Value::build_cass_bytes("cass_bytes".to_string().into_bytes()));
    map.append_decimal(Value::build_cass_decimal(1234567890,3));
    map.append_double(1234.392832f64);
    map.append_float(1234.39232f32);
    map.append_inet(Value::build_cass_inet(match from_str("127.0.0.1") {
      Some(ip) => ip,
      None => panic!("failed to parse inet address")
    }));
    map.append_int32(42i32);
    map.append_int64(42i64);
    //FIXME this append_str causes segfault
    //map.append_str("abcdefg");
    //FIXME most uuid stuff either doesn't work, or needs a better wrapper
    //list.append_uuid(CassValue::cass_uuid_from_time());
  }

  //#[test]
  fn append_set() {
    let mut set = super::Collection::new_set(10);
    set.append_bool(true);
    set.append_bytes(Value::build_cass_bytes("cass_bytes".to_string().into_bytes()));
    set.append_decimal(Value::build_cass_decimal(1234567890,3));
    set.append_double(1234.392832f64);
    set.append_float(1234.39232f32);
    set.append_inet(Value::build_cass_inet(match from_str("127.0.0.1") {
      Some(ip) => ip,
      None => panic!("failed to parse inet address")
    }));
    set.append_int32(42i32);
    set.append_int64(42i64);
    //set.append_str("abcdefg");
    //FIXME most uuid stuff either doesn't work, or needs a better wrapper
    //list.append_uuid(CassValue::cass_uuid_from_time());
  }
}

