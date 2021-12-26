import * as m from '../pkg'
//import * as m from '../pkg'

//import '../pkg/index';
import { LitElement, html, css } from "lit-element/lit-element.js";
// import "@material/mwc-list";
// import "@material/mwc-list/mwc-list-item";

class WebrsView extends LitElement {
  static get styles() {
    return [
      css`
      canvas {
        display: block;
        width: 97%;
        height: 90%;
        margin: auto;
        display: block;
        position: absolute;
        top: 0;
        bottom: 0;
        left: 0;
        right: 0;
        border: #3460FF 6px solid;
      `,
    ];
  }
  
  constructor() {
    super();
  }
  
  render() {
    return html`
    <canvas id="rustCanvas"></canvas>
    `;
  }
  
  firstUpdated() {
    super.firstUpdated();
    const rust = import('../pkg/index');
    const canvas = this.shadowRoot.getElementById('rustCanvas');
    console.log(canvas)
    canvas.width = window.innerWidth;
    canvas.height =  window.innerHeight;
    const gl = canvas.getContext('webgl2', { antialias: true });
    
    rust.then(m => {
      if (!gl) {  
        alert( 'webgl is broken' );
          return; 
        }
      const throt = 1000.0 / 30.0;
      const myclient = new m.GlClient(canvas);
      const initTime = Date.now();
      let last_draw_time = -1;
      
      function render() {
          window.requestAnimationFrame(render);
          const currTime = Date.now();
          
          if (currTime >= last_draw_time + throt ) {
              last_draw_time = currTime;

              gl.viewport(0, 0, canvas.width, canvas.height);

              let timePassed = currTime - initTime;
                // myclient.update(timePassed, window.innerHeight, window.innerWidth);
              myclient.render_client();
          }
      }
      //myclient.renderClient()
      render();
    }
    ).catch(err => { console.log("Rust Error "+ err) });
  }
}
  customElements.define("webrs-view", WebrsView);
