#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate redis;

use rocket_contrib::databases::redis::Commands;

#[database("redis_pool")]
struct LogsDbConn(rocket_contrib::databases::redis::Connection);

#[get("/")]
fn index(con: LogsDbConn) -> String {
    con.get("abc").unwrap()
}

fn main() {
    rocket::ignite()
        .attach(LogsDbConn::fairing())
        .mount("/", routes![index])
        .launch();
}
