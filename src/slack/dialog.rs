#[derive(Serialize)]
pub struct Request {
    pub token: String,
    pub dialog: Dialog,
    pub trigger_id: String,
}

#[derive(Serialize)]
pub struct Dialog {
    pub callback_id: String,
    pub title: String,
    pub submit_label: String,
    pub elements: Vec<Element>,
}

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum Element {
    #[serde(rename = "text")] Text(Text),
    #[serde(rename = "textarea")] TextArea(TextArea),
    #[serde(rename = "select")] Select(Select),
}

#[derive(Serialize)]
pub struct Text {
    pub label: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtype: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
}

#[derive(Serialize)]
pub struct TextArea {
    pub label: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtype: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
}

#[derive(Serialize)]
pub struct Select {
    pub label: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
    pub options: Vec<SelectOption>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
}

#[derive(Serialize, Clone)]
pub struct SelectOption {
    pub label: String,
    pub value: String,
}

#[derive(Deserialize)]
pub struct Response<R> {
    pub submission: Vec<R>,
    pub callback_id: String,
    pub token: String,
}
