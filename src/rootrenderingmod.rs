//! **rootrenderingmod - Root Rendering Component is the base struct for Dodrio Virtual Dom**  
//! rrc contains all the data needed for UI and reacts to all the events of the UI.  
//! Other subcomponents can be more or less smart, with or without their own data,
//! but that data are only copied values from the rrc component.  

#![allow(clippy::needless_pass_by_value)]

//region use
use crate::logmod;
use crate::fetchjsonformatmod;
use crate::unwrapmod;
use crate::fetchjsonaccommodationmod;
use crate::unwrapmod::required;
use crate::stringmod;

//use std::collections::HashMap;
use indexmap::IndexMap;
use dodrio::builder::text;
use dodrio::bumpalo::{self, Bump};
use dodrio::{Node, Render};
use typed_html::dodrio;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast; //must be for dyn_into
use web_sys::{console};
//use conv::{ConvUtil};
use urlencoding;
//use conv::{ConvAsUtil};
//use futures::Future;
//use serde_json::map::Entry;
//use wasm_bindgen::JsValue;
//endregion

///the struct with the only mutable data and the code for rendering it as html
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RootRenderingComponent {
    pub json_format: Vec<fetchjsonformatmod::CtrlFormat>,
    pub json_result: IndexMap<String, String>,
    pub accommodation_data: Option<fetchjsonaccommodationmod::AccommodationData>,
}

impl RootRenderingComponent {
    ///constructor
    pub fn new() -> RootRenderingComponent {
        //return
        RootRenderingComponent {
            json_format: Vec::new(),
            json_result: IndexMap::new(),
            accommodation_data: None,
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
        //let _start = SystemTime::now();
        //create the virtual Dom with typed-html macro dodrio!
        //https://www.w3schools.com/w3css/tryit.asp?filename=tryw3css_input_select_border
        let xdiv = dodrio!(bump,
        <div id="div_all">
             <div class="w3-row-padding" >
                <div>
                {div_img_accommodation(&self,&bump)}
                    <div>
                    <h1 class="w3-center w3-text-yellow">
                        {vec![text(bumpalo::format!(in bump, "{}",
                            "Form C guest data")
                            .into_bump_str()
                        )]}
                    </h1>
                    </div>
                    <div>
                        <p>
                            {vec![text(bumpalo::format!(in bump, "{}",
                                "Form C is compulsory in India: 'Accommodation providers to foreigners MUST \
                                submit the details of the residing foreigner in Form C to the Registration \
                                authorities within 24 hours of the arrival.'\n\
                                For security and privacy of personal data this web page saves data only in the local storage of this device. \
                                No data is ever sent over the network. \
                                This web page works in all modern mobile browsers with Webassembly/Wasm enabled (chrome, firefox, safari). \
                                Avoid old or non updated browsers (for your own security and comfort).")
                                .into_bump_str()
                            )]}
                        </p>
                    </div>
                    {div_inputs(self, bump)}
                    <div>
                    <h3 class="w3-center w3-text-yellow">
                        {vec![text(bumpalo::format!(in bump, "{}",
                            "JSON result")
                            .into_bump_str()
                        )]}
                    </h3>
                    </div>
                    <div>
                    <textarea readonly="true" class="w3-input w3-dark-grey w3-border-0 w3-round" name="json_result" id="json_result" >
                        {vec![text(
                            bumpalo::format!(in bump, "{}",
                            unwrapmod::unwrap_result_abort(serde_json::to_string_pretty(&self.json_result)))
                                .into_bump_str()
                            )]}
                    </textarea>
                    </div>
                    <div>
                        <p>
                            {vec![text(
                                bumpalo::format!(in bump, "{}",
                                "The result of the web page is a simple JSON text. \
                                It can be easily saved, copied, modified and sent (by email, WhatsApp, messanger,...). \
                                But all this actions are outside of this web page for security and privacy reasons.
                                ")
                                .into_bump_str()
                            )]}
                        </p>
                    </div>
                    {div_send_email(self,bump)}
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
                    {div_version(self,bump)}
                    <h6 class="w3-text-yellow">
                        {vec![text(bumpalo::format!(in bump, "Instructions and source code:{}", "").into_bump_str(),)]}
                        </h6>
                        <h6 class="w3-text-yellow">
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
pub fn div_inputs<'b>(rrc: &RootRenderingComponent, bump: &'b Bump) -> Vec<Node<'b>> {
    let mut vec_node = Vec::new();
    //json is an object that has a map
    if !rrc.json_format.is_empty() {
        //the content of the last column is smaller than the others
        // height of columns: 90% for 1st column, 100% second, 100% third, 30% fourth

        //logmod::debug_write(format!("",lin))
        for ctrl_format in &rrc.json_format {
            let ctrl_name = bumpalo::format!(in bump, "{}",&ctrl_format.name).into_bump_str();

            let str_caption = ctrl_format
                .caption
                .clone()
                .unwrap_or_else(|| ctrl_format.name.clone());
            let caption = bumpalo::format!(in bump, "{}",str_caption).into_bump_str();
            let value = bumpalo::format!(in bump, "{}",ctrl_format.value).into_bump_str();
            let ctrl_type = bumpalo::format!(in bump, "{}",ctrl_format.ctrl_type).into_bump_str();

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
                            let rrc = root.unwrap_mut::<RootRenderingComponent>();
                         on_input(rrc, vdom_weak, ctrl.name().to_string(),ctrl.value().to_string());
                    }}>
                    {select_options(rrc,bump,
                        required(ctrl_format.options.as_ref())[..].as_ref(),
                        &ctrl_format.value)}
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
                        let rrc = root.unwrap_mut::<RootRenderingComponent>();
                        on_input(rrc, vdom_weak, ctrl.name().to_string(),ctrl.value().to_string());
                    }}>
                    </input>
                </div>
                ));
            }
        }
    }
    //return
    vec_node
}

/// event on input for ctrl input and select
pub fn on_input(
    rrc: &mut RootRenderingComponent,
    vdom_weak: dodrio::VdomWeak,
    ctrl_name: String,
    ctrl_value: String,
) {
    logmod::debug_write(&format!("select_on_input{}", ""));
    //save on every key stroke

    let map = fetchjsonformatmod::get_mut_by_name_req(&mut rrc.json_format, ctrl_name.as_str());
    logmod::debug_write(&format!("map from json {:?}", map));
    map.value = ctrl_value.clone();

    rrc.json_result[ctrl_name.as_str()] = ctrl_value;

    let window = unwrapmod::unwrap_option_abort(web_sys::window());
    let ls = unwrapmod::unwrap_option_abort(unwrapmod::unwrap_result_abort(window.local_storage()));
    let _x = ls.set_item(
        "json_string",
        unwrapmod::unwrap_result_abort(serde_json::to_string_pretty(&rrc.json_result)).as_str(),
    );

    vdom_weak.schedule_render();
}

/// render the option of select
pub fn select_options<'b>(
    _rrc: &RootRenderingComponent,
    bump: &'b Bump,
    vec_options: &[fetchjsonformatmod::CtrlOption],
    selected: &str,
) -> Vec<Node<'b>> {
    let mut vec_node = Vec::new();

    for x in vec_options {
        //logmod::debug_write(&format!("x {:?}", x));
        let value = bumpalo::format!(in bump, "{}",x.option).into_bump_str();
        let caption = {
            vec![text(
                bumpalo::format!(in bump, "{}",x.caption).into_bump_str(),
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
    let window = unwrapmod::unwrap_option_abort(web_sys::window());
    let document = unwrapmod::unwrap_option_abort(window.document());

    let t = unwrapmod::unwrap_option_abort(document.get_element_by_id("json_result"));
    let el = unwrapmod::unwrap_result_abort(t.dyn_into::<web_sys::HtmlTextAreaElement>());
    //and again Safari is the problem
    if is_iphone() {
        let range = unwrapmod::unwrap_result_abort(document.create_range());
        let _x = range.select_node_contents(&el);
        let selection =
            unwrapmod::unwrap_option_abort(unwrapmod::unwrap_result_abort(window.get_selection()));
        let _x = selection.remove_all_ranges();
        let _x = selection.add_range(&range);
        let _x = el.set_selection_range(0, 999_999);
    } else {
        el.select();
    }

    let hd = unwrapmod::unwrap_result_abort(document.dyn_into::<web_sys::HtmlDocument>());
    let _x = hd.exec_command("copy");
    let _x = window.alert_with_message("JSON text copied to clipboard.");
}

///detect iphone
pub fn is_iphone() -> bool {
    let window = unwrapmod::unwrap_option_abort(web_sys::window());
    let navigator = window.navigator();
    let user_agent = unwrapmod::unwrap_result_abort(navigator.user_agent());
    user_agent.to_ascii_lowercase().contains("iphone")
}

///open email client for send email
pub fn send_email(rrc: &RootRenderingComponent) {
    if let Some(accommodation_data) = &rrc.accommodation_data {
        let window = unwrapmod::unwrap_option_abort(web_sys::window());

        let link = format!(
            "mailto:{}?subject={}&body={}",
            urlencoding::encode(&accommodation_data.email),
            stringmod::concat_4(
                "Form C data ",
                fetchjsonformatmod::get_by_name_req(&rrc.json_format, "applicant_surname")
                    .value
                    .as_str(),
                " ",
                fetchjsonformatmod::get_by_name_req(&rrc.json_format, "applicant_givenname")
                    .value
                    .as_str()
            ),
            urlencoding::encode(&unwrapmod::unwrap_result_abort(
                serde_json::to_string_pretty(&rrc.json_result)
            ))
        );
        let _x = window.open_with_url_and_target(&link, "_blank");
    }
}

///node for img
pub fn div_img_accommodation<'b>(
    rrc: &'b RootRenderingComponent,
    bump: &'b Bump,
) -> Option<Node<'b>> {
    match &rrc.accommodation_data {
        Some(accommodation_data) => {
            let str_src = bumpalo::format!(in bump, "accommodations/{}/header_img.jpg",
                 accommodation_data.id)
            .into_bump_str();
            let alt = bumpalo::format!(in bump, "{}",
                  accommodation_data.name)
            .into_bump_str();
            let href = bumpalo::format!(in bump, "{}",
                  accommodation_data.web)
            .into_bump_str();
            //return
            Some(dodrio!(bump,
                <div>
                    <a href={href} target="_blank">
                        <img src={str_src} alt={alt} ></img>
                    </a>
                    {
                    let mut ppp =vec![ dodrio!(bump,
                        <h2 class="w3-center w3-text-yellow">
                        {vec![text(
                            bumpalo::format!(in bump, "{}",
                            accommodation_data.name)
                            .into_bump_str()
                        )]}
                        </h2>)];

                    for x in &accommodation_data.text_vector{
                        ppp.push(
                            dodrio!(bump,
                                <h4>
                                {vec![text(
                                    bumpalo::format!(in bump, "{}",
                                    x)
                                    .into_bump_str()
                                )]}
                                </h4>)
                            );
                    }
                    let mut delim = "";
                    for x in &accommodation_data.urls{
                        ppp.push(
                            dodrio!(bump,
                                <b>
                                {vec![text(
                                    bumpalo::format!(in bump, "{}",
                                    delim)
                                    .into_bump_str()
                                )]}
                                </b>
                            ));
                        ppp.push(
                        dodrio!(bump,
                            <a href={&x.url} target="_blank">
                            {vec![text(
                                bumpalo::format!(in bump, "{}",
                                x.name)
                                .into_bump_str()
                            )]}
                            </a>
                        ));
                        delim = " , ";
                    }
                    //result
                    ppp
                    }
                </div>
            ))
        }
        None => None,
    }
}

///div for send email
pub fn div_send_email<'b>(rrc: &'b RootRenderingComponent, bump: &'b Bump) -> Option<Node<'b>> {
    match &rrc.accommodation_data {
        Some(_accommodation_data) => {
            //return
            Some(dodrio!(bump,
                <div id="div_send_email">
                    <button class="w3-button w3-block w3-round w3-green w3-text-bold w3-text-color-black"
                        onclick={ move |root, vdom_weak, event| {
                            let rrc = root.unwrap_mut::<RootRenderingComponent>();
                            send_email(rrc);
                        }}>
                    {vec![text(
                        bumpalo::format!(in bump, "{}",
                        "Send from default email client")
                        .into_bump_str()
                    )]}
                    </button>
                    <p>
                        {vec![text(
                            bumpalo::format!(in bump, "{}","or").into_bump_str()
                        )]}
                    </p>
                </div>))
        }
        None => None,
    }
}

/// div version
pub fn div_version<'b>(rrc: &'b RootRenderingComponent, bump: &'b Bump) -> Vec<Node<'b>> {
    let version = env!("CARGO_PKG_VERSION");
    let mut vec_node = Vec::new();
    vec_node.push(
    match &rrc.accommodation_data {
        Some(_accommodation_data) => dodrio!(bump,
                    <div>
                    <h6 class="w3-text-yellow">
                        {vec![text(bumpalo::format!(in bump, "Version: {}", version).into_bump_str(),)]}                
                    </h6>
                    </div>),
        None => dodrio!(bump,
                    <div>
                    <h6 class="w3-text-yellow">
                        {vec![text(bumpalo::format!(in bump, "This is the basic free fully functional version of the web page. Contact the author to personalize with accommodation's data, logo and email address for the easy-to-use button 'Send email' for your guest's comfort. Version: {}", version).into_bump_str(),)]}
                    </h6>
                    <h6 class="w3-text-yellow">
                    <a href= "https://bestia.dev/formc/example/FormCInvite.html" target="_blank">
                        {vec![text(bumpalo::format!(in bump, "Invite the accommodation to use and personalize this web page.{}", "").into_bump_str(),)]}
                    </a>
                </h6>
                    </div>),
    });
    //return
    vec_node
}
