extern crate config;
extern crate reqwest;
extern crate serde_json;
extern crate serde_urlencoded;

error_chain!{
    foreign_links {
        Reqwest(self::reqwest::Error);
        Json(self::serde_json::Error);
        Io(::std::io::Error);
        Config(self::config::ConfigError);
        UrlencodedDe(self::serde_urlencoded::de::Error);
        UrlencodedSer(self::serde_urlencoded::ser::Error);
    }
}
