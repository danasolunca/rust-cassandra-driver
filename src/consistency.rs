pub use cass_internal_api::CASS_CONSISTENCY_ONE as CONSISTENCY_ONE;
pub use cass_internal_api::CASS_CONSISTENCY_TWO as CONSISTENCY_TWO;
pub use cass_internal_api::CASS_CONSISTENCY_SERIAL as CONSISTENCY_SERIAL;

#[allow(non_camel_case_types)] pub type CASS_CONSISTENCY = u32;

#[allow(dead_code)]
pub struct CassConsistency {
  pub cass_consistency:u64,
}
