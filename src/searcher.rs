use crate::{
    error::{Result, ZennoLabError},
    product::ZennoLabProduct,
    product_detector::ProductDetector,
};
use log::{debug, info, warn};
use std::path::PathBuf;
use windows_registry::CURRENT_USER;

const ZL_REGISTRY_PATH: &str = r"Software\ZennoLab";

pub fn search_products() -> Result<Vec<ZennoLabProduct>> {
    info!("Starting search for ZennoLab products...");
    
    let mut products = Vec::new();
    
    let zenno_key = match CURRENT_USER.open(ZL_REGISTRY_PATH) {
        Ok(key) => key,
        Err(_) => {
            return Err(ZennoLabError::RegistryKeyNotFound {
                key: ZL_REGISTRY_PATH.to_owned(),
            });
        }
    };

    let language_keys = match zenno_key.keys() {
        Ok(keys) => keys,
        Err(e) => return Err(ZennoLabError::Registry(e.into())),
    };

    for language_code in language_keys {
        if !ProductDetector::is_valid_language_code(&language_code) {
            debug!("Skipping invalid language code: {}", language_code);
            continue;
        }

        match search_products_for_language(&zenno_key, &language_code) {
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
    zenno_key: &windows_registry::Key,
    language_code: &str,
) -> Result<Vec<ZennoLabProduct>> {
    let mut products = Vec::new();
    
    let lang_key = match zenno_key.open(language_code) {
        Ok(key) => key,
        Err(_) => {
            return Err(ZennoLabError::RegistryKeyNotFound {
                key: format!("{}/{}", ZL_REGISTRY_PATH, language_code),
            });
        }
    };

    let product_names = match lang_key.keys() {
        Ok(names) => names,
        Err(e) => return Err(ZennoLabError::Registry(e.into())),
    };

    for product_name in product_names {
        match search_product_versions(&lang_key, &product_name, language_code) {
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
    lang_key: &windows_registry::Key,
    product_name: &str,
    language_code: &str,
) -> Result<Vec<ZennoLabProduct>> {
    let mut products = Vec::new();
    
    let product_key = match lang_key.open(product_name) {
        Ok(key) => key,
        Err(_) => {
            return Err(ZennoLabError::RegistryKeyNotFound {
                key: format!("{}/{}/{}", ZL_REGISTRY_PATH, language_code, product_name),
            });
        }
    };

    let versions = match product_key.keys() {
        Ok(versions) => versions,
        Err(e) => return Err(ZennoLabError::Registry(e.into())),
    };

    for version in versions {
        match create_product(&product_key, product_name, &version, language_code) {
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
    product_key: &windows_registry::Key,
    product_name: &str,
    version: &str,
    language_code: &str,
) -> Result<ZennoLabProduct> {
    let version_key = match product_key.open(version) {
        Ok(key) => key,
        Err(_) => {
            return Err(ZennoLabError::RegistryKeyNotFound {
                key: format!("{}/{}/{}/{}", ZL_REGISTRY_PATH, language_code, product_name, version),
            });
        }
    };

    // Check installation status
    let is_fully_installed = version_key
        .get_string("SuccessInstall")
        .map(|status| status == "True")
        .unwrap_or(false);

    // Skip products that are not fully installed
    if !is_fully_installed {
        debug!("Product not fully installed, skipping: {} {} {}", product_name, version, language_code);
        return Err(ZennoLabError::IncompleteInstallation {
            name: product_name.to_string(),
            version: version.to_string(),
            language: language_code.to_string(),
        });
    }

    // Get installation directory
    let install_dir = match version_key.get_string("InstallDir") {
        Ok(dir) => dir,
        Err(_) => {
            return Err(ZennoLabError::InstallDirNotFound {
                name: product_name.to_string(),
                version: version.to_string(),
                language: language_code.to_string(),
            });
        }
    };

    // Detect product type
    let product_type = ProductDetector::detect_product_type(product_name, version)?;

    Ok(ZennoLabProduct::new(
        product_type,
        product_name.to_string(),
        version.to_string(),
        language_code.to_string(),
        PathBuf::from(install_dir),
    ))
}