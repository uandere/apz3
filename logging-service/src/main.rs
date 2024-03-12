use rocket::{http::Status, post, serde::json::Json, routes, launch};
use rocket::response::status;
use redis::Commands;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Deserialize, Serialize)]
struct LoggedMessage {
    uuid: String,
    msg: String,
}

#[post("/log", format = "json", data = "<message>")]
fn log_message(message: Json<LoggedMessage>) -> Result<status::Custom<Json<LoggedMessage>>, status::Custom<String>> {
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");

    let client = match redis::Client::open(redis_url) {
        Ok(client) => client,
        Err(_) => return Err(status::Custom(Status::InternalServerError, "Failed to connect to Redis".into())),
    };

    let mut con = match client.get_connection() {
        Ok(con) => con,
        Err(_) => return Err(status::Custom(Status::InternalServerError, "Failed to connect to Redis".into())),
    };

    let logged_message = message.into_inner();

    match con.set::<_, _, String>(&logged_message.uuid, &logged_message.msg) {
        Ok(_) => Ok(status::Custom(Status::Ok, Json(logged_message))),
        Err(_) => Err(status::Custom(Status::InternalServerError, "Failed to log message".into())),
    }
}

#[launch]
fn rocket() -> _ {
    let port: u16 = env::var("ROCKET_PORT").unwrap_or_else(|_| "8001".to_string()).parse().expect("ROCKET_PORT must be a number");
    rocket::build()
        .configure(rocket::Config::figment().merge(("port", port)))
        .mount("/", routes![log_message])
}
