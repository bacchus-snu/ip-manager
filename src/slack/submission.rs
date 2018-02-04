use std::collections::HashMap;

#[derive(Deserialize)]
pub struct Submission {
    pub payload: Payload,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum Payload {
    #[serde(rename = "interactive_message")] Interactive,
    #[serde(rename = "dialog_submission")] Dialog,
}

#[derive(Deserialize)]
pub struct Interactive {
    pub actions: Vec<Action>,
    pub callback_id: String,
    pub token: String,
    pub response_url: String,
}

#[derive(Deserialize)]
pub struct Action {
    pub name: String,
    #[serde(rename = "type")]
    pub action_type: String,
    pub value: String,
}

#[derive(Deserialize)]
pub struct Dialog {
    pub submission: HashMap<String, String>,
    pub callback_id: String,
    pub token: String,
}
