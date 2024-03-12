use rand::{rngs::StdRng, Rng, SeedableRng};
use rocket::{get, launch, post, routes, serde::json::Json, State};
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Debug, Deserialize)]
struct Message {
    msg: String,
}

#[derive(Debug, Serialize)]
struct LoggedMessage {
    uuid: String,
    msg: String,
}

struct AppState {
    rng: Arc<Mutex<StdRng>>,
}

#[post("/", format = "json", data = "<message>")]
async fn index_post(message: Json<Message>, state: &State<AppState>) -> Result<String, status::Custom<String>> {
    let selected_port = {
        let mut rng = state.rng.lock().unwrap();
        let ports = [8001, 8002, 8003];
        // Select a port and immediately release the lock by limiting the scope
        ports[rng.gen_range(0..ports.len())]
    };

    let client = reqwest::Client::new();
    let uuid = uuid::Uuid::new_v4();

    let logged_message = LoggedMessage {
        uuid: uuid.to_string(),
        msg: message.msg.clone(),
    };

    let res = client.post(format!("http://localhost:{}/log", selected_port))
        .json(&logged_message)
        .send()
        .await;

    match res {
        Ok(_) => Ok(uuid.to_string()),
        Err(_) => Err(status::Custom(Status::InternalServerError, "Failed to send message to logging-service".into())),
    }
}

#[get("/")]
async fn index_get(state: &State<AppState>) -> Result<String, status::Custom<String>> {
    let selected_port = {
        let mut rng = state.rng.lock().unwrap();
        let ports = [8001, 8002, 8003];
        // Select a port and immediately release the lock by limiting the scope
        ports[rng.gen_range(0..ports.len())]
    };

    let client = reqwest::Client::new();
    let logging_service_response = client.get(format!("http://localhost:{}/logs", selected_port))
        .send()
        .await;

    match logging_service_response {
        Ok(resp) => {
            match resp.text().await {
                Ok(text) => Ok(text),
                Err(_) => Err(status::Custom(Status::InternalServerError, "Failed to read logs response".into())),
            }
        },
        Err(_) => Err(status::Custom(Status::InternalServerError, "Failed to get logs from logging-service".into())),
    }
}

#[launch]
fn rocket() -> _ {
    let rng = Arc::new(Mutex::new(StdRng::from_entropy()));
    rocket::build()
        .manage(AppState { rng })
        .configure(rocket::Config::figment().merge(("port", 8000)))
        .mount("/", routes![index_post, index_get])
}
