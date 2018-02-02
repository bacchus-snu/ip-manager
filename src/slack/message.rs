#[derive(Serialize, Deserialize)]
pub struct Message {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    pub attachments: Option<Vec<Attachment>>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Attachment {
    Basic(BasicAttachment),
    Interactive(InteractiveAttachment),
}

#[derive(Serialize, Deserialize)]
pub struct BasicAttachment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    pub fields: Vec<AttachmentField>,
}

#[derive(Serialize, Deserialize)]
pub struct AttachmentField {
    pub title: String,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct InteractiveAttachment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    pub callback_id: String,
    pub actions: Vec<Action>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Action {
    #[serde(rename = "button")] Button(Button),
    #[serde(rename = "select")] Select(Select),
}

#[derive(Serialize, Deserialize)]
pub struct Button {
    pub name: String,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    pub value: String,
}

#[derive(Serialize, Deserialize)]
pub struct Select {
    pub name: String,
    pub options: Vec<SelectOption>,
}

#[derive(Serialize, Deserialize)]
pub struct SelectOption {
    pub text: String,
    pub value: String,
}
