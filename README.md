# Wasm-gl
Using WASM and Rust, with opengl for 3d rendering on web.
https://docs.rs/web-sys/0.3.50/web_sys/

### Requirements
- webpack  
- wasm  
- npm

### Getting Started
1.
```
wasm-pack build

./node_modules/.bin/webpack
```

### Basic Rust Program
``` rust

fn foo() {
    let x = format!("hallo welt");
    web_sys::console::log_1(&x.into());
}

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

// fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    let rc = format!("Hello, {}!", name);
    unsafe {alert(&rc)};
}
```