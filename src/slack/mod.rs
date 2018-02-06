pub mod message;

pub mod dialog;
pub use self::dialog::Dialog;

pub mod submission;

pub mod slash_command {
    extern crate serde_urlencoded;
    use std::str::FromStr;
    use errors::{Error, Result};

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

    impl Request {
        pub fn from_str(s: &str) -> Result<Self> {
            FromStr::from_str(s)
        }
    }

    impl FromStr for Request {
        type Err = Error;

        fn from_str(s: &str) -> Result<Self> {
            serde_urlencoded::from_str(s).map_err(|e| e.into())
        }
    }
}
