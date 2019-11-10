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
use wasm_bindgen::JsCast; //must be for dyn_into
                          //use futures::Future;
                          //use serde_json::map::Entry;
                          //use wasm_bindgen::JsValue;
                          //endregion

///the struct with the only mutable data and the code for rendering it as html
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RootRenderingComponent {
    pub json_format: IndexMap<String, serde_json::Value>,
    pub json_result: IndexMap<String, serde_json::Value>,
}

impl RootRenderingComponent {
    ///constructor
    pub fn new() -> RootRenderingComponent {
        //return
        RootRenderingComponent {
            json_format: IndexMap::new(),
            json_result: IndexMap::new(),
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
                <div class="w3-third">
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
                    {div_inputs(self, bump,"first_third")}
                </div>
                <div class="w3-third">
                    {div_inputs(self, bump,"second_third")}
                </div>
                <div class="w3-third">
                    {div_inputs(self, bump,"third_third")}
                    <div>
                        <label for="json_result" >
                        {vec![text(
                            bumpalo::format!(in bump, "{}",
                                "json_result")
                                .into_bump_str()
                        )]}
                    </label>
                    <textarea style="height:400px" readonly="true" class="w3-input w3-dark-grey w3-border-0 w3-round" name="json_result" id="json_result" >
                        {vec![text(
                            bumpalo::format!(in bump, "{}",
                            unwrap!(serde_json::to_string_pretty(&self.json_result)))
                                .into_bump_str()
                            )]}
                    </textarea>
                    </div>
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
    if !rrc.json_format.is_empty() {
        let len_map = rrc.json_format.len();
        let third_len = len_map / 3;
        let mut i = 0;
        //add a <div class="w3-third"> in the middle
        for (key, val) in &rrc.json_format {
            if (what_half == "first_third" && i < third_len)
                || (what_half == "second_third" && i >= third_len && i < 2 * third_len)
                || (what_half == "third_third" && i >= 2 * third_len)
            {
                let ctrl_name = bumpalo::format!(in bump, "{}",&key).into_bump_str();

                let str_caption = val
                    .get("caption")
                    .unwrap_or(&json!(key))
                    .as_str()
                    .unwrap()
                    .to_string();
                let caption = bumpalo::format!(in bump, "{}",str_caption).into_bump_str();
                let str_value = val
                    .get("value")
                    .unwrap_or(&json!(""))
                    .as_str()
                    .unwrap()
                    .to_string();
                let value = bumpalo::format!(in bump, "{}",str_value).into_bump_str();
                let str_ctrl_type = val
                    .get("ctrl_type")
                    .unwrap_or(&json!("text"))
                    .as_str()
                    .unwrap()
                    .to_string();;
                let ctrl_type = bumpalo::format!(in bump, "{}",str_ctrl_type).into_bump_str();

                vec_node.push(dodrio!(bump,
                <div >
                    <label for={ctrl_name} >
                        {vec![text(caption)]}
                    </label>
                    <input type="text" class="w3-input w3-dark-grey w3-border-0 w3-round"
                    name={ctrl_name} id={ctrl_name} value={value}
                     onkeyup={ move |root, vdom_weak, event| {
                         on_key(root, vdom_weak, event);
                    }}>
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
pub fn on_key(
    root: &mut (dyn dodrio::RootRender + 'static),
    vdom_weak: dodrio::VdomWeak,
    event: web_sys::Event,
) {
    //save on every key stroke
    let rrc = root.unwrap_mut::<RootRenderingComponent>();
    // get the ctrl (target)
    let ctrl = match event
        .target()
        .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
    {
        None => return,
        //?? Don't understand what this does. The original was written for Input element.
        Some(input) => input,
    };

    let v2 = vdom_weak.clone();
    //save_to_localstorage(&v2, ctrl.name(), ctrl.value());
    //rrc.json_result = format!("{}{}{}", rrc.json_result, ctrl.name(), ctrl.value());
    //rrc.json_result["aa"]=json!("object");
    // map.entry("whatever").or_insert(Vec::new());
    logmod::debug_write(&format!("ctrl name {:?}", &ctrl.name()));

    let map = unwrap!(rrc.json_format.get_mut(ctrl.name().as_str()));
    logmod::debug_write(&format!("{:?}", map));
    map["value"] = json!(ctrl.value());

    rrc.json_result[ctrl.name().as_str()] = json!(ctrl.value());

    let window = unwrap!(web_sys::window(), "window");
    let document = unwrap!(window.document(), "document");
    let ls = unwrap!(unwrap!(window.local_storage()));
    let x = ls.set_item(
        "json_string",
        unwrap!(serde_json::to_string_pretty(&rrc.json_result)).as_str(),
    );

    v2.schedule_render();
}
