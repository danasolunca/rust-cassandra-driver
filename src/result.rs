use std::fmt::Show;
use std::fmt::Formatter;
use std::fmt;

use iterator::_CassIterator;
use types::_CassBoolType;
use types::CassValueType;
use types::_CassSizeType;
use types::_CassString;
use row::_CassRow;

use row::CassRow;
use ResultIterator;
use result;

impl Drop for CassResult {
  fn drop(&mut self) {unsafe{
    cass_result_free(&*self)
  }}
}

impl Show for CassResult {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "(Result:{})", self)
  }
}

#[allow(dead_code)]
impl CassResult {
  pub fn has_more_pages(&self) -> bool {unsafe{
    result::cass_result_has_more_pages(self) > 0
  }}

  pub fn row_count(&self) -> u64 {unsafe{
    result::cass_result_row_count(self)
  }}

  pub fn column_count(&self) -> u64 {unsafe{
    result::cass_result_column_count(self)
  }}

  pub fn column_name(&self, index: u64) -> String {unsafe{
    let cass_str = result::cass_result_column_name(self,index);
    let raw = cass_str.data as *mut u8;
    let length = cass_str.length as uint;
    String::from_raw_parts(raw, length, length)
  }}

  pub fn column_type(&self, index: u64) -> CassValueType {unsafe{
    result::cass_result_column_type(self,index)
  }}

  pub fn first_row(&self) -> Option<CassRow> {unsafe{
    match self.row_count() {
      0 => None,
      _ => Some(CassRow{row:result::cass_result_first_row(self)})
    }
  }}

  pub fn iterator(&self) -> ResultIterator {unsafe{
    ResultIterator{iter:result::cass_iterator_from_result(self)}
  }}
}

pub enum CassResult { }
#[link(name = "cassandra")]
extern "C" {
  pub fn cass_result_free(result: *const CassResult);
  pub fn cass_result_row_count(result: *const CassResult) -> _CassSizeType;
  pub fn cass_result_column_count(result: *const CassResult) -> _CassSizeType;
  pub fn cass_result_column_name(result: *const CassResult, index: _CassSizeType) -> _CassString;
  pub fn cass_result_column_type(result: *const CassResult, index: _CassSizeType) -> CassValueType;
  pub fn cass_result_first_row(result: *const CassResult) -> *const _CassRow;
  pub fn cass_result_has_more_pages(result: *const CassResult) -> _CassBoolType;
  pub fn cass_iterator_from_result(result: *const CassResult) -> *mut _CassIterator;
}

