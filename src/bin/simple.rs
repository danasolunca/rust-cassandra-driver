extern crate log;
extern crate libc;
extern crate cassandra;

use cassandra::CassCluster;
use cassandra::CassStatement;

#[deriving(Show)]
pub struct Simple {
  pub keyspace_name: String,
  pub durable_writes: bool,
  pub strategy_class: String,
  pub strategy_options: String
}

fn main()  {
  let cluster = CassCluster::new();
  cluster.set_contact_points("127.0.0.1").unwrap();

  let session_future = cluster.connect_async();
  session_future.wait();

  if !session_future.error_code().is_error() {
    let mut session = session_future.get_session();

    let statement = CassStatement::build_from_str("SELECT * FROM system.schema_keyspaces;", 0);

    let result_future = session.execute_async(statement);
    result_future.wait();

    if !result_future.error_code().is_error() {
      let result = result_future.get_result();
      let mut rows = result.iterator();

      let mut output:Simple = Simple {
        keyspace_name:"abc".to_string(),
        durable_writes:false,
        strategy_class:"def".to_string(),
        strategy_options:"ghi".to_string(),
      };
      for row in rows {
        match row.get_column(0).get_string() {Err(err) => println!("{}0-",err),Ok(col) => output.keyspace_name=col};
        match row.get_column(1).get_bool() {Err(err) => println!("{}1-",err),Ok(col) => output.durable_writes=col};
        match row.get_column(2).get_string() {Err(err) => println!("{}2-",err),Ok(col) => output.strategy_class=col};
        match row.get_column(3).get_string() {Err(err) => println!("{}3-",err),Ok(col) => output.strategy_options=col};
        println!("output:{}",output);
      }
    } else {
      println!("{}",result_future.error_message());
    }

    //FIXME
    //let close_future = session.close_async();
    //close_future.wait();
  } else {
    let message = session_future.error_message();
    println!("Error: {}", message);
  }
}
