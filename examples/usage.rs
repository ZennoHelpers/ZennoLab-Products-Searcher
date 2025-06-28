use zennolab_products_searcher::{search_zennolab_products, ProductType, ZennoLabError};

fn main() {
    // Initialize logger to see debug information
    env_logger::init();

    match search_zennolab_products() {
        Ok(products) => {
            println!("✅ Found {} ZennoLab products:\n", products.len());
            
            for product in &products {
                println!("📦 {}", product);
                println!("   Type: {}", product.product_type);
                println!("   Path: {}", product.install_path.display());
                println!("   Executables: {:?}", product.executable_names());
                println!("   Accessible: {}", product.is_accessible());
                
                if !product.is_fully_installed {
                    println!("   ⚠️  Warning: Product installation appears incomplete");
                }
                
                println!();
            }
            
            // Group products by type
            let mut by_type = std::collections::HashMap::new();
            for product in products {
                by_type.entry(product.product_type.clone()).or_insert_with(Vec::new).push(product);
            }
            
            println!("📊 Products by type:");
            for (product_type, products) in by_type {
                println!("   {}: {} installations", product_type, products.len());
            }
        }
        Err(ZennoLabError::NoProductsFound) => {
            println!("❌ No ZennoLab products found on this system.");
            println!("   Make sure you have ZennoLab products installed.");
        }
        Err(ZennoLabError::Registry(e)) => {
            println!("❌ Registry access error: {}", e);
            println!("   This might be a permissions issue or the registry key doesn't exist.");
        }
        Err(e) => {
            println!("❌ Error: {}", e);
        }
    }
}