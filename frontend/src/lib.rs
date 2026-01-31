use yew::prelude::*;
use wasm_bindgen::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class=\"container\">
            <h1>{ \"Evo4mixer\" }</h1>
            <p>{ \"ALSA-based Audient EVO 4 Control\" }</p>
        </div>
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    console_error_panic_hook::set_once();
    web_sys::console::log_1(&\"Starting Yew app...\".into());
    yew::Renderer::<App>::new().render();
    web_sys::console::log_1(&\"Yew renderer initialized\".into());
}
