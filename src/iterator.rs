#[allow(dead_code)]
use row::Row as CassRow;
use types::CassValue;

#[allow(non_camel_case_types)] pub enum IteratorType {
  RESULT=0,
  ROW=1,
  COLLECTION=2,
  MAP=3,
  SCHEMA_META=4,
  SCHEMA_META_FIELD=5,
}


impl Drop for RowIterator {
  fn drop(&mut self) {unsafe{
    internal::cass_iterator_free(self.cass_iterator)
  }}
}

#[allow(dead_code)]
pub struct ResultIterator {
  pub cass_iterator:*mut internal::CassIterator,
}

#[allow(dead_code)]
pub struct RowIterator {
  pub cass_iterator:*mut internal::CassIterator,
}

#[allow(dead_code)]
pub struct CollectionIterator {
  pub cass_iterator:*mut internal::CassIterator,
}

impl Iterator<CassRow> for ResultIterator {
    // The 'Iterator' trait only requires the 'next' method to be defined. The
    // return type is 'Option<T>', 'None' is returned when the 'Iterator' is
    // over, otherwise the next value is returned wrapped in 'Some'
    fn next(&mut self) -> Option<CassRow> {
      if self.has_next() {Some(self.get_next_row())}
      else {None}
    }
}

#[allow(dead_code)]
impl ResultIterator {
  pub fn has_next(&mut self) -> bool {unsafe{
    internal::cass_iterator_next(self.cass_iterator) > 0
  }}

  pub fn get_next_row(&self) -> CassRow {unsafe{
    CassRow{cass_row:internal::cass_iterator_get_row(self.cass_iterator)}
  }}

}

impl Iterator<CassValue> for CollectionIterator {
    // The 'Iterator' trait only requires the 'next' method to be defined. The
    // return type is 'Option<T>', 'None' is returned when the 'Iterator' is
    // over, otherwise the next value is returned wrapped in 'Some'
    fn next(&mut self) -> Option<CassValue> {
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

  pub fn get_next_value(&self) -> CassValue {unsafe{
    println!("iterator selfie: {}",&self.cass_iterator);
    CassValue{cass_value:internal::cass_iterator_get_value(self.cass_iterator)}
  }}


  pub fn get_next_row(&self) -> CassRow {unsafe{
    CassRow{cass_row:internal::cass_iterator_get_row(self.cass_iterator)}
  }}

  pub fn get_next_map_key(&self) -> CassValue {unsafe{
    CassValue{cass_value:internal::cass_iterator_get_map_key(self.cass_iterator)}
  }}
  
  pub fn get_next_map_value(self) -> CassValue {unsafe{
    CassValue{cass_value:internal::cass_iterator_get_map_value(self.cass_iterator)}
  }}

}

impl Iterator<CassValue> for RowIterator {
    // The 'Iterator' trait only requires the 'next' method to be defined. The
    // return type is 'Option<T>', 'None' is returned when the 'Iterator' is
    // over, otherwise the next value is returned wrapped in 'Some'
  fn next(&mut self) -> Option<CassValue> {
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

  pub fn get_next_column(&self) -> CassValue {unsafe{
    CassValue{cass_value:internal::cass_iterator_get_column(self.cass_iterator)}
  }}

  pub fn get_next_value(&self) -> CassValue {unsafe{
    CassValue{cass_value:internal::cass_iterator_get_value(self.cass_iterator)}
  }}
}

pub mod internal {
  use schema::internal as schema_internal;
  use types::internal as types_internal;
  use row::internal as row_internal;
  
  pub type Enum_CassIteratorType_ = ::libc::c_uint;
  pub type CassIteratorType = Enum_CassIteratorType_;

  pub enum Struct_CassIterator_ { }
  pub type CassIterator = Struct_CassIterator_;
  #[link(name = "cassandra")]
  extern "C" {
    pub fn cass_iterator_free(iterator: *mut CassIterator);
    pub fn cass_iterator_type(iterator: *mut CassIterator) -> CassIteratorType;
    pub fn cass_iterator_from_schema(schema: *const schema_internal::CassSchema) -> *mut CassIterator;
    pub fn cass_iterator_from_schema_meta(meta: *const schema_internal::CassSchemaMeta) -> *mut CassIterator;
    pub fn cass_iterator_fields_from_schema_meta(meta: *const schema_internal::CassSchemaMeta) -> *mut CassIterator;
    pub fn cass_iterator_next(iterator: *mut CassIterator) -> types_internal::cass_bool_t;
    pub fn cass_iterator_get_row(iterator: *mut CassIterator) -> *const row_internal::CassRow;
    pub fn cass_iterator_get_column(iterator: *mut CassIterator) -> *const types_internal::CassValue;
    pub fn cass_iterator_get_value(iterator: *mut CassIterator) -> *const types_internal::CassValue;
    pub fn cass_iterator_get_map_key(iterator: *mut CassIterator) -> *const types_internal::CassValue;
    pub fn cass_iterator_get_map_value(iterator: *mut CassIterator) -> *const types_internal::CassValue;
    pub fn cass_iterator_get_schema_meta(iterator: *mut CassIterator) -> *const schema_internal::CassSchemaMeta;
    pub fn cass_iterator_get_schema_meta_field(iterator: *mut CassIterator) -> *const schema_internal::CassSchemaMetaField;
  }
}
