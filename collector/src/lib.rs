extern crate proc_macro;
use quote::ToTokens;
use proc_macro::TokenStream;

use std::{env, fs::{read_dir, read_to_string}, path::{Path, PathBuf}};

use syn::parse_file;
use quote::{format_ident, quote};

#[proc_macro]
pub fn collect_modules(_: TokenStream) -> TokenStream {
    let collection_directory = collection_directory();

    let mod_names = get_modules_in_directory(&collection_directory)
        .into_iter()
        .map(|module_name| format_ident!("{}", module_name));

    let functions = get_modules_in_directory(&collection_directory)
        .into_iter()
        .map(|module_name| pub_functions_in_module(&module_name));

    (quote! {
        #( mod #mod_names; )*

        fn collector() -> Vec<Box<dyn Display>> {
            let mut things_to_print: Vec<Box<dyn Display>> = vec![];

            #(#functions)*

            things_to_print
        }
    }).into()
}

fn collection_directory() -> PathBuf {
    let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    dir.push("..");
    dir.push("printer");
    dir.push("src");

    dir
}

fn get_modules_in_directory(dir: &Path) -> Vec<String> {
    let modules: Vec<String> = read_dir(&dir)
        .expect("failed to read directory")
        .map(|dir_entry_res| dir_entry_res.unwrap())
        .map(|dir_entry| dir_entry.file_name().to_string_lossy().into_owned())
        .filter(|file_name| !file_name.starts_with('.'))
        .filter(|file_name| file_name.ends_with(".rs"))
        .map(|file_name| file_name.trim_end_matches(".rs").to_owned())
        .filter(|module_name| module_name != "main")
        .collect();

    modules
}

struct Module {
    module_name: syn::Ident,
    functions: Vec<syn::Ident>,
}

impl ToTokens for Module {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Module { module_name, functions } = self;
        
        tokens.extend(
            quote!{
                #(things_to_print.push(Box::new(#module_name::#functions()));)*
            }
        );
    }
}

fn file_path_for_module(module: &str) -> PathBuf {
    let mut path = collection_directory();
    path.push(module);
    path.set_extension("rs");

    path
}

fn pub_functions_in_module(module: &str) -> Module {
    let file_path_to_module = file_path_for_module(module);
    let file_contents = read_to_string(file_path_to_module).expect("failed to read file");
    let parsed_file: syn::File = parse_file(&file_contents).expect("failed to parse file");

    let mut functions = vec![];

    for item in parsed_file.items {
        if let syn::Item::Fn(
            syn::ItemFn {
                vis: syn::Visibility::Public(_),
                sig: syn::Signature { ident, .. },
                ..
            }
        ) = item {
            functions.push(ident);
        }
    }

    Module {
        module_name: format_ident!("{}", module),
        functions,
    }
}
