use tower_lsp::lsp_types::Url;

use super::get_install_dir;

pub mod exp;
pub mod global;
pub mod stmnts;

pub fn map_import_path(uri: &Url, path: &str) -> Url {
    if path == "std" {
        let std_file = get_install_dir().join("resources/alpha034/std/main.ab");

        return Url::from_file_path(std_file).unwrap();
    }

    let path = uri.to_file_path().unwrap().parent().unwrap().join(path);

    Url::from_file_path(path).unwrap()
}
