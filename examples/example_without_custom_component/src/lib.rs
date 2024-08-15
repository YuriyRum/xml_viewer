use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn convert_to_html(xml_source: String) -> String {
    xml_viewer::convert_to_html(xml_source)
}
