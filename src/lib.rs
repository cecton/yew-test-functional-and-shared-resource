#![cfg(target_arch = "wasm32")]

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_functional::{function_component, use_state};
use std::time::Duration;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(message: &str);
}

#[function_component(Test)]
pub fn test() -> Html {
    let starting = use_state(|| true);
    let res = use_state(|| 0_u32);

    if *starting {
        let res = res.clone();
        starting.set(false);
        spawn_local(async move {
            loop {
                log(&res.to_string());
                let _ = futures_timer::Delay::new(Duration::from_secs(1)).await;
            }
        });
    }

    let label = res.to_string();

    html! {
        <div onclick=Callback::from(move |_| res.set(*res + 1))>{ label }</div>
    }
}

#[wasm_bindgen(start)]
pub fn run_app() -> Result<(), JsValue> {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    log("Hello World!");

    yew::start_app::<Test>();

    Ok(())
}
