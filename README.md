The project creates an interactive HTML page from an XML string. 

The usage is quite simple (Hope sope 😀). 
The function 'create_component' creates a custom component with a specific name and the attribute 'source'. 
If an xml string is passed to the 'source' attribute, the component renders an interactive xml tree on a page.

v0.1.1
- Example of usage without custom component is added


Basic example:
```
use wasm_bindgen::prelude::*;
use xml_viewer::create_component;

#[wasm_bindgen(start)]
fn run() {
    create_component("xml-view".to_string());
}

```

Example without custom component:

```
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn convert_to_html(xml_source: String) -> String {
    xml_viewer::convert_to_html(xml_source)
}
```

Examples:
- [Basic example](https://github.com/YuriyRum/xml_viewer/tree/master/examples/basic_example)
- [Example without custom component](https://github.com/YuriyRum/xml_viewer/tree/master/examples/example_without_custom_component)