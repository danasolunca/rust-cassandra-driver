use RowIterator;
use Value;

use iterator::CassIterator;
use types::CassValue;
use types::CassSizeType;
  
pub struct Row {
  pub cass_row:*const CassRow,
}

impl Row {
  pub fn iterator(&mut self) -> RowIterator {unsafe{
    let ref cass_row = *self.cass_row;
    let my_iter = cass_iterator_from_row(cass_row);
    RowIterator{cass_iterator:my_iter}
  }}

  pub fn get_column(&self, index: u64) -> Value {unsafe{
    let ref cass_row = *self.cass_row;
    let col = cass_row_get_column(cass_row,index);
    Value{cass_value:col}
  }}

  pub fn get_column_by_name(&self, name: &str) -> Value {unsafe{
    let ref cass_row = *self.cass_row;
    let col = cass_row_get_column_by_name(cass_row,name.as_ptr() as *const i8);
    Value{cass_value:col}
  }}
}
  pub enum CassRow { }
  #[link(name = "cassandra")]
  extern "C" {
    pub fn cass_iterator_from_row(row: *const CassRow) -> *mut CassIterator;
    fn cass_row_get_column(row: *const CassRow, index: CassSizeType) -> *const CassValue;
    fn cass_row_get_column_by_name(row: *const CassRow, name: *const ::libc::c_char) -> *const CassValue;
  }

