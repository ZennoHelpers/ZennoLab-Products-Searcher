use crate::{
    error::{Result, ZennoLabError},
    product::ZennoLabProduct,
    product_detector::ProductDetector,
};
use log::{debug, info, warn};
use std::path::PathBuf;
use windows_registry::CURRENT_USER;

pub struct ZennoLabSearcher {}

impl ZennoLabSearcher {
    const ZL_REGISTRY_PATH: &str = r"Software\ZennoLab";

    pub fn search_products(&self) -> Result<Vec<ZennoLabProduct>> {
        info!("Starting search for ZennoLab products...");
        
        let mut products = Vec::new();
        
        let zenno_key = CURRENT_USER
            .open(Self::ZL_REGISTRY_PATH)
            .map_err(ZennoLabError::Registry)?;

        for language_code in zenno_key.keys().map_err(ZennoLabError::Registry)? {
            if !ProductDetector::is_valid_language_code(&language_code) {
                debug!("Skipping invalid language code: {}", language_code);
                continue;
            }

            match self.search_products_for_language(&zenno_key, &language_code) {
                Ok(mut lang_products) => {
                    info!("Found {} products for language '{}'", lang_products.len(), language_code);
                    products.append(&mut lang_products);
                }
                Err(e) => {
                    warn!("Failed to search products for language '{}': {}", language_code, e);
                }
            }
        }

        if products.is_empty() {
            return Err(ZennoLabError::NoProductsFound);
        }

        info!("Found {} total products", products.len());
        Ok(products)
    }

    fn search_products_for_language(
        &self,
        zenno_key: &windows_registry::Key,
        language_code: &str,
    ) -> Result<Vec<ZennoLabProduct>> {
        let mut products = Vec::new();
        
        let lang_key = zenno_key
            .open(language_code)
            .map_err(ZennoLabError::Registry)?;

        for product_name in lang_key.keys().map_err(ZennoLabError::Registry)? {
            match self.search_product_versions(&lang_key, &product_name, language_code) {
                Ok(mut product_versions) => {
                    products.append(&mut product_versions);
                }
                Err(e) => {
                    warn!("Failed to search versions for product '{}': {}", product_name, e);
                }
            }
        }

        Ok(products)
    }

    fn search_product_versions(
        &self,
        lang_key: &windows_registry::Key,
        product_name: &str,
        language_code: &str,
    ) -> Result<Vec<ZennoLabProduct>> {
        let mut products = Vec::new();
        
        let product_key = lang_key
            .open(product_name)
            .map_err(ZennoLabError::Registry)?;

        for version in product_key.keys().map_err(ZennoLabError::Registry)? {
            match self.create_product(&product_key, product_name, &version, language_code) {
                Ok(product) => {
                    info!("Found product: {}", product);
                    products.push(product);
                }
                Err(e) => {
                    warn!("Failed to create product '{}' version '{}': {}", product_name, version, e);
                }
            }
        }

        Ok(products)
    }

    fn create_product(
        &self,
        product_key: &windows_registry::Key,
        product_name: &str,
        version: &str,
        language_code: &str,
    ) -> Result<ZennoLabProduct> {
        let version_key = product_key
            .open(version)
            .map_err(ZennoLabError::Registry)?;

        // Check installation status
        let is_fully_installed = version_key
            .get_string("SuccessInstall")
            .map(|status| status == "True")
            .unwrap_or(false);

        if !is_fully_installed {
            debug!("Product not fully installed: {} {} {}", product_name, version, language_code);
        }

        // Get installation directory
        let install_dir = version_key
            .get_string("InstallDir")
            .map_err(|_| ZennoLabError::InstallDirNotFound {
                name: product_name.to_string(),
                version: version.to_string(),
                language: language_code.to_string(),
            })?;

        // Detect product type
        let product_type = ProductDetector::detect_product_type(product_name, version)?;

        Ok(ZennoLabProduct::new(
            product_type,
            product_name.to_string(),
            version.to_string(),
            language_code.to_string(),
            PathBuf::from(install_dir),
            is_fully_installed,
        ))
    }
}

pub fn search_zennolab_products() -> Result<Vec<ZennoLabProduct>> {
    ZennoLabSearcher::new().search_products()
}