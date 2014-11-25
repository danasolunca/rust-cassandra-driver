use error::Error as CassError;
use types::CassDecimal;
use types::CassInet;
use types::CassUuid;
use types::CassBytes;
use types::CassValue;
use types::ValueType;
use result::CassResult;
use row::Row;

use types::internal as types_internal;
use row::internal as row_internal;
use collection::internal as collection_internal;
use result::internal as result_internal;

use iterator::CollectionIterator;

#[allow(dead_code)]
pub enum CassCollectionType {
  LIST=32 as int,
  MAP=33 as int,
  SET=34 as int,
}

#[allow(dead_code)]
pub struct CassCollection {
  pub cass_collection:*mut internal::CassCollection,
}

#[allow(dead_code)]
impl CassCollection {
  pub fn new_list(item_count: u64) -> CassCollection {unsafe{
    CassCollection{cass_collection:internal::cass_collection_new(CassCollectionType::LIST as u32,item_count)}
  }}

  pub fn new_map(item_count: u64) -> CassCollection {unsafe{
    CassCollection{cass_collection:internal::cass_collection_new(ValueType::MAP as u32,item_count)}
  }}

  pub fn new_set(item_count: u64) -> CassCollection {unsafe{
    CassCollection{cass_collection:internal::cass_collection_new(ValueType::SET as u32,item_count)}
  }}

  pub fn free(&mut self) {unsafe{
    internal::cass_collection_free(self.cass_collection)
  }}

  pub fn append_int32(&mut self, value: i32) -> CassError {unsafe{
    CassError{cass_error:internal::cass_collection_append_int32(self.cass_collection,value)}
  }}

  pub fn append_int64(&mut self, value: i64) -> CassError {unsafe{
    CassError{cass_error:internal::cass_collection_append_int64(self.cass_collection,value)}
  }}

  pub fn append_float(&mut self, value: f32) -> CassError {unsafe{
      CassError{cass_error:internal::cass_collection_append_float(self.cass_collection,value)}
  }}

  pub fn append_double(&mut self, value: f64) -> CassError {unsafe{
    CassError{cass_error:internal::cass_collection_append_double(self.cass_collection,value)}
  }}

  pub fn append_bool(&mut self, value: bool) -> CassError {unsafe{
    CassError{cass_error:internal::cass_collection_append_bool(self.cass_collection,match value {true=>1,false=>0})}
  }}

  pub fn append_string(&mut self, value: &String) -> CassError {unsafe{
    let cass_string = CassValue::str_to_cass_string(value);
   CassError{cass_error:internal::cass_collection_append_string(self.cass_collection,cass_string)}
  }}

  pub fn append_str(&mut self, value: &str) -> CassError {unsafe{
    let cass_string = CassValue::str_to_cass_string(&value.to_string());
    CassError{cass_error:internal::cass_collection_append_string(self.cass_collection,cass_string)}
  }}

  pub fn append_bytes(&mut self, value: CassBytes) -> CassError {unsafe{
    CassError{cass_error:internal::cass_collection_append_bytes(self.cass_collection,value.cass_bytes)}
  }}

  pub fn append_uuid(&mut self, value: CassUuid) -> CassError {unsafe{
    CassError{cass_error:internal::cass_collection_append_uuid(self.cass_collection,value.cass_uuid)}
  }}

  pub fn append_inet(&mut self, value: CassInet) -> CassError {unsafe{
    CassError{cass_error:internal::cass_collection_append_inet(self.cass_collection,value.cass_inet)}
    }}

  pub fn append_decimal(&mut self, value: CassDecimal) -> CassError {unsafe{
    CassError::new(internal::cass_collection_append_decimal(self.cass_collection,value.cass_decimal))
    }}

  pub fn collection_iterator_from_collection(collection:CassValue) -> CollectionIterator {unsafe{
    CollectionIterator{cass_iterator:internal::cass_iterator_from_collection(collection.cass_value)}
  }}

  pub fn collection_iterator_from_map(collection:CassValue) -> CollectionIterator {unsafe{
    CollectionIterator{cass_iterator:internal::cass_iterator_from_map(collection.cass_value)}
  }}

  pub fn collection_iterator_from_result(result:CassResult) -> CollectionIterator {unsafe{
    CollectionIterator{cass_iterator:result_internal::cass_iterator_from_result(result.cass_result)}
  }}

  pub fn collection_iterator_from_row(row:Row) -> CollectionIterator {unsafe{
    CollectionIterator{cass_iterator:row_internal::cass_iterator_from_row(row.cass_row)}
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

pub mod internal {
  use error::internal as error_internal;
  use types::internal as types_internal;
  use iterator::internal as iterator_internal;
  pub enum Struct_CassCollection_ { }
  pub type CassCollection = Struct_CassCollection_;
  pub type Enum_CassCollectionType_ = ::libc::c_uint;
  pub type CassCollectionType = Enum_CassCollectionType_;

  #[link(name = "cassandra")]
  extern "C" {  
    pub fn cass_collection_new(_type: CassCollectionType, item_count: types_internal::cass_size_t) -> *mut CassCollection;
    pub fn cass_collection_free(collection: *mut CassCollection);
    pub fn cass_collection_append_int32(collection: *mut CassCollection, value: i32) -> error_internal::CassError;
    pub fn cass_collection_append_int64(collection: *mut CassCollection, value: i64) -> error_internal::CassError;
    pub fn cass_collection_append_float(collection: *mut CassCollection, value: f32) -> error_internal::CassError;
    pub fn cass_collection_append_double(collection: *mut CassCollection, value: f64) -> error_internal::CassError;
    pub fn cass_collection_append_bool(collection: *mut CassCollection, value: types_internal::cass_bool_t) -> error_internal::CassError;
    pub fn cass_collection_append_string(collection: *mut CassCollection, value: types_internal::CassString) -> error_internal::CassError;
    pub fn cass_collection_append_bytes(collection: *mut CassCollection, value: types_internal::CassBytes) -> error_internal::CassError;
    pub fn cass_collection_append_uuid(collection: *mut CassCollection, value: types_internal::CassUuid) -> error_internal::CassError;
    pub fn cass_collection_append_inet(collection: *mut CassCollection, value: types_internal::CassInet) -> error_internal::CassError;
    pub fn cass_collection_append_decimal(collection: *mut CassCollection, value: types_internal::CassDecimal) -> error_internal::CassError;
    pub fn cass_iterator_from_collection(value: *const types_internal::CassValue) -> *mut iterator_internal::CassIterator;
    pub fn cass_iterator_from_map(value: *const types_internal::CassValue) -> *mut iterator_internal::CassIterator;
  }
}
