use std::fmt::Show;
use std::fmt::Formatter;
use std::fmt;

use std::string::raw;

use row::Row;
use ResultIterator;
use error::Error as CassError;
use iterator::internal as iterator_internal;
use result::internal as result_internal;
use types::internal as types_internal;

pub type CResult = Result<CassResult,CassError>;

#[allow(dead_code)]
#[allow(raw_pointer_deriving)]
#[deriving(Clone)]
pub struct CassResult {
  pub cass_result:*const internal::CassResult
}

impl Drop for CassResult {
  fn drop(&mut self) {unsafe{
    internal::cass_result_free(self.cass_result)
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
    result_internal::cass_result_has_more_pages(self.cass_result) > 0
  }}

  pub fn row_count(&self) -> u64 {unsafe{
    result_internal::cass_result_row_count(self.cass_result)
  }}

  pub fn column_count(&self) -> u64 {unsafe{
    result_internal::cass_result_column_count(self.cass_result)
  }}

  pub fn column_name(&self, index: u64) -> String {unsafe{
    let cass_str = result_internal::cass_result_column_name(self.cass_result,index);
    let raw = cass_str.data as *mut u8;
    let length = cass_str.length as uint;
    raw::from_parts(raw, length, length)
  }}

  pub fn column_type(&self, index: u64) -> types_internal::CassValueType {unsafe{
    result_internal::cass_result_column_type(self.cass_result,index)
  }}

  pub fn first_row(&self) -> Option<Row> {unsafe{
    match self.row_count() {
      0 => None,
      _ => Some(Row{cass_row:result_internal::cass_result_first_row(self.cass_result)})
    }
  }}

  pub fn iterator(&self) -> ResultIterator {unsafe{
    ResultIterator{cass_iterator:result_internal::cass_iterator_from_result(self.cass_result)}
  }}
}

pub mod internal {
  use iterator::internal as iterator_internal;
  use types::internal as types_internal;
  use row::internal as row_internal;
  
  pub enum Struct_CassResult_ { }
  pub type CassResult = Struct_CassResult_;
  #[link(name = "cassandra")]
  extern "C" {
    pub fn cass_result_free(result: *const CassResult);
    pub fn cass_result_row_count(result: *const CassResult) -> types_internal::cass_size_t;
    pub fn cass_result_column_count(result: *const CassResult) -> types_internal::cass_size_t;
    pub fn cass_result_column_name(result: *const CassResult, index: types_internal::cass_size_t) -> types_internal::CassString;
    pub fn cass_result_column_type(result: *const CassResult, index: types_internal::cass_size_t) -> types_internal::CassValueType;
    pub fn cass_result_first_row(result: *const CassResult) -> *const row_internal::CassRow;
    pub fn cass_result_has_more_pages(result: *const CassResult) -> types_internal::cass_bool_t;
    pub fn cass_iterator_from_result(result: *const CassResult) -> *mut iterator_internal::CassIterator;
  }
}
