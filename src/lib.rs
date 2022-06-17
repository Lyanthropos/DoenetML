mod utils;

extern crate web_sys;

use wasm_bindgen::prelude::*;
use serde::Serialize;
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

use std::fmt;


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// A macro to provide println! style syntax for console.log logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

// Render Tree Component
#[derive(Serialize, Debug)]
enum RTC {
    String(String),
    Array(Vec<RTC>),
    Object(HashMap<String, RTC>),
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct DoenetCore {
    render_tree_base: RTC,
}


trait ComponentLike {
    fn name(&self) -> String;
    fn parent_name(&self) -> String;

    // fn get_parent_from_list(&self, components: &HashMap<String, Component>) -> Option<Rc<dyn ComponentLike>> {

    //     let possible_parent = components.get(&self.parent_name());
    //     match possible_parent {
    //         Some(parent) => Option::Some(parent.to_component_like()),
    //         None => Option::None,
    //     }
    // }

    fn set_parent(&self, parent_name: String);

}



#[derive(Debug)]
struct Text {
    name: String,
    value: String,
    parent: RefCell<String>,
    children: RefCell<Vec<TextChildRef>>,
}

trait TextLike: ComponentLike {
    fn text_value(&self) -> String;
}

#[derive(Debug, Clone)]
enum TextChildRef {
    String(String),
    TextLike(Rc<dyn TextLike>),
}

impl fmt::Debug for dyn TextLike {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let debug_text = format!("{}:{}", self.name(), self.text_value());
        f.write_str(&debug_text)
    }
}


impl ComponentLike for Text {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn parent_name(&self) -> String {
        self.parent.borrow().clone()
    }

    fn set_parent(&self, parent_name: String) {
        let mut parent = self.parent.borrow_mut();
        *parent = parent_name;
    }
}




#[wasm_bindgen]
impl DoenetCore {
    pub fn new(program: &str) -> DoenetCore {
        log!("core recieved the string: {}", program);

        let json_deserialized: serde_json::Value = serde_json::from_str(program).unwrap();



        


        let render_tree_base: RTC = RTC::Array(Vec::from([
            RTC::String(String::from("Duckling")),
            RTC::String(String::from("Quack")),
            RTC::Object(HashMap::from([
                (String::from("Mercury"), RTC::String(String::from("0.4"))),
                (String::from("Earth"), RTC::Array(Vec::from([
                    RTC::String(String::from("1.0")),
                    RTC::String(String::from("3.0")),
                    RTC::String(String::from("5.0")),
                ]))),
                (String::from("Mars"), RTC::String(String::from("1.5"))),
            ])),
        ]));

        log!("rust deserialized: {:?}", json_deserialized);
        log!("render_tree: {:?}", &render_tree_base);

        DoenetCore {
            render_tree_base,
        }
    }

    pub fn render_tree(&self) -> String {
        let rtb = &self.render_tree_base;
        let result = serde_json::to_string(&rtb).unwrap();

        result
    }
}