extern crate config;
extern crate reqwest;
extern crate serde_json;

error_chain!{
    foreign_links {
        Reqwest(self::reqwest::Error);
        Json(self::serde_json::Error);
        Io(::std::io::Error);
        Config(self::config::ConfigError);
    }
}
