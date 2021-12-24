
```
npm install @material/mwc-button @material/mwc-icon-button @material/mwc-list @material/mwc-top-app-bar-fixed lit-element --save-dev
```

Instructions
```
cargo build
npm i
npm run build
npm run serve
```
Instead of 
    ```
    let window = window().unwrap();
    et document = window.document().unwrap();
    let canvas = document.get_element_by_id("rustCanvas").unwrap();
    let canvas:HtmlCanvasElement = c.dyn_into::<HtmlCanvasElement>()?;
    let context = canvas.get_context("webgl")?
    ```
We Directly Pass in the canvas and rust can get access to it instead of lookin for it