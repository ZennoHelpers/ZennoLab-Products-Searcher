use std::fmt::{Display, Formatter};
use std::path::PathBuf;

pub struct ZennoLabProduct<'a> {
    pub(crate) name: String,
    pub(crate) ver: String,
    pub(crate) lang: String,
    pub(crate) install_path: PathBuf,
    pub(crate) exe_names: &'a [&'a str],
}

impl<'a> ZennoLabProduct<'a> {
    pub(crate) fn new(name: String, ver: String, lang: String, install_path: String, exe_names: &'a [&'a str]) -> Self {
        Self {
            name,
            ver,
            lang,
            install_path: PathBuf::from(install_path),
            exe_names,
        }
    }

    pub fn name(&'a self) -> &'a str {
        &self.name
    }

    pub fn ver(&'a self) -> &'a str {
        &self.ver
    }

    pub fn lang(&'a self) -> &'a str {
        &self.lang
    }

    pub fn install_path(&'a self) -> &'a PathBuf {
        &self.install_path
    }

    pub fn exe_names(&'a self) -> &'a [&'a str] {
        &self.exe_names
    }
}

impl Display for ZennoLabProduct<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.name, self.ver, self.lang)
    }
}