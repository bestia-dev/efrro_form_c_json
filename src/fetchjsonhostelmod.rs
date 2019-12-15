// fetchjsonhostelmod.rs
//! fetch JSON hostel

//region: use
use crate::rootrenderingmod::RootRenderingComponent;
use crate::fetchmod;
use crate::stringmod;
use crate::unwrapmod;
//use crate::logmod;

//use unwrap::unwrap;
use web_sys::{Request, RequestInit};
use serde::{Deserialize, Serialize};
//use indexmap::IndexMap;
//endregion

///url struct
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Url {
    pub name: String,
    pub url: String,
}

///hostel data saved in hostels folder JSON
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HostelData {
    pub id: String,
    pub name: String,
    pub email: String,
    pub web: String,
    pub applicant_refaddr: String,
    pub applicant_refstate: String,
    pub applicant_refstatedistr: String,
    pub applicant_refpincode: String,
    pub text_vector: Vec<String>,
    pub urls: Vec<Url>,
}

///async fetch_response() for JSON format
pub fn fetch_json_format_request(
    vdom_weak: dodrio::VdomWeak,
    location_href: &str,
    hostel_id: &str,
) {
    let url_json = stringmod::concat_4(
        location_href,
        "/efrro_form_c_json_hostels/",
        hostel_id,
        "/hostel.json",
    );
    //logmod::debug_write(url_json.as_str());
    let webrequest = create_webrequest(&url_json);
    fetchmod::fetch_response(vdom_weak, &webrequest, &set_hostel_id);
}

///create web request from string
pub fn create_webrequest(location_href: &str) -> web_sys::Request {
    let mut opts = RequestInit::new();
    opts.method("GET");

    //let w_webrequest =
    //return
    unwrapmod::unwrap_result_abort(Request::new_with_str_and_init(location_href, &opts))

    //logmod::debug_write("let w_webrequest =");
    //return
    //w_webrequest
}

#[allow(clippy::needless_pass_by_value)]
/// update a field in the struct
pub fn set_hostel_id(rrc: &mut RootRenderingComponent, respbody: String) {
    //respbody is JSON, parse it
    rrc.hostel_data = unwrapmod::unwrap_result_abort(serde_json::from_str(respbody.as_str()));
}
