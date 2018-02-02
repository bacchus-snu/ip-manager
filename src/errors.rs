error_chain!{
    foreign_links {
        Reqwest(::reqwest::Error);
        Json(::serde_json::Error);
        Io(::std::io::Error);
        Config(::config::ConfigError);
    }
}
