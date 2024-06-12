use k3_wasm_macros::http_handler;
use k3_wasm_sdk::http::{Request, Response, StatusCode};
use k3_wasm_sdk::{set_env_context, ENV_VAR_LEN, ENV_VAR_PTR};
use reqwest::header;
use std::sync::{Arc, Mutex};
use std::vec::Vec;
use wasm_bindgen_futures::spawn_local;

#[http_handler]
pub fn get(_req: Request<Vec<u8>>) -> Response<Vec<u8>> {
    set_env_context();

    let api_key = std::env::var("API_KEY").expect("API_KEY env var must be present.");
    let json_endpoint =
        std::env::var("JSON_ENDPOINT").expect("JSON_ENDPOINT env var must be present.");

    let response_body = Arc::new(Mutex::new(Vec::new()));
    let response_body_clone = Arc::clone(&response_body);

    spawn_local(async move {
        let client = reqwest::Client::new();
        let response = client
            .get(&json_endpoint)
            .header(header::AUTHORIZATION, &api_key)
            .send()
            .await
            .expect(&format!("Request to {} failed", &json_endpoint));

        let bytes = match response.bytes().await {
            Ok(bytes) => bytes,
            Err(err) => {
                let mut response_body = response_body_clone.lock().unwrap();
                response_body.extend_from_slice(
                    format!("Failed to read response body: {:?}", err).as_bytes(),
                );
                return;
            }
        };

        let mut response_body = response_body_clone.lock().unwrap();
        response_body.extend_from_slice(&bytes);
    });

    while response_body.lock().unwrap().is_empty() {}

    let response_body = response_body.lock().unwrap().clone();

    Response::builder()
        .status(StatusCode::OK)
        .body(response_body)
        .unwrap()
}

k3_wasm_macros::init!();
