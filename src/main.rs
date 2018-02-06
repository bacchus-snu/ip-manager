extern crate ip_manager;
extern crate tiny_http;

use ip_manager::*;
use tiny_http::{Header, Method, ResponseBox, Server};

fn main() {
    let server = Server::http("localhost:8000").unwrap();

    server.incoming_requests().for_each(|mut request| {
        let mut body = String::new();
        request
            .as_reader()
            .read_to_string(&mut body)
            .ok()
            .map(|_| match (request.method(), request.url()) {
                (&Method::Post, "//command") => resp_into_resp(handle_slash_command(&body)),
                (&Method::Post, "//submission") => resp_into_resp(handle_submission(&body)),
                (_, "//command") | (_, "//submission") => tiny_http::Response::empty(405).boxed(),
                _ => tiny_http::Response::empty(404).boxed(),
            })
            .and_then(|resp| request.respond(resp).ok())
            .unwrap()
    });
}

fn resp_into_resp(resp: Response) -> ResponseBox {
    match resp {
        Response::Unimplemented => tiny_http::Response::empty(501).boxed(),
        Response::Unauthorized => tiny_http::Response::empty(401).boxed(),
        Response::Json(s) => tiny_http::Response::from_string(s)
            .with_status_code(200)
            .with_header(
                Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap(),
            )
            .boxed(),
        Response::Error => tiny_http::Response::empty(500).boxed(),
    }
}
