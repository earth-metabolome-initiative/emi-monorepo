use std::io::Write;
use quote::quote;

/// Add a `fn main() {}` to the end of a file.
pub fn add_main_to_file(file_path: &str) {
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .open(file_path)
        .unwrap();

    let main = quote! {
        fn main() {}
    };

    file.write_all(main.to_string().as_bytes()).unwrap();
    file.flush().unwrap();
    file.sync_all().unwrap();
}