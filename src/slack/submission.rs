#[derive(Deserialize)]
pub struct Response {
    pub payload: Payload,
}

#[derive(Deserialize)]
#[serde(tags = "type")]
pub enum Payload {
    #[serde(rename = "interactive_message")]
    Interactive,
    #[serde(rename = "dialog_submission")]
    Dialog,
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
    pub type: String,
    pub value: String,
}

#[derive(Deserialize)]
pub struct Dialog {
    pub submission: Vec<Submission>,
    pub callback_id: String,
    pub token: String,
}

pub type Submission = std::collections::HashMap<String, String>;
