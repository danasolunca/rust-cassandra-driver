#[allow(non_camel_case_types)]
#[repr(C)]
pub enum CassConsistency {
  ANY=0,
  ONE=1,
  TWO=2,
  THREE=3,
  QUORUM=4,
  ALL=5,
  LOCAL_QUORUM=6,
  EACH_QUORUM=7,
  SERIAL=8,
  LOCAL_SERIAL=9,
  LOCAL_ONE=10,
}

impl Copy for CassConsistency {}
