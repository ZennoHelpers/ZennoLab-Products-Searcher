use thiserror::Error;
use std::io;

pub type Result<T> = std::result::Result<T, ZennoLabError>;

#[derive(Error, Debug)]
pub enum ZennoLabError {
    #[error("Registry access error: {0}")]
    Registry(#[from] io::Error),
    
    #[error("Unsupported product: {name} {version} {language}")]
    UnsupportedProduct {
        name: String,
        version: String,
        language: String,
    },
    
    #[error("Product not fully installed: {name} {version} {language}")]
    IncompleteInstallation {
        name: String,
        version: String,
        language: String,
    },
    
    #[error("Installation directory not found for product: {name} {version} {language}")]
    InstallDirNotFound {
        name: String,
        version: String,
        language: String,
    },
    
    #[error("No ZennoLab products found")]
    NoProductsFound,
    
    #[error("Invalid language code: {code}")]
    InvalidLanguageCode { code: String },
    
    #[error("Registry key not found: {key}")]
    RegistryKeyNotFound { key: String },
    
    #[error("Registry value not found: {value}")]
    RegistryValueNotFound { value: String },
}