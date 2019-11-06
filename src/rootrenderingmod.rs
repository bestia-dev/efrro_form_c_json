//! **rootrenderingmod - Root Rendering Component is the base struct for Dodrio Virtual Dom**  
//! rrc contains all the data needed for UI and reacts to all the events of the UI.  
//! Other subcomponents can be more or less smart, with or without their own data,
//! but that data are only copied values from the rrc component.  

#![allow(clippy::needless_pass_by_value)]

//region use
use crate::logmod;

//use std::collections::HashMap;
use indexmap::IndexMap;
use dodrio::builder::text;
use dodrio::bumpalo::{self, Bump};
use dodrio::{Node, Render};
use typed_html::dodrio;
use serde::{Deserialize, Serialize};
use serde_json::json;
use web_sys::{console};
//use wasm_bindgen::JsCast; //must be for dyn_into
//endregion

///the struct with the only mutable data and the code for rendering it as html
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RootRenderingComponent {
    pub json_format: String,
    pub json_result: String,
}

impl RootRenderingComponent {
    ///constructor
    pub fn new() -> RootRenderingComponent {
        //return
        RootRenderingComponent {
            json_format: "".to_string(),
            json_result: "".to_string(),
        }
    }
}

// The `Render` implementation. It is called for every Dodrio animation frame to render the vdom.
// It renders if the render is scheduled. If not I believe nothing happens.
impl Render for RootRenderingComponent {
    #[allow(clippy::cognitive_complexity)]
    ///render
    fn render<'a, 'bump>(&'a self, bump: &'bump Bump) -> Node<'bump>
    where
        'a: 'bump,
    {
        console::time_with_label("render");
        let version = env!("CARGO_PKG_VERSION");
        //let _start = SystemTime::now();
        //create the virtual Dom with typed-html macro dodrio!
        //https://www.w3schools.com/w3css/tryit.asp?filename=tryw3css_input_select_border
        let xdiv = dodrio!(bump,
        <div id="div_all">
             <div id="div_two_col" class="w3-row-padding" >
                <div class="w3-half">
                    <div>
                        <h1 class="yellow">
                            {vec![text(
                                bumpalo::format!(in bump, "e-frro form c guest data{}","")
                                .into_bump_str()
                            )]}
                        </h1>
                        <p>
                            {vec![text(
                                bumpalo::format!(in bump, "{}",
                                "1. write data into raw json")
                                .into_bump_str()
                            )]}
                        </p>
                    </div>
                    {div_inputs(self, bump,"first_half")}
                </div>
                <div class="w3-half">
                    {div_inputs(self, bump,"second_half")}
                    <div>
                        <p>
                            {vec![text(
                                bumpalo::format!(in bump, "{}",
                                "2. copy/paste the filled json into the chrome extension for e-frro form C.")
                                .into_bump_str()
                            )]}
                        </p>
                    </div>
                    <div>
                        <h6 class="yellow">
                            {vec![text(bumpalo::format!(in bump, "Version: {}", version).into_bump_str(),)]}
                        </h6>
                    </div>
                    <div>
                        <h6 class="yellow">
                            {vec![text(bumpalo::format!(in bump, "Github repository: {}", "").into_bump_str(),)]}
                            <a href= "https://github.com/LucianoBestia/efrro_form_c_json" target="_blank">
                                {vec![text(bumpalo::format!(in bump, "https://github.com/LucianoBestia/efrro_form_c_json{}", "").into_bump_str(),)]}
                            </a>
                        </h6>
                    </div>
                </div>
            </div>
        </div>
        );
        console::time_end_with_label("render");
        //return
        xdiv
    }
}

///render all the inputs
pub fn div_inputs<'b>(
    rrc: &RootRenderingComponent,
    bump: &'b Bump,
    what_half: &str,
) -> Vec<Node<'b>> {
    let mut vec_node = Vec::new();
    //json is an object that has a map
    if rrc.json_format != "" {
        let serde_map: IndexMap<String, serde_json::Value> =
            unwrap!(serde_json::from_str(rrc.json_format.as_str()));

        let len_map = serde_map.len();
        let middle_len = len_map / 2;
        let mut i = 0;
        //add a <div class="w3-half"> in the middle
        for (key, val) in serde_map {
            if (what_half == "first_half" && i < middle_len)
                || (what_half == "second_half" && i >= middle_len)
            {
                let str_caption = val
                    .get("caption")
                    .unwrap_or(&json!(key))
                    .as_str()
                    .unwrap()
                    .to_string();
                let caption = bumpalo::format!(in bump, "{}",str_caption).into_bump_str();
                logmod::debug_write(caption);
                let str_value = val
                    .get("value")
                    .unwrap_or(&json!(""))
                    .as_str()
                    .unwrap()
                    .to_string();;
                let value = bumpalo::format!(in bump, "{}",str_value).into_bump_str();

                vec_node.push(dodrio!(bump,
                <div >
                    <label for={caption} >
                        {vec![text(caption)]}
                    </label>
                    <input type="text" class="w3-input" name={caption} id={caption} value={value} >
                    </input>
                </div>
                ));
            }
            i += 1;
        }
    }
    //return
    vec_node
}
