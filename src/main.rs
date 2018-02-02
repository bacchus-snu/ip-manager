extern crate ip_manager;
extern crate serde;
extern crate serde_json;
extern crate serde_urlencoded;
extern crate tiny_http;

use tiny_http::{Method, Request, Response, Server};
use ip_manager::Result;
use ip_manager::slack::{Dialog, Message, SlashCommandRequest};

const IP_MESSAGE: &str = include_str!("ip_message.json");

fn main() {
    let server = Server::http("localhost:8000").unwrap();

    server.incoming_requests().for_each(|mut request| {
        let mut body = String::new();
        request.as_reader().read_to_string(&mut body).unwrap();
        let resp = match (request.method(), request.url()) {
            (&Method::Post, "//command") => slash_command(&body)
                .map(|_| Response::empty(200))
                .unwrap_or_else(|_| Response::empty(500)),
            (method, url) => {
                println!("{} {}", method, url);
                Response::empty(404)
            }
        };
        request.respond(resp).unwrap();
    });
}

fn slash_command(body: &str) -> Result<()> {
    println!("body: {}", body);
    let command = serde_urlencoded::from_str::<SlashCommandRequest>(body)?;
    println!("{:?}", command);
    Ok(())
}
