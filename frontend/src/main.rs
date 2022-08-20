use bounce::{use_atom, use_atom_setter, Atom, BounceRoot};
use std::cmp::Ordering;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;
use yew::prelude::*;

#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name = invokeReadDir, catch)]
    pub async fn js_read_dir(root: String) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(js_name = invokeReadFile, catch)]
    pub async fn js_read_file(filename: String) -> Result<JsValue, JsValue>;
}

fn main() {
    yew::start_app::<App>();
}

#[function_component(Container)]
pub fn container() -> Html {
    let filename = use_atom::<FileView>();
    let content = use_state_eq(String::default);

    {
        let content = content.clone();
        use_effect_with_deps(
            move |filename| {
                if !filename.is_empty() {
                    read_file(filename, move |c| content.set(c));
                }
                || ()
            },
            filename.filename.clone(),
        );
    }
    html! {
        <div class="container">
        <div class="folder">
            <h2 class={"heading"}>{{ "Test" }}</h2>
            <ul><Folder name="." root="." open=true /></ul>
        </div>
        <div class="file">
            <h2 class={"heading"}>{{"Pane 2"}}</h2>
            {{ filename.filename.clone() }}
        <pre>
            {{ (*content).clone() }}
        </pre>
        </div>
        </div>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BounceRoot>
        <Container />
        </BounceRoot>
    }
}

#[derive(Properties, PartialEq)]
pub struct FileProps {
    pub root: String,
    pub name: String,
}

#[derive(Properties, PartialEq)]
pub struct FolderProps {
    pub root: String,
    pub name: String,
    pub open: bool,
}

#[derive(PartialEq, Atom, Default)]
pub struct FileView {
    filename: String,
}
impl From<String> for FileView {
    fn from(filename: String) -> Self {
        Self { filename }
    }
}

#[function_component(File)]
pub fn file(props: &FileProps) -> Html {
    let set_filename = use_atom_setter::<FileView>();

    let file_click = {
        let fullname = format!("{}/{}", props.root, props.name);
        Callback::from(move |_e: MouseEvent| {
            set_filename(fullname.clone().into());
        })
    };
    html! {
        <li onclick={file_click}>{ props.name.clone() }</li>
    }
}

#[function_component(Folder)]
pub fn folder(props: &FolderProps) -> Html {
    let entries = use_state_eq(Payload::default);
    let open = use_state_eq(|| props.open);

    // The effect will run every time `open` changes.
    {
        let entries = entries.clone();
        let root = props.root.clone();
        use_effect_with_deps(
            move |open| {
                if *open {
                    read_dir(entries, root);
                } else {
                    entries.set(Payload::default());
                }
                || ()
            },
            *open,
        );
    }

    let mut entries = (*entries).clone().entries;
    entries.sort_by(|a, b| match (a.0, b.0) {
        (true, true) | (false, false) => a.1.cmp(&b.1),
        (true, false) => Ordering::Less,
        (false, true) => Ordering::Greater,
    });
    let entries = entries
        .iter()
        .map(|entry| {
            if entry.0 {
                html! {
                    <Folder name={entry.1.clone()} root={format!("{}/{}", props.root, entry.1)} open=false />
                }
            } else {
                html! {
                    <File root={props.root.clone()} name={ entry.1.clone()}/>
                }
            }
        })
        .collect::<Html>();

    let class = if *open {
        "bi bi-chevron-down"
    } else {
        "bi bi-chevron-right"
    };

    let toggle_open = {
        let open = open;
        Callback::from(move |_| {
            open.set(!*open);
        })
    };
    html! {
        <>
        if props.name != "." {
            <li class={{class}} onclick={toggle_open}>{{ props.name.clone() }}</li>
            <ul>{entries}</ul>
        } else {
            {entries}
        }
        </>
    }
}

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
struct Payload {
    entries: Vec<(bool, String)>,
}

fn read_dir(entries: UseStateHandle<Payload>, root: String) {
    spawn_local(async move {
        // This will call our glue code all the way through to the tauri
        // back-end command and return the `Result<String, String>` as
        // `Result<JsValue, JsValue>`.
        match js_read_dir(root).await {
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

fn read_file<F>(filename: &str, set_content: F)
where
    F: FnOnce(String) + 'static,
{
    let filename = filename.to_string();
    spawn_local(async move {
        match js_read_file(filename).await {
            Ok(file_content) => {
                let file_content = file_content.as_string().unwrap();

                set_content(file_content);
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
