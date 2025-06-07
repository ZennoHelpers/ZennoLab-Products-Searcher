use crate::{exe_filter::check_exe_names, product::ZennoLabProduct};
use windows_registry::CURRENT_USER;

trait ResultConsumerTrait<R, E> {
    fn consume<F1: FnOnce(R), F2: FnOnce(E)>(self, f1: F1, f2: F2);
}

impl<R, E> ResultConsumerTrait<R, E> for Result<R, E> {
    #[inline]
    fn consume<F1: FnOnce(R), F2: FnOnce(E)>(self, f1: F1, f2: F2) {
        match self {
            Ok(r) => f1(r),
            Err(e) => f2(e),
        }
    }
}

pub fn products_searcher<'a>() -> Result<Vec<ZennoLabProduct<'a>>, String> {
    println!("Started searching for supported products...\n");

    let mut products = Vec::<ZennoLabProduct>::with_capacity(10);

    CURRENT_USER.open(r"Software\ZennoLab").consume(|zl_root| {
        zl_root.keys().consume(| key_it | key_it.for_each(|key| {
            if key.len() == 2 && key == key.to_uppercase() {
                let lang = key;

                zl_root.open(&lang).consume(|lang_key| lang_key.keys().consume(|prod_iter| prod_iter.for_each(|prod_name|
                    lang_key.open(&prod_name).consume(|prod_key| prod_key.keys().consume(|ver_iter| ver_iter.for_each(|ver|
                        prod_key.open(&ver).consume(|ver_key| {
                            ver_key.get_string("SuccessInstall").consume(|install| {
                                if install != "True" {
                                    println!(r"Found not fully installed product: '{prod_name} {ver} {lang}'");
                                }
                            }, |e| {
                                println!(r"Failed to get the product installation status: '{prod_name} {ver} {lang}'. Инфо: {e}");
                            });

                            ver_key.get_string("InstallDir").consume(|install_path: String| {
                                check_exe_names(&prod_name, &ver, &lang).consume(|exe_names| {
                                        let product = ZennoLabProduct::new(
                                            prod_name.clone(),
                                            ver.clone(),
                                            lang.to_owned(),
                                            install_path,
                                            exe_names
                                        );
                                        println!("Found product: '{}'", &product);
                                        products.push(product)
                                    }, |e| {
                                        println!("{}", e);
                                    });
                            }, |e| {
                                println!(
                                    r"Failed to retrieve product installation path: '{prod_name} {ver} {lang}'. Info: {e}"
                                );
                            })
                        }, |e| {
                            println!(
                                r"Failed to open product section: '{prod_name} {ver} {lang}'. Info: {e}"
                            );
                        })), |e| {
                        println!(
                            r"Error when parsing version section in product section: '{prod_name}' lang: '{lang}'. Info: '{e}'"
                        );
                    }), |e| {
                        println!(
                            r"Failed to open product section: '{prod_name}' lang: '{lang}'. Info: '{e}'"
                        );
                    })
                ), |e| {
                    println!(
                        r"Error when getting product section in language section: '{lang}'. Info: '{e}'"
                    );
                }), |e|{
                    println!(r"Failed to open the language section: '{lang}'. Info: '{e}'");
                })
            }
        }), |e|{
            println!(r"Failed to retrieve registry section в 'HKEY_CURRENT_USER\Software\ZennoLab'. Info: '{e}'");
        })
    },|e| {
        println!(r"Failed to open 'HKEY_CURRENT_USER\Software\ZennoLab'. Info: '{e}'")
    });

    if products.is_empty() {
        return Err(r"Not found any installed product.".to_string());
    }

    println!();

    Ok(products)
}