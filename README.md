
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

# Model View and Projection Matrice

The model, view and projection matrices are three separate matrices. Model maps from an object's local coordinate space into world space, view from world space to camera space, projection from camera to screen.

If you compose all three, you can use the one result to map all the way from object space to screen space, making you able to work out what you need to pass on to the next stage of a programmable pipeline from the incoming vertex positions.

In the fixed functionality pipelines of old, you'd apply model and view together, then work out lighting using another result derived from them (with some fixes so that e.g. normals are still unit length even if you've applied some scaling to the object), then apply projection. You can see that reflected in OpenGL, which never separates the model and view matrices — keeping them as a single modelview matrix stack. You therefore also sometimes see that reflected in shaders.

So: the composed model view projection matrix is often used by shaders to map from the vertices you loaded for each model to the screen. It's not required, there are lots of ways of achieving the same thing, it's just usual because it allows all possible linear transforms. Because of that, a lesser composed version of it was also the norm in ye olde fixed pipeline world.