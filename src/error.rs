use thiserror::Error;

pub type Result<T> = std::result::Result<T, ZennoLabError>;

#[derive(Error, Debug)]
pub enum ZennoLabError {
    #[error("Registry error: {0}")]
    Registry(#[from] windows_registry::Error),
    
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
}