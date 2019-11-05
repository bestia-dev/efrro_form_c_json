//! **rootrenderingmod - Root Rendering Component is the base struct for Dodrio Virtual Dom**  
//! rrc contains all the data needed for UI and reacts to all the events of the UI.  
//! Other subcomponents can be more or less smart, with or without their own data,
//! but that data are only copied values from the rrc component.  

#![allow(clippy::needless_pass_by_value)]

//region use
//use crate::log1;

extern crate csv;

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
    pub json: String,
}

impl RootRenderingComponent {
    ///constructor
    pub fn new() -> RootRenderingComponent {
        let json = r#"
       {
"_json_comment_sample_data_for":"efrro_form_c",

"applicant_surname":"val_applicant_surname",
"applicant_givenname":"val_applicant_givenname",
"applicant_dob":"val_applicant_dob",
"applicant_age":"val_applicant_age",
"actualDOB":"val_actualDOB",
"applicant_permcity":"val_applicant_permcity",
"applicant_refpincode":"val_applicant_refpincode",
"applicant_passpno":"val_applicant_passpno",
"applicant_passplcofissue":"val_applicant_passplcofissue",
"applicant_passpdoissue":"val_applicant_passpdoissue",
"applicant_passpvalidtill":"val_applicant_passpvalidtill",
"applicant_visano":"val_applicant_visano",
"applicant_visaplcoissue":"val_applicant_visaplcoissue",
"applicant_visadoissue":"val_applicant_visadoissue",
"applicant_visavalidtill":"val_applicant_visavalidtill",
"applicant_arrivedfromcity":"val_applicant_arrivedfromcity",
"applicant_arrivedfromplace":"val_applicant_arrivedfromplace",
"applicant_doarrivalindia":"val_applicant_doarrivalindia",
"applicant_doarrivalhotel":"val_applicant_doarrivalhotel",
"applicant_timeoarrivalhotel":"val_applicant_timeoarrivalhotel",
"applicant_intnddurhotel":"val_applicant_intnddurhotel",
"applicant_next_destination_place_IN":"val_applicant_next_destination_place_IN",
"applicant_contactnoinindia":"val_applicant_contactnoinindia",
"applicant_mcontactnoinindia":"val_applicant_mcontactnoinindia",
"applicant_contactnoperm":"val_applicant_contactnoperm",
"applicant_mcontactnoperm":"val_applicant_mcontactnoperm",
"applicant_remark":"val_applicant_remark",

"_json_comment_radio_employed":"Y, N",
"employed":"val_Y",
"_json_comment_radio_applicant_next_dest_country_flag_r":"I, O",
"applicant_next_dest_country_flag_r":"val_I",
"_json_comment_hidden_applicant_next_dest_country_flag":"I, O",
"applicant_next_dest_country_flag":"val_I"
}
       "#
        .to_string();
        //return
        RootRenderingComponent { json }
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
            <div id="json">
                <label>
                    {vec![text(
                        bumpalo::format!(in bump, "{}",
                        "json:")
                        .into_bump_str()
                    )]}
                </label>
                <textarea id="json" name="json" class="w3-input">
                    {vec![text(
                        bumpalo::format!(in bump, "{}",
                        self.json)
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
        );
        console::time_end_with_label("render");
        //return
        xdiv
    }
}
