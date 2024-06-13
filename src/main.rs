use k3_wasm_macros::http_handler;
use k3_wasm_sdk::http::{get, get_auth, Request, Response, StatusCode};
use k3_wasm_sdk::{set_env_context, ENV_VAR_LEN, ENV_VAR_PTR};

#[http_handler]
pub fn get(_req: Request<Vec<u8>>) -> Response<Vec<u8>> {
    set_env_context();

    let api_key = std::env::var("API_KEY");
    let json_endpoint =
        std::env::var("JSON_ENDPOINT").expect("JSON_ENDPOINT env var must be present.");

    let json_selector = std::env::var("JSON_SELECTOR").expect("JSON_SELECTOR variable must be set");
    let json_selector = parse_selector(&json_selector);

    let res = match api_key {
        Ok(api_key) => {
            let res = get_auth(&json_endpoint, &api_key).unwrap();
            let json = serde_json::from_slice::<serde_json::Value>(&res).unwrap();
            execute_selector(&json_selector, json).unwrap()
        }
        Err(_) => {
            let res = get(&json_endpoint).unwrap();
            let json = serde_json::from_slice::<serde_json::Value>(&res).unwrap();
            execute_selector(&json_selector, json).unwrap()
        }
    };

    Response::builder()
        .status(StatusCode::OK)
        .body(
            String::from_utf8(res.to_string().into())
                .unwrap()
                .as_bytes()
                .to_vec(),
        )
        .unwrap()
}

#[derive(Debug, Clone)]
enum Selector {
    Key(String),
    Index(usize),
}

fn parse_selector(selector: &str) -> Vec<Selector> {
    let mut parts = vec![];

    let bytes = selector.as_bytes();
    let mut buffer = vec![];
    let mut buffer_is_key = true;
    let mut offset = 0usize;
    while offset < bytes.len() {
        match bytes[offset] {
            b'[' => {
                offset += 1;
                buffer_is_key = false;
                while bytes[offset].is_ascii_digit() {
                    buffer.push(bytes[offset]);
                    offset += 1;
                }
                if bytes[offset] != b']' {
                    panic!("Unclosed index in selector: {}", bytes[offset] as char)
                }
                offset += 1;
            }
            b'.' => {
                parts.push(if buffer_is_key {
                    Selector::Key(String::from_utf8(buffer.clone()).unwrap())
                } else {
                    Selector::Index(String::from_utf8(buffer.clone()).unwrap().parse().unwrap())
                });
                offset += 1;
                buffer.clear();
            }
            _ => {
                buffer_is_key = true;
                while offset < bytes.len() && bytes[offset] != b'.' {
                    buffer.push(bytes[offset]);
                    offset += 1;
                }
            }
        }
    }

    if !buffer.is_empty() {
        parts.push(if buffer_is_key {
            Selector::Key(String::from_utf8(buffer.clone()).unwrap())
        } else {
            Selector::Index(String::from_utf8(buffer.clone()).unwrap().parse().unwrap())
        });
    }

    parts
}

fn execute_selector(selector: &[Selector], json: serde_json::Value) -> Option<serde_json::Value> {
    let mut current = json;
    for selector in selector {
        match selector {
            Selector::Key(key) => {
                current = current.as_object()?.get(key)?.clone();
            }
            Selector::Index(idx) => {
                current = current.as_array()?.get(*idx)?.clone();
            }
        }
    }
    Some(current)
}

k3_wasm_macros::init!();
