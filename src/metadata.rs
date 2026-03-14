use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::error::Error;

#[derive(Debug, Deserialize, Default)]
pub struct Metadata {
    pub kniha: Option<Kniha>,
    pub sazba: Option<Sazba>,
    pub cesty: Option<Cesty>,
    pub styl: Option<Styl>,
}

#[derive(Debug, Deserialize)]
pub struct Kniha {
    pub nazev: Option<String>,
    pub autor: Option<String>,
    pub rok: Option<u32>,
    pub isbn: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Sazba {
    pub papir: Option<String>,
    pub font: Option<String>,
    pub zakladni_vel: Option<String>,
    pub odstavec: Option<String>,
    pub okraj_vlevo: Option<u32>,
    pub okraj_vpravo: Option<u32>,
    pub okraj_nahore: Option<u32>,
    pub okraj_dole: Option<u32>,
    pub zahlaví: Option<String>,
    pub zapati: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Cesty {
    pub obrazky: Option<PathBuf>,
    pub hyphenation: Option<PathBuf>,
}

#[derive(Debug, Deserialize)]
pub struct Styl {
    pub styl: Option<String>,
}

impl Metadata {
    pub fn load(path: &Path) -> Result<Self, Error> {
        let content = fs::read_to_string(path)?;
        let meta: Metadata = toml::from_str(&content)?;
        Ok(meta)
    }
}
