// fetchjsonformat.rs
//! fetch json format

//region: use
use crate::rootrenderingmod::RootRenderingComponent;
use crate::fetchmod;
//use crate::logmod;

use unwrap::unwrap;
use web_sys::{Request, RequestInit};
//endregion

///async fetch_response() for json format
pub fn fetch_json_format_request(vdom_weak: dodrio::VdomWeak, location_href: &str) {
    let url_json = format!("{}efrro_form_c_format.json", location_href);
    //logmod::debug_write(url_json.as_str());
    let webrequest = create_webrequest(&url_json);
    fetchmod::fetch_response(vdom_weak, &webrequest, &set_json_format);
}

///create web request from string
pub fn create_webrequest(location_href: &str) -> web_sys::Request {
    let mut opts = RequestInit::new();
    opts.method("GET");

    let w_webrequest = unwrap!(Request::new_with_str_and_init(location_href, &opts));

    //logmod::debug_write("let w_webrequest =");
    //return
    w_webrequest
}

#[allow(clippy::needless_pass_by_value)]
/// update a field in the struct
pub fn set_json_format(rrc: &mut RootRenderingComponent, respbody: String) {
    //respbody is json.
    //logmod::debug_write(format!("respbody: {}", respbody).as_str());
    rrc.json_format = respbody;
}
