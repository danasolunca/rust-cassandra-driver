extern crate libc;
use cass_ssl::CassSsl;
use error::_CassError;
use error::CassError;
use future::CassFuture;
use log::CassLogLevelType;
use log::_CassLogLevel;
use log::CassLogCallback;
use session::CassSession;
use types::_CassBoolType;
use types;

use libc::c_char;
use libc::c_void;
use libc::c_uint;
use libc::c_int;

// phantom types
struct Complete;
struct Incomplete;

trait ClusterBuilder {
    fn set_contact_points(&mut self,contact_points:&str) -> &mut CassCluster;
    fn name(mut self, &str) -> CassCluster;
}

#[allow(dead_code)]
#[allow(unused_variables)]
impl CassCluster {

  pub fn new() -> &'static mut CassCluster {unsafe{
    &mut*cass_cluster_new()
  }}
  
  pub fn set_contact_points(&mut self,contact_points:&str) -> &mut CassCluster {unsafe{
    let points = contact_points.to_c_str();
    let my_self_ptr: *mut CassCluster = self;
    let err = cass_cluster_set_contact_points(my_self_ptr,types::cass_string_init(points.as_ptr()).data);
    self
  }}

  pub fn connect_async(&mut self) -> &mut CassFuture{unsafe{
    let my_self_ptr: *mut CassCluster = self;
    &mut*cass_cluster_connect(my_self_ptr)
  }}

  pub fn connect(&mut self) -> Result<&mut CassSession,CassError> {
    let mut future = self.connect_async();
//    future.wait();
    let rc = future.error_code();
    let session = future.get_session();
    if rc.is_error() {Err(rc)} else {Ok(session)}
  }

  pub fn connect_keyspace(&mut self, keyspace: *const c_char) -> &CassFuture {unsafe{
    let my_self_ptr: *mut CassCluster = self;
    &*cass_cluster_connect_keyspace(my_self_ptr,keyspace)
  }}

  fn free(&mut self) {unsafe{
    cass_cluster_free(self)
  }}

  pub fn set_port(&mut self,port:i32) -> Result<&mut CassCluster,CassError> {unsafe{
    let err = cass_cluster_set_port(self,port);
    Ok(self)
  }}

  //Need to test this
  pub fn set_ssl(&mut self,ssl:&mut CassSsl) -> Result<&mut CassCluster,CassError> {unsafe{
    let my_self_ptr: *mut CassCluster = self;    
    let err = cass_cluster_set_ssl(my_self_ptr,ssl);
    Ok(self)
  }}

  pub fn set_protocol_version(&mut self,protocol_version:i32) -> Result<&mut CassCluster,CassError> {unsafe{
    let err = cass_cluster_set_protocol_version(self,protocol_version);
    Ok(self)
  }}

  pub fn set_num_threads_io(&mut self,num_threads:u32) -> Result<&mut CassCluster,CassError> {unsafe{
    let err = cass_cluster_set_num_threads_io(self,num_threads);
    Ok(self)
  }}

  pub fn set_queue_size_io(&mut self,queue_size:u32) -> Result<&mut CassCluster,CassError> {unsafe{
    let err = cass_cluster_set_queue_size_io(self,queue_size);
    Ok(self)
  }}

  pub fn set_queue_size_event(&mut self,queue_size:u32) -> Result<&mut CassCluster,CassError> {unsafe{
    let err = cass_cluster_set_queue_size_event(self,queue_size);
    Ok(self)
  }}

  pub fn set_queue_size_log(&mut self,queue_size:u32) -> Result<&mut CassCluster,_CassError> {unsafe{
    let err = cass_cluster_set_queue_size_log(self,queue_size);
    Ok(self)
  }}

  pub fn set_core_connections_per_host(&mut self,connections:u32) -> Result<&mut CassCluster,CassError> {unsafe{
    let err = cass_cluster_set_core_connections_per_host(self,connections);
    Ok(self)
  }}
  
  pub fn set_max_connections_per_host(&mut self,connections:u32) -> Result<&mut CassCluster,CassError> {unsafe{
    let err = cass_cluster_set_max_connections_per_host(self,connections);
    Ok(self)
  }}

  pub fn set_reconnect_wait_time(&mut self,wait_time:u32) -> Result<&mut CassCluster,CassError> {unsafe{
    let err = cass_cluster_set_reconnect_wait_time(self,wait_time);
    Ok(self)
  }}

  pub fn set_max_concurrent_creation(&mut self,num_connections:u32) -> Result<&mut CassCluster,CassError> {unsafe{
    let err = cass_cluster_set_max_concurrent_creation(self,num_connections);
    Ok(self)
  }}

  pub fn set_max_concurrent_requests_threshold(&mut self,num_requests:u32) -> Result<&mut CassCluster,CassError> {unsafe{
    let err = cass_cluster_set_max_concurrent_requests_threshold(self,num_requests);
    Ok(self)
  }}

  pub fn set_max_requests_per_flush(&mut self,num_requests:u32) -> Result<&mut CassCluster,CassError> {unsafe{
    let err = cass_cluster_set_max_requests_per_flush(self,num_requests);
    Ok(self)
  }}

  pub fn set_write_bytes_high_water_mark(&mut self,num_bytes:u32) -> Result<&mut CassCluster,CassError> {unsafe{
    let err = cass_cluster_set_write_bytes_high_water_mark(self,num_bytes);
    Ok(self)
  }}

  pub fn set_write_bytes_low_water_mark(&mut self,num_bytes:u32) -> Result<&mut CassCluster,CassError> {unsafe{
    let err = cass_cluster_set_write_bytes_low_water_mark(self,num_bytes);
    Ok(self)
  }}

  pub fn set_pending_requests_high_water_mark(&mut self,num_requests:u32) -> Result<&mut CassCluster,CassError> {unsafe{
    let err = cass_cluster_set_pending_requests_high_water_mark(self,num_requests);
    Ok(self)
  }}

  pub fn set_pending_requests_low_water_mark(&mut self,num_requests:u32) -> Result<&mut CassCluster,CassError> {unsafe{
    let err = cass_cluster_set_pending_requests_low_water_mark(self,num_requests);
    Ok(self)
  }}

  pub fn set_connect_timeout(&mut self,timeout_ms:u32) -> Result<&mut CassCluster,CassError> {unsafe{
    let err = cass_cluster_set_connect_timeout(self,timeout_ms);
    Ok(self)
  }}

  pub fn set_request_timeout(&mut self,timeout_ms:u32) -> Result<&mut CassCluster,CassError> {unsafe{
    let err = cass_cluster_set_request_timeout(self,timeout_ms);
    Ok(self)
  }}

  pub fn set_log_level(&mut self,log_level:CassLogLevelType) -> Result<&mut CassCluster,CassError> {unsafe{
    let err = cass_cluster_set_log_level(self,log_level as u32);
    Ok(self)
  }}

  pub fn set_credentials(&mut self,username:&str, password:&str) -> Result<&mut CassCluster,CassError> {unsafe{
    let err = cass_cluster_set_credentials(self,username.as_ptr() as *const i8,password.as_ptr() as *const i8);
    Ok(self)
  }}

  pub fn set_load_balance_round_robin(&mut self) -> Result<&mut CassCluster,CassError> {unsafe{
    let err = cass_cluster_set_load_balance_round_robin(self);
    Ok(self)
  }}

  pub fn set_load_balance_dc_aware(&mut self,local_dc:&str) -> Result<&mut CassCluster,CassError> {unsafe{
    let err = cass_cluster_set_load_balance_dc_aware(self,local_dc.as_ptr() as *const i8);
    Ok(self)
  }}

  pub fn set_token_aware_routing(&mut self,enabled:bool) -> Result<&mut CassCluster,CassError> {unsafe{
    let err = cass_cluster_set_token_aware_routing(self,match enabled {true=>1,false=>0});
    Ok(self)
  }}

}

#[unsafe_destructor]
impl Drop for CassCluster {
  fn drop(&mut self) {
    self.free();
  }
}

  pub enum CassCluster<C=Incomplete>{ }
  #[link(name = "cassandra")]
  extern "C" {
    fn cass_cluster_new() -> *mut CassCluster;
    fn cass_cluster_free(cluster: *mut CassCluster);
    fn cass_cluster_set_contact_points(cluster: *mut CassCluster, contact_points: *const c_char) -> _CassError;
    fn cass_cluster_set_port(cluster: *mut CassCluster, port: ::libc::c_int) -> _CassError;
    fn cass_cluster_set_ssl(cluster: *mut CassCluster, ssl: *mut CassSsl) -> _CassError;
    fn cass_cluster_set_protocol_version(cluster: *mut CassCluster, protocol_version: c_int) -> _CassError;
    fn cass_cluster_set_num_threads_io(cluster: *mut CassCluster, num_threads: c_uint) -> _CassError;
    fn cass_cluster_set_queue_size_io(cluster: *mut CassCluster, queue_size: c_uint) -> _CassError;
    fn cass_cluster_set_queue_size_event(cluster: *mut CassCluster, queue_size: c_uint) -> _CassError;
    fn cass_cluster_set_queue_size_log(cluster: *mut CassCluster, queue_size: c_uint) -> _CassError;
    fn cass_cluster_set_core_connections_per_host(cluster: *mut CassCluster, num_connections: c_uint) -> _CassError;
    fn cass_cluster_set_max_connections_per_host(cluster: *mut CassCluster, num_connections: c_uint) -> _CassError;
    fn cass_cluster_set_reconnect_wait_time(cluster: *mut CassCluster, wait_time: ::libc::c_uint) -> _CassError;
    fn cass_cluster_set_max_concurrent_creation(cluster: *mut CassCluster, num_connections: c_uint) -> _CassError;
    fn cass_cluster_set_max_concurrent_requests_threshold(cluster: *mut CassCluster, num_requests: c_uint) -> _CassError;
    fn cass_cluster_set_max_requests_per_flush(cluster: *mut CassCluster, num_requests: c_uint) -> _CassError;
    fn cass_cluster_set_write_bytes_high_water_mark(cluster: *mut CassCluster, num_bytes: c_uint) -> _CassError;
    fn cass_cluster_set_write_bytes_low_water_mark(cluster: *mut CassCluster, num_bytes: c_uint) -> _CassError;
    fn cass_cluster_set_pending_requests_high_water_mark(cluster: *mut CassCluster, num_requests: c_uint) -> _CassError;
    fn cass_cluster_set_pending_requests_low_water_mark(cluster: *mut CassCluster, num_requests: c_uint) -> _CassError;
    fn cass_cluster_set_connect_timeout(cluster: *mut CassCluster, timeout_ms: c_uint) -> _CassError;
    fn cass_cluster_set_request_timeout(cluster: *mut CassCluster, timeout_ms: c_uint) -> _CassError;
    fn cass_cluster_set_log_level(cluster: *mut CassCluster, level: _CassLogLevel) -> _CassError;
    fn cass_cluster_set_log_callback(cluster: *mut CassCluster, callback: CassLogCallback, data: *mut c_void) -> _CassError;
    fn cass_cluster_set_credentials(cluster: *mut CassCluster, username: *const c_char, password: *const c_char) -> _CassError;
    fn cass_cluster_set_load_balance_round_robin(cluster: *mut CassCluster) -> _CassError;
    fn cass_cluster_set_load_balance_dc_aware(cluster: *mut CassCluster, local_dc: *const c_char) -> _CassError;
    fn cass_cluster_set_token_aware_routing(cluster: *mut CassCluster, enabled: _CassBoolType);
    fn cass_cluster_connect(cluster: *mut CassCluster) -> *mut CassFuture;
    fn cass_cluster_connect_keyspace(cluster: *mut CassCluster,keyspace: *const c_char) -> *mut CassFuture;
  }


#[cfg(test)]
mod tests {
    #[test]
    fn new() {
      super::CassCluster::new();
    }

    #[test]
    fn create() {
      let cluster = super::CassCluster::new();
      cluster.set_contact_points("127.0.0.1").unwrap();
    }

    #[test]
    fn connect() {
      let cluster = super::CassCluster::new();
      cluster.set_contact_points("127.0.0.1").unwrap()
            .connect();
    }

    #[test_should_fail]
    fn connect_bad_host() {
     let cluster = super::CassCluster::new();
     cluster.set_contact_points("10.254.254.254").unwrap()
          .connect();
    }
}
