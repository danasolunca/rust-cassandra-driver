extern crate log;
extern crate libc;
extern crate cassandra;

use cassandra::CassCluster;
use cassandra::CassSession;
use cassandra::CassStatement;
use cassandra::CassError;
use cassandra::CassResult;

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

pub fn insert_into_basic(session:&mut CassSession, insert_statement: &str, key:&str, basic:Basic) -> Result<CassResult,CassError> {
  let mut statement = CassStatement::build_from_str(insert_statement, 6);
  println!("inserting key:{}",key);
  statement.bind(key.clone()).unwrap()
        .bind(basic.bln).unwrap()
        .bind(basic.flt).unwrap()
        .bind(basic.dbl).unwrap()
        .bind(basic.i32).unwrap()
        .bind(basic.i64).unwrap();
  session.execute(&mut statement)
}

pub fn select_from_basic(session:&CassSession, select_statement: &str, key:&str) -> Result<CassResult,CassError> {
  session.execute(
    CassStatement::build_from_str(select_statement, 1)
      .bind_by_idx(0, key.to_string()).unwrap()
  )
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

  match CassCluster::new().set_contact_points(contact_points).unwrap().connect() {
    Err(fail) => println!("fail: {}",fail),
    Ok(session) => {
      let mut session = session;

      for cmd in [cmds.create_ks,cmds.use_ks,cmds.create_table].iter() {
        assert!(session.execute_str(*cmd).is_ok());
      }

      match insert_into_basic(&mut session, cmds.insert, "test", input) {
        Err(fail) => println!("result: {}",fail),
        Ok(results) => {}
      }

      match select_from_basic(&mut session, cmds.select, "test") {
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
