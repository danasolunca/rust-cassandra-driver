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
fn insert_into_collections(session:&CassSession, cmd:&str, key:&str, items:Vec<&str>) -> Result<CassResult,CassError> {
   println!("inserting key:{}",key);
  let mut statement = CassStatement::build_from_str(cmd, 2);
  
  statement.bind_by_idx(0, key.to_string());
  let collection = CassCollection::new_list(2);
  for item  in items.iter() {
    collection.append_str(*item);
  }
  statement.bind_collection(1, collection);
  let future=session.execute(&mut statement);
  match future {
    Err(err) => Err(err),
    Ok(result) => Ok(result)
  }
}

#[allow(unused_must_use)]
fn select_from_collections(session:&CassSession, cmd:&str, key:&str) {
  let mut statement = CassStatement::build_from_str(cmd, 1);
  statement.bind_by_idx(0, key.to_string());
 
  match session.execute(&statement) {
    Err(fail) => println!("fail: {}",fail),
    Ok(result) => {
      for row in result.iterator() {
        let (key,value) = (row.get_column(0),row.get_column(1));
        println!("key={}",key.get_string());
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
		create_table: "CREATE TABLE IF NOT EXISTS examples.collections (key text, items set<text>, PRIMARY KEY (key))",
		insert: "INSERT INTO examples.collections (key, items) VALUES (?, ?);",
    select: "SELECT key,items FROM examples.collections WHERE key = ?",
	};

  let items = [ "apple", "orange", "banana", "mango"].to_vec();

  match CassCluster::new().set_contact_points("127.0.0.1").unwrap().connect() {
    Err(fail) => println!("fail: {}",fail),
    Ok(session) => {
      for cmd in [cmds.create_ks,cmds.use_ks,cmds.create_table].iter() {
        assert!((&session).execute_str(*cmd).is_ok());
      }
      assert!(insert_into_collections(&session, cmds.insert,"test23", items).is_ok());
      let collection = select_from_collections(&session, cmds.select,"test23");
      println!("collection:{}",collection);
      let close_future = session.close_async();
      close_future.wait();
    }
  }
}
