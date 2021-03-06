#![feature(globs)]
#![feature(phase)]
#[allow(dead_code)]
#[phase(plugin, link)]

extern crate log;
extern crate libc;
extern crate collections;
extern crate cassandra;
// extern crate uuid;

use cassandra::CassCluster;
use cassandra::CassFuture;
use cassandra::CassSession;
use cassandra::CassStatement;

//use uuid::Uuid;

use std::vec::Vec;

static NUM_CONCURRENT_REQUESTS:uint = 100;

#[allow(unused_variables)]
fn insert_into_paging(session:&mut CassSession, key:&str) {
  let query = "INSERT INTO paging (key, value) VALUES (?, ?);";
  let mut futures:Vec<CassFuture> = Vec::new();
  for i in range(1,NUM_CONCURRENT_REQUESTS) {
    let statement = CassStatement::new(query, 2);
//    let uuid1 = Uuid::new_v4();
//    statement.bind_string(0, &uuid1.to_string());
//    statement.bind_string(1,&i.to_string());
//    let future:CassFuture = session.execute_async(&mut statement);
//    futures.push(future);
  }

  for future in futures.iter_mut() {
    future.wait();
  }
}

fn select_from_paging(session:&mut CassSession) {
   //let has_more_pages = true;
   //while has_more_pages {
     for row in session.execute_async(&mut CassStatement::build_from_str("SELECT * FROM paging", 0)).get_result().iterator() {
       let key = row.get_column(0).get_string();
       let value = row.get_column(1).get_string();
       match (key,value) {
        (Ok(key),Ok(value)) => println!("key: '{}' value: '{}'", key, value),
       _ => panic!("bad key or value")
       }
       //if result.has_more_pages() {
      //FIXME
      //   statement.set_paging_state(&mut result);
      // } else {
       //  has_more_pages = false;
       //}
    // }
   }
 }

#[allow(unused_variables)]
fn main() {
  let contact_points = "127.0.0.1";

  let cluster = CassCluster::new();
  cluster.set_contact_points(contact_points).unwrap();

  match cluster.connect() {
    Err(fail) => println!("fail: {}",fail),
    Ok(session) => {
      let mut session=session;
      let result = session.execute(&mut CassStatement::build_from_str("CREATE KEYSPACE IF NOT EXISTS examples WITH replication = { 'class': 'SimpleStrategy', 'replication_factor': '1' };",0));
      match result {
        Err(fail) => println!("fail: {}",fail),
        Ok(response) => {}
        }
      let result = session.execute(&mut CassStatement::build_from_str("CREATE TABLE IF NOT EXISTS examples.paging (key text, value text, PRIMARY KEY (key));",0));

      let result=session.execute(&mut CassStatement::build_from_str("USE examples",0));

      insert_into_paging(session, "test");
      select_from_paging(session);
     }
  }
}
