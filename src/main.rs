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
use ip_manager::ip;
use ip_manager::slack::{Message, SlashCommandRequest};
use ip_manager::slack::message::{Action, Attachment, Button};
use regex::Regex;

const RAW_IP_MESSAGE: &str = include_str!("ip_message.json");

fn main() {
    let server = Server::http("localhost:8000").unwrap();

    server.incoming_requests().for_each(|mut request| {
        let mut body = String::new();
        request.as_reader().read_to_string(&mut body).unwrap();
        let resp: ResponseBox = match (request.method(), request.url()) {
            (&Method::Post, "//command") => slash_command(&body),
            _ => Response::empty(404).boxed(),
        };
        request.respond(resp).unwrap();
    });
}

fn slash_command(body: &str) -> ResponseBox {
    lazy_static! {
        static ref SETTINGS: Settings =
            Settings::try_new().unwrap();
        static ref REGEX_IP: Regex =
            Regex::new(r"^\d{1,3}.\d{1,3}.\d{1,3}.\d{1,3}$")
            .unwrap();
    }

    serde_urlencoded::from_str::<SlashCommandRequest>(body)
        .ok()
        .and_then(|command| REGEX_IP.find(&command.text).map(|m| m.as_str().to_owned()))
        .and_then(|sip| ip::get_or_create(&sip, SETTINGS.data_path()))
        .map(generate_ip_message)
        .and_then(|message| serde_json::to_string(&message).ok())
        .map(|s| {
            Response::from_string(s)
                .with_status_code(200)
                .with_header(
                    Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap(),
                )
                .boxed()
        })
        .unwrap_or_else(|| Response::empty(500).boxed())
}

fn generate_ip_message(entry: Entry) -> Message {
    macro_rules! edit_attachment {
        ($t:ident($a:ident) = $b:expr, $c:block) => {
            if let Attachment::$t(ref mut $a) = $b {
                $a.callback_id.push_str("-");
                $a.callback_id.push_str(&entry.ip);
                $c
            }
        };
    }
    macro_rules! edit_action {
        ($t:ident($a:ident) = $b:expr, $c:block) => {
            if let Action::$t(ref mut $a) = $b {
                $c
            }
        };
    }

    let mut message: Message = serde_json::from_str(RAW_IP_MESSAGE).unwrap();
    {
        message.text = Some(entry.ip.clone());
        let attachments: &mut Vec<Attachment> = message.attachments.as_mut().unwrap();
        edit_attachment!(Interactive(attachment) = attachments[0], {
            edit_action!(Button(button) = attachment.actions[0], {
                button.text = entry.domain.unwrap_or_else(|| "추가".to_owned());
            });
        });
        edit_attachment!(Interactive(attachment) = attachments[1], {
            edit_action!(Button(button) = attachment.actions[0], {
                button.text = if entry.using {
                    "사용중"
                } else {
                    "미사용"
                }.to_owned();
            });
        });
        edit_attachment!(Interactive(attachment) = attachments[2], {
            attachment.actions = entry
                .open_ports
                .into_iter()
                .map(|port| Button {
                    name: format!("port-{}", port),
                    text: format!("{}", port),
                    color: None,
                    style: None,
                    value: format!("port-{}", port),
                })
                .chain(
                    vec![
                        Button {
                            name: "add".to_owned(),
                            text: "추가".to_owned(),
                            color: None,
                            style: Some("primary".to_owned()),
                            value: "add".to_owned(),
                        },
                    ].into_iter(),
                )
                .map(Action::Button)
                .collect();
        });
        edit_attachment!(Interactive(attachment) = attachments[3], {
            attachment.text = entry.description;
        });
        edit_attachment!(Interactive(attachment) = attachments[4], {});
    }
    message
}
