// fetchjsonformatmod.rs
//! fetch JSON format

//region: use
use crate::rootrenderingmod::RootRenderingComponent;
use crate::fetchmod;
use crate::logmod;
use crate::stringmod;
use crate::unwrapmod;
use crate::unwrapmod::required;

//use unwrap::unwrap;
use indexmap::IndexMap;
use web_sys::{Request, RequestInit};
use serde_json;
use serde::{Deserialize, Serialize};
//use indexmap::IndexMap;
//endregion

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CtrlOption {
    pub option: String,
    pub caption: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CtrlFormat {
    pub name: String,
    pub ctrl_type: String,
    pub value: String,
    pub caption: Option<String>,
    pub options: Option<Vec<CtrlOption>>,
}

///async fetch_response() for JSON format
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
    unwrapmod::unwrap_result_abort(Request::new_with_str_and_init(location_href, &opts))

    //logmod::debug_write("let w_webrequest =");
    //return
    //w_webrequest
}

#[allow(clippy::needless_pass_by_value)]
/// update a field in the struct
pub fn set_json_format(rrc: &mut RootRenderingComponent, respbody: String) {
    logmod::debug_write("set_json_format");
    //respbody is JSON, parse it to IndexMap
    rrc.json_format = unwrapmod::unwrap_result_abort(serde_json::from_str(respbody.as_str()));

    //fill json_result from local storage
    let window = required(web_sys::window());
    let ls = required(unwrapmod::unwrap_result_abort(window.local_storage()));
    let option_stored_str = unwrapmod::unwrap_result_abort(ls.get_item("json_string"));
    logmod::debug_write("ls.get_item(json_string)");
    if let Some(stored_str) = option_stored_str {
        logmod::debug_write("x.is_some");
        let stored_json: IndexMap<String, String> =
            unwrapmod::unwrap_result_abort(serde_json::from_str(&stored_str));
        //fill json_format with local storage values
        for (k, v) in &stored_json {
            logmod::debug_write(&stringmod::concat_4(
                k,
                " ",
                &unwrapmod::unwrap_result_abort(serde_json::to_string(&v)),
                "",
            ));
            let opt_record = get_mut_by_name(&mut rrc.json_format, k);
            if let Some(record) = opt_record {
                if record.ctrl_type != "label" {
                    record.value = v.clone();
                }
            }
        }
        //logmod::debug_write(&format!("{:?}", rrc.json_format));
        //logmod::debug_write(&format!("{:?}", rrc.json_result));
    }

    //fill accommodation data if values are empty.
    //This is async. No garantee it will be executed after fetch.
    if let Some(accommodation_data) = &rrc.accommodation_data {
        logmod::debug_write("accommodation data");
        //format!("..{}..",x);
        logmod::debug_write(&stringmod::concat_4(
            "..",
            get_mut_by_name_req(&mut rrc.json_format, "applicant_refaddr")
                .value
                .as_str(),
            "..",
            "",
        ));
        if get_by_name_req(&rrc.json_format, "applicant_refaddr")
            .value
            .is_empty()
        {
            get_mut_by_name_req(&mut rrc.json_format, "applicant_refaddr").value =
                accommodation_data.applicant_refaddr.clone();
        }
        if get_by_name_req(&rrc.json_format, "applicant_refstate")
            .value
            .is_empty()
        {
            get_mut_by_name_req(&mut rrc.json_format, "applicant_refstate").value =
                accommodation_data.applicant_refstate.clone();
        }
        if get_by_name_req(&rrc.json_format, "applicant_refstatedistr")
            .value
            .is_empty()
        {
            get_mut_by_name_req(&mut rrc.json_format, "applicant_refstatedistr").value =
                accommodation_data.applicant_refstatedistr.clone();
        }
        if get_by_name_req(&rrc.json_format, "applicant_refpincode")
            .value
            .is_empty()
        {
            get_mut_by_name_req(&mut rrc.json_format, "applicant_refpincode").value =
                accommodation_data.applicant_refpincode.clone();
        }
    }
    //fill rrc.json_result with json_format values and preserve order
    for ctrl_format in &rrc.json_format {
        if ctrl_format.ctrl_type != "label" {
            rrc.json_result
                .insert(ctrl_format.name.clone(), ctrl_format.value.clone());
        }
    }
}

//get by name from vector
pub fn get_mut_by_name<'a>(v: &'a mut Vec<CtrlFormat>, name: &str) -> Option<&'a mut CtrlFormat> {
    match v.iter().position(|r| r.name.as_str() == name) {
        Some(iindex) => v.get_mut(iindex),
        None => None,
    }
}

//get by name from vector
pub fn get_by_name<'a>(v: &'a [CtrlFormat], name: &str) -> Option<&'a CtrlFormat> {
    match v.iter().position(|r| r.name.as_str() == name) {
        Some(iindex) => v.get(iindex),
        None => None,
    }
}

//the name must exist, else abort
pub fn get_by_name_req<'a>(v: &'a [CtrlFormat], name: &str) -> &'a CtrlFormat {
    required(get_by_name(v, name))
}

//the name must exist, else abort
pub fn get_mut_by_name_req<'a>(v: &'a mut Vec<CtrlFormat>, name: &str) -> &'a mut CtrlFormat {
    required(get_mut_by_name(v, name))
}
