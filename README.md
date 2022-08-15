# special-octo-journey

# Building
* Recommend use of [sccache](https://github.com/mozilla/sccache)

# Re-creation
## Frontend
### New package
cargo new --bin frontend
cd frontend
mkdir public

### Install build setup
rustup target add wasm32-unknown-unknown
cargo install trunk
cargo install wasm-bindgen-cli

### index.html
```html
<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8" />
    <title>Tauri Yew Demo App</title>

    <link data-trunk rel="css" href="/public/main.css"/>
  </head>
  <body></body>
</html>
```

### public/main.css
```css
body {
    margin: 20px;
    background: #2d2d2d;
    color: white;
}

.heading {
    text-align: center;
}
```

### Test
```
trunk build
trunk serve
```

Load: http://localhost:8080

### Add Yew to dependencies
```
$ cargo add yew
```

### Replace src/main.rs
```rs
use yew::prelude::*;

fn main() {
    yew::start_app::<App>();
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div>
            <h2 class={"heading"}>{"Hello, World!"}</h2>
        </div>
    }
}
```

## Tauri
See [Official docs](https://tauri.app/v1/guides/getting-started/prerequisites/#installing).

### Install
```
cargo install tauri-cli
```

### Setup project
```
cargo tauri init
✔ What is your app name? · special-octo-journey
✔ What should the window title be? · Special Octo Journey
✔ Where are your web assets (HTML/CSS/JS) located, relative to the "<current dir>/src-tauri/tauri.conf.json" file that will be created? · ../frontend/dist
✔ What is the url of your dev server? · http://localhost:8080
```

### Setup for yew hot-build and reload
```
vim src-tauri/tauri.conf.json
```
Replace `build` section with:
```
"build": {
    "distDir": "../frontend/dist",
    "devPath": "http://localhost:8080",
    "beforeDevCommand": "cd frontend && trunk serve",
    "beforeBuildCommand": "cd frontend && trunk build",
    "withGlobalTauri": true
},
```

### Start Tauri Dev Server
```
cargo tauri dev
```

### Add PoC Tauri command:
Open `src-tauri/src/main.rs`, add (below `main()` function):
```
#[tauri::command]
fn hello(name: &str) -> Result<String, String> {
  // This is a very simplistic example but it shows how to return a Result
  // and use it in the front-end.
  if name.contains(' ') {
    Err("Name should not contain spaces".to_string())
  } else {
    Ok(format!("Hello, {}", name))
  }
}
```

Add the following in `main()`, before `.run(...)`:
```
    .invoke_handler(tauri::generate_handler![hello])
```

### Add Javascript glue code
Open `frontend/public/glue.js`, add the following:
```
const invoke = window.__TAURI__.invoke

export async function invokeHello(name) {
    return await invoke("hello", {name: name});
}
```

### Add glue code to Rust
Open `frontend/src/main.rs`, add:
```
#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name = invokeHello, catch)]
    pub async fn hello(name: String) -> Result<JsValue, JsValue>;
}
```

### Add required new Rust dependencies
```
cargo add wasm-bindgen
cargo add wasm-bindgen-futures
cargo add web-sys
cargo add js-sys
```

Update `frontend/src/main.rs` with new imports:
```
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;
```

### Replace `app` function component
```
#[function_component(App)]
pub fn app() -> Html {
    let welcome = use_state_eq(|| "".to_string());
    let name = use_state_eq(|| "World".to_string());

    // Execute tauri command via effects.
    // The effect will run every time `name` changes.
    {
        let welcome = welcome.clone();
        use_effect_with_deps(
            move |name| {
                update_welcome_message(welcome, name.clone());
                || ()
            },
            (*name).clone(),
        );
    }

    let message = (*welcome).clone();

    html! {
        <div>
            <h2 class={"heading"}>{message}</h2>
        </div>
    }
}

fn update_welcome_message(welcome: UseStateHandle<String>, name: String) {
    spawn_local(async move {
        // This will call our glue code all the way through to the tauri
        // back-end command and return the `Result<String, String>` as
        // `Result<JsValue, JsValue>`.
        match hello(name).await {
            Ok(message) => {
                welcome.set(message.as_string().unwrap());
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
```

# Application
```
cd frontend/
cargo add serde serde_json
```