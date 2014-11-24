extern crate libc;

use libc::c_char;

use future::Future as CassFuture;
use session::Session;
use error::Error as CassError;
use cass_internal_api;

#[allow(dead_code)]
pub struct Cluster {
  cass_cluster:*mut cass_internal_api::CassCluster
}

#[allow(dead_code)]
#[allow(unused_variables)]
impl Cluster {

  fn new() -> Cluster {unsafe{
    Cluster{cass_cluster:cass_internal_api::cass_cluster_new()}
  }}

  pub fn create(contact_points:String) -> Cluster {unsafe{
    let cluster = Cluster::new();
    let points = contact_points.to_c_str();
    let err = cass_internal_api::cass_cluster_set_contact_points(cluster.cass_cluster,cass_internal_api::cass_string_init(points.as_ptr()).data);
    cluster
  }}

  pub fn connect_async(&mut self) -> CassFuture{unsafe{
    CassFuture{cass_future:cass_internal_api::cass_cluster_connect( self.cass_cluster)}
  }}

  pub fn connect(mut self) -> Result<Session,CassError> {
    let mut future: CassFuture = self.connect_async();
    future.wait();
    let rc = future.error_code();
    let session = future.get_session();
    if rc.is_error() {return Err(rc);} else {return Ok(session);}
  }

  pub fn connect_keyspace(&mut self, keyspace: *const c_char) -> CassFuture {unsafe{
    CassFuture{cass_future:cass_internal_api::cass_cluster_connect_keyspace(self.cass_cluster,keyspace)}
  }}

  fn free(&mut self) {unsafe{
    cass_internal_api::cass_cluster_free(self.cass_cluster)
  }}
}

impl Drop for Cluster {
  fn drop(&mut self) {
    self.free();
  }
}

#[cfg(test)]
mod tests {
    #[test]
    fn new() {
      super::Cluster::new();
    }

    #[test]
    fn create() {
      super::Cluster::create("127.0.0.1".to_string());
    }

    #[test]
    fn connect() {
      let cluster = super::Cluster::create("127.0.0.1".to_string());
      cluster.connect();
    }

    #[test_should_fail]
    fn connect_bad_host() {
      let cluster = super::Cluster::create("10.254.254.254".to_string());
      cluster.connect();
    }
}
