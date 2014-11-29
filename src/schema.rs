#[allow(dead_code)]

pub struct CassSchema {
  pub cass_schema:*const internal::CassSchema
}

impl Drop for CassSchema {
  fn drop(&mut self) {
    //self.free();
  }
}

//~ impl CassSchema {
  //~ pub fn get_iterator(&self) -> CassIterator {unsafe{
    //~ CassIterator{cass_iterator:cass_schema_free(&self.cass_schema)}
  //~ }}
//~ }

pub mod internal {
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
