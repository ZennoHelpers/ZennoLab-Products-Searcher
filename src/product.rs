use std::fmt::{Display, Formatter};
use std::path::PathBuf;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ProductType {
    ZennoPoster,
    ZennoProjectMaker,
    ZennoDroid,
    ZennoBox,
    ProxyChecker,
    CapMonster,
}

impl ProductType {
    pub fn executable_names(&self) -> &'static [&'static str] {
        match self {
            ProductType::ZennoPoster => &["ProjectMaker", "ZennoPoster"],
            ProductType::ZennoProjectMaker => &["ProjectMaker", "ProjectMakerZD"],
            ProductType::ZennoDroid => &["ProjectMakerZD", "ZennoDroid"],
            ProductType::ZennoBox => &["ZennoBox"],
            ProductType::ProxyChecker => &["ProxyChecker"],
            ProductType::CapMonster => &["CapMonster", "CapMonsterMCS", "LicenseHelper"],
        }
    }
}

impl Display for ProductType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ProductType::ZennoPoster => write!(f, "ZennoPoster"),
            ProductType::ZennoProjectMaker => write!(f, "ZennoProjectMaker"),
            ProductType::ZennoDroid => write!(f, "ZennoDroid"),
            ProductType::ZennoBox => write!(f, "ZennoBox"),
            ProductType::ProxyChecker => write!(f, "ProxyChecker"),
            ProductType::CapMonster => write!(f, "CapMonster"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ZennoLabProduct {
    pub product_type: ProductType,
    pub name: String,
    pub version: String,
    pub language: String,
    pub install_path: PathBuf,
}

impl ZennoLabProduct {
    pub fn new(
        product_type: ProductType,
        name: String,
        version: String,
        language: String,
        install_path: PathBuf,
    ) -> Self {
        Self {
            product_type,
            name,
            version,
            language,
            install_path,
        }
    }

    pub fn executable_names(&self) -> &'static [&'static str] {
        self.product_type.executable_names()
    }
}

impl Display for ZennoLabProduct {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.name,
            self.version,
            self.language
        )
    }
}