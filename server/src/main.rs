#[macro_use] extern crate rocket;

mod mysql_conn;
use mysql_conn::select_database;
use rocket::serde::{Serialize, Deserialize};
use rocket::serde::json::{Json, Value, json};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Test {
    name: String,
    id: i32
}

#[get("/", format="json")]
fn index() -> Json<Vec<mysql_conn::models::Test>> {
    let data = select_database().unwrap();
    return Json(data)
}


#[launch]
fn rocket() -> _ {
    // let conn = match mysql_conn::create_connection() {
    //     Ok(x) => x,
    //     Err(err) => panic!("Error creating connection: {}", err)
    // };
    
    rocket::build().mount("/", routes![index])
}
