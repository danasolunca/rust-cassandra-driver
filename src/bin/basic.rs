extern crate log;
extern crate libc;
extern crate cassandra;

use cassandra::Cluster;
use cassandra::Session;
use cassandra::CResult;
use cassandra::Statement;

#[deriving(Show, Clone)]
pub struct Basic {
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

pub fn insert_into_basic(session:&mut Session, insert_statement: &str, key:&str, basic:Basic) -> CResult {
  let mut statement = Statement::build_from_str(insert_statement, 6);
  println!("inserting key:{}",key);
  statement.bind_string(0, key);
  statement.bind_bool(1, basic.bln as u32);
  statement.bind_float(2, basic.flt);
  statement.bind_double(3, basic.dbl);
  statement.bind_int32(4, basic.i32);
  statement.bind_int64(5, basic.i64);
  session.execute(&mut statement)
}

pub fn select_from_basic(session:&mut Session, select_statement: &str, key:&str) -> CResult {
  let mut statement = Statement::build_from_str(select_statement, 1);
  statement.bind_string(0, key);
  let future:CResult=session.execute(&mut statement);
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
		create_table: "CREATE TABLE IF NOT EXISTS examples.basic (key text, bln boolean, flt float, dbl double, i32 int, i64 bigint, PRIMARY KEY (key));",
		insert: "INSERT INTO examples.basic (key, bln, flt, dbl, i32, i64) VALUES (?, ?, ?, ?, ?, ?);",
		select: "SELECT * FROM examples.basic WHERE key = ?;",
	};
	
	
  let input = Basic{bln:true, dbl:0.001f64, flt:0.0002f32, i32:1, i64:2 };
  let mut output=  Basic{bln:false, dbl:0.0f64, flt:0.00f32, i32:0, i64:0};

  let contact_points = "127.0.0.1";
  let mut cluster = Cluster::new();
  cluster = cluster.set_contact_points(contact_points).unwrap();

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
