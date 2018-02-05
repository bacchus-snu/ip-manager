extern crate ip_manager;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate serde;
extern crate serde_json;
extern crate serde_urlencoded;
extern crate tiny_http;

use tiny_http::{Header, Method, Response, ResponseBox, Server};
use ip_manager::Settings;
use ip_manager::ip::Entry;
use ip_manager::slack::*;
use regex::{Captures, Regex};

const IP_MESSAGE: &str = include_str!("json/ip_message.json");
const CREATE_NEW_MESSAGE: &str = include_str!("json/create_new_message.json");
const NOT_FOUND_MESSAGE: &str = include_str!("json/not_found_message.json");
const EMPTY_QUERY_MESSAGE: &str = include_str!("json/empty_query_message.json");

fn main() {
    let server = Server::http("localhost:8000").unwrap();

    server.incoming_requests().for_each(|mut request| {
        let mut body = String::new();
        request.as_reader().read_to_string(&mut body).unwrap();
        let resp: ResponseBox = match (request.method(), request.url()) {
            (&Method::Post, "//command") => handle_slash_command(&body),
            (&Method::Post, "//submission") => handle_submission(&body),
            (_, "//command") | (_, "//submission") => Response::empty(405).boxed(),
            _ => Response::empty(404).boxed(),
        };
        request.respond(resp).unwrap();
    });
}

fn handle_slash_command(body: &str) -> ResponseBox {
    lazy_static! {
        static ref SETTINGS: Settings =
            Settings::try_new().unwrap();
        static ref REGEX_IP: Regex =
            Regex::new(r"^\d{1,3}.\d{1,3}.\d{1,3}.\d{1,3}$")
            .unwrap();
    }

    serde_urlencoded::from_str::<slash_command::Request>(body)
        .ok()
        .and_then(|command| {
            if !SETTINGS.verify(&command.token) {
                Some(
                    Response::from_string("Invalid token")
                        .with_status_code(401)
                        .boxed(),
                )
            } else if command.text.is_empty() {
                Some(
                    Response::from_string(EMPTY_QUERY_MESSAGE)
                        .with_status_code(200)
                        .with_header(
                            Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..])
                                .unwrap(),
                        )
                        .boxed(),
                )
            } else {
                REGEX_IP
                    .find(&command.text)
                    .map(|m| m.as_str().to_owned())
                    .map(|sip| {
                        Entry::from_ip(&sip, SETTINGS.data_path())
                            .map(|entry| generate_ip_message(&entry))
                            .unwrap_or_else(|| generate_create_new_message(&sip))
                    })
                    .map(|s| {
                        Response::from_string(s)
                            .with_status_code(200)
                            .with_header(
                                Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..])
                                    .unwrap(),
                            )
                            .boxed()
                    })
            }
        })
        .unwrap_or_else(|| Response::empty(500).boxed())
}

fn handle_submission(body: &str) -> ResponseBox {
    println!("{}", body);
    Response::empty(501).boxed()
}

fn generate_ip_message(entry: &Entry) -> String {
    lazy_static! {
        static ref REGEX_INFOS: Regex =
            Regex::new(r"(?:/(callback|ip|description|domain|using|using_style|ports)/)+?")
            .unwrap();
    }
    REGEX_INFOS
        .replace_all(IP_MESSAGE, |caps: &Captures| match &caps[1] {
            "callback" | "ip" => entry.ip.clone(),
            "description" => entry.description.clone().unwrap_or_default(),
            "domain" => entry
                .domain
                .clone()
                .unwrap_or_else(|| "도메인 추가".to_owned()),
            "using" => if entry.using {
                "사용중"
            } else {
                "미사용"
            }.to_owned(),
            "using_style" => if entry.using { "danger" } else { "primary" }.to_owned(),
            "ports" => serde_json::to_string(&entry
                .open_ports
                .iter()
                .map(|port| {
                    message::Button {
                        name: "port".to_owned(),
                        text: format!("{}", port),
                        style: None,
                        value: format!("{}", port),
                        confirm: None,
                    }
                })
                .chain(
                    vec![
                        message::Button {
                            name: "add_port".to_owned(),
                            text: "추가".to_owned(),
                            style: Some("primary".to_owned()),
                            value: "add_port".to_owned(),
                            confirm: None,
                        },
                    ].into_iter(),
                )
                .map(message::Action::Button)
                .collect::<Vec<message::Action>>())
                .unwrap(),
            _ => String::new(),
        })
        .into_owned()
}

fn generate_create_new_message(ip: &str) -> String {
    lazy_static! {
        static ref REGEX_SMALL_INFOS: Regex =
            Regex::new(r"(?:/(callback|ip)/)+?")
            .unwrap();
    }
    REGEX_SMALL_INFOS
        .replace_all(CREATE_NEW_MESSAGE, ip)
        .into_owned()
}
