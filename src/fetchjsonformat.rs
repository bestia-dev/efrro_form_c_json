// fetchjsonformat.rs
//! fetch json format

//region: use
use crate::rootrenderingmod::RootRenderingComponent;
use crate::fetchmod;
use crate::logmod;

use unwrap::unwrap;
use web_sys::{Request, RequestInit};
use serde_json::json;
//use indexmap::IndexMap;
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
    //respbody is json, parse it to IndexMap
    rrc.json_format = unwrap!(serde_json::from_str(respbody.as_str()));
    //fill json_result from local storage
    let window = unwrap!(web_sys::window(), "window");
    let ls = unwrap!(unwrap!(window.local_storage()));
    let x = unwrap!(ls.get_item("json_string"));
    logmod::debug_write("ls.get_item(json_string)");
    if let Some(x) = x {
        //logmod::debug_write("x.is_some");
        rrc.json_result = unwrap!(serde_json::from_str(&x));
        //fill json_format with local storage values
        for (k, _v) in &rrc.json_result {
            if rrc.json_format.contains_key(k.as_str())
                && rrc.json_format[k]["value"] != rrc.json_result[k.as_str()]
            {
                rrc.json_format[k]["value"] = rrc.json_result[k.as_str()].clone();
            }
        }
        //logmod::debug_write(&format!("{:?}", rrc.json_format));
        //logmod::debug_write(&format!("{:?}", rrc.json_result));
    }
    //fill result with missing keys
    for (k, v) in &rrc.json_format {
        if !rrc.json_result.contains_key(k.as_str()) {
            rrc.json_result
                .insert(k.to_string(), v.get("value").unwrap_or(&json!("")).clone());
        }
    }
}
