use cass_internal_api;

use std::fmt::Show;
use std::fmt::Formatter;
use std::fmt;

use std::string::raw;

use row::Row;
use ResultIterator;
use error::Error as CassError;

mod cassandra {
  #[path="../types.rs"] pub mod types;
}

	pub type CResult = Result<CassResult,CassError>;

#[allow(dead_code)]
#[allow(raw_pointer_deriving)]
#[deriving(Clone)]
pub struct CassResult {
  pub cass_result:*const cass_internal_api::CassResult
}

impl Drop for CassResult {
  fn drop(&mut self) {unsafe{
    cass_internal_api::cass_result_free(self.cass_result)
  }}
}

impl Show for CassResult {
   fn fmt(&self, f: &mut Formatter) -> fmt::Result {
     write!(f, "(Result:{})", self.cass_result)
    }
}

#[allow(dead_code)]
impl CassResult {
  pub fn has_more_pages(&self) -> bool {unsafe{
    cass_internal_api::cass_result_has_more_pages(self.cass_result) > 0
  }}

  pub fn row_count(&self) -> cass_internal_api::cass_size_t {unsafe{
    cass_internal_api::cass_result_row_count(self.cass_result)
  }}

  pub fn column_count(&self) -> cass_internal_api::cass_size_t {unsafe{
    cass_internal_api::cass_result_column_count(self.cass_result)
  }}

  pub fn column_name(&self, index: cass_internal_api::cass_size_t) -> String {unsafe{
    let cass_str = cass_internal_api::cass_result_column_name(self.cass_result,index);
    let raw = cass_str.data as *mut u8;
    let length = cass_str.length as uint;
    raw::from_parts(raw, length, length)
  }}

  pub fn column_type(&self, index: cass_internal_api::cass_size_t) -> cass_internal_api::CassValueType {unsafe{
    cass_internal_api::cass_result_column_type(self.cass_result,index)
  }}

  pub fn first_row(&self) -> Option<Row> {unsafe{
    match self.row_count() {
      0 => None,
      _ => Some(Row{cass_row:cass_internal_api::cass_result_first_row(self.cass_result)})
    }
  }}

  pub fn iterator(&self) -> ResultIterator {unsafe{
    ResultIterator{cass_iterator:cass_internal_api::cass_iterator_from_result(self.cass_result)}
  }}
}
