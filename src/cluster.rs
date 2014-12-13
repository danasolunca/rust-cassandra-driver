extern crate libc;
use cass_ssl::Ssl;
use cass_ssl::CassSsl;
use error::CassError;
use error::Error;
use future::Future;
use future::CassFuture;
use log::LogLevelType;
use log::CassLogLevel;
use log::CassLogCallback;
use session::Session;
use types::CassBoolType;
use types;

use libc::c_char;
use libc::c_void;
use libc::c_uint;
use libc::c_int;



#[allow(dead_code)]
pub struct Cluster {
  cass_cluster:*mut CassCluster
}

#[allow(dead_code)]
#[allow(unused_variables)]
impl Cluster {

  pub fn new() -> Cluster {unsafe{
    Cluster{cass_cluster:cass_cluster_new()}
  }}
  
  pub fn set_contact_points(&self,contact_points:&str) -> Result<&Cluster,CassError> {unsafe{
    let points = contact_points.to_c_str();
    let err = cass_cluster_set_contact_points(self.cass_cluster,types::cass_string_init(points.as_ptr()).data);
    Ok(self)
  }}

  pub fn connect_async(&self) -> Future{unsafe{
    Future{cass_future:cass_cluster_connect(self.cass_cluster)}
  }}

  pub fn connect(&self) -> Result<Session,Error> {
    let future: Future = self.connect_async();
    future.wait();
    let mut rc = future.error_code();
    let session = future.get_session();
    if rc.is_error() {Err(rc)} else {Ok(session)}
  }

  pub fn connect_keyspace(&self, keyspace: *const c_char) -> Future {unsafe{
    Future{cass_future:cass_cluster_connect_keyspace(self.cass_cluster,keyspace)}
  }}

  fn free(&self) {unsafe{
    cass_cluster_free(self.cass_cluster)
  }}

  pub fn set_port(&self,port:i32) -> Result<&Cluster,Error> {unsafe{
    let err = cass_cluster_set_port(self.cass_cluster,port);
    Ok(self)
  }}


  //Need to test this
  pub fn set_ssl(self,ssl:Ssl) -> Result<Cluster,Error> {unsafe{
    let err = cass_cluster_set_ssl(self.cass_cluster,ssl.cass_ssl);
    Ok(self)
  }}

  pub fn set_protocol_version(&self,protocol_version:i32) -> Result<&Cluster,Error> {unsafe{
    let err = cass_cluster_set_protocol_version(self.cass_cluster,protocol_version);
    Ok(self)
  }}

  pub fn set_num_threads_io(&self,num_threads:u32) -> Result<&Cluster,Error> {unsafe{
    let err = cass_cluster_set_num_threads_io(self.cass_cluster,num_threads);
    Ok(self)
  }}

  pub fn set_queue_size_io(&self,queue_size:u32) -> Result<&Cluster,Error> {unsafe{
    let err = cass_cluster_set_queue_size_io(self.cass_cluster,queue_size);
    Ok(self)
  }}

  pub fn set_queue_size_event(&self,queue_size:u32) -> Result<&Cluster,Error> {unsafe{
    let err = cass_cluster_set_queue_size_event(self.cass_cluster,queue_size);
    Ok(self)
  }}

  pub fn set_queue_size_log(&self,queue_size:u32) -> Result<&Cluster,CassError> {unsafe{
    let err = cass_cluster_set_queue_size_log(self.cass_cluster,queue_size);
    Ok(self)
  }}

  pub fn set_core_connections_per_host(&self,connections:u32) -> Result<&Cluster,Error> {unsafe{
    let err = cass_cluster_set_core_connections_per_host(self.cass_cluster,connections);
    Ok(self)
  }}
  
  pub fn set_max_connections_per_host(&self,connections:u32) -> Result<&Cluster,Error> {unsafe{
    let err = cass_cluster_set_max_connections_per_host(self.cass_cluster,connections);
    Ok(self)
  }}

  pub fn set_reconnect_wait_time(&self,wait_time:u32) -> Result<&Cluster,Error> {unsafe{
    let err = cass_cluster_set_reconnect_wait_time(self.cass_cluster,wait_time);
    Ok(self)
  }}

  pub fn set_max_concurrent_creation(&self,num_connections:u32) -> Result<&Cluster,Error> {unsafe{
    let err = cass_cluster_set_max_concurrent_creation(self.cass_cluster,num_connections);
    Ok(self)
  }}

  pub fn set_max_concurrent_requests_threshold(&self,num_requests:u32) -> Result<&Cluster,Error> {unsafe{
    let err = cass_cluster_set_max_concurrent_requests_threshold(self.cass_cluster,num_requests);
    Ok(self)
  }}

  pub fn set_max_requests_per_flush(&self,num_requests:u32) -> Result<&Cluster,Error> {unsafe{
    let err = cass_cluster_set_max_requests_per_flush(self.cass_cluster,num_requests);
    Ok(self)
  }}

  pub fn set_write_bytes_high_water_mark(&self,num_bytes:u32) -> Result<&Cluster,Error> {unsafe{
    let err = cass_cluster_set_write_bytes_high_water_mark(self.cass_cluster,num_bytes);
    Ok(self)
  }}

  pub fn set_write_bytes_low_water_mark(&self,num_bytes:u32) -> Result<&Cluster,Error> {unsafe{
    let err = cass_cluster_set_write_bytes_low_water_mark(self.cass_cluster,num_bytes);
    Ok(self)
  }}

  pub fn set_pending_requests_high_water_mark(&self,num_requests:u32) -> Result<&Cluster,Error> {unsafe{
    let err = cass_cluster_set_pending_requests_high_water_mark(self.cass_cluster,num_requests);
    Ok(self)
  }}

  pub fn set_pending_requests_low_water_mark(&self,num_requests:u32) -> Result<&Cluster,Error> {unsafe{
    let err = cass_cluster_set_pending_requests_low_water_mark(self.cass_cluster,num_requests);
    Ok(self)
  }}

  pub fn set_connect_timeout(&self,timeout_ms:u32) -> Result<&Cluster,Error> {unsafe{
    let err = cass_cluster_set_connect_timeout(self.cass_cluster,timeout_ms);
    Ok(self)
  }}

  pub fn set_request_timeout(&self,timeout_ms:u32) -> Result<&Cluster,Error> {unsafe{
    let err = cass_cluster_set_request_timeout(self.cass_cluster,timeout_ms);
    Ok(self)
  }}

  pub fn set_log_level(&self,log_level:LogLevelType) -> Result<&Cluster,Error> {unsafe{
    let err = cass_cluster_set_log_level(self.cass_cluster,log_level as u32);
    Ok(self)
  }}

  pub fn set_credentials(&self,username:&str, password:&str) -> Result<&Cluster,Error> {unsafe{
    let err = cass_cluster_set_credentials(self.cass_cluster,username.as_ptr() as *const i8,password.as_ptr() as *const i8);
    Ok(self)
  }}

  pub fn set_load_balance_round_robin(&self) -> Result<&Cluster,Error> {unsafe{
    let err = cass_cluster_set_load_balance_round_robin(self.cass_cluster);
    Ok(self)
  }}

  pub fn set_load_balance_dc_aware(&self,local_dc:&str) -> Result<&Cluster,Error> {unsafe{
    let err = cass_cluster_set_load_balance_dc_aware(self.cass_cluster,local_dc.as_ptr() as *const i8);
    Ok(self)
  }}

  pub fn set_token_aware_routing(&self,enabled:bool) -> Result<&Cluster,Error> {unsafe{
    let err = cass_cluster_set_token_aware_routing(self.cass_cluster,match enabled {true=>1,false=>0});
    Ok(self)
  }}

}

impl Drop for Cluster {
  fn drop(&mut self) {
    self.free();
  }
}

  pub enum CassCluster { }
  #[link(name = "cassandra")]
  extern "C" {
    fn cass_cluster_new() -> *mut CassCluster;
    fn cass_cluster_free(cluster: *mut CassCluster);
    fn cass_cluster_set_contact_points(cluster: *mut CassCluster, contact_points: *const c_char) -> CassError;
    fn cass_cluster_set_port(cluster: *mut CassCluster, port: ::libc::c_int) -> CassError;
    fn cass_cluster_set_ssl(cluster: *mut CassCluster, ssl: *mut CassSsl) -> CassError;
    fn cass_cluster_set_protocol_version(cluster: *mut CassCluster, protocol_version: c_int) -> CassError;
    fn cass_cluster_set_num_threads_io(cluster: *mut CassCluster, num_threads: c_uint) -> CassError;
    fn cass_cluster_set_queue_size_io(cluster: *mut CassCluster, queue_size: c_uint) -> CassError;
    fn cass_cluster_set_queue_size_event(cluster: *mut CassCluster, queue_size: c_uint) -> CassError;
    fn cass_cluster_set_queue_size_log(cluster: *mut CassCluster, queue_size: c_uint) -> CassError;
    fn cass_cluster_set_core_connections_per_host(cluster: *mut CassCluster, num_connections: c_uint) -> CassError;
    fn cass_cluster_set_max_connections_per_host(cluster: *mut CassCluster, num_connections: c_uint) -> CassError;
    fn cass_cluster_set_reconnect_wait_time(cluster: *mut CassCluster, wait_time: ::libc::c_uint) -> CassError;
    fn cass_cluster_set_max_concurrent_creation(cluster: *mut CassCluster, num_connections: c_uint) -> CassError;
    fn cass_cluster_set_max_concurrent_requests_threshold(cluster: *mut CassCluster, num_requests: c_uint) -> CassError;
    fn cass_cluster_set_max_requests_per_flush(cluster: *mut CassCluster, num_requests: c_uint) -> CassError;
    fn cass_cluster_set_write_bytes_high_water_mark(cluster: *mut CassCluster, num_bytes: c_uint) -> CassError;
    fn cass_cluster_set_write_bytes_low_water_mark(cluster: *mut CassCluster, num_bytes: c_uint) -> CassError;
    fn cass_cluster_set_pending_requests_high_water_mark(cluster: *mut CassCluster, num_requests: c_uint) -> CassError;
    fn cass_cluster_set_pending_requests_low_water_mark(cluster: *mut CassCluster, num_requests: c_uint) -> CassError;
    fn cass_cluster_set_connect_timeout(cluster: *mut CassCluster, timeout_ms: c_uint) -> CassError;
    fn cass_cluster_set_request_timeout(cluster: *mut CassCluster, timeout_ms: c_uint) -> CassError;
    fn cass_cluster_set_log_level(cluster: *mut CassCluster, level: CassLogLevel) -> CassError;
    fn cass_cluster_set_log_callback(cluster: *mut CassCluster, callback: CassLogCallback, data: *mut c_void) -> CassError;
    fn cass_cluster_set_credentials(cluster: *mut CassCluster, username: *const c_char, password: *const c_char) -> CassError;
    fn cass_cluster_set_load_balance_round_robin(cluster: *mut CassCluster) -> CassError;
    fn cass_cluster_set_load_balance_dc_aware(cluster: *mut CassCluster, local_dc: *const c_char) -> CassError;
    fn cass_cluster_set_token_aware_routing(cluster: *mut CassCluster, enabled: CassBoolType);
    fn cass_cluster_connect(cluster: *mut CassCluster) -> *mut CassFuture;
    fn cass_cluster_connect_keyspace(cluster: *mut CassCluster,keyspace: *const c_char) -> *mut CassFuture;
  }


#[cfg(test)]
mod tests {
    #[test]
    fn new() {
      super::Cluster::new();
    }

    #[test]
    fn create() {
      let cluster = super::Cluster::new();
      cluster.set_contact_points("127.0.0.1").unwrap();
    }

    #[test]
    fn connect() {
      let cluster = super::Cluster::new();
      cluster.set_contact_points("127.0.0.1").unwrap()
            .connect();
    }

    #[test_should_fail]
    fn connect_bad_host() {
     let cluster = super::Cluster::new();
     cluster.set_contact_points("10.254.254.254").unwrap()
          .connect();
    }
}
