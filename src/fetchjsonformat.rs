// fetchjsonformat.rs
//! fetch json format

//region: use
use crate::rootrenderingmod::RootRenderingComponent;
use crate::fetchmod;
use crate::logmod;

use unwrap::unwrap;
use indexmap::IndexMap;
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
    let option_stored_str = unwrap!(ls.get_item("json_string"));
    logmod::debug_write("ls.get_item(json_string)");
    if let Some(stored_str) = option_stored_str {
        logmod::debug_write("x.is_some");
        let stored_json: IndexMap<String, serde_json::Value> =
            unwrap!(serde_json::from_str(&stored_str));
        //fill json_format with local storage values
        for (k, v) in &stored_json {
            logmod::debug_write(&format!("{:?} {:?}", k, v));
            let record = rrc.json_format.get_mut(k);
            if let Some(record) = record {
                if record.get("ctrl_type").unwrap_or(&json!("")) != "label" {
                    record["value"] = v.clone();
                }
            }
        }
        //logmod::debug_write(&format!("{:?}", rrc.json_format));
        //logmod::debug_write(&format!("{:?}", rrc.json_result));
    }
    //fill rrc.json_result with json_format values and preserve order
    for (k, v) in &rrc.json_format {
        if v.get("ctrl_type").unwrap_or(&json!("text")) != "label" {
            rrc.json_result
                .insert(k.to_string(), v.get("value").unwrap_or(&json!("")).clone());
        }
    }
}
