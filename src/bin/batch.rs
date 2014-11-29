extern crate log;
extern crate libc;
extern crate collections;
extern crate cassandra;

use cassandra::Statement;
use cassandra::Cluster;
use cassandra::Session;
use cassandra::Prepared;
use cassandra::Error as CassError;
use cassandra::Batch;

use std::collections::DList;

pub const CASS_BATCH_TYPE_LOGGED: ::libc::c_uint = 0;
pub const CASS_BATCH_TYPE_UNLOGGED: ::libc::c_uint = 1;
pub const CASS_BATCH_TYPE_COUNTER: ::libc::c_uint = 2;


struct Pair {
  key:&static str,
  value:&static str
}

struct Commands {
	use_ks:&static str,
	insert:&static str,
	create_ks:&static str,
	create_table:&static str,
} 

fn prepare_insert_into_batch(session:Session, query:&str) -> Result<Prepared,CassError> {

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

fn insert_into_batch_with_prepared(session:Session , mut prepared:Prepared, pairs:&mut DList<Pair>) -> CassError {
  let batch = &mut Batch::new(BATCH_TYPE::LOGGED_BATCH);
  for pair in pairs.iter_mut() {
    let mut statement = prepared.bind(2).unwrap()
          .bind_string(0, &pair.key).unwrap()
          .bind_string(1, &pair.value).unwrap();
    batch.add_statement(statement);
  }
  let st2 = Statement::build_from_str("INSERT INTO examples.pairs (key, value) VALUES ('c', '3')",0);
  batch.add_statement(st2);

  
  let mut statement = Statement::build_from_str("INSERT INTO examples.pairs (key, value) VALUES (?, ?)",2);
  statement.bind_str(0, "d");
  statement.bind_str(1, "4");
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
		use_ks:"Use examples".to_string(),
		create_ks: "CREATE KEYSPACE IF NOT EXISTS examples WITH replication = { 'class': 'SimpleStrategy', 'replication_factor': '1' }".to_string(),
		create_table: "CREATE TABLE IF NOT EXISTS examples.pairs (key text, value text, PRIMARY KEY (key));".to_string(),
		insert: "INSERT INTO examples.pairs (key, value) VALUES (?, ?)".to_string(),
	};
	
	
  let contact_points = "127.0.0.1".to_string();
  let cluster = Cluster::create(contact_points);
  let pairs:&mut DList<Pair> = &mut DList::new();
  pairs.push_front(Pair{key:"a".to_string(), value:"1".to_string()});
  pairs.push_front(Pair{key:"b".to_string(), value:"2".to_string()});

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
