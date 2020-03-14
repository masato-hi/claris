use std::fmt;
use std::fs::File;
use std::io::Read;
use yaml_rust::{Yaml, YamlLoader};

#[derive(Debug)]
pub enum LoadError {
    OpenError(String),
    ReadError(String),
    ParseError(String),
    NoEntryError,
    TooManyEntryError,
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LoadError::OpenError(x) => f.write_fmt(format_args!("file open error! path: '{}'", x)),
            LoadError::ReadError(x) => f.write_fmt(format_args!("file read error! path: '{}'", x)),
            LoadError::ParseError(x) => {
                f.write_fmt(format_args!("invalid yaml format! path: '{}'", x))
            }
            LoadError::NoEntryError => f.write_str("yaml has no entry!"),
            LoadError::TooManyEntryError => f.write_str("yaml has too many entry!"),
        }
    }
}

pub struct SourceLoader {}

impl SourceLoader {
    pub fn load(path: &str) -> Result<Yaml, LoadError> {
        let mut file = File::open(path).map_err(|_| LoadError::OpenError(path.to_string()))?;
        let mut data = String::new();
        file.read_to_string(&mut data)
            .or(Err(LoadError::ReadError(path.to_string())))?;
        let docs = YamlLoader::load_from_str(&data)
            .map_err(|_| LoadError::ParseError(path.to_string()))?;

        if docs.len() < 1 {
            return Err(LoadError::NoEntryError);
        } else if docs.len() > 1 {
            return Err(LoadError::TooManyEntryError);
        }

        let doc = &docs[0];

        return Ok(doc.clone());
    }
}
