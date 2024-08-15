The project creates an interactive HTML page from an XML string. 

The usage is quite simple (Hope sope 😀). 
The function 'create_component' creates a custom component with a specific name and the attribute 'source'. 
If an xml string is passed to the 'source' attribute, the component renders an interactive xml tree on a page.

TODOs: 
    - add an example without custom component
    - add documentation    

Basic example:
```
use wasm_bindgen::prelude::*;
use xml_viewer::create_component;

#[wasm_bindgen(start)]
fn run() {
    create_component("xml-view".to_string());
}

```

Examples:
- [Basic example](https://github.com/YuriyRum/xml_viewer/tree/master/examples/basic_example)