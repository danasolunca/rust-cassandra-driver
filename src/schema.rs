#[allow(dead_code)]

pub struct CassSchema {
  pub cass_schema:*const internal::CassSchema
}
//~ impl Drop for CassSchema {
  //~ fn drop(&mut self) {
    //~ self.free();
  //~ }
//~ }

pub mod internal {
  use types::internal as types_internal;
  
  pub enum CassSchema { }
  pub enum CassSchemaMeta { }
  pub enum CassSchemaMetaField { }
  pub enum CassSchemaMetaType {
    KEYSPACE=0,
    TABLE=1,
    COLUMN=2,
}

  #[link(name = "cassandra")]
  extern "C" {
    pub fn cass_schema_free(schema: *const CassSchema);
    pub fn cass_schema_get_keyspace(schema: *const CassSchema, keyspace_name: *const ::libc::c_char) -> *const CassSchemaMeta;
    pub fn cass_schema_meta_type(meta: *const CassSchemaMeta) -> CassSchemaMetaType;
    pub fn cass_schema_meta_get_entry(meta: *const CassSchemaMeta, name: *const ::libc::c_char) -> *const CassSchemaMeta;
    pub fn cass_schema_meta_get_field(meta: *const CassSchemaMeta, name: *const ::libc::c_char) -> *const CassSchemaMetaField;
    pub fn cass_schema_meta_field_name(field: *const CassSchemaMetaField) -> types_internal::CassString;
    pub fn cass_schema_meta_field_value(field: *const CassSchemaMetaField) -> *const types_internal::CassValue;
  }
}
