// fetchjsonformat.rs
//! fetch json format

//region: use
use crate::rootrenderingmod::RootRenderingComponent;
use crate::fetchmod;
use crate::logmod;
use crate::*;

//use unwrap::unwrap;
use indexmap::IndexMap;
use web_sys::{Request, RequestInit};
use serde_json::json;
//use indexmap::IndexMap;
//endregion

///async fetch_response() for json format
pub fn fetch_json_format_request(vdom_weak: dodrio::VdomWeak, location_href: &str) {
    let url_json = stringmod::concat_4(location_href, "efrro_form_c_format.json", "", "");
    //format!("{}efrro_form_c_format.json", location_href);
    //logmod::debug_write(url_json.as_str());
    let webrequest = create_webrequest(&url_json);
    fetchmod::fetch_response(vdom_weak, &webrequest, &set_json_format);
}

///create web request from string
pub fn create_webrequest(location_href: &str) -> web_sys::Request {
    let mut opts = RequestInit::new();
    opts.method("GET");

    //return
    //let w_webrequest =
    unwrap_result_abort(Request::new_with_str_and_init(location_href, &opts))

    //logmod::debug_write("let w_webrequest =");
    //return
    //w_webrequest
}

#[allow(clippy::needless_pass_by_value)]
/// update a field in the struct
pub fn set_json_format(rrc: &mut RootRenderingComponent, respbody: String) {
    //respbody is json, parse it to IndexMap
    rrc.json_format = unwrap_result_abort(serde_json::from_str(respbody.as_str()));

    //fill json_result from local storage
    let window = unwrap_option_abort(web_sys::window());
    let ls = unwrap_option_abort(unwrap_result_abort(window.local_storage()));
    let option_stored_str = unwrap_result_abort(ls.get_item("json_string"));
    logmod::debug_write("ls.get_item(json_string)");
    if let Some(stored_str) = option_stored_str {
        logmod::debug_write("x.is_some");
        let stored_json: IndexMap<String, serde_json::Value> =
            unwrap_result_abort(serde_json::from_str(&stored_str));
        //fill json_format with local storage values
        for (k, v) in &stored_json {
            logmod::debug_write(&stringmod::concat_4(k, &v.to_string(), "", ""));
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

    //fill hostel data if values are empty.
    //This is async. No garantee it will be executed after fetch.
    if let Some(hostel_data) = &rrc.hostel_data {
        logmod::debug_write("hostel data");
        //format!("..{}..",x);
        logmod::debug_write(&stringmod::concat_4(
            "..",
            unwrap_option_abort(
                unwrap_option_abort(rrc.json_format.get_mut("applicant_refaddr"))["value"].as_str(),
            ),
            "..",
            "",
        ));
        if unwrap_option_abort(
            unwrap_option_abort(rrc.json_format.get_mut("applicant_refaddr"))["value"].as_str(),
        ) == ""
        {
            logmod::debug_write("writing to json_format applicant_refaddr");

            logmod::debug_write(&stringmod::concat_4(
                "..",
                json!(hostel_data.applicant_refaddr.as_str())
                    .to_string()
                    .as_str(),
                "..",
                "",
            ));
            unwrap_option_abort(rrc.json_format.get_mut("applicant_refaddr"))["value"] =
                json!(hostel_data.applicant_refaddr.as_str());
        }
        if unwrap_option_abort(
            unwrap_option_abort(rrc.json_format.get_mut("applicant_refstate"))["value"].as_str(),
        ) == ""
        {
            unwrap_option_abort(rrc.json_format.get_mut("applicant_refstate"))["value"] =
                json!(hostel_data.applicant_refstate.as_str());
        }
        if unwrap_option_abort(
            unwrap_option_abort(rrc.json_format.get_mut("applicant_refstatedistr"))["value"]
                .as_str(),
        ) == ""
        {
            unwrap_option_abort(rrc.json_format.get_mut("applicant_refstatedistr"))["value"] =
                json!(hostel_data.applicant_refstatedistr.as_str());
        }
        if unwrap_option_abort(
            unwrap_option_abort(rrc.json_format.get_mut("applicant_refpincode"))["value"].as_str(),
        ) == ""
        {
            unwrap_option_abort(rrc.json_format.get_mut("applicant_refpincode"))["value"] =
                json!(hostel_data.applicant_refpincode.as_str());
        }
    }
    //fill rrc.json_result with json_format values and preserve order
    for (k, v) in &rrc.json_format {
        if v.get("ctrl_type").unwrap_or(&json!("text")) != "label" {
            rrc.json_result
                .insert(k.to_string(), v.get("value").unwrap_or(&json!("")).clone());
        }
    }
}
