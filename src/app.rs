use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct ChatArgs<'a> {
    message: &'a str,
}

#[function_component(App)]
pub fn app() -> Html {
    let msg_input_ref = use_node_ref();
    let my_message = use_state(String::new);
    let total_msg = use_state(Vec::<String>::new);

    {
        let total_msg = total_msg.clone();
        let my_message = my_message.clone();
        let my_message2 = my_message.clone();
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    if my_message.is_empty() {
                        return;
                    }

                    let args = to_value(&ChatArgs {
                        message: &my_message,
                    })
                    .unwrap();
                    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
                    let new_msg = invoke("send_message", args).await.as_string().unwrap();
                    let mut total_clone = (*total_msg).clone();
                    total_clone.push(new_msg);
                    total_msg.set(total_clone);
                    my_message.set(String::new());
                });

                || {}
            },
            my_message2,
        );
    }

    let send_message = {
        let my_message = my_message.clone();
        let msg_input_ref = msg_input_ref.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            my_message.set(
                msg_input_ref
                    .cast::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value(),
            );
            msg_input_ref
                .cast::<web_sys::HtmlInputElement>()
                .unwrap()
                .set_value("");
        })
    };

    html! {
        <main class="container">
            {
                total_msg.iter().map(|msg| {
                    html!{<div>{ msg }</div>}
                }).collect::<Html>()
            }
            <form class="row" onsubmit={send_message} autocomplete="off">
                <input id="greet-input" ref={msg_input_ref} placeholder="Enter a message..." />
                <button type="submit">{"Send"}</button>
            </form>
        </main>
    }
}
