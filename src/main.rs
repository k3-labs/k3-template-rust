use k3_wasm_macros::http_handler;
use k3_wasm_sdk::http::{Request, Response};

mod api;

#[http_handler]
pub fn get(_req: Request<Vec<u8>>) -> Response<Vec<u8>> {
    Response::builder()
        .status(401)
        .body(
            "USAGE:\n\nGET /api/users/[id]\nPOST /api/users\n"
                .as_bytes()
                .to_vec(),
        )
        .unwrap()
}

k3_wasm_macros::init!();
