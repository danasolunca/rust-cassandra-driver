extern crate log;
extern crate libc;
extern crate cassandra;

use cassandra::CassCluster;
use cassandra::CassCollection;
use cassandra::CassError;
use cassandra::CassFuture;
use cassandra::CassResult;
use cassandra::CassSession;
use cassandra::CassStatement;

use std::collections::HashMap;

struct Commands {
	use_ks:&'static str,
	insert:&'static str,
	create_ks:&'static str,
	create_table:&'static str,
  select:&'static str,
} 

#[allow(unused)]
fn print_error(future:&mut CassFuture) {
  let message = future.error_message();
  println!("Error: {}", message);
}

#[allow(unused_must_use)]
fn insert_into_collections<'a>(session:&'a mut CassSession, cmd:&str, key:&str, items:HashMap<&str,u32>) -> Result<&'a CassResult,CassError> {
   println!("inserting key:{}",key);
  let statement = CassStatement::build_from_str(cmd, 2);

  statement.bind_by_idx(0, key.to_string());
  let collection = CassCollection::new_list(2);
  for item  in items.keys() {
    collection.append_str(*item);
  }
  statement.bind_collection(1, collection);
  let future=session.execute(statement);
  match future {
    Err(err) => Err(err),
    Ok(result) => Ok(result)
  }
}

#[allow(unused_must_use)]
fn select_from_collections(session:&mut CassSession, cmd:&str, key:&str) {
  let statement = CassStatement::build_from_str(cmd, 1);
  statement.bind_by_idx(0, key.to_string());
 
  match session.execute(statement) {
    Err(fail) => println!("fail: {}",fail),
    Ok(result) => {
      let rows = result.iterator();
        while rows.has_next() {
        let row = rows.get_next_row();
        let key = row.get_column(0);
        println!("key={}",key.get_string());
        let value = row.get_column(1);
        for item in value.get_collection_iterator() {
          let item_string = item.get_string();
          println!("item: {}", item_string);
        }
      }
    }
  }
}

fn main() {
	let cmds = Commands{
		use_ks:"Use examples",
		create_ks: "CREATE KEYSPACE IF NOT EXISTS examples WITH replication = { 'class': 'SimpleStrategy', 'replication_factor': '1' }",
		create_table: "CREATE TABLE IF NOT EXISTS examples.maps (key text, items map<text, int>, PRIMARY KEY (key))",
    insert: "INSERT INTO examples.maps (key, items) VALUES (?, ?);",
    select: "SELECT items FROM examples.maps WHERE key = ?",
	};
  
  let mut items:HashMap<&str,u32> = HashMap::new();
  items.insert("apple",1);
  items.insert("orange",2);
  items.insert("banana",3);
  items.insert("mango",4);

  let cluster = CassCluster::new();
  cluster.set_contact_points("127.0.0.1").unwrap();

  match cluster.connect() {
    Err(fail) => println!("fail: {}",fail),
    Ok(session) => {
      assert!(session.execute_str(cmds.create_ks).is_ok());
      assert!(session.execute_str(cmds.use_ks).is_ok());
      assert!(session.execute_str(cmds.create_table).is_ok());
      assert!(insert_into_collections(session, cmds.insert,"test23", items).is_ok());
      let collection = select_from_collections(session, cmds.select,"test23");
      println!("collection:{}",collection);
      let close_future = session.close_async();
      close_future.wait();
    }
  }
}
