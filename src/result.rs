use std::fmt::Show;
use std::fmt::Formatter;
use std::fmt;
use std::string::raw;

use iterator::CassIterator;
use types::CassBoolType;
use types::CassValueType;
use types::CassSizeType;
use types::CassString;
use row::CassRow;

use row::Row;
use ResultIterator;
use result;

#[allow(dead_code)]
#[allow(raw_pointer_deriving)]
#[deriving(Clone)]
pub struct Result {
  pub cass_result:*const CassResult
}

impl Drop for Result {
  fn drop(&mut self) {unsafe{
    cass_result_free(self.cass_result)
  }}
}

impl Show for Result {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "(Result:{})", self.cass_result)
  }
}

#[allow(dead_code)]
impl Result {
  pub fn has_more_pages(&self) -> bool {unsafe{
    result::cass_result_has_more_pages(self.cass_result) > 0
  }}

  pub fn row_count(&self) -> u64 {unsafe{
    result::cass_result_row_count(self.cass_result)
  }}

  pub fn column_count(&self) -> u64 {unsafe{
    result::cass_result_column_count(self.cass_result)
  }}

  pub fn column_name(&self, index: u64) -> String {unsafe{
    let cass_str = result::cass_result_column_name(self.cass_result,index);
    let raw = cass_str.data as *mut u8;
    let length = cass_str.length as uint;
    raw::from_parts(raw, length, length)
  }}

  pub fn column_type(&self, index: u64) -> CassValueType {unsafe{
    result::cass_result_column_type(self.cass_result,index)
  }}

  pub fn first_row(&self) -> Option<Row> {unsafe{
    match self.row_count() {
      0 => None,
      _ => Some(Row{cass_row:result::cass_result_first_row(self.cass_result)})
    }
  }}

  pub fn iterator(&self) -> ResultIterator {unsafe{
    ResultIterator{cass_iterator:result::cass_iterator_from_result(self.cass_result)}
  }}
}

pub enum CassResult { }
#[link(name = "cassandra")]
extern "C" {
  pub fn cass_result_free(result: *const CassResult);
  pub fn cass_result_row_count(result: *const CassResult) -> CassSizeType;
  pub fn cass_result_column_count(result: *const CassResult) -> CassSizeType;
  pub fn cass_result_column_name(result: *const CassResult, index: CassSizeType) -> CassString;
  pub fn cass_result_column_type(result: *const CassResult, index: CassSizeType) -> CassValueType;
  pub fn cass_result_first_row(result: *const CassResult) -> *const CassRow;
  pub fn cass_result_has_more_pages(result: *const CassResult) -> CassBoolType;
  pub fn cass_iterator_from_result(result: *const CassResult) -> *mut CassIterator;
}

