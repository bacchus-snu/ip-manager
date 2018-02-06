extern crate regex;
extern crate reqwest;
extern crate serde_json;

use ip::Entry;
use errors::Result;

const EDIT_DIALOG: &str = include_str!("json/edit_dialog.json");
const EDIT_DESCRIPTION_DIALOG: &str = include_str!("json/edit_description_dialog.json");
const ADD_PORT_DIALOG: &str = include_str!("json/add_port_dialog.json");

fn show(dialog: &str, trigger_id: &str, token: &str) -> Result<()> {
    super::request_api(
        "dialog.open",
        &json!({
            "dialog": dialog,
            "trigger_id": trigger_id,
        }),
        token,
    )
}

fn generate_edit_dialog(
    title: &str,
    ip: &str,
    callback: &str,
    label: &str,
    name: &str,
    value: &str,
) -> String {
    lazy_static! {
        static ref REGEX_EDIT: regex::Regex =
            regex::Regex::new(r"(?:/(title|name|callback|value|label)/)+?")
            .unwrap();
    }
    REGEX_EDIT
        .replace_all(EDIT_DIALOG, |caps: &regex::Captures| match &caps[1] {
            "title" => title.to_owned(),
            "name" => name.to_owned(),
            "callback" => format!("{}-{}", callback, ip),
            "value" => value.to_owned(),
            "label" => label.to_owned(),
            _ => String::new(),
        })
        .into_owned()
}

pub fn show_edit_domain_dialog(entry: &Entry, trigger_id: &str, token: &str) -> Result<()> {
    show(
        &generate_edit_dialog(
            "도메인 추가/수정",
            &entry.ip,
            "edit_domain",
            "도메인",
            "domain",
            &entry.domain.clone().unwrap_or_default(),
        ),
        trigger_id,
        token,
    )
}

pub fn show_edit_description_dialog(entry: &Entry, trigger_id: &str, token: &str) -> Result<()> {
    lazy_static! {
        static ref REGEX_DESCRIPTION: regex::Regex =
            regex::Regex::new(r"(?:/(ip|value)/)+?")
            .unwrap();
    }
    show(
        &REGEX_DESCRIPTION
            .replace_all(
                EDIT_DESCRIPTION_DIALOG,
                |caps: &regex::Captures| match &caps[1] {
                    "ip" => entry.ip.clone(),
                    "value" => entry.description.clone().unwrap_or_default(),
                    _ => String::new(),
                },
            )
            .into_owned(),
        trigger_id,
        token,
    )
}

pub fn show_edit_port_dialog(ip: &str, port: &str, trigger_id: &str, token: &str) -> Result<()> {
    show(
        &generate_edit_dialog("포트 수정", ip, "edit_port", "포트", port, port),
        trigger_id,
        token,
    )
}

pub fn show_add_port_dialog(ip: &str, trigger_id: &str, token: &str) -> Result<()> {
    show(&ADD_PORT_DIALOG.replace("/ip/", ip), trigger_id, token)
}
