#[allow(dead_code)]
use iterator::_CassIterator;
use iterator::CassIterator;
use types::_CassValue;
use types::_CassString;
use types::CassValue;
use libc::c_char;

impl Copy for CassSchemaMeta {}

impl CassSchema {
  pub fn get_iterator(&self) -> CassIterator<CassSchema> {unsafe{
    CassIterator{iter:cass_iterator_from_schema(self)}
  }}

  pub fn free(&self) {unsafe{
    cass_schema_free(self);
  }}

  pub fn get_keyspace(&self,  keyspace_name:&str) -> &CassSchemaMeta {unsafe{
    &*cass_schema_get_keyspace(self,keyspace_name.as_ptr() as *const i8)
  }}
}

impl CassSchemaMeta {
  pub fn meta_type(&self) -> CassSchemaMetaType {unsafe{
    cass_schema_meta_type(self)
  }}

  pub fn get_entry(&self, name:&str) -> &CassSchemaMeta {unsafe{
    &*cass_schema_meta_get_entry(self,name.as_ptr() as *const i8)
  }}

  pub fn get_field(&self, name:&str) -> *const CassSchemaMetaField {unsafe{
    cass_schema_meta_get_field(self,name.as_ptr() as *const i8)
  }}
}

impl Drop for CassSchema {
  fn drop(&mut self) {
    self.free();
  }
}

pub enum CassSchema { }
#[allow(dead_code)]
pub enum CassSchemaMeta { }
#[allow(dead_code)]
pub enum CassSchemaMetaField { }
impl Copy for CassSchemaMetaField {}
#[repr(C)]
#[allow(dead_code)]
pub enum CassSchemaMetaType {
  KEYSPACE=0,
  TABLE=1,
  COLUMN=2,
}

impl CassSchemaMetaField {
  pub fn name(&self,) -> String {unsafe{
    CassValue::cass_string_to_string(cass_schema_meta_field_name(self))
  }}

  pub fn value(&self,) -> CassValue {unsafe{
    CassValue{val:cass_schema_meta_field_value(self)}
  }}
}

impl CassSchemaMeta {
 pub fn get_iterator(&self) -> &_CassIterator {unsafe{
    &*cass_iterator_from_schema_meta(&*self)
 }}

  pub fn get_iterator_fields(&self) -> &_CassIterator {unsafe{
    &*cass_iterator_fields_from_schema_meta(&*self)
 }}
}

#[link(name = "cassandra")]
extern "C" {
  pub fn cass_iterator_from_schema_meta(meta: *const CassSchemaMeta) -> *mut _CassIterator;
  pub fn cass_iterator_fields_from_schema_meta(meta: *const CassSchemaMeta) -> *mut _CassIterator;
  pub fn cass_iterator_from_schema(schema: *const CassSchema) -> *mut _CassIterator;
  pub fn cass_schema_free(schema: *const CassSchema);
  pub fn cass_schema_get_keyspace(schema: *const CassSchema, keyspace_name: *const c_char) -> *const CassSchemaMeta;
  pub fn cass_schema_meta_type(meta: *const CassSchemaMeta) -> CassSchemaMetaType;
  pub fn cass_schema_meta_get_entry(meta: *const CassSchemaMeta, name: *const c_char) -> *const CassSchemaMeta;
  pub fn cass_schema_meta_get_field(meta: *const CassSchemaMeta, name: *const c_char) -> *const CassSchemaMetaField;
  pub fn cass_schema_meta_field_name(field: *const CassSchemaMetaField) -> _CassString;
  pub fn cass_schema_meta_field_value(field: *const CassSchemaMetaField) -> *const _CassValue;
}
