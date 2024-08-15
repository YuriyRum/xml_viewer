use rs_web_component::Component;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlElement, ShadowRoot, ShadowRootInit, ShadowRootMode};

use crate::converter::XmlConverter;

/// Implements a new custom element for the XML view
pub struct XmlView {
    root: Option<ShadowRoot>,
    this: Option<HtmlElement>,
    attribute_name: String,
}

impl Component for XmlView {
    fn init(&mut self, this: HtmlElement) {
        self.this = Some(this);
    }

    fn observed_attributes(&self) -> Vec<String> {
        vec![self.attribute_name.clone()]
    }

    fn attribute_changed_callback(&self, _name: String, _old_value: JsValue, _new_value: JsValue) {
        if let Some(_) = self.root {
            if _old_value != _new_value {
                self.get_root().set_inner_html(self.render().as_str())
            }
        }
    }

    fn connected_callback(&mut self) {
        self.root = Some(
            self.get_this()
                .attach_shadow(&ShadowRootInit::new(ShadowRootMode::Open))
                .unwrap(),
        );
        self.get_root().set_inner_html(self.render().as_str())
    }

    fn disconnected_callback(&self) {}
}

impl XmlView {
    /// Creates a new instance of the web component with the default attribute "source"
    pub fn new() -> Self {
        XmlView {
            root: None,
            this: None,
            attribute_name: "source".to_string(),
        }
    }

    fn render(&self) -> String {
        let mut source = if let Some(value) = self.get_this().get_attribute(&self.attribute_name) {
            value
        } else {
            String::from("<div class='xml-error'>XML string is empty</div>")
        };
        if source.is_empty() {
            source.push_str("Preparing...");
        } else {
            let conv = XmlConverter::new();
            source = conv.to_html(source);
        };
        r#"
            <style>
                .xml-node {
                    color: rgb(136, 18, 128);
                }
                .xml-expand {
                    opacity: 0;
                    pointer-events: none;
                    font-size:.6rem;
                }
                .xml-node:has(.xml-node) > .xml-expand {
                    opacity: 1;
                    pointer-events: auto;
                    color: rgb(149, 156, 166);                    
                    cursor:pointer
                }
                .xml-error {
                    display: flex;
                    justify-content: center;
                    align-items: center;
                    font-size: 1.5rem
                }
                .xml-container {
                    width: 100%;
                    height: 100%;
                }
                .xml-attr-prefix {
                    color: rgb(153, 69, 0);
                }
                .xml-attr-value {
                    color: rgb(26, 26, 166);
                }
                .xml-text-value {
                    color: rgb(0, 0, 0);
                }
            </style>
            <div class="xml-container">
                {source}
            </div>
        "#
        .replace("{source}", source.as_str())
        .to_string()
    }

    fn get_root(&self) -> &ShadowRoot {
        return match &self.root {
            Some(root) => &root,
            None => panic!("not a root!"),
        };
    }

    fn get_this(&self) -> &HtmlElement {
        match &self.this {
            Some(val) => val,
            None => panic!("not an HtmlElement"),
        }
    }
}
