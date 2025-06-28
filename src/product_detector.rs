use crate::{error::ZennoLabError, product::ProductType};

pub struct ProductDetector;

impl ProductDetector {
    pub fn detect_product_type(name: &str, version: &str) -> Result<ProductType, ZennoLabError> {
        if name.contains("ZennoPoster") && name.contains("V7") {
            Ok(ProductType::ZennoPoster)
        } else if name.contains("ZennoProjectMaker") {
            Ok(ProductType::ZennoProjectMaker)
        } else if name.contains("ZennoDroid") {
            Ok(ProductType::ZennoDroid)
        } else if name.contains("ZennoBox") && name.contains("V7") {
            Ok(ProductType::ZennoBox)
        } else if name.contains("ProxyChecker") {
            Ok(ProductType::ProxyChecker)
        } else if name.contains("CapMonster") {
            Ok(ProductType::CapMonster)
        } else {
            Err(ZennoLabError::UnsupportedProduct {
                name: name.to_string(),
                version: version.to_string(),
                language: "unknown".to_string(),
            })
        }
    }

    pub fn is_valid_language_code(code: &str) -> bool {
        code.len() == 2 && code.chars().all(|c| c.is_ascii_uppercase())
    }
}