#[allow(dead_code)]
pub enum CassSchemaMetaType {
  KEYSPACE=0,
  TABLE=1,
  COLUMN=2,
}

pub mod internal {
  use types::internal as types_internal;
  
  pub enum Struct_CassSchema_ { }
  pub type CassSchema = Struct_CassSchema_;
  pub enum Struct_CassSchemaMeta_ { }
  pub type CassSchemaMeta = Struct_CassSchemaMeta_;
  pub enum Struct_CassSchemaMetaField_ { }
  pub type CassSchemaMetaField = Struct_CassSchemaMetaField_;
  pub type Enum_CassSchemaMetaType_ = ::libc::c_uint;
  pub type CassSchemaMetaType = Enum_CassSchemaMetaType_;
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
