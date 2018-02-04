extern crate toml;

use std::path::Path;
use std::fs::{read_dir, remove_file, DirEntry, File, ReadDir};
use std::io::{Read, Write};
use super::Result;

#[derive(Serialize, Deserialize)]
pub struct Entry {
    pub ip: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    pub using: bool,
    pub open_ports: Vec<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl Entry {
    pub fn new(ip: &str) -> Self {
        Entry {
            ip: ip.to_owned(),
            domain: None,
            using: false,
            open_ports: vec![],
            description: None,
        }
    }

    pub fn ports_as_string(&self) -> String {
        let mut s = String::new();
        for p in &self.open_ports {
            s.push_str(&format!("{}, ", p));
        }
        s.pop();
        s.pop();
        s
    }
}

pub struct Query {
    pub ip: String,
    pub element: String,
}

pub fn create(ip: &str, data_path: &Path) -> Result<Entry> {
    let entry = Entry {
        ip: ip.to_owned(),
        domain: None,
        using: false,
        open_ports: vec![],
        description: None,
    };
    let s = toml::to_string_pretty(&entry)?;
    let p = data_path.join(Path::new(&ip.replace(".", "-")).with_extension("toml"));
    let mut file: File = File::create(&p)?;
    file.write_all(s.as_bytes())?;

    Ok(entry)
}

pub fn add(entry: &Entry, data_path: &Path) -> Result<()> {
    let s = toml::to_string_pretty(entry)?;
    let p = data_path.join(Path::new(&entry.ip.replace(".", "-")).with_extension("toml"));
    let mut file: File = File::create(&p)?;
    file.write_all(s.as_bytes())?;

    Ok(())
}

pub fn get(ip: &str, data_path: &Path) -> Option<Entry> {
    let p = data_path.join(Path::new(&ip.replace(".", "-")).with_extension("toml"));
    let mut file: File = match File::open(&p) {
        Ok(f) => f,
        Err(_) => return None,
    };
    let mut content = String::new();
    if file.read_to_string(&mut content).is_err() {
        return None;
    }
    toml::from_str(&content).ok()
}

pub fn get_or_create(ip: &str, data_path: &Path) -> Option<Entry> {
    get(ip, data_path).or_else(|| create(ip, data_path).ok())
}

pub fn list(query: &str, data_path: &Path) -> Vec<Query> {
    let dir_entries: ReadDir = match read_dir(data_path) {
        Ok(d) => d,
        Err(_) => return vec![],
    };
    let files: Vec<DirEntry> = dir_entries
        .filter(|e| e.is_ok())
        .map(|e| e.unwrap())
        .collect();
    let entries = files.into_iter().map(|f| {
        let mut file = File::open(f.path()).unwrap();
        let mut content: String = String::new();
        file.read_to_string(&mut content).unwrap();
        toml::from_str::<Entry>(&content).unwrap()
    });
    let entries: Vec<Query> = if !query.is_empty() {
        entries
            .filter_map(|e| {
                query
                    .split(' ')
                    .filter(|q| !q.is_empty())
                    .filter_map(|q| generate_query(&e, q))
                    .next()
            })
            .take(8)
            .collect()
    } else {
        entries
            .map(|e| Query {
                ip: e.ip.clone(),
                element: {
                    let mut s = String::new();
                    if let Some(ref domain) = e.domain {
                        s.push_str(domain);
                        s.push_str("\n");
                    }
                    s.push_str(if e.using { "사용중" } else { "미사용" });
                    s
                },
            })
            .take(8)
            .collect()
    };
    entries
}

fn generate_query(entry: &Entry, q: &str) -> Option<Query> {
    if entry.ip.contains(q) {
        return Some(Query {
            ip: entry.ip.clone(),
            element: {
                let mut s = String::new();
                if let Some(ref domain) = entry.domain {
                    s.push_str(domain);
                    s.push_str("\n");
                }
                s.push_str(if entry.using {
                    "사용중"
                } else {
                    "미사용"
                });
                s
            },
        });
    }
    if let Some(ref domain) = entry.domain {
        if domain.contains(q) {
            return Some(Query {
                ip: entry.ip.clone(),
                element: entry.domain.as_ref().unwrap().clone(),
            });
        }
    }
    if entry.using && q == "사용중" {
        return Some(Query {
            ip: entry.ip.clone(),
            element: "사용중".to_owned(),
        });
    }
    if !entry.using && q == "미사용" {
        return Some(Query {
            ip: entry.ip.clone(),
            element: "미사용".to_owned(),
        });
    }
    if let Ok(i) = q.parse::<u32>() {
        if entry.open_ports.contains(&i) {
            return Some(Query {
                ip: entry.ip.clone(),
                element: entry.ports_as_string(),
            });
        }
    }
    if entry.description.is_some() && entry.description.as_ref().unwrap().contains(q) {
        return Some(Query {
            ip: entry.ip.clone(),
            element: entry.description.as_ref().unwrap().clone(),
        });
    }
    None
}

pub fn issue(required_ports: &[u32], data_path: &str) -> Option<Entry> {
    let dir_entries: ReadDir = match read_dir(data_path) {
        Ok(r) => r,
        Err(_) => return None,
    };
    let files: Vec<DirEntry> = dir_entries
        .filter(|e| e.is_ok())
        .map(|e| e.unwrap())
        .collect();
    files
        .into_iter()
        .map(|f| {
            let mut file = File::open(f.path()).unwrap();
            let mut content: String = String::new();
            file.read_to_string(&mut content).unwrap();
            toml::from_str::<Entry>(&content).unwrap()
        })
        .find(|e| {
            !e.using
                && (required_ports.is_empty()
                    || required_ports.into_iter().all(|p| e.open_ports.contains(p)))
        })
}

pub fn delete(ip: &str, data_path: &Path) -> Result<()> {
    let p = data_path.join(Path::new(&ip.replace(".", "-")).with_extension("toml"));
    remove_file(p)?;
    Ok(())
}
