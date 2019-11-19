//region: lmake_readme insert "readme.md"

//endregion: lmake_readme insert "readme.md"
//

//region: Clippy
#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    //variable shadowing is idiomatic to Rust, but unnatural to me.
    clippy::shadow_reuse,
    clippy::shadow_same,
    clippy::shadow_unrelated,

)]
#![allow(
    //library from dependencies have this clippy warnings. Not my code.
    //Why is this bad: It will be more difficult for users to discover the purpose of the crate, 
    //and key information related to it.
    clippy::cargo_common_metadata,
    //Why is this bad : This bloats the size of targets, and can lead to confusing error messages when 
    //structs or traits are used interchangeably between different versions of a crate.
    clippy::multiple_crate_versions,
    //Why is this bad : As the edition guide says, it is highly unlikely that you work with any possible 
    //version of your dependency, and wildcard dependencies would cause unnecessary 
    //breakage in the ecosystem.
    clippy::wildcard_dependencies,
    //Rust is more idiomatic without return statement
    //Why is this bad : Actually omitting the return keyword is idiomatic Rust code. 
    //Programmers coming from other languages might prefer the expressiveness of return. 
    //It’s possible to miss the last returning statement because the only difference 
    //is a missing ;. Especially in bigger code with multiple return paths having a 
    //return keyword makes it easier to find the corresponding statements.
    clippy::implicit_return,
    //I have private function inside a function. Self does not work there.
    //Why is this bad: Unnecessary repetition. Mixed use of Self and struct name feels inconsistent.
    clippy::use_self,
    //Cannot add #[inline] to the start function with #[wasm_bindgen(start)]
    //because then wasm-pack build --target web returns an error: export run not found 
    //Why is this bad: In general, it is not. Functions can be inlined across crates when that’s profitable 
    //as long as any form of LTO is used. When LTO is disabled, functions that are not #[inline] 
    //cannot be inlined across crates. Certain types of crates might intend for most of the 
    //methods in their public API to be able to be inlined across crates even when LTO is disabled. 
    //For these types of crates, enabling this lint might make sense. It allows the crate to 
    //require all exported methods to be #[inline] by default, and then opt out for specific 
    //methods where this might not make sense.
    clippy::missing_inline_in_public_items,
    //Why is this bad: This is only checked against overflow in debug builds. In some applications one wants explicitly checked, wrapping or saturating arithmetic.
    //clippy::integer_arithmetic,
    //Why is this bad: For some embedded systems or kernel development, it can be useful to rule out floating-point numbers.
    clippy::float_arithmetic,
    //Why is this bad : Doc is good. rustc has a MISSING_DOCS allowed-by-default lint for public members, but has no way to enforce documentation of private items. This lint fixes that.
    clippy::doc_markdown,
    //Why is this bad : Splitting the implementation of a type makes the code harder to navigate.
    clippy::multiple_inherent_impl,

    clippy::missing_docs_in_private_items,
)]
//endregion

//region: extern and use statements
//needed for dodrio! macro (typed-html)
#![recursion_limit = "5012"]
//rust modules system
mod rootrenderingmod;
mod fetchmod;
mod fetchjsonformat;
mod fetchjsonhostel;
mod logmod;

extern crate console_error_panic_hook;
extern crate log;
extern crate serde;
//#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate web_sys;
#[macro_use]
extern crate unwrap;
extern crate wasm_bindgen_futures;

use wasm_bindgen::prelude::wasm_bindgen;
//use web_sys::{console};
//use wasm_bindgen::JsValue;
//endregion

///this is the start function that wasm_bindgen calls
#[wasm_bindgen(start)]
pub fn wasm_bindgen_start() {
    // Initialize debugging for when/if something goes wrong.
    console_error_panic_hook::set_once();

    // Get the div for rendering html inside
    let window = unwrap!(web_sys::window());
    let document = unwrap!(window.document());
    let div_for_virtual_dom = unwrap!(
        document.get_element_by_id("div_for_virtual_dom"),
        "No #div_for_virtual_dom"
    );

    // Construct a new rendering component.
    let rrc = rootrenderingmod::RootRenderingComponent::new();
    // Mount the component to the div
    let vdom = dodrio::Vdom::new(&div_for_virtual_dom, rrc);

    //region: find out URL and parameters
    let mut location_href = unwrap!(window.location().href(), "href not known");
    logmod::debug_write(&location_href);
    //everything after the first ? is parameters.
    let mut parameters = "".to_string();
    let mut hostel_id: Option<String> = None;
    if let Some(x) = location_href.find('?') {
        parameters = unwrap!(location_href.get(x..)).to_string();
        //only 1 parameter allowed ex. ?id=sturmfrei_goa
        hostel_id = Some(parameters.replace("?id=", ""));

        //href without parameters
        location_href = unwrap!(location_href.get(..x)).to_string();
    }
    //without /index.html
    location_href = location_href.to_lowercase().replace("index.html", "");
    logmod::debug_write(&location_href);
    logmod::debug_write(&parameters);
    //endregion

    //logmod::debug_write(&format!("location_href: {}", &location_href));
    //fetch the json_format
    let v2 = vdom.weak();
    fetchjsonformat::fetch_json_format_request(v2, &location_href);
    if let Some(str_hostel_id) = hostel_id {
        //fetch the json_format
        let v3 = vdom.weak();
        fetchjsonhostel::fetch_json_format_request(v3, &location_href, &str_hostel_id);
    }
    // Run the component forever. Never drop the memory.
    vdom.forget();
}

/*
//region: Helper functions
///simple console write with a string
fn log1(x: &str) {
    console::log_1(&JsValue::from_str(x));
}
//endregion
*/
