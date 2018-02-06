extern crate serde_json;
extern crate serde_urlencoded;

use std::collections::HashMap;
use std::str::FromStr;
use errors::{Error, Result};

#[derive(Deserialize)]
struct Request {
    pub payload: String,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum Submission {
    #[serde(rename = "interactive_message")] Interactive(Interactive),
    #[serde(rename = "dialog_submission")] Dialog(Dialog),
}

impl FromStr for Submission {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        serde_urlencoded::from_str::<Request>(s)
            .map_err(|e| e.into())
            .map(|subm| subm.payload)
            .and_then(|payload| {
                serde_json::from_str(&payload).map_err(|e| e.into())
            })
    }
}

impl Submission {
    pub fn from_str(s: &str) -> Result<Self> {
        FromStr::from_str(s)
    }
}

#[derive(Deserialize, Debug)]
pub struct Interactive {
    pub actions: Vec<Action>,
    pub callback_id: String,
    pub channel: Channel,
    pub message_ts: String,
    pub token: String,
    pub response_url: String,
    pub trigger_id: String,
}

#[derive(Deserialize, Debug)]
pub struct Action {
    pub name: String,
    #[serde(rename = "type")] pub action_type: String,
    pub value: String,
}

#[derive(Deserialize, Debug)]
pub struct Dialog {
    pub submission: HashMap<String, String>,
    pub callback_id: String,
    pub token: String,
    pub trigger_id: String,
}

#[derive(Deserialize, Debug)]
pub struct Channel {
    pub id: String,
    pub name: String,
}
