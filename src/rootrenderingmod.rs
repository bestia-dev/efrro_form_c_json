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
                <div class="w3-quarter">
                    {div_inputs(self, bump,"first_quarter")}
                </div>
                <div class="w3-quarter">
                    {div_inputs(self, bump,"second_quarter")}
                </div>
                <div class="w3-quarter">
                    {div_inputs(self, bump,"third_quarter")}
                </div>
                <div class="w3-quarter">
                    {div_inputs(self, bump,"fourth_quarter")}
                    <div>
                        <label for="json_result" >
                        {vec![text(
                            bumpalo::format!(in bump, "{}",
                                "json_result")
                                .into_bump_str()
                        )]}
                    </label>
                    <textarea style="height: 200px;" readonly="true" class="w3-input w3-dark-grey w3-border-0 w3-round" name="json_result" id="json_result" >
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
                                "Your data is stored only on this device locally. Copy/paste the json_result and send it to the hostel manager.")
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
                            <a href= "https://github.com/LucianoBestia/efrro_form_c_json" target="_blank">
                                {vec![text(bumpalo::format!(in bump, "https://github.com / LucianoBestia / efrro_form_c_json{}", "").into_bump_str(),)]}
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

#[allow(clippy::cognitive_complexity)]
///render all the inputs
pub fn div_inputs<'b>(
    rrc: &RootRenderingComponent,
    bump: &'b Bump,
    what_quarter: &str,
) -> Vec<Node<'b>> {
    let mut vec_node = Vec::new();
    //json is an object that has a map
    if !rrc.json_format.is_empty() {
        let len_map = rrc.json_format.len();
        //the content of the last column is smaller than the others
        // height of columns: 100% for 1st column, 100% second, 100% third, 30% fourth
        let line_1_proc = len_map as f64 / (100.0 + 100.0 + 100.0 + 30.0);
        let first_line_len = line_1_proc * 100.0;
        let second_line_len = line_1_proc * 100.0;
        let third_line_len = line_1_proc * 100.0;
        //let fourth_line_len = line_1_proc * 30.0;

        let first_line_len = first_line_len as usize;
        let second_line_len = second_line_len as usize;
        let third_line_len = third_line_len as usize;
        //let fourth_line_len = fourth_line_len as usize;

        //logmod::debug_write(format!("",lin))
        let mut i = 0;
        //add a <div class="w3-quarter"> in the middle
        for (key, val) in &rrc.json_format {
            if (what_quarter == "first_quarter" && i < first_line_len)
                || (what_quarter == "second_quarter"
                    && i >= first_line_len
                    && i < (first_line_len + second_line_len))
                || (what_quarter == "third_quarter"
                    && i >= (first_line_len + second_line_len)
                    && i < (first_line_len + second_line_len + third_line_len))
                || (what_quarter == "fourth_quarter"
                    && i >= (first_line_len + second_line_len + third_line_len))
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
                    .to_string();
                let ctrl_type = bumpalo::format!(in bump, "{}",str_ctrl_type).into_bump_str();

                if ctrl_type == "label" {
                    vec_node.push(dodrio!(bump,
                    <div >
                        <h3 class="w3-border-0 w3-round w3-yellow "
                         id={ctrl_name}
                        >
                        {vec![text(value)]}
                        </h3>
                    </div>
                    ));
                } else if ctrl_type == "select" || ctrl_type == "radio" || ctrl_type == "checkbox" {
                    vec_node.push(dodrio!(bump,
                    <div >
                        <label for={ctrl_name} >
                            {vec![text(caption)]}
                        </label>
                        <select class="w3-input w3-dark-grey w3-border-0 w3-round"
                        name={ctrl_name} id={ctrl_name}
                         oninput={ move |root, vdom_weak, event| {
                             select_on_input(root, vdom_weak, event);
                        }}>
                        {select_options(rrc,bump,val,&str_value)}
                        </select>
                    </div>
                    ));
                } else if ctrl_type == "hidden" {
                    vec_node.push(dodrio!(bump,
                    <div >
                        <input type="hidden"
                        name={ctrl_name} id={ctrl_name} value={value}>
                        </input>
                    </div>
                    ));
                } else {
                    //to je za type=text
                    vec_node.push(dodrio!(bump,
                    <div >
                        <label for={ctrl_name} >
                            {vec![text(caption)]}
                        </label>
                        <input type="text" class="w3-input w3-dark-grey w3-border-0 w3-round"
                        name={ctrl_name} id={ctrl_name} value={value}
                        oninput={ move |root, vdom_weak, event| {
                            input_on_input(root, vdom_weak, event);
                        }}>
                        </input>
                    </div>
                    ));
                }
            }
            i += 1;
        }
    }
    //return
    vec_node
}

/// event on change
pub fn input_on_input(
    root: &mut (dyn dodrio::RootRender + 'static),
    vdom_weak: dodrio::VdomWeak,
    event: web_sys::Event,
) {
    logmod::debug_write(&format!("input_on_input{}", ""));
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
    //logmod::debug_write(&format!("ctrl name {:?}", &ctrl.name()));

    let map = unwrap!(rrc.json_format.get_mut(ctrl.name().as_str()));
    logmod::debug_write(&format!("map from json {:?}", map));
    map["value"] = json!(ctrl.value());

    rrc.json_result[ctrl.name().as_str()] = json!(ctrl.value());

    let window = unwrap!(web_sys::window(), "window");
    let ls = unwrap!(unwrap!(window.local_storage()));
    let _x = ls.set_item(
        "json_string",
        unwrap!(serde_json::to_string_pretty(&rrc.json_result)).as_str(),
    );

    v2.schedule_render();
}

/// event on change
pub fn select_on_input(
    root: &mut (dyn dodrio::RootRender + 'static),
    vdom_weak: dodrio::VdomWeak,
    event: web_sys::Event,
) {
    logmod::debug_write(&format!("select_on_input{}", ""));
    //save on every key stroke
    let rrc = root.unwrap_mut::<RootRenderingComponent>();
    // get the ctrl (target)
    let ctrl = match event
        .target()
        .and_then(|t| t.dyn_into::<web_sys::HtmlSelectElement>().ok())
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
    //logmod::debug_write(&format!("ctrl name {:?}", &ctrl.name()));

    let map = unwrap!(rrc.json_format.get_mut(ctrl.name().as_str()));
    logmod::debug_write(&format!("map from json {:?}", map));
    map["value"] = json!(ctrl.value());

    rrc.json_result[ctrl.name().as_str()] = json!(ctrl.value());

    let window = unwrap!(web_sys::window(), "window");
    let ls = unwrap!(unwrap!(window.local_storage()));
    let _x = ls.set_item(
        "json_string",
        unwrap!(serde_json::to_string_pretty(&rrc.json_result)).as_str(),
    );

    v2.schedule_render();
}

/// render the option of select
pub fn select_options<'b>(
    _rrc: &RootRenderingComponent,
    bump: &'b Bump,
    val_options: &serde_json::Value,
    selected: &str,
) -> Vec<Node<'b>> {
    let mut vec_node = Vec::new();

    let default = json!(r#"[{"option": "","caption": ""},]"#);
    let str_ctrl_options = unwrap!(val_options.get("options").unwrap_or(&default).as_array());

    for x in str_ctrl_options {
        //logmod::debug_write(&format!("x {:?}", x));
        let value = bumpalo::format!(in bump, "{}",x.get("option")
        .unwrap_or(&json!(""))
        .as_str()
        .unwrap()
        .to_string())
        .into_bump_str();
        let caption = {
            vec![text(
                bumpalo::format!(in bump, "{}",x.get("caption")
                .unwrap_or(&json!(""))
                .as_str()
                .unwrap()
                .to_string())
                .into_bump_str(),
            )]
        };
        if selected == value {
            vec_node.push(dodrio!(bump,
                <option value={value} selected={true} >
                {caption}
                </option>
            ));
        } else {
            vec_node.push(dodrio!(bump,
                <option value={value}  >
                {caption}
                </option>
            ));
        }
    }
    //return
    vec_node
}
