use RowIterator;
use CassValue;

use cass_internal_api;



pub struct Row {
  pub cass_row:*const cass_internal_api::CassRow,
}

impl Row {
  #[allow(dead_code)]
  pub fn iterator(&mut self) -> RowIterator {unsafe{
    let ref cass_row = *self.cass_row;
    let my_iter = cass_internal_api::cass_iterator_from_row(cass_row);
    RowIterator{cass_iterator:my_iter}
  }}

  #[allow(dead_code)]
  pub fn get_column(&self, index: u64) -> CassValue {unsafe{
    let ref cass_row = *self.cass_row;
    let col = cass_internal_api::cass_row_get_column(cass_row,index);
    CassValue{cass_value:col}
  }}

}
