//! # ZennoLab Products Searcher
//! 
//! ## Example
//! 
//! ```rust
//! use zennolab_products_searcher::{search_zennolab_products, ZennoLabProduct};
//! 
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let products = search_zennolab_products()?;
//!     
//!     for product in products {
//!         println!("Found: {}", product);
//!         println!("  Path: {}", product.install_path.display());
//!         println!("  Executables: {:?}", product.executable_names());
//!         println!("  Accessible: {}", product.is_accessible());
//!     }
//!     
//!     Ok(())
//! }
//! ```

pub mod error;
pub mod product;
pub mod product_detector;
pub mod searcher;

// Re-export main types for convenience
pub use error::{Result, ZennoLabError};
pub use product::{ProductType, ZennoLabProduct};
pub use searcher::{search_zennolab_products, ZennoLabSearcher};