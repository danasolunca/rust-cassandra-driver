use error::Error as CassError;
use types::CassDecimal;
use types::CassInet;
use types::CassUuid;
use types::CassBytes;
use types::CassValue;
use result::CassResult;
use row::Row;



pub use cass_internal_api::CASS_COLLECTION_TYPE_LIST as LIST_TYPE;
pub use cass_internal_api;

use iterator::CollectionIterator;

pub type CassCollectionType = u32;

#[allow(dead_code)]
pub struct CassCollection {
  pub cass_collection:*mut cass_internal_api::CassCollection,
}

#[allow(dead_code)]
impl CassCollection {
  pub fn new_list(item_count: u64) -> CassCollection {unsafe{
    CassCollection{cass_collection:cass_internal_api::cass_collection_new(LIST_TYPE,item_count)}
  }}

  pub fn new_map(item_count: u64) -> CassCollection {unsafe{
    CassCollection{cass_collection:cass_internal_api::cass_collection_new(cass_internal_api::CASS_VALUE_TYPE_MAP,item_count)}
  }}

  pub fn new_set(item_count: u64) -> CassCollection {unsafe{
    CassCollection{cass_collection:cass_internal_api::cass_collection_new(cass_internal_api::CASS_VALUE_TYPE_SET,item_count)}
  }}

  pub fn free(&mut self) {unsafe{
    cass_internal_api::cass_collection_free(self.cass_collection)
  }}

  pub fn append_int32(&mut self, value: i32) -> CassError {unsafe{
    CassError{cass_error:cass_internal_api::cass_collection_append_int32(self.cass_collection,value)}
  }}

  pub fn append_int64(&mut self, value: i64) -> CassError {unsafe{
    CassError{cass_error:cass_internal_api::cass_collection_append_int64(self.cass_collection,value)}
  }}

  pub fn append_float(&mut self, value: f32) -> CassError {unsafe{
      CassError{cass_error:cass_internal_api::cass_collection_append_float(self.cass_collection,value)}
  }}

  pub fn append_double(&mut self, value: f64) -> CassError {unsafe{
    CassError{cass_error:cass_internal_api::cass_collection_append_double(self.cass_collection,value)}
  }}

  pub fn append_bool(&mut self, value: bool) -> CassError {unsafe{
    CassError{cass_error:cass_internal_api::cass_collection_append_bool(self.cass_collection,match value {true=>1,false=>0})}
  }}

  pub fn append_string(&mut self, value: &String) -> CassError {unsafe{
    let cass_string = CassValue::str_to_cass_string(value);
   CassError{cass_error:cass_internal_api::cass_collection_append_string(self.cass_collection,cass_string)}
  }}

  pub fn append_str(&mut self, value: &str) -> CassError {unsafe{
    let cass_string = CassValue::str_to_cass_string(&value.to_string());
   CassError{cass_error:cass_internal_api::cass_collection_append_string(self.cass_collection,cass_string)}
  }}

  pub fn append_bytes(&mut self, value: CassBytes) -> CassError {unsafe{
    CassError{cass_error:cass_internal_api::cass_collection_append_bytes(self.cass_collection,value.cass_bytes)}
  }}

  pub fn append_uuid(&mut self, value: CassUuid) -> CassError {unsafe{
    CassError{cass_error:cass_internal_api::cass_collection_append_uuid(self.cass_collection,value.cass_uuid)}
  }}

  pub fn append_inet(&mut self, value: CassInet) -> CassError {unsafe{
    CassError{cass_error:cass_internal_api::cass_collection_append_inet(self.cass_collection,value.cass_inet)}
    }}

  pub fn append_decimal(&mut self, value: CassDecimal) -> CassError {unsafe{
    CassError::new(cass_internal_api::cass_collection_append_decimal(self.cass_collection,value.cass_decimal))
    }}

  pub fn collection_iterator_from_collection(collection:CassValue) -> CollectionIterator {unsafe{
    CollectionIterator{cass_iterator:cass_internal_api::cass_iterator_from_collection(collection.cass_value)}
  }}

  pub fn collection_iterator_from_map(collection:CassValue) -> CollectionIterator {unsafe{
    CollectionIterator{cass_iterator:cass_internal_api::cass_iterator_from_map(collection.cass_value)}
  }}

  pub fn collection_iterator_from_result(result:CassResult) -> CollectionIterator {unsafe{
    CollectionIterator{cass_iterator:cass_internal_api::cass_iterator_from_result(result.cass_result)}
  }}

  pub fn collection_iterator_from_row(row:Row) -> CollectionIterator {unsafe{
    CollectionIterator{cass_iterator:cass_internal_api::cass_iterator_from_row(row.cass_row)}
  }}
}

//~ //FIXME this should not require a lifetime 
//~ impl Drop for Collection {
  //~ fn drop(&mut self) {
    //~ self.free();
  //~ }
//~ }

#[cfg(test)]
mod tests {
use std::str::FromStr;
use std::io::net::ip::IpAddr;
use std::io::net::ip::Ipv4Addr;
use std::io::net::ip::Ipv6Addr;
use types::CassBytes;
use super::CassCollection;
use types::CassValue;
use types::CassDecimal;
  #[test]
  fn new() {
    super::CassCollection::new_list(4);
    super::CassCollection::new_map(5);
    super::CassCollection::new_set(6);
  }

  #[test]
  fn append_list() {
    let mut list = CassCollection::new_list(10);
    list.append_bool(true);
    list.append_bytes(CassValue::build_cass_bytes("cass_bytes".to_string().into_bytes()));
    list.append_decimal(CassValue::build_cass_decimal(1234567890,3));
    list.append_double(1234.392832f64);
    list.append_float(1234.39232f32);
    list.append_inet(CassValue::build_cass_inet(match from_str("127.0.0.1") {
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
    let mut map = CassCollection::new_map(10);
    map.append_bool(true);
    map.append_bytes(CassValue::build_cass_bytes("cass_bytes".to_string().into_bytes()));
    map.append_decimal(CassValue::build_cass_decimal(1234567890,3));
    map.append_double(1234.392832f64);
    map.append_float(1234.39232f32);
    map.append_inet(CassValue::build_cass_inet(match from_str("127.0.0.1") {
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
    let mut set = CassCollection::new_set(10);
    set.append_bool(true);
    set.append_bytes(CassValue::build_cass_bytes("cass_bytes".to_string().into_bytes()));
    set.append_decimal(CassValue::build_cass_decimal(1234567890,3));
    set.append_double(1234.392832f64);
    set.append_float(1234.39232f32);
    set.append_inet(CassValue::build_cass_inet(match from_str("127.0.0.1") {
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
