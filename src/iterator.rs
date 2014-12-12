#[allow(dead_code)]
use collection::CassCollection;
use result::Result;
use row::CassRow;
use row::Row;
use schema::CassSchemaMetaField;
use schema::CassSchemaMeta;
use types::CassBoolType;
use types::Value;
use types::CassValue;

use libc::c_uint;

#[allow(dead_code)]
#[allow(non_camel_case_types)] pub enum IteratorType {
  RESULT=0,
  ROW=1,
  COLLECTION=2,
  MAP=3,
  SCHEMA_META=4,
  SCHEMA_META_FIELD=5,
}

//~ impl Drop for RowIterator {
  //~ fn drop(&mut self) {unsafe{
    //~ cass_iterator_free(self.cass_iterator)
  //~ }}
//~ }

pub struct CIterator<T> {
  pub cass_iterator:*mut CassIterator
}

pub type ResultIterator = CIterator<Result>;
pub type RowIterator = CIterator<CassRow>;
pub type CollectionIterator = CIterator<CassCollection>;


#[allow(dead_code)]
enum CIteratorType {
  ResultIterator,
  RowIterator,
  CollectionIterator
}

impl Iterator<Row> for ResultIterator {
  fn next(&mut self) -> Option<Row> {
    if self.has_next() {Some(self.get_next_row())}
    else {None}
  }
}

#[allow(dead_code)]
impl ResultIterator {
  pub fn has_next(&self) -> bool {unsafe{
    cass_iterator_next(self.cass_iterator) > 0
  }}

  pub fn get_next_row(&self) -> Row {unsafe{
    Row{cass_row:cass_iterator_get_row(self.cass_iterator)}
  }}
}

impl Iterator<Value> for CollectionIterator {
  fn next(&mut self) -> Option<Value> {
    if self.has_next() {Some(self.get_next_value())}
    else {None}
  }
}

#[allow(dead_code)]
impl CollectionIterator {
  
  pub fn has_next(&mut self) -> bool {unsafe{
    if self.cass_iterator.is_null() {return false;}
    cass_iterator_next(self.cass_iterator) > 0
  }}

  pub fn get_next_value(&self) -> Value {unsafe{
    println!("iterator selfie: {}",&self.cass_iterator);
    Value{cass_value:cass_iterator_get_value(self.cass_iterator)}
  }}


  pub fn get_next_row(&self) -> Row {unsafe{
    Row{cass_row:cass_iterator_get_row(self.cass_iterator)}
  }}

  pub fn get_next_map_key(&self) -> Value {unsafe{
    Value{cass_value:cass_iterator_get_map_key(self.cass_iterator)}
  }}
  
  pub fn get_next_map_value(self) -> Value {unsafe{
    Value{cass_value:cass_iterator_get_map_value(self.cass_iterator)}
  }}
}

impl Iterator<Value> for RowIterator {
  fn next(&mut self) -> Option<Value> {
    if self.has_next() {Some(self.get_next_value())}
    else {None}
  }
}

#[allow(dead_code)]
impl RowIterator {
  pub fn has_next(&mut self) -> bool {unsafe{
    if self.cass_iterator.is_null() {return false;}
    cass_iterator_next(self.cass_iterator) > 0
  }}

  pub fn get_next_column(&self) -> Value {unsafe{
    Value{cass_value:cass_iterator_get_column(self.cass_iterator)}
  }}

  pub fn get_next_value(&self) -> Value {unsafe{
    Value{cass_value:cass_iterator_get_value(self.cass_iterator)}
  }}
}

  #[repr(C)]
  pub type CassIterator = c_uint;
  #[link(name = "cassandra")]
  extern "C" {
    pub fn cass_iterator_free(iterator: *mut CassIterator);
    pub fn cass_iterator_type(iterator: *mut CassIterator) -> CassIterator;
    pub fn cass_iterator_from_schema_meta(meta: *const CassSchemaMeta) -> *mut CassIterator;
    pub fn cass_iterator_fields_from_schema_meta(meta: *const CassSchemaMeta) -> *mut CassIterator;
    pub fn cass_iterator_next(iterator: *mut CassIterator) -> CassBoolType;
    pub fn cass_iterator_get_row(iterator: *mut CassIterator) -> *const CassRow;
    pub fn cass_iterator_get_column(iterator: *mut CassIterator) -> *const CassValue;
    pub fn cass_iterator_get_value(iterator: *mut CassIterator) -> *const CassValue;
    pub fn cass_iterator_get_map_key(iterator: *mut CassIterator) -> *const CassValue;
    pub fn cass_iterator_get_map_value(iterator: *mut CassIterator) -> *const CassValue;
    pub fn cass_iterator_get_schema_meta(iterator: *mut CassIterator) -> *const CassSchemaMeta;
    pub fn cass_iterator_get_schema_meta_field(iterator: *mut CassIterator) -> *const CassSchemaMetaField;
  }

