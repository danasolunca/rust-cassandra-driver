#[allow(dead_code)]
use row::Row as CassRow;
use types::CassValue;
use cass_internal_api;

impl Drop for RowIterator {
  fn drop(&mut self) {unsafe{
    cass_internal_api::cass_iterator_free(self.cass_iterator)
  }}
}

#[allow(dead_code)]
pub struct ResultIterator {
  pub cass_iterator:*mut cass_internal_api::CassIterator,
}

#[allow(dead_code)]
pub struct RowIterator {
  pub cass_iterator:*mut cass_internal_api::CassIterator,
}

#[allow(dead_code)]
pub struct CollectionIterator {
  pub cass_iterator:*mut cass_internal_api::CassIterator,
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
    cass_internal_api::cass_iterator_next(self.cass_iterator) > 0
  }}

  pub fn get_next_row(&self) -> CassRow {unsafe{
    CassRow{cass_row:cass_internal_api::cass_iterator_get_row(self.cass_iterator)}
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
    cass_internal_api::cass_iterator_next(self.cass_iterator) > 0
  }}

  pub fn get_next_value(&self) -> CassValue {unsafe{
    println!("iterator selfie: {}",&self.cass_iterator);
    CassValue{cass_value:cass_internal_api::cass_iterator_get_value(self.cass_iterator)}
  }}


  pub fn get_next_row(&self) -> CassRow {unsafe{
    CassRow{cass_row:cass_internal_api::cass_iterator_get_row(self.cass_iterator)}
  }}

  pub fn get_next_map_key(&self) -> CassValue {unsafe{
    CassValue{cass_value:cass_internal_api::cass_iterator_get_map_key(self.cass_iterator)}
  }}
  
  pub fn get_next_map_value(self) -> CassValue {unsafe{
    CassValue{cass_value:cass_internal_api::cass_iterator_get_map_value(self.cass_iterator)}
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
    cass_internal_api::cass_iterator_next(self.cass_iterator) > 0
  }}

  pub fn get_next_column(&self) -> CassValue {unsafe{
    CassValue{cass_value:cass_internal_api::cass_iterator_get_column(self.cass_iterator)}
  }}

  pub fn get_next_value(&self) -> CassValue {unsafe{
    CassValue{cass_value:cass_internal_api::cass_iterator_get_value(self.cass_iterator)}
  }}
}
