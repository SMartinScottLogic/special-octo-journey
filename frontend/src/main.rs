use yew::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;

#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name = invokeHello, catch)]
    pub async fn hello(root: String) -> Result<JsValue, JsValue>;
}

fn main() {
    yew::start_app::<App>();
}

#[function_component(App)]
pub fn app() -> Html {
    let entries = use_state_eq(Payload::default);
    let name = use_state_eq(|| ".".to_string());

    // Execute tauri command via effects.
    // The effect will run every time `name` changes.
    {
        let entries = entries.clone();
        use_effect_with_deps(
            move |name| {
                update_root_dir(entries, name.clone());
                || ()
            },
            (*name).clone(),
        );
    }

    let mut entries = (*entries).clone().entries;
    entries.sort_by(|a, b| a.1.cmp(&b.1));
    let entries = entries.iter().map(|entry| if entry.0 {
        html! {
            <>
            <li style="list-style-type: disclosure-closed;">{ entry.1.clone() }</li>
            <ul></ul>
            </>
        }
    } else {
        html! {
            <li>{ entry.1.clone() }</li>
        }
    }).collect::<Html>();

    html! {
        <div class="container">
        <div class="folder">
            <h2 class={"heading"}>{{ "Test" }}</h2>
            <ul>{entries}</ul>
        </div>
        <div class="file">
            <h2>{{"Pane 2"}}</h2>
        </div>
        </div>
    }
}

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
struct Payload {
    entries: Vec<(bool, String)>,
}

fn update_root_dir(entries: UseStateHandle<Payload>, root: String) {
    spawn_local(async move {
        // This will call our glue code all the way through to the tauri
        // back-end command and return the `Result<String, String>` as
        // `Result<JsValue, JsValue>`.
        match hello(root).await {
            Ok(message) => {
                let payload: Payload = serde_json::from_str(&message.as_string().unwrap()).unwrap();
                entries.set(payload);
            }
            Err(e) => {
                let window = window().unwrap();
                window
                    .alert_with_message(&format!("Error: {:?}", e))
                    .unwrap();
            }
        }
    });
}

