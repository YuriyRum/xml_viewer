use wasm_bindgen::prelude::*;
use xml_viewer::create_component;

#[wasm_bindgen(start)]
fn run() {
    create_component("xml-view".to_string());
}
