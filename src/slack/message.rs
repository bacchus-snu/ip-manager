#[derive(Serialize)]
pub struct Message {
    pub text: Option<String>,
    pub attachments: Option<Vec<Attachment>>,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum Attachment {
    Basic(BasicAttachment),
    Interactive(InteractiveAttachment),
}

#[derive(Serialize)]
pub struct BasicAttachment {
    pub title: Option<String>,
    pub text: Option<String>,
    pub fields: Vec<AttachmentField>,
}

#[derive(Serialize)]
pub struct AttachmentField {
    pub title: String,
    pub value: String,
    pub short: Option<bool>,
}

#[derive(Serialize)]
pub struct InteractiveAttachment {
    pub title: String,
    pub text: Option<String>,
    pub callback_id: String,
    pub actions: Vec<Action>,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum Action {
    Button(Button),
    Select(Select),
}

#[derive(Serialize)]
pub struct Button {
    pub name: String,
    pub text: String,
    pub value: String,
}

#[derive(Serialize)]
pub struct Select {
    pub name: String,
    pub options: Vec<SelectOption>,
}

#[derive(Serialize)]
pub struct SelectOption {
    pub text: String,
    pub value: String,
}
