//! **rootrenderingmod - Root Rendering Component is the base struct for Dodrio Virtual Dom**  
//! rrc contains all the data needed for UI and reacts to all the events of the UI.  
//! Other subcomponents can be more or less smart, with or without their own data,
//! but that data are only copied values from the rrc component.  

#![allow(clippy::needless_pass_by_value)]

//region use
use crate::logmod;

use std::collections::HashMap;

use dodrio::builder::text;
use dodrio::bumpalo::{self, Bump};
use dodrio::{Node, Render};
use typed_html::dodrio;
use serde::{Deserialize, Serialize};
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
            <div id="json_format">
                <label>
                    {vec![text(
                        bumpalo::format!(in bump, "{}",
                        "json_format:")
                        .into_bump_str()
                    )]}
                </label>
                <textarea id="json_format" name="json_format" class="w3-input">
                    {vec![text(
                        bumpalo::format!(in bump, "{}",
                        self.json_format)
                        .into_bump_str()
                    )]}
                </textarea>
            </div>
            {div_inputs(self, bump)}
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
        );
        console::time_end_with_label("render");
        //return
        xdiv
    }
}

///render all the inputs
pub fn div_inputs<'b>(rrc: &RootRenderingComponent, bump: &'b Bump) -> Vec<Node<'b>> {
    let mut vec_node = Vec::new();
    //json is an object that has a map
    if rrc.json_format != "" {
        let serde_map: HashMap<String, serde_json::Value> =
            unwrap!(serde_json::from_str(rrc.json_format.as_str()));

        for x in serde_map {
            let caption = bumpalo::format!(in bump, "{}",x.0).into_bump_str();
            let value = bumpalo::format!(in bump, "{}",x.1["value"]).into_bump_str();
            vec_node.push(dodrio!(bump,
            <div >
                <label for={caption} >
                    {vec![text(caption)]}
                </label>
                <input type="text" name={caption} id={caption} value={value} >
                </input>
            </div>
            ));
        }
    }
    //return
    vec_node
}
