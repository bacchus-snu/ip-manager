#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate lazy_static;
extern crate regex;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

pub mod errors;
pub use errors::{Error, ErrorKind, Result};

pub mod settings;
pub use settings::Settings;

pub mod slack;
pub mod ip;

pub enum Response {
    Unimplemented,
    Unauthorized,
    Json(String),
    Error,
}

pub fn handle_slash_command(body: &str) -> Response {
    lazy_static! {
        static ref SETTINGS: Settings =
            Settings::try_new().unwrap();
        static ref REGEX_IP: regex::Regex =
            regex::Regex::new(r"^\d{1,3}.\d{1,3}.\d{1,3}.\d{1,3}$")
            .unwrap();
    }

    slack::slash_command::Request::from_str(body)
        .ok()
        .and_then(|command| {
            if !SETTINGS.verify(&command.token) {
                Some(Response::Unauthorized)
            } else if command.text.is_empty() {
                Some(Response::Json(slack::message::generate_list_message(
                    "IP 목록",
                    "",
                    &ip::Entry::list(SETTINGS.data_path()),
                    0,
                )))
            } else if REGEX_IP.is_match(&command.text) {
                REGEX_IP
                    .find(&command.text)
                    .map(|m| m.as_str().to_owned())
                    .map(|sip| {
                        ip::Entry::from_ip(&sip, SETTINGS.data_path())
                            .map(|entry| slack::message::generate_ip_message(&entry))
                            .unwrap_or_else(|| slack::message::generate_create_new_message(&sip))
                    })
                    .map(Response::Json)
            } else {
                Some(Response::Json(slack::message::generate_list_message(
                    &format!("{} 검색 결과", &command.text),
                    &command.text,
                    &ip::Entry::search(&command.text, SETTINGS.data_path()),
                    0,
                )))
            }
        })
        .unwrap_or_else(|| Response::Error)
}

pub fn handle_submission(body: &str) -> Response {
    slack::submission::Submission::from_str(body)
        .ok()
        .map(|submission| {
            use slack::submission::Submission;
            match submission {
                Submission::Interactive(interactive) => {
                    println!("{:?}", interactive);
                    Response::Unimplemented
                },
                Submission::Dialog(dialog) => {
                    println!("{:?}", dialog);
                    Response::Unimplemented
                },
            }
        })
        .unwrap_or_else(|| Response::Error)
}
