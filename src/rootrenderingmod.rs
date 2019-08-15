//! **rootrenderingmod - Root Rendering Component is the base struct for Dodrio Virtual Dom**  
//! rrc contains all the data needed for UI and reacts to all the events of the UI.  
//! Other subcomponents can be more or less smart, with or without their own data,
//! but that data are only copied values from the rrc component.  

#![allow(
clippy::needless_pass_by_value
)]

//region use
use crate::log1;
use crate::reqbodymod;
use crate::fetchmod;
use crate::amazonmod;
use crate::credentialsmod;

extern crate csv;

use dodrio::builder::text;
use dodrio::bumpalo::{self, Bump};
use dodrio::{Node, Render};
use typed_html::dodrio;
use serde::{Deserialize, Serialize};
use web_sys::{console};
use wasm_bindgen::JsCast; //must be for dyn_into
//endregion

///the struct with the only mutable data and the code for rendering it as html
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RootRenderingComponent {
    pub api_owner: String,
    pub api_uid: String,
    pub api_key: String,
    pub url: String,
    pub tipo_doc: String,
    pub amazon_txt_disabled: String,
    pub amazon_txt: String,
    pub vec_rec: Vec<amazonmod::Record>,
    pub vec_order_id: Vec<String>,
    pub reqbody: String,
    pub respbody: String,
}

impl RootRenderingComponent {
    ///constructor
    pub fn new() -> RootRenderingComponent {
        //the function returns a tuple with 3 values
        let (api_owner,api_uid,api_key) = credentialsmod::load_credentials();

        //return
        RootRenderingComponent {
            api_owner,
            api_uid,
            api_key,
            url: "".to_string(),
            tipo_doc: "".to_string(),
            amazon_txt_disabled: "w3-disabled".to_string(),
            amazon_txt: "".to_string(),
            vec_rec: vec![],
            vec_order_id: vec![],
            //raw literals with r# don't need most escape sequences. Good for json.
            reqbody: r#""#.to_string(),
            respbody: "".to_string(),
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
                    <h1 class="yellow">
                        {vec![text(
                            bumpalo::format!(in bump, "amafatt version {}",
                            version)
                            .into_bump_str()
                        )]}
                    </h1>
                       <div>
                        <p>
                            {vec![text(
                                bumpalo::format!(in bump, "{}",
                                "Save in the local browser your credentials: your name, api_uid and api_key.")
                                .into_bump_str()
                            )]}
                        </p>
                    </div>
                    {vec![div_credentials(self,bump)]}
                    <div>
                        <p>
                            {vec![text(
                                bumpalo::format!(in bump, "{}",
                                "1. click on a button to choose a request type.")
                                .into_bump_str()
                            )]}
                        </p>
                    </div>
                    <div class="w3-bar w3-border w3-black">
                        <a href="#" class="w3-bar-item w3-button w3-dark-grey w3-border w3-mobile" onclick={move |root, vdom_weak, _event| {
                                    let mut rrc = root.unwrap_mut::<RootRenderingComponent>();
                                    let request = reqbodymod::create_reqbody_info(rrc);
                                    vdom_weak.schedule_render();
                                }}>
                            {vec![text(
                                bumpalo::format!(in bump, "{}",
                                "Info")
                                .into_bump_str()
                            )]}
                        </a>
                        <a href="#" class="w3-bar-item w3-button w3-dark-grey w3-border w3-mobile" onclick={move |root, vdom_weak, _event| {
                                    let mut rrc = root.unwrap_mut::<RootRenderingComponent>();
                                    let request = reqbodymod::create_reqbody_ricevute_nuovo(rrc);
                                    vdom_weak.schedule_render();
                                }}>
                            {vec![text(
                                bumpalo::format!(in bump, "{}",
                                "ricevuta nuovo ( and ddt )")
                                .into_bump_str()
                            )]}
                        </a>
                    </div>
                    <div id="div_url_tipo_doc" class="preservewhitespace">
                        {vec![text(
                            bumpalo::format!(in bump, "url: {}        tipo_doc: {}",
                            self.url, self.tipo_doc)
                            .into_bump_str()
                        )]}
                    </div>
                    <div class={bumpalo::format!(in bump, "{}",
                            self.amazon_txt_disabled)
                            .into_bump_str()
                        }>
                        <div>
                            <p>
                                {vec![text(
                                    bumpalo::format!(in bump, "{}",
                                    "2. copy/paste the text content of amazon txt.")
                                    .into_bump_str()
                                )]}
                            </p>
                        </div>
                        <div id="div_amazon_txt">
                            <label>
                                {vec![text(
                                    bumpalo::format!(in bump, "{}",
                                    "amazon_txt:")
                                    .into_bump_str()
                                )]}
                            </label>
                            <textarea id="amazon_txt" name="amazon_txt" class="w3-input">
                                {vec![text(
                                    bumpalo::format!(in bump, "{}",
                                    self.amazon_txt)
                                    .into_bump_str()
                                )]}
                            </textarea>
                        </div>
                        <div>
                            <p>
                                {vec![text(
                                    bumpalo::format!(in bump, "{}",
                                    "3. click button 'extract order-id' and then click on one order-id.")
                                    .into_bump_str()
                                )]}
                            </p>
                        </div>
                        <div class="w3-bar w3-border w3-black">
                            <a href="#" class="w3-bar-item w3-button w3-dark-grey w3-border w3-mobile" onclick={move |root, vdom_weak, _event| {
                                    let mut rrc = root.unwrap_mut::<RootRenderingComponent>();
                                    // Get the value of the textarea input
                                    let window = unwrap!(web_sys::window(),"window");
                                    let document = unwrap!(window.document(),"document");
                                    log1("before get_element_by_id");
                                    let textarea_element_amazon_txt = unwrap!(document.get_element_by_id("amazon_txt"),"amazon_txt");
                                    log1("before dyn_into");
                                    let textarea_html_element_amazon_txt = unwrap!(textarea_element_amazon_txt.dyn_into::<web_sys::HtmlTextAreaElement>(),"dyn_into");
                                    log1("before value()");
                                    let val =  textarea_html_element_amazon_txt.value();
                                    log1("before as_str");
                                    let val_string = val.as_str();
                                    log1(val_string);

                                    amazonmod::amazon_txt_create_vector_of_order_id(&mut rrc, val_string);
                                    vdom_weak.schedule_render();
                                }}>
                                {vec![text(
                                    bumpalo::format!(in bump, "{}",
                                    "extract order-id")
                                    .into_bump_str()
                                )]}
                            </a>
                            
{
    let mut vec_butt = Vec::new();
    let vec_order_id_clone = self.vec_order_id.clone();
    for order_id in vec_order_id_clone {
        let ooo1 = order_id.clone();
        let ooo2= order_id.clone();
        let xdiv = dodrio!(bump,
            <a href="#" class="w3-bar-item w3-button w3-dark-grey w3-border w3-mobile" onclick={move |root, vdom_weak, _event| {
                    let mut rrc = root.unwrap_mut::<RootRenderingComponent>();
                    let v2=vdom_weak.clone();
                    amazonmod::amazon_single_order_id_process(&mut rrc, &ooo1, v2);
                    vdom_weak.schedule_render();
                }}>
                {vec![text(
                    bumpalo::format!(in bump, "{}",
                    ooo2)
                    .into_bump_str()
                )]}
            </a>);
        vec_butt.push(xdiv);
    }
    //return
    vec_butt
}


                            
                        </div>
                    </div>
                     <div>
                        <p>
                            {vec![text(
                                bumpalo::format!(in bump, "{}",
                                "4. check the request json and if needed manually edit.")
                                .into_bump_str()
                            )]}
                        </p>
                    </div>
                    <div id="div_reqbody">
                        <label>
                            {vec![text(
                                bumpalo::format!(in bump, "{}",
                                "Request body:")
                                .into_bump_str()
                            )]}
                        </label>
                        <textarea id="reqbody" name="reqbody" class="w3-input">
                            {vec![text(
                                bumpalo::format!(in bump, "{}",
                                self.reqbody)
                                .into_bump_str()
                            )]}
                        </textarea>
                    </div>
                </div>
                <div class="w3-half">
                     <div>
                        <p>
                            {vec![text(
                                bumpalo::format!(in bump, "{}",
                                "5. click button 'send json' and wait the response.")
                                .into_bump_str()
                            )]}
                        </p>
                    </div>
                    <div class="w3-bar w3-border w3-black">
                        <a href="#" class="w3-bar-item w3-button w3-dark-grey w3-border w3-mobile" onclick={move |root, vdom_weak, _event| {
                                let rrc = root.unwrap_mut::<RootRenderingComponent>();
                                //copy the text from the textarea into the struct field
                                let window = unwrap!(web_sys::window(),"window");
                                let document = unwrap!(window.document(),"document");
                                log1("before get_element_by_id");
                                let textarea_element_reqbody = unwrap!(document.get_element_by_id("reqbody"),"reqbody");
                                log1("before dyn_into");
                                let textarea_html_element_reqbody = unwrap!(textarea_element_reqbody.dyn_into::<web_sys::HtmlTextAreaElement>(),"dyn_into");
                                log1("before value()");
                                let val =  textarea_html_element_reqbody.value();
                                log1("before as_str");
                                let val_string = val.to_string();
                                log1(&val_string);
                                rrc.reqbody = val_string;

                                let webrequest = reqbodymod::create_webrequest_from_rrc(&rrc);
                                let v2=vdom_weak.clone();
                                //call async fetch
                                //the last parameter is the reference to the function to execute after fetch
                                fetchmod::fetch_response (v2,&webrequest,"".to_string(), &update_rrc_respbody);
                                //call to async must be the last command. The schedule_render is inside the async code.
                            }}>
                            {vec![text(
                                bumpalo::format!(in bump, "{}",
                                "send json")
                                .into_bump_str()
                            )]}
                        </a>
                    </div>
                    <div id="div_respbody">
                        <label>
                            {vec![text(
                                bumpalo::format!(in bump, "{}",
                                "Response body:")
                                .into_bump_str()
                            )]}
                        </label>
                        <textarea id="respbody" name="respbody" class="w3-input">
                            {vec![text(
                                bumpalo::format!(in bump, "{}",
                                self.respbody)
                                .into_bump_str()
                            )]}
                        </textarea>
                    </div>
                    <div>
                        <p>
                            {vec![text(
                                bumpalo::format!(in bump, "{}",
                                "6. open FattureInCloud.it and check the imported data. If needed edit, modify, delete, print the data there.")
                                .into_bump_str()
                            )]}
                        </p>
                    </div>
                     <div class={bumpalo::format!(in bump, "{}",
                            self.amazon_txt_disabled)
                            .into_bump_str()
                        }>
                        <p>
                            {vec![text(
                                bumpalo::format!(in bump, "{}",
                                "7. Click on the next order-id to process and then send the next json. Only one order-id at a time can be sent to FattureInCloud.it.")
                                .into_bump_str()
                            )]}
                        </p>
                    </div>
                    <div>
                        <h6 class="yellow">
                            {vec![text(bumpalo::format!(in bump, "Github repository: {}", "").into_bump_str(),)]}
                            <a href= "https://github.com/LucianoBestia/amafatt" target="_blank">
                                {vec![text(bumpalo::format!(in bump, "https://github.com/LucianoBestia/amafatt{}", "").into_bump_str(),)]}
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

fn div_credentials<'a, 'bump>(rrc: &'a RootRenderingComponent, bump: &'bump Bump) -> Node<'bump>
where
    'a: 'bump,
{
    //return
    dodrio!(bump,
    <div id="credentials" class="w3-row w3-border w3-black" >
        <div class="w3-col m2">
        <input id="api_owner" name="api_owner" value={bumpalo::format!(in bump, "{}",
                rrc.api_owner)
                .into_bump_str()
                } class="w3-border w3-input w3-mobile" >
        </input>
        </div>
        <div class="w3-col m2">
            <input id="api_uid" name="api_uid" value={bumpalo::format!(in bump, "{}",
                rrc.api_uid)
                .into_bump_str()
                } class="w3-border w3-input w3-mobile" >
        </input>
            </div>
        <div class="w3-col m6">
            <input id="api_key" name="api_key" value={bumpalo::format!(in bump, "{}",
                rrc.api_key)
                .into_bump_str()
                } class="w3-border w3-input w3-mobile" >
        </input>
            </div>
        <div class="w3-col m2">
        <a href="#" class="w3-bar-item w3-button w3-dark-grey w3-border w3-mobile" 
        onclick={move |root, vdom_weak, _event| {
                    let v2 = vdom_weak.clone();
                    credentialsmod::save_credentials_to_localstorage(v2);
                }}>
            {vec![text(
                bumpalo::format!(in bump, "{}",
                "save")
                .into_bump_str()
            )]}
        </a>
        </div>
    </div>
    )
}

//region: code to execute at the end after fetch (the async monster)
///change respbody and pretty json
#[allow(clippy::needless_pass_by_value)]
fn update_rrc_respbody(rrc: &mut RootRenderingComponent, respbody: String, _placeholder:String) {
    log1("update_rrc_respbody");
    //pretty json
    let untyped_json: serde_json::Value = unwrap!(serde_json::from_str(&respbody));
    let prettybody = unwrap!(serde_json::to_string_pretty(&untyped_json));

    rrc.respbody = prettybody;
}

///in rrs.reqbody replace a placeholder with a new value (fetched from the webservice)
pub fn replace_product_name(rrc:&mut RootRenderingComponent, respbody:String, placeholder:String){
    //inside txt_response find product-name. It is a json.
    //serde will deserialize only wanted data and ignore the rest

#[derive(Serialize, Deserialize, Debug)]
    struct ListaProdotti{
        cod:String,
        nome:String,
    }

#[derive(Serialize, Deserialize, Debug)]
    struct JsonRoot{
        lista_prodotti :Vec<ListaProdotti>,
    }
    
    log1(format!("fn replace_product_name placeholder {} \n {}",placeholder,respbody).as_str());
    
    let pr: JsonRoot = unwrap!(serde_json::from_str(&respbody));
    let name_product = &unwrap!(pr.lista_prodotti.get(0)).nome;
    
    rrc.reqbody = rrc.reqbody.replace(placeholder.as_str(), name_product);

}
//endregion

