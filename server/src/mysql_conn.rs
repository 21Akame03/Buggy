pub mod models;
use mysql::prelude::Queryable;
use models::Test;


pub fn create_connection() -> Result<mysql::PooledConn, mysql::Error> {
    let url = "mysql://akame:CLakame2103~@localhost:3306/BUGGY";
    let opt = mysql::Opts::from_url(url)?;
    let pool = mysql::Pool::new(opt)?;
    let conn = pool.get_conn()?;

    Ok(conn)
}

pub fn select_database() -> Result<Vec<Test>, mysql::Error> {
    let mut conn = create_connection().unwrap();

    let results = conn.query_map("SELECT id, name FROM TEST", |(id, name)| {
        Test{id, name}
    });
    // let mut arr = Vec::new();
    // for i in results.unwrap() {
    //     arr.push(i.to_string());
    // }

    return Ok(results.unwrap())
}
