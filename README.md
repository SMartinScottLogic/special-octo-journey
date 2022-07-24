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
See [Official docs](https://tauri.studio/docs/getting-started/beginning-tutorial#alternatively-install-tauri-cli-as-a-cargo-subcommand).
