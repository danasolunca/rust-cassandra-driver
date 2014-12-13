extern crate log;
extern crate libc;
extern crate cassandra;

use cassandra::CassCluster;
use cassandra::CassError;
use cassandra::CassLogLevelType;
use cassandra::CassResult;
use cassandra::CassSession;
use cassandra::CassStatement;

#[deriving(Show, Clone)]
pub struct Extended {
  pub bln: bool,
  pub flt: f32,
  pub dbl: f64,
  pub i32: i32,
  pub i64: i64,
  pub string: String
}

struct Commands {
	use_ks:&'static str,
	insert:&'static str,
	create_ks:&'static str,
	create_table:&'static str,
	select:&'static str
} 

pub fn insert_into_basic(session:&mut CassSession, insert_statement: &str, key:&str, extended:&Extended) -> Result<CassResult,CassError> {
  let mut statement = CassStatement::build_from_str(insert_statement, 7);
  println!("inserting key:{}",key);
  statement
        .bind(key.to_string()).unwrap()
        .bind(extended.bln).unwrap()
        .bind(extended.flt).unwrap()
        .bind(extended.dbl).unwrap()
        .bind(extended.i32).unwrap()
        .bind(extended.i64).unwrap()
        .bind(extended.string.clone()).unwrap();
  session.execute(&mut statement)
}

pub fn select_from_basic(session:&mut CassSession, select_statement: &str, key:&String) -> Result<CassResult,CassError> {
  let mut statement = CassStatement::build_from_str(select_statement, 1);
  statement.bind_by_idx(0, key.clone()).unwrap();
  match session.execute(&mut statement) {
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
		create_table: "CREATE TABLE IF NOT EXISTS examples.extended (key text, bln boolean, flt float, dbl double, i32 int, i64 bigint, string text, PRIMARY KEY (key));",
		insert: "INSERT INTO examples.extended (key, bln, flt, dbl, i32, i64, string) VALUES (?, ?, ?, ?, ?, ?, ?);",
		select: "SELECT * FROM examples.extended WHERE key = ?;",
	};
	
  let input = Extended{bln:true, dbl:0.001f64, flt:0.0002f32, i32:1, i64:2, string: "String1".to_string()};
  let mut output=  Extended{bln:false, dbl:0.0f64, flt:0.00f32, i32:0, i64:0, string: "".to_string()};

  let contact_points = "127.0.0.1";
  let cluster = CassCluster::new();
  cluster.set_contact_points(contact_points).unwrap()
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
    Ok(mut session) => {
      for cmd in [cmds.create_ks,cmds.use_ks,cmds.create_table].iter() {
        assert!(session.execute_str(*cmd).is_ok());
      }
      match insert_into_basic(&mut session, cmds.insert, "test", &input) {
        Err(fail) => println!("result: {}",fail),
        Ok(results) => {}
      }

      match select_from_basic(&mut session, cmds.select, &"test".to_string()) {
        Err(fail) => println!("result: {}",fail),
        Ok(results) => {
          for row in results.iterator() {	
            match row.get_column(1).get_bool() {Err(err) => println!("{}--",err),Ok(col) => output.bln=col}
            match row.get_column(2).get_double() {Err(err) => println!("{}--",err),Ok(col) => output.dbl=col}
            match row.get_column(3).get_float() {Err(err) => println!("{}--",err),Ok(col) => output.flt=col}
            match row.get_column(4).get_int32() {Err(err) => println!("{}--",err),Ok(col) => output.i32=col}
            match row.get_column(5).get_int64() {Err(err) => println!("{}--",err),Ok(col) => output.i64=col}
            match row.get_column(6).get_string() {Err(err) => println!("{}--",err),Ok(col) => output.string=col}
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
  assert!(input.string == output.string);
  println!("select and insert matched");
}
