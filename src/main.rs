#[macro_use] extern crate rocket;
use rocket::fs::{FileServer, relative};
use rocket::get;

mod i2p;

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Deserialize, Serialize, Debug)]
struct ModelRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Default, Deserialize, Serialize, Debug)]
struct ModelResponse {
    response: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Query {
    search: String,
}

#[get("/?<search..>")]
async fn response(search: Option<&str>) -> String { 
    log::info!("fetching model response for query {:?}", search);
    let client = reqwest::Client::new();
    let host = "http://127.0.0.1:11434/api/generate";
    let req = ModelRequest {
        model: String::from("zephyr-local"),
        prompt: search.unwrap_or("").to_string(),
        stream: false,
    };
    let res = client
        .post(host)
        .json(&req)
        .send()
        .await;
    match res {
        Ok(r) => {
            let res = r.json::<ModelResponse>().await;
            match res {
                Ok(r) => r.response,
                Err(_) => Default::default(),
            }
        }
        Err(_) => Default::default(),
    }
}

#[launch]
async fn rocket() -> _ {
    env_logger::init();
    let _ = i2p::start_tunnel();
    let str_port = std::env::var("I2GPT_PORT").unwrap_or(String::from("3141"));
    let port = str_port.parse::<u16>().unwrap_or(3141);
    let config = rocket::Config {
        ident: rocket::config::Ident::none(),
        port,
        ip_header: None,

        ..rocket::Config::debug_default()
    };
    log::info!("i2gpt is running on port {}", port);
    rocket::custom(&config)
        .mount("/", FileServer::from(relative!("static")))
        .mount("/response", routes![response])
}
