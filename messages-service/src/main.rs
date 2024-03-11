use rocket::{get, launch, routes};

#[get("/message")]
fn message() -> &'static str {
    "not implemented yet"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .configure(rocket::Config::figment().merge(("port", 8002)))
        .mount("/", routes![message])
}
