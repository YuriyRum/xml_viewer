mod component;
mod converter;

use component::XmlView;
use rs_web_component::{define_element, Component};

/// Create a cutom element with the given name and default attribute 'source'
/// #Arguments
///
/// * `name` - name of the component. The component name must follow requirements: https://html.spec.whatwg.org/multipage/custom-elements.html#valid-custom-element-name
pub fn create_component(name: String) {
    define_element(name, || -> Box<dyn Component> { Box::new(XmlView::new()) });
}
