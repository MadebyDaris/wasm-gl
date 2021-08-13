# Wasm-gl
Using WASM and Rust, with opengl for 3d rendering on web.
https://docs.rs/web-sys/0.3.50/web_sys/

Prior Knowledge Needed
### What Are We Going to Use
- webpack  
- wasm  
- npm

### Getting Started
```
1. npm install

2. wasm-pack build

3. ./node_modules/.bin/webpack

4. npm run build

5. npm run serve
```

### Basic Rust WASM Program
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

For more Resources Look at
 - Mozilla
 https://developer.mozilla.org/en-US/docs/Web/API/WebGL_API