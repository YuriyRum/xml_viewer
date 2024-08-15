use std::collections::VecDeque;
use wasm_bindgen::JsValue;
use web_sys::js_sys::Function;
use xml::{Parser, Event};

fn create_handler_function() {
    let _ = Function::new_no_args("        
        if(window._handleXMLNodeExpand) {
            return;
        }
        window._handleXMLNodeExpand = function(that) {            
            const opened = '&#9660;';
            const closed = '&#9658;';

            that.classList.toggle('xml-expand--is_closed');
            const isClosed = that.classList.contains('xml-expand--is_closed');

            that.innerHTML = isClosed ? closed : opened;
            
            isClosed 
                ? Array.from(that.parentElement.querySelectorAll('div')).forEach(item => item.style.display = 'none') 
                : Array.from(that.parentElement.querySelectorAll('div')).forEach(item => item.style.display = 'block') ;
        }
    ").call0(&JsValue::default());
}

/// Represents an XML converter
pub struct XmlConverter {    
    style: Option<String>
}

impl XmlConverter {
        /// Creates an XML converter.
        ///
        /// # Examples
        ///
        /// ```        
        /// use xml_viewer::XmlConverter;
        /// let converter = XmlConverter::new();
        /// ```
        pub fn new() -> Self {
            XmlConverter {
                style: Some("
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
                ".to_string()) 
            }
        }
        /// Creates an XML converter with a given style as a string.
        ///
        /// # Examples
        ///
        /// ```        
        /// use xml_viewer::XmlConverter;
        /// let converter = XmlConverter::with_style(
        ///    "
        ///                    .xml-node {
        ///                         color: rgb(136, 18, 128); 
        ///                     }
        ///                     .xml-container {
        ///                         width: 100%;
        ///                         height: 100%;
        ///                     }
        ///                     .xml-attr-prefix {
        ///                         color: rgb(153, 69, 0);
        ///                     }
        ///                     .xml-attr-value {
        ///                         color: rgb(26, 26, 166);
        ///                     }
        ///                     .xml-text-value {
        ///                         color: rgb(0, 0, 0);
        ///                     } 
        /// " 
        /// );
        /// ```
        #[allow(dead_code)]
        pub fn with_style(style: String) -> Self {
            XmlConverter {
                style: Some(style),
            }
        }

        /// Converts output HTML string
        pub fn to_html(&self, source: String) -> String {
            create_handler_function();

            let mut p = Parser::new();
            
            let mut buffer = String::new();
            let mut margin: f32;
           
            p.feed_str(&source);
            
            let mut ids = VecDeque::<String>::new();
            for event in p {   
                let e = match event {
                    Ok(e) => e,
                    Err(_) => {
                        buffer.push_str("<div>XML parser error</div>");
                        break;
                    },
                };
                
                match e {
                    Event::ElementStart(tag) => {
                        let id = format!("{}", ids.len().to_string());
                        ids.push_front(id.clone());
                        margin = ids.len() as f32 * 10.0;

                        let mut attr_string: String = String::new();
                        for pair in tag.attributes.into_iter() {                            
                            let ns_prefix = match pair.0.1 {
                                Some(val) => format!("{}:", val),
                                None => "".to_string(),
                            };
                            attr_string.push_str(format!(
                                "<span class='xml-attr-prefix'>{}{}</span>=<span class='xml-attr-value'>\"{}\"</span>",
                                ns_prefix, pair.0.0, 
                                pair.1).as_str()
                            );
                        }
                        let ns: String = match tag.prefix {
                            Some(val) => format!("{}:", val),
                            None => "".to_string(),
                        };
                        
                        buffer.push_str(
                            &r#"
                                    <div class="xml-node" data-id={id} style="margin-left:{margin}px;">
                                        <a  class="xml-expand" onclick=window._handleXMLNodeExpand(this)>&#9660;</a>
                                        <span>&lt;{ns}{name}</span> {attrs}<span>&gt;</span>"#.replace("{id}", &id.to_string())
                                .replace("{margin}", &margin.to_string())
                                .replace("{ns}", &ns)
                                .replace("{name}", &tag.name) 
                                .replace("{attrs}", &attr_string)
                            
                        );
                    },
                    Event::ElementEnd(tag) => {                        
                        let _ = ids.pop_front();                        
                        let ns: String = match tag.prefix {
                            Some(val) => format!("{}:", val),
                            None => "".to_string(),
                        };
                        buffer.push_str(
                            &format!(r#"<span>&lt;/{}{}&gt;</span></div>"#,                                 
                                ns,
                                tag.name
                            )
                        );
                    },
                    Event::Characters(text) => {                        
                        buffer.push_str(
                            &format!(                                
                                r#"<span class="xml-text-value">{}</span>"#,                                
                                &text,
                            )
                        );
                    },
                    _ => ()
                }
            }
            if let Some(style) = &self.style {
                buffer.insert_str(0, format!("<style>{}</style>", style).as_str());
            }
            buffer
        }

}