use k3_wasm_macros::http_handler;
use k3_wasm_sdk::{
    http::{Request, Response},
    kv::Db,
};
use uuid::Uuid;

#[http_handler]
pub fn get(req: Request<Vec<u8>>) -> Response<Vec<u8>> {
    let target = req.uri().to_string();
    let uid = target.split('/').last().unwrap();
    if uid == "users" {
        Response::builder()
            .status(400)
            .body("USAGE: /api/users/[id]".as_bytes().to_vec())
            .unwrap()
    } else {
        let db = Db::open_default();
        if let Some(name) = db.get(&format!("users:{}", uid)) {
            Response::builder()
                .status(200)
                .body(name.as_bytes().to_vec())
                .unwrap()
        } else {
            Response::builder()
                .status(404)
                .body("User with that ID not found".as_bytes().to_vec())
                .unwrap()
        }
    }
}

#[http_handler]
pub fn post(req: Request<Vec<u8>>) -> Response<Vec<u8>> {
    let name = String::from_utf8(req.into_body()).unwrap();
    let mut db = Db::open_default();
    let uid = Uuid::new_v4().to_string();
    db.set(&format!("users:{}", uid), name);
    Response::builder()
        .status(200)
        .body(uid.as_bytes().to_vec())
        .unwrap()
}
