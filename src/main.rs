use k3_wasm_macros::http_handler;
use k3_wasm_sdk::http::{get_auth, Request, Response, StatusCode};
use k3_wasm_sdk::{set_env_context, ENV_VAR_LEN, ENV_VAR_PTR};


#[http_handler]
pub fn get(_req: Request<Vec<u8>>) -> Response<Vec<u8>> {
    set_env_context();

    let api_key = std::env::var("API_KEY").expect("API_KEY env var must be present.");
    let json_endpoint =
        std::env::var("JSON_ENDPOINT").expect("JSON_ENDPOINT env var must be present.");

    let res = get_auth(&json_endpoint, &api_key);

    Response::builder()
        .status(StatusCode::OK)
        .body(res.unwrap())
        .unwrap()
}

k3_wasm_macros::init!();
