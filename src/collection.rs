use error::_CassError;
use error::CassError;
use types::CassValue;
use types::_CassValue;
use types::Decimal;
use types::_CassInet;
use types::_CassBytes;
use types::_CassString;
use types::_CassBoolType;
use types::_CassSizeType;
use types::_CassUuid;
use types::CassValueType;

use std::io::net::ip::IpAddr;
 
use uuid::Uuid;
use result::CassResult;
use row::CassRow;
use row;
use iterator::_CassIterator;


use result;

use iterator::CollectionIterator;

#[allow(dead_code)]
pub enum CollectionType {
  LIST=32 as int,
  MAP=33 as int,
  SET=34 as int,
}

#[allow(dead_code)]
pub struct CassCollection {
  pub collection:*mut _CassCollection,
}

#[allow(dead_code)]
impl CassCollection {
  pub fn new_list(item_count: u64) -> CassCollection {unsafe{
    CassCollection{collection:cass_collection_new(CollectionType::LIST as u32,item_count)}
  }}

  pub fn new_map(item_count: u64) -> CassCollection {unsafe{
    CassCollection{collection:cass_collection_new(CassValueType::MAP as u32,item_count)}
  }}

  pub fn new_set(item_count: u64) -> CassCollection {unsafe{
    CassCollection{collection:cass_collection_new(CassValueType::SET as u32,item_count)}
  }}

  pub fn free(&self) {unsafe{
    cass_collection_free(self.collection)
  }}

  pub fn append_int32(&self, value: i32) -> CassError {unsafe{
    CassError{err:cass_collection_append_int32(self.collection,value)}
  }}

  pub fn append_int64(&self, value: i64) -> CassError {unsafe{
    CassError{err:cass_collection_append_int64(self.collection,value)}
  }}

  pub fn append_float(&self, value: f32) -> CassError {unsafe{
    CassError{err:cass_collection_append_float(self.collection,value)}
  }}

  pub fn append_double(&self, value: f64) -> CassError {unsafe{
    CassError{err:cass_collection_append_double(self.collection,value)}
  }}

  pub fn append_bool(&self, value: bool) -> CassError {unsafe{
    CassError{err:cass_collection_append_bool(self.collection,match value {true=>1,false=>0})}
  }}

  pub fn append_string(&self, value: &String) -> CassError {unsafe{
    let cass_string = CassValue::string_to_cass_string(value);
    CassError{err:cass_collection_append_string(self.collection,cass_string)}
  }}

  pub fn append_str(&self, value: &str) -> CassError {unsafe{
    let cass_string = CassValue::string_to_cass_string(&value.to_string());
    CassError{err:cass_collection_append_string(self.collection,cass_string)}
  }}

  pub fn append_bytes(&self, value: Vec<u8>) -> CassError {unsafe{
    CassError{err:cass_collection_append_bytes(self.collection,CassValue::bytes2cassbytes(&value))}
  }}

  pub fn append_uuid(&self, value: &Uuid) -> CassError {unsafe{
     CassError{err:cass_collection_append_uuid(self.collection, CassValue::uuid_to_cassuuid(value))}
  }}


  pub fn append_inet(&self, value: IpAddr) -> CassError {unsafe{
    CassError{err:cass_collection_append_inet(self.collection,CassValue::ipaddr2cassinet(value))}
  }}

  pub fn append_decimal(&self, value: Decimal) -> CassError {unsafe{
    CassError::new(cass_collection_append_decimal(self.collection,value))
  }}

  pub fn collection_iterator_from_collection(collection:CassValue) -> CollectionIterator {unsafe{
    CollectionIterator{iter:cass_iterator_from_collection(collection.val)}
  }}

  pub fn collection_iterator_from_map(collection:CassValue) -> CollectionIterator {unsafe{
    CollectionIterator{iter:cass_iterator_from_map(collection.val)}
  }}

  pub fn collection_iterator_from_result(result:&CassResult) -> CollectionIterator {unsafe{
    CollectionIterator{iter:result::cass_iterator_from_result(result)}
  }}

  pub fn collection_iterator_from_row(row:CassRow) -> CollectionIterator {unsafe{
    CollectionIterator{iter:row::cass_iterator_from_row(row.row)}
  }}
}

impl Drop for CassCollection {
  fn drop(&mut self) {
    self.free();
  }
}


  pub enum _CassCollection { }
  type CassCollectionType = u32;

  #[link(name = "cassandra")]
  extern "C" {  
    fn cass_collection_new(_type: CassCollectionType, item_count: _CassSizeType) -> *mut _CassCollection;
    fn cass_collection_free(collection: *mut _CassCollection);
    fn cass_collection_append_int32(collection: *mut _CassCollection, value: i32) -> _CassError;
    fn cass_collection_append_int64(collection: *mut _CassCollection, value: i64) -> _CassError;
    fn cass_collection_append_float(collection: *mut _CassCollection, value: f32) -> _CassError;
    fn cass_collection_append_double(collection: *mut _CassCollection, value: f64) -> _CassError;
    fn cass_collection_append_bool(collection: *mut _CassCollection, value: _CassBoolType) -> _CassError;
    fn cass_collection_append_string(collection: *mut _CassCollection, value: _CassString) -> _CassError;
    fn cass_collection_append_bytes(collection: *mut _CassCollection, value: _CassBytes) -> _CassError;
    fn cass_collection_append_uuid(collection: *mut _CassCollection, value: _CassUuid) -> _CassError;
    fn cass_collection_append_inet(collection: *mut _CassCollection, value: _CassInet) -> _CassError;
    fn cass_collection_append_decimal(collection: *mut _CassCollection, value: Decimal) -> _CassError;
    pub fn cass_iterator_from_collection(value: *const _CassValue) -> *mut _CassIterator;
    fn cass_iterator_from_map(value: *const _CassValue) -> *mut _CassIterator;
  }



#[cfg(test)]
mod tests {
use std::str::FromStr;
use std::io::net::ip::IpAddr;
use std::io::net::ip::Ipv4Addr;
use std::io::net::ip::Ipv6Addr;
use super::CassCollection;
use types::CassValue;
//use types::CassDecimal;
  #[test]
  fn new() {
    super::CassCollection::new_list(4);
    super::CassCollection::new_map(5);
    super::CassCollection::new_set(6);
  }

  #[test]
  fn append_list() {
    let mut list = super::CassCollection::new_list(10);
    list.append_bool(true);
    list.append_bytes("cass_bytes".to_string().into_bytes());
    //list.append_decimal(Value::build_cass_decimal(1234567890,3));
    list.append_double(1234.392832f64);
    list.append_float(1234.39232f32);
    list.append_inet(match from_str("127.0.0.1") {
      Some(ip) => ip,
      None => panic!("failed to parse inet address")
    });
    list.append_int32(42i32);
    list.append_int64(42i64);
    list.append_str("abcdefg");
    //FIXME most uuid stuff either doesn't work, or needs a better wrapper
    //list.append_uuid(CassValue::cass_uuid_from_time());
  }

  #[test]
  fn append_map() {
    let mut map = super::CassCollection::new_map(10);
    map.append_bool(true);
    map.append_bytes("cass_bytes".to_string().into_bytes());
    //map.append_decimal(Value::build_cass_decimal(1234567890,3));
    map.append_double(1234.392832f64);
    map.append_float(1234.39232f32);
    map.append_inet(match from_str("127.0.0.1") {
      Some(ip) => ip,
      None => panic!("failed to parse inet address")
    });
    map.append_int32(42i32);
    map.append_int64(42i64);
    //FIXME this append_str causes segfault
    //map.append_str("abcdefg");
    //FIXME most uuid stuff either doesn't work, or needs a better wrapper
    //list.append_uuid(CassValue::cass_uuid_from_time());
  }

  //#[test]
  fn append_set() {
    let mut set = super::CassCollection::new_set(10);
    set.append_bool(true);
    set.append_bytes("cass_bytes".to_string().into_bytes());
    //set.append_decimal(Value::build_cass_decimal(1234567890,3));
    set.append_double(1234.392832f64);
    set.append_float(1234.39232f32);
    set.append_inet(match from_str("127.0.0.1") {
      Some(ip) => ip,
      None => panic!("failed to parse inet address")
    });
    set.append_int32(42i32);
    set.append_int64(42i64);
    //set.append_str("abcdefg");
    //FIXME most uuid stuff either doesn't work, or needs a better wrapper
    //list.append_uuid(CassValue::cass_uuid_from_time());
  }
}

