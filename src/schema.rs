#[allow(dead_code)]
use iterator::CassIterator;
use iterator::CIterator;
use types::CassValue;
use types::CassString;
use types::Value;
use libc::c_char;

pub struct Schema {
  pub cass_schema:*const CassSchema
}

pub struct SchemaMeta {
  pub cass_schema_meta:*const CassSchemaMeta
}

impl Schema {
  pub fn get_iterator(&self) -> CIterator<Schema> {unsafe{
    CIterator{cass_iterator:cass_iterator_from_schema(self.cass_schema)}
  }}

  pub fn free(&self) {unsafe{
    cass_schema_free(self.cass_schema);
  }}

  pub fn get_keyspace(&self,  keyspace_name:&str) -> SchemaMeta {unsafe{
    SchemaMeta{cass_schema_meta:cass_schema_get_keyspace(self.cass_schema,keyspace_name.as_ptr() as *const i8)}
  }}
}

impl SchemaMeta {
  pub fn meta_type(&self) -> CassSchemaMetaType {unsafe{
    cass_schema_meta_type(self.cass_schema_meta)
  }}

  pub fn get_entry(&self, name:&str) -> SchemaMeta {unsafe{
    SchemaMeta{cass_schema_meta:cass_schema_meta_get_entry(self.cass_schema_meta,name.as_ptr() as *const i8)}
  }}

  pub fn get_field(&self, name:&str) -> *const CassSchemaMetaField {unsafe{
    cass_schema_meta_get_field(self.cass_schema_meta,name.as_ptr() as *const i8)
  }}
}

impl Drop for Schema {
  fn drop(&mut self) {
    self.free();
  }
}

  
pub enum CassSchema { }
#[allow(dead_code)]
pub enum CassSchemaMeta { }
#[allow(dead_code)]
pub enum CassSchemaMetaField { }
#[repr(C)]
#[allow(dead_code)]
pub enum CassSchemaMetaType {
  KEYSPACE=0,
  TABLE=1,
  COLUMN=2,
}

impl CassSchemaMetaField {
  pub fn name(&self,) -> String {unsafe{
    Value::cass_string_to_string(cass_schema_meta_field_name(self))
  }}

  pub fn value(&self,) -> Value {unsafe{
    Value{cass_value:cass_schema_meta_field_value(self)}
  }}
}

#[link(name = "cassandra")]
extern "C" {
  pub fn cass_iterator_from_schema(schema: *const CassSchema) -> *mut CassIterator;
  pub fn cass_schema_free(schema: *const CassSchema);
  pub fn cass_schema_get_keyspace(schema: *const CassSchema, keyspace_name: *const c_char) -> *const CassSchemaMeta;
  pub fn cass_schema_meta_type(meta: *const CassSchemaMeta) -> CassSchemaMetaType;
  pub fn cass_schema_meta_get_entry(meta: *const CassSchemaMeta, name: *const c_char) -> *const CassSchemaMeta;
  pub fn cass_schema_meta_get_field(meta: *const CassSchemaMeta, name: *const c_char) -> *const CassSchemaMetaField;
  pub fn cass_schema_meta_field_name(field: *const CassSchemaMetaField) -> CassString;
  pub fn cass_schema_meta_field_value(field: *const CassSchemaMetaField) -> *const CassValue;
}
