pub mod message;
pub use self::message::Message;

pub mod dialog;
pub use self::dialog::Dialog;

pub mod submission;

pub mod slash_command {
    #[derive(Deserialize, Debug)]
    pub struct Request {
        pub token: String,
        pub team_id: String,
        pub team_domain: String,
        pub channel_id: String,
        pub channel_name: String,
        pub user_id: String,
        pub user_name: String,
        pub text: String,
        pub response_url: String,
        pub trigger_id: String,
    }
}
