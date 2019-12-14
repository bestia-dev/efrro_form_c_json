// fetchjsonhostel.rs
//! fetch json hostel

//region: use
use crate::rootrenderingmod::RootRenderingComponent;
use crate::fetchmod;
use crate::*;
//use crate::logmod;

//use unwrap::unwrap;
use web_sys::{Request, RequestInit};
//use indexmap::IndexMap;
//endregion

///async fetch_response() for json format
pub fn fetch_json_format_request(
    vdom_weak: dodrio::VdomWeak,
    location_href: &str,
    hostel_id: &str,
) {
    let url_json = format!(
        "{}/efrro_form_c_json_hostels/{}/hostel.json",
        location_href, hostel_id
    );
    //logmod::debug_write(url_json.as_str());
    let webrequest = create_webrequest(&url_json);
    fetchmod::fetch_response(vdom_weak, &webrequest, &set_hostel_id);
}

///create web request from string
pub fn create_webrequest(location_href: &str) -> web_sys::Request {
    let mut opts = RequestInit::new();
    opts.method("GET");

    let w_webrequest = unwrap_result_abort(Request::new_with_str_and_init(location_href, &opts));

    //logmod::debug_write("let w_webrequest =");
    //return
    w_webrequest
}

#[allow(clippy::needless_pass_by_value)]
/// update a field in the struct
pub fn set_hostel_id(rrc: &mut RootRenderingComponent, respbody: String) {
    //respbody is json, parse it
    rrc.hostel_data = unwrap_result_abort(serde_json::from_str(respbody.as_str()));
}
