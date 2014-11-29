#[allow(dead_code)]
use iterator::CassIterator;
pub use schema::internal::CassSchemaMetaType;
pub use schema::internal::CassSchemaMetaField;

pub struct CassSchema {
  pub cass_schema:*const internal::CassSchema
}

pub struct CassSchemaMeta {
  pub cass_schema_meta:*const internal::CassSchemaMeta
}

impl CassSchema {
  pub fn get_iterator(&self) -> CassIterator<CassSchema> {unsafe{
    CassIterator{cass_iterator:internal::cass_iterator_from_schema(self.cass_schema)}
  }}

  pub fn free(&self) {unsafe{
    internal::cass_schema_free(self.cass_schema);
  }}

  pub fn get_keyspace(&self,  keyspace_name:&str) -> CassSchemaMeta {unsafe{
    CassSchemaMeta{cass_schema_meta:internal::cass_schema_get_keyspace(self.cass_schema,keyspace_name.as_ptr() as *const i8)}
  }}
}

impl CassSchemaMeta {
  pub fn meta_type(&self) -> CassSchemaMetaType {unsafe{
    internal::cass_schema_meta_type(self.cass_schema_meta)
  }}

  pub fn get_entry(&self, name:&str) -> CassSchemaMeta {unsafe{
    CassSchemaMeta{cass_schema_meta:internal::cass_schema_meta_get_entry(self.cass_schema_meta,name.as_ptr() as *const i8)}
  }}

  pub fn get_field(&self, name:&str) -> CassSchemaMetaField {unsafe{
    *internal::cass_schema_meta_get_field(self.cass_schema_meta,name.as_ptr() as *const i8)
  }}
}
impl Drop for CassSchema {
  fn drop(&mut self) {
    self.free();
  }
}



pub mod internal {

  use types::CassValue;
  use types::internal as types_internal;
  use iterator::internal as iterator_internal;
  
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
      CassValue::cass_string_to_string(cass_schema_meta_field_name(self))
    }}

    pub fn value(&self,) -> CassValue {unsafe{
      CassValue{cass_value:cass_schema_meta_field_value(self)}
    }}
  }

  
  #[link(name = "cassandra")]
  extern "C" {
    pub fn cass_iterator_from_schema(schema: *const CassSchema) -> *mut iterator_internal::CassIterator;
    pub fn cass_schema_free(schema: *const CassSchema);
    pub fn cass_schema_get_keyspace(schema: *const CassSchema, keyspace_name: *const ::libc::c_char) -> *const CassSchemaMeta;
    pub fn cass_schema_meta_type(meta: *const CassSchemaMeta) -> CassSchemaMetaType;
    pub fn cass_schema_meta_get_entry(meta: *const CassSchemaMeta, name: *const ::libc::c_char) -> *const CassSchemaMeta;
    pub fn cass_schema_meta_get_field(meta: *const CassSchemaMeta, name: *const ::libc::c_char) -> *const CassSchemaMetaField;
    pub fn cass_schema_meta_field_name(field: *const CassSchemaMetaField) -> types_internal::CassString;
    pub fn cass_schema_meta_field_value(field: *const CassSchemaMetaField) -> *const types_internal::CassValue;
  }
}
