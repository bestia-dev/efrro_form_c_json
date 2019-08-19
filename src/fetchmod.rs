//! **fetchmod - isolate/encapsulate fetch api in a module because it is all async**  
//region: lmake_readme insert "readme_fetchmod.md"
//
//endregion lmake_readme insert "readme_fetchmod.md"

//region: use
use crate::log1;
use crate::rootrenderingmod::RootRenderingComponent;

use wasm_bindgen::JsValue;
use wasm_bindgen::JsCast;
use web_sys::{console, Response};
use futures::Future;
//endregion

/// The only public function that starts the code flow around fetch_with_request()->Promise, text()->Promise  
/// This function returns nothing. All the code will be executed inside it.  
/// The last parameter is a reference to a (normal) function that will be executed at the end of this code flow.  
/// This cannot be very generic, because the parameters must be fixed.
pub fn fetch_response(
    vdom_weak: dodrio::VdomWeak,
    request: &web_sys::Request,
    placeholder: String,
    call_function_after_fetch: &'static (dyn for<'r> std::ops::Fn(
        &'r mut RootRenderingComponent,
        std::string::String,
        std::string::String,
    ) + 'static),
) {
    let window = unwrap!(web_sys::window());
    //1. wasm_bindgen knows only method fetch_with_request, and that returns a promise
    let request_promise = window.fetch_with_request(request);
    //transform promise into future
    let future = wasm_bindgen_futures::JsFuture::from(request_promise)
        .and_then(|resp_value| {
            // `resp_value` is a `Response` object.
            assert!(resp_value.is_instance_of::<Response>());
            let resp: Response = unwrap!(resp_value.dyn_into());
            //the text() method returns a promise
            resp.text()
        })
        .and_then(|text_promise: js_sys::Promise| {
            // Convert this other `Promise` into a rust `Future`.
            wasm_bindgen_futures::JsFuture::from(text_promise)
        })
        .and_then(move |text_jsvalue| {
            let txt_response: String = unwrap!(text_jsvalue.as_string());
            //To change the data in rrc I must use the future `vdom.with_component`
            //it will be executed at the next tick to avoid concurrent data races.
            wasm_bindgen_futures::spawn_local(
                vdom_weak
                    .with_component({
                        move |root| {
                            console::log_1(&JsValue::from_str(&format!(
                                "vdom.with_component: {}  ",
                                ""
                            )));
                            let rrc = root.unwrap_mut::<RootRenderingComponent>();

                            //and now at the end of the fetch Odyssey
                            //call the reference to the function passed as parameter
                            //The txt_response is captured by the Closure.
                            //This capture thing is so invisible and non intuitive.
                            //This is a catastrophe for readability and encapsulation.
                            //So non intuitive and non expressive. Where are good old parameters?

                            call_function_after_fetch(rrc, txt_response, placeholder);
                        }
                    })
                    .map_err(|_| ()),
            );

            vdom_weak.schedule_render();
            log1("vdom.schedule_render");

            // Send something back to JS as JsValue
            futures::future::ok(JsValue::from_str("ok"))
        });
    // future_to_promise() converts `Future` into `Promise` and schedules it to be executed
    wasm_bindgen_futures::future_to_promise(future);
}
