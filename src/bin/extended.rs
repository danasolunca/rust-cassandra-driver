extern crate log;
extern crate libc;
extern crate cassandra;

use cassandra::CassCluster;
use cassandra::CassSession;
use cassandra::CassResult;
use cassandra::CassStatement;
use cassandra::CassLogLevelType;
use cassandra::CassError;

#[deriving(Show, Clone)]
pub struct Extended {
  pub bln: bool,
  pub flt: f32,
  pub dbl: f64,
  pub i32: i32,
  pub i64: i64
}

struct Commands {
	use_ks:&'static str,
	insert:&'static str,
	create_ks:&'static str,
	create_table:&'static str,
	select:&'static str
} 

pub fn insert_into_basic(session:&mut CassSession, insert_statement: &str, key:&str, extended:Extended) -> Result<CassResult,CassError> {
  let mut statement = CassStatement::build_from_str(insert_statement, 6);
  println!("inserting key:{}",key);
  statement.bind_string(0, key).unwrap()
        .bind_bool(1, extended.bln as u32).unwrap()
        .bind_float(2, extended.flt).unwrap()
        .bind_double(3, extended.dbl).unwrap()
        .bind_int32(4, extended.i32).unwrap()
        .bind_int64(5, extended.i64).unwrap();
  session.execute(&mut statement)
}

pub fn select_from_basic(session:&mut CassSession, select_statement: &str, key:&str) -> Result<CassResult,CassError> {
  let mut statement = CassStatement::build_from_str(select_statement, 1);
  statement.bind_string(0, key).unwrap();
  let future=session.execute(&mut statement);
  match future {
    Err(err) => return Err(err),
    Ok(result) => {
      return Ok(result)
    }
  }
}

#[allow(unused_variables)]
fn main()  {
	
	let cmds = Commands{
		use_ks:"Use examples",
		create_ks: "CREATE KEYSPACE IF NOT EXISTS examples WITH replication = { 'class': 'SimpleStrategy', 'replication_factor': '1' }",
		create_table: "CREATE TABLE IF NOT EXISTS examples.extended (key text, bln boolean, flt float, dbl double, i32 int, i64 bigint, PRIMARY KEY (key));",
		insert: "INSERT INTO examples.extended (key, bln, flt, dbl, i32, i64) VALUES (?, ?, ?, ?, ?, ?);",
		select: "SELECT * FROM examples.extended WHERE key = ?;",
	};
	
  let input = Extended{bln:true, dbl:0.001f64, flt:0.0002f32, i32:1, i64:2 };
  let mut output=  Extended{bln:false, dbl:0.0f64, flt:0.00f32, i32:0, i64:0};

  let contact_points = "127.0.0.1";
  let mut cluster = CassCluster::new();
  cluster = cluster.set_contact_points(contact_points).unwrap()
        .set_port(9042).unwrap()
        .set_protocol_version(3).unwrap()
        .set_queue_size_io(3).unwrap()
        .set_queue_size_event(3).unwrap()
        .set_queue_size_log(3).unwrap()
        .set_core_connections_per_host(3).unwrap()
        .set_max_connections_per_host(3).unwrap()
        .set_reconnect_wait_time(3).unwrap()
        .set_max_concurrent_creation(3).unwrap()
        .set_max_concurrent_requests_threshold(3).unwrap()
        .set_max_requests_per_flush(3).unwrap()
        .set_write_bytes_high_water_mark(3).unwrap()
        .set_write_bytes_low_water_mark(3).unwrap()
        .set_pending_requests_high_water_mark(3).unwrap()
        .set_pending_requests_low_water_mark(3).unwrap()
        .set_connect_timeout(300).unwrap()
        .set_request_timeout(300).unwrap()
        .set_log_level(CassLogLevelType::DEBUG).unwrap()
        .set_credentials("foo","bar").unwrap()
        .set_load_balance_round_robin().unwrap()
        .set_load_balance_dc_aware("dc1").unwrap()
        .set_token_aware_routing(true).unwrap();

  

  match cluster.connect() {
    Err(fail) => println!("fail: {}",fail),
    Ok(session) => {
      let mut session = session;
      
      assert!(session.execute_str(cmds.create_ks).is_ok());
      assert!(session.execute_str(cmds.use_ks).is_ok());
      assert!(session.execute_str(cmds.create_table).is_ok());

      let insert = insert_into_basic(&mut session, cmds.insert, "test", input);
      match insert {
        Err(fail) => println!("result: {}",fail),
        Ok(results) => {}
      }

      let response = select_from_basic(&mut session, cmds.select, "test");
      match response {
        Err(fail) => println!("result: {}",fail),
        Ok(results) => {
          for row in results.iterator() {	
            match row.get_column(1).get_bool() {Err(err) => println!("{}--",err),Ok(col) => output.bln=col}
            match row.get_column(2).get_double() {Err(err) => println!("{}--",err),Ok(col) => output.dbl=col}
            match row.get_column(3).get_float() {Err(err) => println!("{}--",err),Ok(col) => output.flt=col}
            match row.get_column(4).get_int32() {Err(err) => println!("{}--",err),Ok(col) => output.i32=col}
            match row.get_column(5).get_int64() {Err(err) => println!("{}--",err),Ok(col) => output.i64=col}
          }
        }
      }
    }
  }

  println!("input :{}",input);
  println!("output:{}",output);
  assert!(input.dbl == output.dbl);
  assert!(input.i32 == output.i32);
  assert!(input.i64 == output.i64);
  assert!(input.bln == output.bln);
  println!("select and insert matched");
}
