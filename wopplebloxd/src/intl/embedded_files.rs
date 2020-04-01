use std::path::Path;

#[derive(RustEmbed)]
#[folder = "wopplebloxd/translations"]
pub struct EmbeddedFiles;

pub fn get_embedded_file(path: &str) -> String {
    let content = EmbeddedFiles::get(path).unwrap();
    std::str::from_utf8(content.as_ref()).expect("Error: Failed to decode the specified embedded file's contents as utf8 (did you save as another encoding by accident?)").to_string()
}
