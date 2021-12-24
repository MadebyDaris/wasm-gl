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
        margin-top: 75px;
        margin-left: 10px;
        margin-right:10px;
        display: relative;
        // background-color: white
        box-sizing: padding-box;
        border:black 3px solid;
      }
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
    canvas.width =1000;
    canvas.height =  800;
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

              gl.viewport(0, 0, window.innerWidth, window.innerHeight);

              let timePassed = currTime - initTime;
                // myclient.update(timePassed, window.innerHeight, window.innerWidth);
              myclient.renderClient();
          }
      }
      //myclient.renderClient()
      render();
    }
    ).catch(err => { console.log(err) });
  }
}
  customElements.define("webrs-view", WebrsView);
