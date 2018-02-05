extern crate toml;

use std::path::{Path, PathBuf};
use std::fs::{remove_file, File, OpenOptions};
use std::io::{Read, Write};
use std::convert::Into;
use super::Result;

pub struct Query {
    pub ip: String,
    pub element: String,
}

#[derive(Deserialize)]
struct InnerEntry {
    ip: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain: Option<String>,
    using: bool,
    open_ports: Vec<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}

impl InnerEntry {
    fn into_entry(self, path: PathBuf) -> Entry {
        Entry {
            ip: self.ip,
            domain: self.domain,
            using: self.using,
            open_ports: self.open_ports,
            description: self.description,
            path,
        }
    }
}

#[derive(Serialize)]
pub struct Entry {
    pub ip: String,
    pub domain: Option<String>,
    pub using: bool,
    pub open_ports: Vec<u32>,
    pub description: Option<String>,
    #[serde(skip_serializing)]
    path: PathBuf,
}

impl Into<InnerEntry> for Entry {
    fn into(self) -> InnerEntry {
        InnerEntry {
            ip: self.ip,
            domain: self.domain,
            using: self.using,
            open_ports: self.open_ports,
            description: self.description,
        }
    }
}

impl Entry {
    pub fn new(ip: &str, data_path: &Path) -> Result<Entry> {
        let p = data_path.join(Path::new(&ip.replace(".", "-")).with_extension("toml"));
        let mut file: File = File::create(&p)?;
        let entry = Entry {
            ip: ip.to_owned(),
            domain: None,
            using: false,
            open_ports: vec![],
            description: None,
            path: p,
        };
        let s = toml::to_string_pretty(&entry)?;
        file.write_all(s.as_bytes())?;

        Ok(entry)
    }

    pub fn from_ip(ip: &str, data_path: &Path) -> Option<Entry> {
        let p = data_path.join(Path::new(&ip.replace(".", "-")).with_extension("toml"));
        let mut file: File = match File::open(&p) {
            Ok(f) => f,
            Err(_) => return None,
        };
        let mut content = String::new();
        if file.read_to_string(&mut content).is_err() {
            return None;
        }
        toml::from_str::<InnerEntry>(&content)
            .ok()
            .map(|ie| ie.into_entry(p))
    }

    pub fn delete(&self, data_path: &Path) -> Result<()> {
        let p = data_path.join(Path::new(&self.ip.replace(".", "-")).with_extension("toml"));
        remove_file(p)?;
        Ok(())
    }

    pub fn save(&self) -> Result<()> {
        let mut file: File = OpenOptions::new().write(true).open(&self.path)?;
        let s = toml::to_string_pretty(&self)?;
        file.write_all(s.as_bytes())?;

        Ok(())
    }
}
