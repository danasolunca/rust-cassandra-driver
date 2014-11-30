#[allow(dead_code)]
use row::Row;
use types::Value;
use result::Result;
use collection::CassCollection;
use row::CassRow;

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
    //~ internal::cass_iterator_free(self.cass_iterator)
  //~ }}
//~ }

pub struct CassIterator<T> {
  pub cass_iterator:*mut internal::CassIterator
}

pub type ResultIterator = CassIterator<Result>;
pub type RowIterator = CassIterator<CassRow>;
pub type CollectionIterator = CassIterator<CassCollection>;


#[allow(dead_code)]
enum CassIteratorType {
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
  pub fn has_next(&mut self) -> bool {unsafe{
    internal::cass_iterator_next(self.cass_iterator) > 0
  }}

  pub fn get_next_row(&self) -> Row {unsafe{
    Row{cass_row:internal::cass_iterator_get_row(self.cass_iterator)}
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
    internal::cass_iterator_next(self.cass_iterator) > 0
  }}

  pub fn get_next_value(&self) -> Value {unsafe{
    println!("iterator selfie: {}",&self.cass_iterator);
    Value{cass_value:internal::cass_iterator_get_value(self.cass_iterator)}
  }}


  pub fn get_next_row(&self) -> Row {unsafe{
    Row{cass_row:internal::cass_iterator_get_row(self.cass_iterator)}
  }}

  pub fn get_next_map_key(&self) -> Value {unsafe{
    Value{cass_value:internal::cass_iterator_get_map_key(self.cass_iterator)}
  }}
  
  pub fn get_next_map_value(self) -> Value {unsafe{
    Value{cass_value:internal::cass_iterator_get_map_value(self.cass_iterator)}
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
    internal::cass_iterator_next(self.cass_iterator) > 0
  }}

  pub fn get_next_column(&self) -> Value {unsafe{
    Value{cass_value:internal::cass_iterator_get_column(self.cass_iterator)}
  }}

  pub fn get_next_value(&self) -> Value {unsafe{
    Value{cass_value:internal::cass_iterator_get_value(self.cass_iterator)}
  }}


}

pub mod internal {
  use row::CassRow;
  use types::CassValue;
  use types::CassBoolType;
//  use schema::internal as schema_internal;
  use row;
  #[repr(C)]
  pub type CassIterator = ::libc::c_uint;
  #[link(name = "cassandra")]
  extern "C" {
    pub fn cass_iterator_free(iterator: *mut CassIterator);
    pub fn cass_iterator_type(iterator: *mut CassIterator) -> CassIterator;
    //~ pub fn cass_iterator_from_schema_meta(meta: *const schema_internal::CassSchemaMeta) -> *mut CassIterator;
    //~ pub fn cass_iterator_fields_from_schema_meta(meta: *const schema_internal::CassSchemaMeta) -> *mut CassIterator;
    pub fn cass_iterator_next(iterator: *mut CassIterator) -> CassBoolType;
    pub fn cass_iterator_get_row(iterator: *mut CassIterator) -> *const CassRow;
    pub fn cass_iterator_get_column(iterator: *mut CassIterator) -> *const CassValue;
    pub fn cass_iterator_get_value(iterator: *mut CassIterator) -> *const CassValue;
    pub fn cass_iterator_get_map_key(iterator: *mut CassIterator) -> *const CassValue;
    pub fn cass_iterator_get_map_value(iterator: *mut CassIterator) -> *const CassValue;
    //~ pub fn cass_iterator_get_schema_meta(iterator: *mut CassIterator) -> *const schema_internal::CassSchemaMeta;
    //~ pub fn cass_iterator_get_schema_meta_field(iterator: *mut CassIterator) -> *const schema_internal::CassSchemaMetaField;
  }
}
