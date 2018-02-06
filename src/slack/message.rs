extern crate regex;
extern crate serde_json;

use ip::Entry;

const IP_MESSAGE: &str = include_str!("json/ip_message.json");
const CREATE_NEW_MESSAGE: &str = include_str!("json/create_new_message.json");
const LIST_MESSAGE: &str = include_str!("json/list_message.json");

fn generate_port_buttons(ip: &str, port: &[u32]) -> String {
    serde_json::to_string(&port.iter()
        .map(|port| {
            json!({
                "name": "edit_port",
                "text": format!("{}", port),
                "type": "button",
                "value": format!("{}-{}", ip, port)
            })
        })
        .chain(
            vec![
                json!({
                    "name": "add_port",
                    "text": "추가",
                    "type": "button",
                    "style": "primary",
                    "value": ip
            }),
            ].into_iter(),
        )
        .collect::<Vec<_>>())
        .unwrap_or_default()
}

pub fn generate_ip_message(entry: &Entry) -> String {
    lazy_static! {
        static ref REGEX_INFOS: regex::Regex =
            regex::Regex::new(r"(?:/(ip|description|domain|using|using_style|ports)/)+?")
            .unwrap();
    }
    REGEX_INFOS
        .replace_all(IP_MESSAGE, |caps: &regex::Captures| match &caps[1] {
            "ip" => entry.ip.clone(),
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
            "ports" => generate_port_buttons(&entry.ip, &entry.open_ports),
            _ => String::new(),
        })
        .into_owned()
}

pub fn generate_create_new_message(ip: &str) -> String {
    CREATE_NEW_MESSAGE.replace("/ip/", ip)
}

pub fn generate_list_fields(entries: &[Entry], page: usize) -> String {
    serde_json::to_string(&entries
        .iter()
        .take((page + 1) * 8)
        .map(|entry| {
            json!({
                "title": entry.ip,
                "value":
                    entry.domain.as_ref()
                        .map(|s| format!("{}\n", s))
                        .unwrap_or_default() +
                    &entry.description.as_ref()
                        .map(|s| format!("{}\n", s))
                        .unwrap_or_default() +
                    if entry.using { "사용중" } else { "미사용" },
                "short": true
            })
        })
        .collect::<Vec<_>>())
        .unwrap_or_default()
}

pub fn generate_list_message(entries: &[Entry], page: usize) -> String {
    lazy_static! {
        static ref REGEX_LIST_INFOS: regex::Regex =
            regex::Regex::new(r"(?:/(title|fields|callback|value)/)+?")
            .unwrap();
    }
    REGEX_LIST_INFOS
        .replace_all(LIST_MESSAGE, |caps: &regex::Captures| match &caps[1] {
            "title" => "IP 목록".to_owned(),
            "fields" => generate_list_fields(entries, page),
            "callback" => "list".to_owned(),
            "value" => format!("{}", page),
            _ => String::new(),
        })
        .into_owned()
}

pub fn generate_query_message(query: &str, entries: &[Entry], page: usize) -> String {
    lazy_static! {
        static ref REGEX_QUERY_INFOS: regex::Regex =
            regex::Regex::new(r"(?:/(title|fields|callback|value)/)+?")
            .unwrap();
    }
    REGEX_QUERY_INFOS
        .replace_all(LIST_MESSAGE, |caps: &regex::Captures| match &caps[1] {
            "title" => format!("{} 검색 결과", query),
            "fields" => generate_list_fields(entries, page),
            "callback" => "query".to_owned(),
            "value" => format!("{}-{}", query, page),
            _ => String::new(),
        })
        .into_owned()
}
