extern crate libc;

use libc::c_char;

use future::Future as CassFuture;
use session::Session;
use error::Error as CassError;
use types::internal as types_internal;

#[allow(dead_code)]
pub struct Cluster {
  cass_cluster:*mut internal::CassCluster
}

#[allow(dead_code)]
#[allow(unused_variables)]
impl Cluster {

  fn new() -> Cluster {unsafe{
    Cluster{cass_cluster:internal::cass_cluster_new()}
  }}

  pub fn create(contact_points:String) -> Cluster {unsafe{
    let cluster = Cluster::new();
    let points = contact_points.to_c_str();
    let err = internal::cass_cluster_set_contact_points(cluster.cass_cluster,types_internal::cass_string_init(points.as_ptr()).data);
    cluster
  }}

  pub fn connect_async(&mut self) -> CassFuture{unsafe{
    CassFuture{cass_future:internal::cass_cluster_connect( self.cass_cluster)}
  }}

  pub fn connect(mut self) -> Result<Session,CassError> {
    let mut future: CassFuture = self.connect_async();
    future.wait();
    let rc = future.error_code();
    let session = future.get_session();
    if rc.is_error() {return Err(rc);} else {return Ok(session);}
  }

  pub fn connect_keyspace(&mut self, keyspace: *const c_char) -> CassFuture {unsafe{
    CassFuture{cass_future:internal::cass_cluster_connect_keyspace(self.cass_cluster,keyspace)}
  }}

  fn free(&mut self) {unsafe{
    internal::cass_cluster_free(self.cass_cluster)
  }}
}

impl Drop for Cluster {
  fn drop(&mut self) {
    self.free();
  }
}

pub mod internal {
  use error::internal as error_internal;
  use log::internal as log_internal;
  use types::internal as types_internal;
  use cass_ssl::internal as cass_ssl_internal;
  use future::internal as future_internal;
  pub enum Struct_CassCluster_ { }
  pub type CassCluster = Struct_CassCluster_;
  #[link(name = "cassandra")]
  extern "C" {
    pub fn cass_cluster_new() -> *mut CassCluster;
    pub fn cass_cluster_free(cluster: *mut CassCluster);
    pub fn cass_cluster_set_contact_points(cluster: *mut CassCluster, contact_points: *const ::libc::c_char) -> error_internal::CassError;
    pub fn cass_cluster_set_port(cluster: *mut CassCluster, port: ::libc::c_int) -> error_internal::CassError;
    pub fn cass_cluster_set_ssl(cluster: *mut CassCluster, ssl: *mut cass_ssl_internal::CassSsl) -> error_internal::CassError;
    pub fn cass_cluster_set_protocol_version(cluster: *mut CassCluster, protocol_version: ::libc::c_int) -> error_internal::CassError;
    pub fn cass_cluster_set_num_threads_io(cluster: *mut CassCluster, num_threads: ::libc::c_uint) -> error_internal::CassError;
    pub fn cass_cluster_set_queue_size_io(cluster: *mut CassCluster, queue_size: ::libc::c_uint) -> error_internal::CassError;
    pub fn cass_cluster_set_queue_size_event(cluster: *mut CassCluster, queue_size: ::libc::c_uint) -> error_internal::CassError;
    pub fn cass_cluster_set_queue_size_log(cluster: *mut CassCluster, queue_size: ::libc::c_uint) -> error_internal::CassError;
    pub fn cass_cluster_set_core_connections_per_host(cluster: *mut CassCluster, num_connections: ::libc::c_uint) -> error_internal::CassError;
    pub fn cass_cluster_set_max_connections_per_host(cluster: *mut CassCluster, num_connections: ::libc::c_uint) -> error_internal::CassError;
    pub fn cass_cluster_set_reconnect_wait_time(cluster: *mut CassCluster, wait_time: ::libc::c_uint) -> error_internal::CassError;
    pub fn cass_cluster_set_max_concurrent_creation(cluster: *mut CassCluster, num_connections: ::libc::c_uint) -> error_internal::CassError;
    pub fn cass_cluster_set_max_concurrent_requests_threshold(cluster: *mut CassCluster, num_requests: ::libc::c_uint) -> error_internal::CassError;
    pub fn cass_cluster_set_max_requests_per_flush(cluster: *mut CassCluster, num_requests: ::libc::c_uint) -> error_internal::CassError;
    pub fn cass_cluster_set_write_bytes_high_water_mark(cluster: *mut CassCluster, num_bytes: ::libc::c_uint) -> error_internal::CassError;
    pub fn cass_cluster_set_write_bytes_low_water_mark(cluster: *mut CassCluster, num_bytes: ::libc::c_uint) -> error_internal::CassError;
    pub fn cass_cluster_set_pending_requests_high_water_mark(cluster: *mut CassCluster, num_requests: ::libc::c_uint) -> error_internal::CassError;
    pub fn cass_cluster_set_pending_requests_low_water_mark(cluster: *mut CassCluster, num_requests: ::libc::c_uint) -> error_internal::CassError;
    pub fn cass_cluster_set_connect_timeout(cluster: *mut CassCluster, timeout_ms: ::libc::c_uint) -> error_internal::CassError;
    pub fn cass_cluster_set_request_timeout(cluster: *mut CassCluster, timeout_ms: ::libc::c_uint) -> error_internal::CassError;
    pub fn cass_cluster_set_log_level(cluster: *mut CassCluster, level: log_internal::CassLogLevel) -> error_internal::CassError;
    //pub fn cass_cluster_set_log_callback(cluster: *mut CassCluster, callback: CassLogCallback, data: *mut ::libc::c_void) -> error_internal::CassError;
    pub fn cass_cluster_set_credentials(cluster: *mut CassCluster, username: *const ::libc::c_char, password: *const ::libc::c_char) -> error_internal::CassError;
    pub fn cass_cluster_set_load_balance_round_robin(cluster: *mut CassCluster) -> error_internal::CassError;
    pub fn cass_cluster_set_load_balance_dc_aware(cluster: *mut CassCluster, local_dc: *const ::libc::c_char) -> error_internal::CassError;
    pub fn cass_cluster_set_token_aware_routing(cluster: *mut CassCluster, enabled: types_internal::cass_bool_t);
    pub fn cass_cluster_connect(cluster: *mut CassCluster) -> *mut future_internal::CassFuture;
    pub fn cass_cluster_connect_keyspace(cluster: *mut CassCluster,keyspace: *const ::libc::c_char) -> *mut future_internal::CassFuture;
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
