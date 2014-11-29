extern crate log;
extern crate libc;
extern crate cassandra;

use cassandra::CassStatement;
use cassandra::CassFuture;
use cassandra::CassSession;
use cassandra::CassCluster;
use cassandra::CassResult;
use cassandra::CassCollection;
use cassandra::CassError;

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
fn insert_into_collections(session:&mut CassSession, cmd:&str, key:&str, items:Vec<&str>) -> Result<CassResult,CassError> {
   println!("inserting key:{}",key);
  let mut statement = CassStatement::build_from_str(cmd, 2);

  //~ CassCollection::new_set(1);
  
  statement.bind_string(0, key);
  let mut collection = CassCollection::new_list(2);
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
  statement.bind_string(0, key);
 
  match session.execute(&statement) {
    Err(fail) => println!("fail: {}",fail),
    Ok(result) => {
      let mut rows = result.iterator();
        while rows.has_next() {
        let row = rows.get_next_row();
        let key = row.get_column(0);
        println!("key={}",key.get_string());
        let value = row.get_column(1);
        let mut items = value.get_collection_iterator();
        for item in items {
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

  let contact_points = "127.0.0.1";
  let mut cluster = CassCluster::new();
  cluster = cluster.set_contact_points(contact_points).unwrap();

  match cluster.connect() {
    Err(fail) => println!("fail: {}",fail),
    Ok(session) => {
      let mut session=session;
      let session = &mut session;
      assert!(session.execute_str(cmds.create_ks).is_ok());
      assert!(session.execute_str(cmds.use_ks).is_ok());
      assert!(session.execute_str(cmds.create_table).is_ok());
      assert!(insert_into_collections(session, cmds.insert,"test23", items).is_ok());
      let collection = select_from_collections(session, cmds.select,"test23");
      println!("collection:{}",collection);
      let mut close_future = session.close_async();
      close_future.wait();
    }
  }
}
