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
use wasm_bindgen::JsCast; //must be for dyn_into
use web_sys::{console};
use conv::{ConvUtil};
//use conv::{ConvAsUtil};
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
                    <div>
                        <p>
                            {vec![text(
                                bumpalo::format!(in bump, "{}",
                                "This webapp saves data only in the local storage of this device. \
                                No data is ever sent over the network. \
                                This webapp works in all modern browsers (chrome, firefox, safari). \
                                Avoid old or non updated browsers (for your own security and comfort).")
                                .into_bump_str()
                            )]}
                        </p>
                    </div>
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
                    <textarea style="height: 250px;" readonly="true" class="w3-input w3-dark-grey w3-border-0 w3-round" name="json_result" id="json_result" >
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
                                "The result of the webapp is a simple json text. \
                                It can be easily saved, copied, modified and sent (by email, WhatsApp, messanger,...). \
                                But all this actions are outside of this webapp for security and privacy reasons.
                                ")
                                .into_bump_str()
                            )]}
                        </p>
                    </div>
                    <div>
                        <button class="w3-button w3-block w3-round w3-green w3-text-bold w3-text-color-black"
                            onclick={ move |root, vdom_weak, event| {
                                copy_to_clipboard();
                            }}>
                        {vec![text(
                            bumpalo::format!(in bump, "{}",
                            "Copy json_result to clipboard")
                            .into_bump_str()
                        )]}
                        </button>
                    </div>
                    <div>
                        <h6 class="yellow">
                            {vec![text(bumpalo::format!(in bump, "Version: {}", version).into_bump_str(),)]}
                        </h6>
                    </div>
                    <div>
                        <h6 class="yellow">
                        {vec![text(bumpalo::format!(in bump, "Instructions and source code:{}", "").into_bump_str(),)]}
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
#[allow(clippy::integer_arithmetic)]
#[allow(clippy::shadow_reuse)]
#[allow(clippy::nonminimal_bool)]
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
        // height of columns: 90% for 1st column, 100% second, 100% third, 30% fourth
        let line_1_proc = unwrap!(len_map.approx_as::<f64>()) / (90.0 + 100.0 + 100.0 + 30.0);
        let first_line_len = line_1_proc * 90.0;
        let second_line_len = line_1_proc * 100.0;
        let third_line_len = line_1_proc * 100.0;
        //let fourth_line_len = line_1_proc * 30.0;
        #[allow(clippy::shadow_reuse)]
        let first_line_len = unwrap!(first_line_len.approx_as::<usize>());
        let second_line_len = unwrap!(second_line_len.approx_as::<usize>());
        let third_line_len = unwrap!(third_line_len.approx_as::<usize>());
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

                let str_caption =
                    unwrap!(val.get("caption").unwrap_or(&json!(key)).as_str()).to_string();
                let caption = bumpalo::format!(in bump, "{}",str_caption).into_bump_str();
                let str_value =
                    unwrap!(val.get("value").unwrap_or(&json!("")).as_str()).to_string();
                let value = bumpalo::format!(in bump, "{}",str_value).into_bump_str();
                let str_ctrl_type =
                    unwrap!(val.get("ctrl_type").unwrap_or(&json!("text")).as_str()).to_string();
                let ctrl_type = bumpalo::format!(in bump, "{}",str_ctrl_type).into_bump_str();

                if ctrl_type == "label" {
                    vec_node.push(dodrio!(bump,
                    <div >
                        <h3 class="w3-center w3-text-yellow"
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
                                // get the ctrl (target)
                                let ctrl = match event
                                    .target()
                                    .and_then(|t| t.dyn_into::<web_sys::HtmlSelectElement>().ok())
                                {
                                    None => return,
                                    //?? Don't understand what this does. The original was written for Input element.
                                    Some(input) => input,
                                };
                             on_input(root, vdom_weak, ctrl.name().to_string(),ctrl.value().to_string());
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
                            // get the ctrl (target)
                            let ctrl = match event
                                .target()
                                .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
                            {
                                None => return,
                                //?? Don't understand what this does. The original was written for Input element.
                                Some(input) => input,
                            };
                            on_input(root, vdom_weak, ctrl.name().to_string(),ctrl.value().to_string());
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

/// event on input for ctrl input and select
pub fn on_input(
    root: &mut (dyn dodrio::RootRender + 'static),
    vdom_weak: dodrio::VdomWeak,
    ctrl_name: String,
    ctrl_value: String,
) {
    logmod::debug_write(&format!("select_on_input{}", ""));
    //save on every key stroke
    let rrc = root.unwrap_mut::<RootRenderingComponent>();
    let v2 = vdom_weak;

    let map = unwrap!(rrc.json_format.get_mut(ctrl_name.as_str()));
    logmod::debug_write(&format!("map from json {:?}", map));
    map["value"] = json!(ctrl_value);

    rrc.json_result[ctrl_name.as_str()] = json!(ctrl_value);

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
        let value = bumpalo::format!(in bump, "{}",unwrap!(x.get("option")
        .unwrap_or(&json!(""))
        .as_str()
        ).to_string())
        .into_bump_str();
        let caption = {
            vec![text(
                bumpalo::format!(in bump, "{}",unwrap!(x.get("caption")
                .unwrap_or(&json!(""))
                .as_str()
                ).to_string())
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

///copy to clipboard
fn copy_to_clipboard() {
    let window = unwrap!(web_sys::window());
    let document = unwrap!(window.document());

    let t = unwrap!(document.get_element_by_id("json_result"));
    let el = unwrap!(t.dyn_into::<web_sys::HtmlTextAreaElement>());
    //and again Safari is the problem
    if is_iphone() {
        let range = unwrap!(document.create_range());
        let _x = range.select_node_contents(&el);
        let selection = unwrap!(unwrap!(window.get_selection()));
        let _x = selection.remove_all_ranges();
        let _x = selection.add_range(&range);
        el.set_selection_range(0, 999_999);
    } else {
        el.select();
    }

    let hd = unwrap!(document.dyn_into::<web_sys::HtmlDocument>());
    let _x = hd.exec_command("copy");
}

///detect iphone
pub fn is_iphone() -> bool {
    let window = unwrap!(web_sys::window());
    let navigator = window.navigator();
    let user_agent = unwrap!(navigator.user_agent());
    user_agent.to_ascii_lowercase().contains("iphone")
}
