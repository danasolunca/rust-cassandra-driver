extern crate log;
extern crate libc;
extern crate collections;
extern crate cassandra;

use cassandra::CassSession;
use cassandra::CassPrepared;
use cassandra::CassBatch;
use cassandra::CassError;
use cassandra::CassStatement;
use cassandra::CassCluster;
use cassandra::BatchType;

use std::collections::DList;

pub const CASS_BATCH_TYPE_LOGGED: ::libc::c_uint = 0;
pub const CASS_BATCH_TYPE_UNLOGGED: ::libc::c_uint = 1;
pub const CASS_BATCH_TYPE_COUNTER: ::libc::c_uint = 2;

struct Pair<'a> {
  key:&'a str,
  value:&'a str
}

struct Commands {
	use_ks:&'static str,
	insert:&'static str,
	create_ks:&'static str,
	create_table:&'static str,
} 

fn prepare_insert_into_batch(session:CassSession, query:&str) -> Result<CassPrepared,CassError> {
  let mut future = session.prepare(query);
  future.wait();
  if future.error_code().is_error() {
     println!("error: {}",future.error_code());
     return Err(future.error_code());
  } else {
    let prepared = future.get_prepared();
    return Ok(prepared);
  }
}

fn insert_into_batch_with_prepared(session:CassSession, prepared:CassPrepared, pairs:&mut DList<Pair>) -> CassError {
  let batch = &mut CassBatch::new(BatchType::LOGGED);
  for pair in pairs.iter_mut() {
    let mut statement = prepared.bind(2)
          .bind(pair.key).unwrap()
          .bind(pair.value).unwrap();
    batch.add_statement(*statement);
  }
  let st2 = CassStatement::build_from_str("INSERT INTO examples.pairs (key, value) VALUES ('c', '3')",0);
  batch.add_statement(st2);
  let mut statement = CassStatement::build_from_str("INSERT INTO examples.pairs (key, value) VALUES (?, ?)",2)
    .bind("d");
    .bind("4");
  batch.add_statement(statement);
  let mut future = session.execute_batch(batch);
  future.wait();
  if !future.error_code().is_error() {
  } else {
    let prepared = future.get_prepared();
  }
  return future.error_code();
}

#[allow(unused_must_use)]
fn main() {
		let cmds = Commands{
		use_ks:"Use examples",
		create_ks: "CREATE KEYSPACE IF NOT EXISTS examples WITH replication = { 'class': 'SimpleStrategy', 'replication_factor': '1' }",
		create_table: "CREATE TABLE IF NOT EXISTS examples.pairs (key text, value text, PRIMARY KEY (key));",
		insert: "INSERT INTO examples.pairs (key, value) VALUES (?, ?)",
	};
	
  let contact_points = "127.0.0.1";
  let cluster = CassCluster::new().set_contact_points(contact_points);
  let pairs:&mut DList<Pair> = &mut DList::new();
  pairs.push_front(Pair{key:"a".to_string(), value:"1"});
  pairs.push_front(Pair{key:"b".to_string(), value:"2"});

  match cluster.connect() {
    Err(fail) => println!("fail: {}",fail),
    Ok(session) => {
      assert!(session.execute_string(&cmds.create_ks).is_ok());
      assert!(session.execute_string(&cmds.use_ks).is_ok());
      assert!(session.execute_string(&cmds.create_table).is_ok());
      let response = prepare_insert_into_batch(session,cmds.insert);
      match response {
        Err(fail) => println!("fail: {}",fail),
        Ok(result) => {insert_into_batch_with_prepared(session, result, pairs);}
      }
    }
  }
}
