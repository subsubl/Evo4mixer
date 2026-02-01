use yew::prelude::*;
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GainArgs {
    channel: u8,
    gain: u8,
}

#[derive(Serialize, Deserialize)]
struct PhantomArgs {
    channel: u8,
    enabled: bool,
}

#[derive(Serialize, Deserialize)]
struct VolumeArgs {
    volume: u8,
}

#[function_component(App)]
fn app() -> Html {
    let set_gain = |channel: u8, gain: u8| {
        wasm_bindgen_futures::spawn_local(async move {
            let args = serde_wasm_bindgen::to_value(&GainArgs { channel, gain }).unwrap();
            invoke("set_gain", args).await;
        });
    };

    let toggle_phantom = |channel: u8, enabled: bool| {
        wasm_bindgen_futures::spawn_local(async move {
            let args = serde_wasm_bindgen::to_value(&PhantomArgs { channel, enabled }).unwrap();
            invoke("toggle_phantom", args).await;
        });
    };

    let set_master_volume = |volume: u8| {
        wasm_bindgen_futures::spawn_local(async move {
            let args = serde_wasm_bindgen::to_value(&VolumeArgs { volume }).unwrap();
            invoke("set_master_volume", args).await;
        });
    };

    html! {
        <div class="min-h-screen bg-neutral-900 text-neutral-100 p-6 font-mono selection:bg-neutral-700">
            <header class="border-b border-neutral-800 pb-4 mb-8 flex justify-between items-end">
                <div>
                    <h1 class="text-2xl font-black tracking-tighter text-white">EVO4MIXER</h1>
                    <p class="text-[10px] text-neutral-500 uppercase tracking-widest mt-1">Linux Hardware Controller</p>
                </div>
                <div class="text-[10px] text-neutral-600 text-right">
                    <div>BUILD: 20260201.01</div>
                    <div>STATUS: WIP / RUSB_ACTIVE</div>
                </div>
            </header>
            
            <main class="grid grid-cols-1 lg:grid-cols-3 gap-12">
                <!-- INPUT SECTION -->
                <section class="lg:col-span-2 space-y-6">
                    <h2 class="text-xs font-bold uppercase tracking-widest text-neutral-500 border-l-2 border-neutral-700 pl-2">Analog Inputs</h2>
                    <div class="flex flex-wrap gap-6">
                        { for (1..=2).map(|ch| html! {
                            <div class="bg-neutral-850 border border-neutral-800 p-6 rounded-sm w-48 shadow-xl">
                                <div class="flex justify-between items-center mb-6">
                                    <span class="text-xs font-bold text-neutral-400">INPUT {ch}</span>
                                    <div class="w-2 h-2 rounded-full bg-green-500/50 shadow-[0_0_8px_rgba(34,197,94,0.3)]"></div>
                                </div>
                                
                                <div class="space-y-8">
                                    <div class="group relative h-48 bg-neutral-900/50 rounded-sm border border-neutral-800 flex flex-col justify-end p-1">
                                        <div class="absolute inset-0 flex flex-col justify-between p-2 pointer-events-none">
                                            { for (0..5).map(|i| html! { <div class="border-t border-neutral-800 w-full h-0"></div> }) }
                                        </div>
                                        <input 
                                            type="range" 
                                            min="0" max="49" 
                                            class="w-full h-full appearance-none bg-transparent cursor-pointer vertical-range"
                                            oninput={move |e: InputEvent| {
                                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                                set_gain(ch, input.value().parse().unwrap_or(0));
                                            }}
                                        />
                                    </div>

                                    <div class="grid grid-cols-2 gap-2">
                                        <button 
                                            onclick={move |_| toggle_phantom(ch, true)}
                                            class="py-2 text-[10px] font-bold border border-neutral-700 hover:bg-red-900/20 hover:border-red-900/50 transition-all rounded-sm"
                                        >
                                            48V
                                        </button>
                                        <button 
                                            class="py-2 text-[10px] font-bold border border-neutral-700 hover:bg-neutral-700 transition-all rounded-sm"
                                        >
                                            LINK
                                        </button>
                                    </div>
                                </div>
                            </div>
                        })}
                    </div>
                </section>

                <!-- MASTER SECTION -->
                <section class="space-y-6">
                    <h2 class="text-xs font-bold uppercase tracking-widest text-neutral-500 border-l-2 border-neutral-700 pl-2">Monitoring</h2>
                    <div class="bg-neutral-850 border border-neutral-800 p-6 rounded-sm shadow-xl">
                        <div class="flex justify-between items-center mb-6">
                            <span class="text-xs font-bold text-neutral-400">MAIN OUT</span>
                            <span class="text-[10px] text-neutral-600 italic">STEREO</span>
                        </div>
                        
                        <div class="space-y-8">
                            <div class="h-48 bg-neutral-900/50 rounded-sm border border-neutral-800 flex p-1 gap-1">
                                <div class="flex-1 bg-neutral-900 rounded-sm overflow-hidden flex flex-col justify-end">
                                    <div class="h-[20%] bg-gradient-to-t from-blue-500 to-cyan-400 w-full"></div>
                                </div>
                                <div class="flex-1 bg-neutral-900 rounded-sm overflow-hidden flex flex-col justify-end">
                                    <div class="h-[18%] bg-gradient-to-t from-blue-500 to-cyan-400 w-full"></div>
                                </div>
                            </div>

                            <div class="space-y-2">
                                <input 
                                    type="range" 
                                    min="0" max="255"
                                    class="w-full h-1 appearance-none bg-neutral-800 rounded-full accent-white"
                                    oninput={move |e: InputEvent| {
                                        let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                        set_master_volume(input.value().parse().unwrap_or(0));
                                    }}
                                />
                                <div class="flex justify-between text-[9px] text-neutral-600 uppercase">
                                    <span>Mute</span>
                                    <span>Master Vol</span>
                                    <span>0dB</span>
                                </div>
                            </div>
                        </div>
                    </div>
                </section>
            </main>

            <style>
                { "
                .vertical-range {
                    writing-mode: bt-lr; /* IE */
                    appearance: slider-vertical; /* Webkit */
                    width: 100%;
                    height: 100%;
                }
                .bg-neutral-850 { background-color: #1e1e1e; }
                " }
            </style>
        </div>
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
