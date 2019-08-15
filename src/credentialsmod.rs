//! **credentialsmod.rs - the credentials are saved in the LocalStorage of the browser**  

//region: use
use crate::log1;
use crate::rootrenderingmod::RootRenderingComponent;

use wasm_bindgen::JsCast;
use futures::Future;
//endregion

///save credentials from html input elements to local storage and rrc
pub fn save_credentials_to_localstorage(vdom_weak: &dodrio::VdomWeak) {
    let window = unwrap!(web_sys::window(), "window");
    let document = unwrap!(window.document(), "document");

    log1("before get_element_by_id");
    let input_api_owner = unwrap!(document.get_element_by_id("api_owner"), "api_owner");
    log1("before dyn_into");
    let input_html_element_api_owner = unwrap!(
        input_api_owner.dyn_into::<web_sys::HtmlInputElement>(),
        "dyn_into"
    );
    log1("before value()");
    let api_owner_string = input_html_element_api_owner.value();
    log1("before as_str");
    let api_owner = api_owner_string.as_str();
    log1(api_owner);

    log1("before get_element_by_id");
    let input_api_uid = unwrap!(document.get_element_by_id("api_uid"), "api_uid");
    log1("before dyn_into");
    let input_html_element_api_uid = unwrap!(
        input_api_uid.dyn_into::<web_sys::HtmlInputElement>(),
        "dyn_into"
    );
    log1("before value()");
    let api_uid_string = input_html_element_api_uid.value();
    log1("before as_str");
    let api_uid = api_uid_string.as_str();
    log1(api_uid);

    log1("before get_element_by_id");
    let input_api_key = unwrap!(document.get_element_by_id("api_key"), "api_key");
    log1("before dyn_into");
    let input_html_element_api_key = unwrap!(
        input_api_key.dyn_into::<web_sys::HtmlInputElement>(),
        "dyn_into"
    );
    log1("before value()");
    let api_key_string = input_html_element_api_key.value();
    log1("before as_str");
    let api_key = api_key_string.as_str();
    log1(api_key);

    let ls = unwrap!(unwrap!(window.local_storage()));
    let _x = ls.set_item("api_owner", api_owner);
    let _x = ls.set_item("api_uid", api_uid);
    let _x = ls.set_item("api_key", api_key);

    //To change the data in rrc I must use the future `vdom.with_component`
    //it will be executed at the next tick to avoid concurrent data races.
    wasm_bindgen_futures::spawn_local(
        vdom_weak
            .with_component({
                move |root| {
                    let rrc = root.unwrap_mut::<RootRenderingComponent>();

                    rrc.api_owner = api_owner_string;
                    rrc.api_uid = api_uid_string;
                    rrc.api_key = api_key_string;
                }
            })
            .map_err(|_| ()),
    );
}

///load credentials from local storage, returns tuple with 3 values
pub fn load_credentials() -> (String, String, String) {
    let window = unwrap!(web_sys::window(), "window");
    let ls = unwrap!(unwrap!(window.local_storage()));
    let empty1 = "".to_string();
    let empty2 = "".to_string();
    let empty3 = "".to_string();
    let api_owner = unwrap!(ls.get_item("api_owner")).unwrap_or(empty1);
    let api_uid = unwrap!(ls.get_item("api_uid")).unwrap_or(empty2);
    let api_key = unwrap!(ls.get_item("api_key")).unwrap_or(empty3);
    //return
    (api_owner, api_uid, api_key)
}