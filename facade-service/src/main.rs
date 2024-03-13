use rand::{rngs::StdRng, Rng, SeedableRng};
use rocket::{get, launch, post, routes, serde::json::Json, State};
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use rand::prelude::SliceRandom;

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
    let selected_service = {
        let mut rng = state.rng.lock().unwrap();
        let services = ["logging-service-1:8001", "logging-service-2:8001", "logging-service-3:8001"];
        services[rng.gen_range(0..services.len())]
    };

    let client = reqwest::Client::new();
    let uuid = uuid::Uuid::new_v4();

    let logged_message = LoggedMessage {
        uuid: uuid.to_string(),
        msg: message.msg.clone(),
    };


    let url = format!("http://{}/log", selected_service);
    match client.post(&url)
        .json(&logged_message)
        .send()
        .await {
        Ok(response) => {
            if response.status().is_success() {
                Ok(uuid.to_string())
            } else {
                // Log the error status and body for inspection
                let status = response.status();
                let error_body = response.text().await.unwrap_or_else(|_| "Failed to get error response body".to_string());
                log::error!("Error sending message to logging-service: HTTP Status: {}, Body: {}", status, error_body);
                Err(status::Custom(Status::InternalServerError, "Failed to send message to logging-service".into()))
            }
        },
        Err(e) => {
            log::error!("Request to logging-service failed: {}", e);
            Err(status::Custom(Status::InternalServerError, "Failed to send message to logging-service".into()))
        },
    }
}

#[get("/")]
async fn index_get(state: &State<AppState>) -> Result<String, status::Custom<String>> {
    let selected_service = {
        let mut rng = state.rng.lock().unwrap();
        let services = ["logging-service-1:8001", "logging-service-2:8001", "logging-service-3:8001"];
        services[rng.gen_range(0..services.len())]
    };

    let client = reqwest::Client::new();
    let logging_service_response = client.get(format!("http://{}/logs", selected_service))
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
