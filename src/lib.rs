mod component;
mod converter;

use component::XmlView;
use converter::XmlConverter;
use rs_web_component::{define_element, Component};

/// Create a cutom element with the given name and default attribute 'source'
///
/// # Arguments
///
/// * `name` - name of the component. The component name must follow requirements: https://html.spec.whatwg.org/multipage/custom-elements.html#valid-custom-element-name
pub fn create_component(name: String) {
    define_element(name, || -> Box<dyn Component> { Box::new(XmlView::new()) });
}

/// Creates HTML string from given XML string
///
/// # Arguments
///
/// * `xml_source` - input XML string
pub fn convert_to_html(xml_source: String) -> String {
    let xml_converter = XmlConverter::new();
    xml_converter.to_html(xml_source)
}
