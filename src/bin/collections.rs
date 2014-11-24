extern crate log;
extern crate libc;
extern crate cassandra;

use cassandra::Statement;
use cassandra::Future as CassFuture;
use cassandra::Session;
use cassandra::Cluster;
use cassandra::CResult;
use cassandra::CassCollection;

struct Commands {
	use_ks:String,
	insert:String,
	create_ks:String,
	create_table:String,
  select:String,
} 

#[allow(unused)]
fn print_error(future:&mut CassFuture) {
  let message = future.error_message();
  println!("Error: {}", message);
}

#[allow(unused_must_use)]
fn insert_into_collections(session:&mut Session, cmd:&String, key:&String, items:Vec<String>) -> CResult {
   println!("inserting key:{}",key);
  let mut statement = Statement::build_from_string(cmd, 2);

  //~ CassCollection::new_set(1);
  
  statement.bind_string(0, key);
  let mut collection = CassCollection::new_list(2);
  for item  in items.iter() {
    collection.append_string(item);
  }
  statement.bind_collection(1, collection);
  let future=session.execute(&mut statement);
  match future {
    Err(err) => Err(err),
    Ok(result) => Ok(result)
  }
}

#[allow(unused_must_use)]
fn select_from_collections(session:&Session, cmd:&String, key:&String) {
  let mut statement = Statement::build_from_string(cmd, 1);
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
		use_ks:"Use examples".to_string(),
		create_ks: "CREATE KEYSPACE IF NOT EXISTS examples WITH replication = { 'class': 'SimpleStrategy', 'replication_factor': '1' }".to_string(),
		create_table: "CREATE TABLE IF NOT EXISTS examples.collections (key text, items set<text>, PRIMARY KEY (key))".to_string(),
		insert: "INSERT INTO examples.collections (key, items) VALUES (?, ?);".to_string(),
    select: "SELECT key,items FROM examples.collections WHERE key = ?".to_string(),
	};

  let items = [ "apple".to_string(), "orange".to_string(), "banana".to_string(), "mango".to_string()].to_vec();

  let contact_points = "127.0.0.1".to_string();
  let cluster = Cluster::create(contact_points);

  match cluster.connect() {
    Err(fail) => println!("fail: {}",fail),
    Ok(session) => {
      let mut session=session;
      let session = &mut session;
      assert!(session.execute_string(&cmds.create_ks).is_ok());
      assert!(session.execute_string(&cmds.use_ks).is_ok());
      assert!(session.execute_string(&cmds.create_table).is_ok());
      assert!(insert_into_collections(session, &cmds.insert,&"test23".to_string(), items).is_ok());
      let collection = select_from_collections(session, &cmds.select,&"test23".to_string());
      println!("collection:{}",collection);
      let mut close_future = session.close_async();
      close_future.wait();
    }
  }
}
