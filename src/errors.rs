extern crate config;
extern crate reqwest;
extern crate serde_json;
extern crate serde_urlencoded;
extern crate toml;

error_chain!{
    foreign_links {
        Reqwest(self::reqwest::Error);
        Json(self::serde_json::Error);
        Io(::std::io::Error);
        Config(self::config::ConfigError);
        UrlencodedDe(self::serde_urlencoded::de::Error);
        TomlSer(self::toml::ser::Error);
    }
}
