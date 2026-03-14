use std::fmt;
use std::io;
use std::path::PathBuf;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    TomlParse(toml::de::Error),
    MissingChaptersDir(PathBuf),
    HyphenationDict(PathBuf, io::Error),
    StyleNotFound(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "I/O chyba: {e}"),
            Error::TomlParse(e) => write!(f, "Chyba v metadata.toml: {e}"),
            Error::MissingChaptersDir(p) => {
                write!(f, "Adresář kapitol nenalezen: {}", p.display())
            }
            Error::HyphenationDict(p, e) => {
                write!(f, "Nelze načíst slovník dělení '{}': {e}", p.display())
            }
            Error::StyleNotFound(s) => write!(f, "Styl '{s}' nenalezen"),
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<toml::de::Error> for Error {
    fn from(e: toml::de::Error) -> Self {
        Error::TomlParse(e)
    }
}
