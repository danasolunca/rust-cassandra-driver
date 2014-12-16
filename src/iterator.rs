#[allow(dead_code)]
use collection::_CassCollection;
use result::CassResult;
use row::_CassRow;
use row::CassRow;
use types::_CassBoolType;
use types::CassValue;
use types::_CassValue;

use libc::c_uint;

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum IteratorType {
  RESULT=0,
  ROW=1,
  COLLECTION=2,
  MAP=3,
  SCHEMA_META=4,
  SCHEMA_META_FIELD=5,
}

//~ impl Drop for CassIterator {
  //~ fn drop(&mut self) {unsafe{
    //~ cass_iterator_free(self.cass_iterator)
  //~ }}
//~ }

pub struct CassIterator<CassIteratorType> {
  pub iter:*mut _CassIterator
}

pub type ResultIterator = CassIterator<CassResult>;
pub type RowIterator = CassIterator<_CassRow>;
pub type CollectionIterator = CassIterator<_CassCollection>;
pub type SetIterator = CassIterator<_CassCollection>;


#[allow(dead_code)]
enum CassIteratorType {
  ResultIterator,
  RowIterator,
  CollectionIterator
}



#[allow(dead_code)]
impl Iterator<CassRow> for ResultIterator {
  fn next(&mut self) -> Option<CassRow> {unsafe{
    match cass_iterator_next(self.iter) > 0 {
      true => Some(CassRow{row:cass_iterator_get_row(self.iter)}),
      false => None
    }
  }}
}

impl Iterator<CassValue> for CollectionIterator {
  fn next(&mut self) -> Option<CassValue> {
    if self.has_next() {Some(self.get_next_value())}
    else {None}
  }
}

#[allow(dead_code)]
impl CollectionIterator {
  
  pub fn has_next(&mut self) -> bool {unsafe{
    if self.iter.is_null() {return false;}
    cass_iterator_next(self.iter) > 0
  }}

  pub fn get_next_value(&self) -> CassValue {unsafe{
    println!("iterator selfie: {}",&self.iter);
    CassValue{val:cass_iterator_get_value(self.iter)}
  }}


  pub fn get_next_row(&self) -> CassRow {unsafe{
    CassRow{row:cass_iterator_get_row(self.iter)}
  }}

  pub fn get_next_map_key(&self) -> CassValue {unsafe{
    CassValue{val:cass_iterator_get_map_key(self.iter)}
  }}
  
  pub fn get_next_map_value(self) -> CassValue {unsafe{
    CassValue{val:cass_iterator_get_map_value(self.iter)}
  }}
}

impl Iterator<CassValue> for RowIterator {
  fn next(&mut self) -> Option<CassValue> {
    if self.has_next() {Some(self.get_next_value())}
    else {None}
  }
}

#[allow(dead_code)]
impl RowIterator {
  pub fn has_next(&mut self) -> bool {unsafe{
    if self.iter.is_null() {return false;}
    cass_iterator_next(self.iter) > 0
  }}

  pub fn get_next_column(&self) -> CassValue {unsafe{
    CassValue{val:cass_iterator_get_column(self.iter)}
  }}

  pub fn get_next_value(&self) -> CassValue {unsafe{
    CassValue{val:cass_iterator_get_value(self.iter)}
  }}
}

#[repr(C)]
pub type _CassIterator = c_uint;
#[link(name = "cassandra")]
extern "C" {
  pub fn cass_iterator_free(iterator: *mut _CassIterator);
  pub fn cass_iterator_type(iterator: *mut _CassIterator) -> CassIteratorType;
  pub fn cass_iterator_next(iterator: *mut _CassIterator) -> _CassBoolType;
  pub fn cass_iterator_get_row(iterator: *mut _CassIterator) -> *const _CassRow;
  pub fn cass_iterator_get_column(iterator: *mut _CassIterator) -> *const _CassValue;
  pub fn cass_iterator_get_value(iterator: *mut _CassIterator) -> *const _CassValue;
  pub fn cass_iterator_get_map_key(iterator: *mut _CassIterator) -> *const _CassValue;
  pub fn cass_iterator_get_map_value(iterator: *mut _CassIterator) -> *const _CassValue;
 }

