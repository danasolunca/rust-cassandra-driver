extern crate log;
extern crate libc;
extern crate collections;
extern crate cassandra;

use cassandra::CassStatement;
use cassandra::CassFuture;
use cassandra::CassSession;
use cassandra::CassCluster;

use std::collections::DList;

struct Commands {
	use_ks:&'static str,
	insert:&'static str,
	create_ks:&'static str,
	create_table:&'static str
} 

static NUM_CONCURRENT_REQUESTS:uint = 5;

fn insert_into_async(session:&CassSession, cmd:&str, key:&str) {
  let mut futures:DList<CassFuture> = DList::new();
  let mut i = 0;
  while i < NUM_CONCURRENT_REQUESTS {
    let mut statement = CassStatement::build_from_str(cmd, 6);
    let wrapped = key.to_string() + i.to_string();
    println!("response:{}",wrapped);
    
    statement.bind_by_idx(0, wrapped).unwrap()
          .bind(if i % 2 == 0 {true} else {false}).unwrap()
          .bind(i as f32 / 2.0).unwrap()
          .bind(i  as f64 / 200.0).unwrap()
          .bind((i as i32 * 10)).unwrap()
          .bind((i as i64 * 100)).unwrap();
    session.execute(&statement).unwrap();
    i+=1;
  }

  while i < futures.len() {
    let future = futures.pop_front();
    let rc =future.unwrap().wait();
    println!("{}",rc);
    i+=1;
  }
}

fn main() {
	let cmds = Commands{
		use_ks:"Use examples",
		create_ks: "CREATE KEYSPACE IF NOT EXISTS examples WITH replication = { 'class': 'SimpleStrategy', 'replication_factor': '1' }",
		create_table: "CREATE TABLE IF NOT EXISTS examples.async (key text, bln boolean, flt float, dbl double, i32 int, i64 bigint, PRIMARY KEY (key));",
		insert: "INSERT INTO async (key, bln, flt, dbl, i32, i64) VALUES (?, ?, ?, ?, ?, ?);"
	};
	
  let contact_points = "127.0.0.1";

  match CassCluster::new().set_contact_points(contact_points).unwrap().connect() {
    Err(fail) => println!("fail: {}",fail),
    Ok(session) => {
      println!("foo");
      assert!(session.execute_str(cmds.create_ks).is_ok());
      assert!(session.execute_str(cmds.use_ks).is_ok());
      assert!(session.execute_str(cmds.create_table).is_ok());
      insert_into_async(&session, cmds.insert,"test");
      let mut close_future = session.close_async();
      close_future.wait();
    }
  }
}
