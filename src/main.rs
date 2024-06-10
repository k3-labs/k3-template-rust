use k3_wasm_macros::http_handler;
use k3_wasm_sdk::http::{Request, Response, StatusCode};
use k3_wasm_sdk::{set_env_context, ENV_VAR_LEN, ENV_VAR_PTR};
use reqwest::{blocking, header};

#[http_handler]
pub fn get(_req: Request<Vec<u8>>) -> Response<Vec<u8>> {
    set_env_context();

    let api_key = std::env::var("API_KEY").expect("API_KEY env var must be present.");
    let json_endpoint =
        std::env::var("JSON_ENDPOINT").expect("JSON_ENDPOINT env var must be present.");

    let client = blocking::Client::new();

    let response = client
        .get(json_endpoint)
        .header(header::AUTHORIZATION, api_key)
        .send()
        .ok()?;

    let response_body = match response {
        Ok(resp) => {
            if resp.status().is_success() {
                String::from_utf8(resp.body().to_vec())
                    .unwrap_or_else(|_| "Failed to read response body".to_string())
            } else {
                format!("Request failed with status: {}", resp.status())
            }
        }
        Err(err) => format!("Request failed with error: {:?}", err),
    };

    Response::builder()
        .status(StatusCode::OK)
        .body(response_body.as_bytes().to_vec())
        .unwrap()
}

k3_wasm_macros::init!();
