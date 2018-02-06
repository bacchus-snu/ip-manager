#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate lazy_static;
extern crate regex;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

mod errors;
mod settings;
mod slack;
mod ip;

lazy_static! {
    static ref SETTINGS: settings::Settings =
        settings::Settings::try_new().unwrap();
}

pub enum Response {
    Unimplemented,
    Unauthorized,
    Empty,
    Json(String),
    Error,
}

pub fn handle_slash_command(body: &str) -> Response {
    lazy_static! {
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
                Some(Response::Json(slack::message::generate_query_message(
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
                    let mut split = interactive.callback_id.split('-');
                    let typ = split.next().unwrap();
                    let para = split.next().unwrap();
                    match typ.as_ref() {
                        "ip" => ip::Entry::from_ip(&para, SETTINGS.data_path())
                            .map(|mut entry| {
                                let action = &interactive.actions[0];
                                match action.name.as_ref() {
                                    "edit_domain" => {
                                        slack::dialog::show_edit_domain_dialog(
                                            &entry,
                                            SETTINGS.token(),
                                            &interactive.trigger_id,
                                        ).unwrap();
                                        Response::Empty
                                    }
                                    "toggle_using" => {
                                        entry.using = !entry.using;
                                        entry.save().unwrap();
                                        Response::Json(slack::message::generate_ip_message(&entry))
                                    }
                                    "edit_port" => {
                                        slack::dialog::show_edit_port_dialog(
                                            &entry.ip,
                                            &action.value,
                                            SETTINGS.token(),
                                            &interactive.trigger_id,
                                        ).unwrap();
                                        Response::Empty
                                    }
                                    "add_port" => {
                                        slack::dialog::show_add_port_dialog(
                                            &entry.ip,
                                            SETTINGS.token(),
                                            &interactive.trigger_id,
                                        ).unwrap();
                                        Response::Empty
                                    }
                                    "edit_description" => {
                                        slack::dialog::show_edit_description_dialog(
                                            &entry,
                                            SETTINGS.token(),
                                            &interactive.trigger_id,
                                        ).unwrap();
                                        Response::Empty
                                    }
                                    "delete_entry" => {
                                        entry.delete().unwrap();
                                        Response::Json(slack::message::generate_deleted_message())
                                    }
                                    _ => Response::Unimplemented,
                                }
                            })
                            .unwrap_or_else(|| {
                                Response::Json(slack::message::generate_inexist_message())
                            }),
                        "list" => Response::Json(slack::message::generate_list_message(
                            &ip::Entry::list(SETTINGS.data_path()),
                            interactive.actions[0].value.parse::<usize>().unwrap() + 1,
                        )),
                        "query" => Response::Json(slack::message::generate_query_message(
                            &para,
                            &ip::Entry::search(&para, SETTINGS.data_path()),
                            interactive.actions[0].value.parse::<usize>().unwrap() + 1,
                        )),
                        "create_new" => {
                            Response::Json(if interactive.actions[0].value == "create_new_entry" {
                                slack::message::generate_ip_message(
                                    &ip::Entry::new(&para, SETTINGS.data_path()).unwrap(),
                                )
                            } else {
                                slack::message::generate_cancelled_message()
                            })
                        }
                        _ => Response::Unimplemented,
                    }
                }
                Submission::Dialog(dialog) => {
                    println!("{:?}", dialog);
                    Response::Unimplemented
                }
            }
        })
        .unwrap_or_else(|| Response::Error)
}
