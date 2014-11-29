use RowIterator;
use CassValue;

pub struct Row {
  pub cass_row:*const internal::CassRow,
}

impl Row {
  pub fn iterator(&mut self) -> RowIterator {unsafe{
    let ref cass_row = *self.cass_row;
    let my_iter = internal::cass_iterator_from_row(cass_row);
    RowIterator{cass_iterator:my_iter}
  }}

  pub fn get_column(&self, index: u64) -> CassValue {unsafe{
    let ref cass_row = *self.cass_row;
    let col = internal::cass_row_get_column(cass_row,index);
    CassValue{cass_value:col}
  }}

  pub fn get_column_by_name(&self, name: &str) -> CassValue {unsafe{
    let ref cass_row = *self.cass_row;
    let col = internal::cass_row_get_column_by_name(cass_row,name.as_ptr() as *const i8);
    CassValue{cass_value:col}
  }}
}

pub mod internal {
  use iterator::internal as iterator_internal;
  use types::internal as types_internal;
  
  pub enum CassRow { }
  #[link(name = "cassandra")]
  extern "C" {
    pub fn cass_iterator_from_row(row: *const CassRow) -> *mut iterator_internal::CassIterator;
    pub fn cass_row_get_column(row: *const CassRow, index: types_internal::CassSizeType) -> *const types_internal::CassValue;
    pub fn cass_row_get_column_by_name(row: *const CassRow, name: *const ::libc::c_char) -> *const types_internal::CassValue;
  }
}
