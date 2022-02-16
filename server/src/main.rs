#[macro_use] extern crate rocket;

mod mysql_conn;
use mysql_conn::select_database;
use rocket::serde::json::Json;



#[get("/", format="json")]
fn index() -> Json<Vec<mysql_conn::models::Test>> {
    let data = select_database().unwrap();
    return Json(data)
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
