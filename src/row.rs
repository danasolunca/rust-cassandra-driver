use RowIterator;
use types::CassValue;

use iterator::_CassIterator;
use types::_CassValue;
use types::_CassSizeType;

use libc::c_char;
  
pub struct CassRow {
  pub row:*const _CassRow,
}
impl Copy for CassRow {}


impl CassRow {
  pub fn iterator(&mut self) -> RowIterator {unsafe{
    let ref cass_row = *self.row;
    let my_iter = cass_iterator_from_row(cass_row);
    RowIterator{iter:my_iter}
  }}

  pub fn get_column(&self, index: u64) -> CassValue {unsafe{
    let ref cass_row = *self.row;
    let col = cass_row_get_column(cass_row,index);
    CassValue{val:col}
  }}

  pub fn get_column_by_name(&self, name: &str) -> CassValue {unsafe{
    let ref cass_row = *self.row;
    let col = cass_row_get_column_by_name(cass_row,name.as_ptr() as *const i8);
    CassValue{val:col}
  }}
}
  pub enum _CassRow { }
  #[link(name = "cassandra")]
  extern "C" {
    pub fn cass_iterator_from_row(row: *const _CassRow) -> *mut _CassIterator;
    fn cass_row_get_column(row: *const _CassRow, index: _CassSizeType) -> *const _CassValue;
    fn cass_row_get_column_by_name(row: *const _CassRow, name: *const c_char) -> *const _CassValue;
  }

